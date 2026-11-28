use crate::OreqlDiagnostic;
use crate::{OreqlDiagnosticKind, OreqlString, OreqlSyntaxKind};

use super::super::DialectParser;
use super::ParserContext;
use super::default_tokenizer_config::*;
use oreql_utils::{BitSet128, TrieResult};

/// Result of peeking a token, holds diagnostics for replay
pub struct PeekResult {
  pub kind: OreqlSyntaxKind,
  pub string: OreqlString,
  pub(crate) pending_diagnostics: Vec<OreqlDiagnostic>,
}

/// Result of peeking past trivia to the next significant token
#[allow(dead_code)]
pub struct PeekSignificantResult {
  /// The significant (non-trivia) token
  pub kind: OreqlSyntaxKind,
  pub string: OreqlString,
  /// Trivia tokens scanned before the significant token
  pub leading_trivia: Vec<(OreqlSyntaxKind, OreqlString)>,
  pub(crate) pending_diagnostics: Vec<OreqlDiagnostic>,
}

impl<D: DialectParser> ParserContext<'_, D> {
  /// Peek the next token without consuming it
  #[inline]
  pub fn peek_next_token(&mut self, config: &TokenizeConfig) -> PeekResult {
    let saved_offset = self.current_byte_offset;
    let diagnostics_before = self.diagnostics.len();
    let (kind, string) = self.scan_next_token(config);
    let pending_diagnostics = self.diagnostics.drain(diagnostics_before..).collect();
    self.current_byte_offset = saved_offset;
    PeekResult {
      kind,
      string,
      pending_diagnostics,
    }
  }

  /// Returns the significant token plus any leading trivia scanned
  #[inline]
  pub fn peek_next_significant_token(&mut self, config: &TokenizeConfig) -> PeekSignificantResult {
    let saved_offset = self.current_byte_offset;
    let diagnostics_before = self.diagnostics.len();
    let mut leading_trivia = Vec::new();

    loop {
      let (kind, string) = self.scan_next_token(config);
      if kind.is_trivia() {
        leading_trivia.push((kind, string));
        continue;
      }
      let pending_diagnostics = self.diagnostics.drain(diagnostics_before..).collect();
      self.current_byte_offset = saved_offset;
      return PeekSignificantResult {
        kind,
        string,
        leading_trivia,
        pending_diagnostics,
      };
    }
  }

  /// Raw scan: advances offset, emits diagnostics, returns token
  ///
  /// INVARIANT: This does NOT push to builder
  #[inline]
  pub(in crate::dialects::parse::context) fn scan_next_token(
    &mut self,
    config: &TokenizeConfig,
  ) -> (OreqlSyntaxKind, OreqlString) {
    if self.is_at_end() {
      let offset = self.current_byte_offset as u32;
      return (
        OreqlSyntaxKind::Eof,
        self.string_pool.create_string(offset, offset),
      );
    }

    let byte = self.peek_byte().unwrap();

    /* Trivia */

    if byte == b' ' || byte == b'\t' {
      return self.tokenize_whitespace();
    }

    if byte == b'\n' || byte == b'\r' {
      return self.tokenize_newline();
    }

    /* Comments (inline, block, hint, jinja) */

    if byte == b'-' && self.peek_byte_at(1) == Some(b'-') {
      return D::tokenize_inline_comment(self);
    }

    if byte == b'/' && self.peek_byte_at(1) == Some(b'*') {
      return D::tokenize_block_comment(self, b"/*", b"*/");
    }

    if byte == b'{' && self.peek_byte_at(1) == Some(b'#') {
      return D::tokenize_block_comment(self, b"{#", b"#}");
    }

    /* String literals */

    if byte == b'\'' {
      return D::tokenize_string_lit(self);
    }

    /* Quoted identifiers ("name", `name`, [name]) */

    if byte == b'"' || byte == b'`' || byte == b'[' {
      return D::tokenize_quoted_identifier(self);
    }

    /* K::Number literals (including 0x hex, 0b binary) */

    if byte.is_ascii_digit() {
      return D::tokenize_number_lit(self);
    }

    /* Prefixed strings (b'...', x'...', N'...', etc) then identifier/keyword */

    if byte.is_ascii_alphabetic() {
      let saved = self.current_byte_offset;
      if let Some(result) = D::tokenize_prefixed_string(self) {
        return result;
      }
      /* Restore offset in case dialect override partially consumed */
      self.current_byte_offset = saved;
      return D::tokenize_identifier_or_keyword(self);
    }

    if byte == b'_' {
      return D::tokenize_identifier_or_keyword(self);
    }

    /* Operators (multi-char first, then single-byte punctuation) */

    if let Some(result) = D::tokenize_operator(self) {
      return result;
    }

    if let Some(kind) = config.single_byte_tokens.get(byte) {
      let start = self.current_byte_offset as u32;
      self.advance_byte();
      return (
        kind,
        self
          .string_pool
          .create_string(start, self.current_byte_offset as u32),
      );
    }

    /* Variable (unrecognized ASCII non-whitespace runs) */

    if byte.is_ascii() && !byte.is_ascii_whitespace() {
      return D::tokenize_variable(self);
    }

    /* Unknown character (non-ASCII or unrecognized whitespace) */

    let start = self.current_byte_offset as u32;
    self.advance_char();
    (
      OreqlSyntaxKind::Error,
      self
        .string_pool
        .create_string(start, self.current_byte_offset as u32),
    )
  }

  /// Scan whitespace (spaces, tabs)
  ///
  /// INVARIANT: current byte must be ' ' or '\t'
  #[inline]
  pub fn tokenize_whitespace(&mut self) -> (OreqlSyntaxKind, OreqlString) {
    debug_assert!(
      matches!(self.peek_byte(), Some(b' ' | b'\t')),
      "whitespace() called but current byte is not whitespace"
    );

    let start = self.current_byte_offset as u32;
    while let Some(byte) = self.peek_byte() {
      if byte == b' ' || byte == b'\t' {
        self.advance_byte();
      } else {
        break;
      }
    }
    (
      OreqlSyntaxKind::Whitespace,
      self
        .string_pool
        .create_string(start, self.current_byte_offset as u32),
    )
  }

  /// Scan newline (\n or \r\n)
  ///
  /// INVARIANT: current byte must be '\n' or '\r'
  #[inline]
  pub fn tokenize_newline(&mut self) -> (OreqlSyntaxKind, OreqlString) {
    debug_assert!(
      matches!(self.peek_byte(), Some(b'\n' | b'\r')),
      "newline() called but current byte is not a newline"
    );
    let start = self.current_byte_offset as u32;
    if self.peek_byte() == Some(b'\r') {
      self.advance_byte();
      if self.peek_byte() == Some(b'\n') {
        self.advance_byte();
      }
    } else {
      self.advance_byte();
    }
    (
      OreqlSyntaxKind::Newline,
      self
        .string_pool
        .create_string(start, self.current_byte_offset as u32),
    )
  }

  /// Scan a numeric literal (integer, decimal, scientific)
  ///
  /// INVARIANT: current byte must be an ASCII digit
  #[inline]
  pub fn tokenize_number_lit(&mut self) -> (OreqlSyntaxKind, OreqlString) {
    debug_assert!(
      self.peek_byte().is_some_and(|byte| byte.is_ascii_digit()),
      "number() called but current byte is not a digit"
    );
    let start = self.current_byte_offset as u32;

    /* 0x hex literal */
    if self.peek_byte() == Some(b'0') {
      let next = self.peek_byte_at(1);
      if next == Some(b'x') || next == Some(b'X') {
        self.advance_bytes(2);
        while self
          .peek_byte()
          .is_some_and(|byte| byte.is_ascii_hexdigit())
        {
          self.advance_byte();
        }
        /* No valid hex digits: fall back to identifier (e.g. 0xZZ) */
        let kind = if self.current_byte_offset as u32 == start + 2 {
          OreqlSyntaxKind::Identifier
        } else {
          OreqlSyntaxKind::HexString
        };
        return (
          kind,
          self
            .string_pool
            .create_string(start, self.current_byte_offset as u32),
        );
      }

      /* 0b binary literal */
      if next == Some(b'b') || next == Some(b'B') {
        self.advance_bytes(2);
        while self
          .peek_byte()
          .is_some_and(|byte| byte == b'0' || byte == b'1')
        {
          self.advance_byte();
        }
        /* No valid binary digits: fall back to identifier (e.g. 0bXX) */
        let kind = if self.current_byte_offset as u32 == start + 2 {
          OreqlSyntaxKind::Identifier
        } else {
          OreqlSyntaxKind::BitString
        };
        return (
          kind,
          self
            .string_pool
            .create_string(start, self.current_byte_offset as u32),
        );
      }
    }

    /* Decimal integer part */
    while self
      .peek_byte()
      .is_some_and(|byte| byte.is_ascii_digit() || byte == b'_')
    {
      self.advance_byte();
    }

    /* Decimal point */
    if self.peek_byte() == Some(b'.')
      && self
        .peek_byte_at(1)
        .is_some_and(|byte| byte.is_ascii_digit())
    {
      self.advance_byte();
      while self
        .peek_byte()
        .is_some_and(|byte| byte.is_ascii_digit() || byte == b'_')
      {
        self.advance_byte();
      }
    }

    /* Scientific notation */
    if self
      .peek_byte()
      .is_some_and(|byte| byte == b'e' || byte == b'E')
    {
      self.advance_byte();
      if self
        .peek_byte()
        .is_some_and(|byte| byte == b'+' || byte == b'-')
      {
        self.advance_byte();
      }
      if !self.peek_byte().is_some_and(|byte| byte.is_ascii_digit()) {
        self.emit_error(
          OreqlDiagnosticKind::ExpectedDigitsAfterExponent,
          start,
          self.current_byte_offset as u32,
        );
      }
      while self.peek_byte().is_some_and(|byte| byte.is_ascii_digit()) {
        self.advance_byte();
      }
    }

    (
      OreqlSyntaxKind::Number,
      self
        .string_pool
        .create_string(start, self.current_byte_offset as u32),
    )
  }

  /// Scan an identifier or keyword
  ///
  /// INVARIANT: current byte must be ASCII alphabetic or '_'
  #[inline]
  pub fn tokenize_identifier_or_keyword(
    &mut self,
    config: &TokenizeIdentifierConfig,
  ) -> (OreqlSyntaxKind, OreqlString) {
    let keywords = config.keywords;
    debug_assert!(
      self
        .peek_byte()
        .is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_'),
      "identifier_or_keyword() called but current byte is not alphabetic or '_'"
    );
    let start = self.current_byte_offset as u32;

    // Scan the first word
    while self
      .peek_byte()
      .is_some_and(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
      self.advance_byte();
    }

    let first_word_end = self.current_byte_offset as u32;
    let first_word = &self.source_bytes()[(start as usize)..(first_word_end as usize)];

    // Build uppercase for keyword lookup
    let mut upper_buf = [0u8; 128];
    if first_word.len() > upper_buf.len() {
      return (
        OreqlSyntaxKind::Identifier,
        self.string_pool.create_string(start, first_word_end),
      );
    }

    let mut upper_len = first_word.len();
    for (index, &byte) in first_word.iter().enumerate() {
      upper_buf[index] = byte.to_ascii_uppercase();
    }

    // Try compound keywords: if first word is a prefix, try extending with spaces + more words (e.g. GROUP BY, ORDER BY, PRIMARY KEY)
    let mut best_kind = keywords.get(&upper_buf[..upper_len]);
    let mut best_end = first_word_end;

    loop {
      self.skip_trivia(
        config.inline_comment_starts,
        config.block_comment_delimiters,
      );

      // Must have a next word
      if !self
        .peek_byte()
        .is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
      {
        break;
      }

      // Append normalized space
      if upper_len >= upper_buf.len() {
        break;
      }
      upper_buf[upper_len] = b' ';
      upper_len += 1;

      // Scan next word
      let word_start = self.current_byte_offset;
      while self
        .peek_byte()
        .is_some_and(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
      {
        self.advance_byte();
      }

      let word = &self.source_bytes()[word_start..self.current_byte_offset];
      if upper_len + word.len() > upper_buf.len() {
        break;
      }
      for (index, &byte) in word.iter().enumerate() {
        upper_buf[upper_len + index] = byte.to_ascii_uppercase();
      }
      upper_len += word.len();

      // Check if the extended text is a keyword or prefix
      match keywords.lookup(&upper_buf[..upper_len]) {
        TrieResult::Found(kind) => {
          best_kind = Some(kind);
          best_end = self.current_byte_offset as u32;
        }
        TrieResult::Prefix => {
          // Keep going, might be a longer compound keyword
        }
        TrieResult::NotFound => break,
      }
    }

    // Restore to best match position
    self.current_byte_offset = best_end as usize;
    let kind = best_kind.unwrap_or(OreqlSyntaxKind::Identifier);

    (kind, self.string_pool.create_string(start, best_end))
  }

  /// Scan a quoted string literal
  ///
  /// INVARIANT: current bytes must match an opening quote from config
  #[inline]
  pub fn tokenize_string_lit(
    &mut self,
    config: &TokenizeStringConfig,
  ) -> (OreqlSyntaxKind, OreqlString) {
    let start = self.current_byte_offset as u32;
    let source = self.source_bytes();
    let remaining = &source[(start as usize)..];

    for &(open, close) in config.quotes.iter() {
      if remaining.starts_with(open.as_bytes()) {
        self.advance_bytes(open.len());
        self.scan_until_delimiter(
          close.as_bytes(),
          config.escape_bytes,
          OreqlDiagnosticKind::UnterminatedStringLiteral {
            expected: close.to_string(),
          },
        );
        return (
          OreqlSyntaxKind::String,
          self
            .string_pool
            .create_string(start, self.current_byte_offset as u32),
        );
      }
    }

    debug_assert!(
      false,
      "tokenize_string_lit() called but current bytes do not match any quote pattern"
    );

    self.advance_byte();
    (
      OreqlSyntaxKind::Error,
      self
        .string_pool
        .create_string(start, self.current_byte_offset as u32),
    )
  }

  /// Scan a quoted identifier ("name", `name`, [name])
  ///
  /// INVARIANT: current bytes must match an opening identifier quote from config
  #[inline]
  pub fn tokenize_quoted_identifier(
    &mut self,
    config: &TokenizeQuotedIdentifierConfig,
  ) -> (OreqlSyntaxKind, OreqlString) {
    let start = self.current_byte_offset as u32;
    let source = self.source_bytes();
    let remaining = &source[(start as usize)..];

    for &(open, close) in config.quotes.iter() {
      if remaining.starts_with(open.as_bytes()) {
        self.advance_bytes(open.len());
        self.scan_until_delimiter(
          close.as_bytes(),
          config.escape_bytes,
          OreqlDiagnosticKind::UnterminatedQuotedIdentifier {
            expected: close.to_string(),
          },
        );
        return (
          OreqlSyntaxKind::Identifier,
          self
            .string_pool
            .create_string(start, self.current_byte_offset as u32),
        );
      }
    }

    debug_assert!(
      false,
      "tokenize_quoted_identifier() called but current bytes do not match any identifier quote pattern"
    );
    self.advance_byte();
    (
      OreqlSyntaxKind::Error,
      self
        .string_pool
        .create_string(start, self.current_byte_offset as u32),
    )
  }

  /// Scan an inline comment (-- to end of line)
  ///
  /// INVARIANT: current bytes must match a start pattern from config
  #[inline]
  pub fn tokenize_inline_comment(
    &mut self,
    start_patterns: &InlineCommentStartsConfig,
  ) -> (OreqlSyntaxKind, OreqlString) {
    let start = self.current_byte_offset as u32;
    let source = self.source_bytes();
    let remaining = &source[(start as usize)..];
    debug_assert!(
      start_patterns
        .iter()
        .any(|pattern| remaining.starts_with(pattern.as_bytes())),
      "tokenize_inline_comment() called but current bytes do not match any start pattern"
    );

    for &pattern in start_patterns.iter() {
      if remaining.starts_with(pattern.as_bytes()) {
        self.advance_bytes(pattern.len());
        self.scan_past_inline_comment();
        return (
          OreqlSyntaxKind::LineComment,
          self
            .string_pool
            .create_string(start, self.current_byte_offset as u32),
        );
      }
    }

    self.advance_byte();
    (
      OreqlSyntaxKind::Error,
      self
        .string_pool
        .create_string(start, self.current_byte_offset as u32),
    )
  }

  /// Scan a block comment without nesting
  ///
  /// INVARIANT: current bytes must match `open`
  #[inline]
  pub fn tokenize_flat_block_comment(
    &mut self,
    open: &[u8],
    close: &[u8],
  ) -> (OreqlSyntaxKind, OreqlString) {
    debug_assert!(
      self.source_bytes()[self.current_byte_offset..].starts_with(open),
      "tokenize_flat_block_comment() called but current bytes do not match open delimiter"
    );
    let start = self.current_byte_offset as u32;
    self.advance_bytes(open.len());

    loop {
      if self.is_at_end() {
        self.emit_error(
          OreqlDiagnosticKind::UnterminatedBlockComment {
            expected: String::from_utf8_lossy(close).into_owned(),
          },
          start,
          self.current_byte_offset as u32,
        );
        break;
      }
      let remaining = &self.source_bytes()[self.current_byte_offset..];
      if remaining.starts_with(close) {
        self.advance_bytes(close.len());
        break;
      }
      self.advance_byte();
    }

    (
      OreqlSyntaxKind::BlockComment,
      self
        .string_pool
        .create_string(start, self.current_byte_offset as u32),
    )
  }

  /// Scan a block comment with nesting support
  ///
  /// INVARIANT: current bytes must match `open`
  #[inline]
  pub fn tokenize_nested_block_comment(
    &mut self,
    open: &[u8],
    close: &[u8],
  ) -> (OreqlSyntaxKind, OreqlString) {
    debug_assert!(
      self.source_bytes()[self.current_byte_offset..].starts_with(open),
      "tokenize_nested_block_comment() called but current bytes do not match open delimiter"
    );
    let start = self.current_byte_offset as u32;
    self.advance_bytes(open.len());
    let terminated = self.scan_past_nested_block_comment(open, close);
    if !terminated {
      self.emit_error(
        OreqlDiagnosticKind::UnterminatedBlockComment {
          expected: String::from_utf8_lossy(close).into_owned(),
        },
        start,
        self.current_byte_offset as u32,
      );
    }

    (
      OreqlSyntaxKind::BlockComment,
      self
        .string_pool
        .create_string(start, self.current_byte_offset as u32),
    )
  }

  /// Scan a multi-character operator using the trie
  #[inline]
  pub fn tokenize_operator(
    &mut self,
    operators: &OperatorsConfig,
  ) -> Option<(OreqlSyntaxKind, OreqlString)> {
    let start = self.current_byte_offset as u32;
    let source = self.source_bytes();
    let remaining = &source[(start as usize)..];

    // Find the longest matching operator
    let mut best_kind = None;
    let mut best_length = 0usize;
    let mut node_index = 0usize;

    for (position, &byte) in remaining.iter().enumerate() {
      match operators.step(node_index, byte) {
        Some(next) => {
          node_index = next;
          if let Some(kind) = operators.value_at(node_index) {
            best_kind = Some(kind);
            best_length = position + 1;
          }
        }
        None => break,
      }
    }

    if let Some(kind) = best_kind {
      self.advance_bytes(best_length);
      Some((
        kind,
        self
          .string_pool
          .create_string(start, self.current_byte_offset as u32),
      ))
    } else {
      None
    }
  }

  /// Scan a prefixed string (bit, hex, raw, etc)
  ///
  /// INVARIANT: current byte must be ASCII alphabetic
  #[inline]
  pub fn tokenize_prefixed_string(
    &mut self,
    config: &TokenizePrefixedStringConfig,
  ) -> Option<(OreqlSyntaxKind, OreqlString)> {
    debug_assert!(
      self
        .peek_byte()
        .is_some_and(|byte| byte.is_ascii_alphabetic()),
      "tokenize_prefixed_string() called but current byte is not ASCII alphabetic"
    );
    let raw_escape_bytes = if **config.are_escapes_allowed_in_raw_strings {
      config.string_escape_bytes as &BitSet128
    } else {
      &BitSet128::new()
    };
    let start = self.current_byte_offset as u32;
    let source = self.source_bytes();
    let remaining = &source[(start as usize)..];

    let delimiter_sets: &[(&[DelimiterPair], OreqlSyntaxKind, &BitSet128)] = &[
      (
        config.bit_strings,
        OreqlSyntaxKind::BitString,
        config.string_escape_bytes,
      ),
      (
        config.hex_strings,
        OreqlSyntaxKind::HexString,
        config.string_escape_bytes,
      ),
      (
        config.byte_strings,
        OreqlSyntaxKind::ByteString,
        config.byte_string_escape_bytes,
      ),
      (
        config.raw_strings,
        OreqlSyntaxKind::RawString,
        raw_escape_bytes,
      ),
      (
        config.national_strings,
        OreqlSyntaxKind::NationalString,
        config.string_escape_bytes,
      ),
      (
        config.unicode_strings,
        OreqlSyntaxKind::UnicodeString,
        config.string_escape_bytes,
      ),
      (
        config.heredoc_strings,
        OreqlSyntaxKind::HeredocString,
        &BitSet128::new(),
      ),
    ];

    for &(delimiters, kind, escape_bytes) in delimiter_sets {
      for &(open, close) in delimiters {
        if remaining.starts_with(open.as_bytes()) {
          self.advance_bytes(open.len());
          if !close.is_empty() {
            self.scan_until_delimiter(
              close.as_bytes(),
              escape_bytes,
              OreqlDiagnosticKind::UnterminatedStringLiteral {
                expected: close.to_string(),
              },
            );
          }
          return Some((
            kind,
            self
              .string_pool
              .create_string(start, self.current_byte_offset as u32),
          ));
        }
      }
    }

    None
  }

  /// Scan a variable name (@var, $var, etc)
  ///
  /// INVARIANT: current byte must be consumable (alphanumeric, '_', or in allowed_bytes) and must consume at least one byte
  #[inline]
  pub fn tokenize_variable(
    &mut self,
    allowed_bytes: &VariableAllowedBytesConfig,
  ) -> (OreqlSyntaxKind, OreqlString) {
    debug_assert!(
      self
        .peek_byte()
        .is_some_and(|byte| byte.is_ascii_alphanumeric()
          || byte == b'_'
          || allowed_bytes.contains(byte)),
      "tokenize_variable() called but current byte is not consumable"
    );
    let start = self.current_byte_offset as u32;

    while let Some(byte) = self.peek_byte() {
      if byte.is_ascii_alphanumeric() || byte == b'_' || allowed_bytes.contains(byte) {
        self.advance_byte();
      } else {
        break;
      }
    }

    debug_assert!(
      self.current_byte_offset as u32 > start,
      "tokenize_variable() consumed zero bytes"
    );

    (
      OreqlSyntaxKind::Variable,
      self
        .string_pool
        .create_string(start, self.current_byte_offset as u32),
    )
  }

  /// Advance past an inline comment body (everything until end of line)
  ///
  /// INVARIANT: the opening pattern (e.g. `--`) must already be consumed
  #[inline]
  fn scan_past_inline_comment(&mut self) {
    while let Some(byte) = self.peek_byte() {
      if byte == b'\n' || byte == b'\r' {
        break;
      }
      self.advance_byte();
    }
  }

  /// Advance past a nested block comment body
  ///
  /// Returns `true` if the closing delimiter was found, `false` if EOF
  ///
  /// INVARIANT: the opening delimiter (e.g. `/*`) must already be consumed
  #[inline]
  fn scan_past_nested_block_comment(&mut self, open: &[u8], close: &[u8]) -> bool {
    let mut depth = 1u32;
    loop {
      if self.is_at_end() {
        return false;
      }
      let at_open = self.source_bytes()[self.current_byte_offset..].starts_with(open);
      if at_open {
        self.advance_bytes(open.len());
        depth += 1;
        continue;
      }
      let at_close = self.source_bytes()[self.current_byte_offset..].starts_with(close);
      if at_close {
        self.advance_bytes(close.len());
        depth -= 1;
        if depth == 0 {
          return true;
        }
        continue;
      }
      self.advance_byte();
    }
  }

  /// Skip trivia bytes (whitespace, newlines, comments) without producing tokens
  #[inline]
  fn skip_trivia(
    &mut self,
    inline_comment_starts: &InlineCommentStartsConfig,
    block_comment_delimiters: &BlockCommentDelimitersConfig,
  ) {
    loop {
      match self.peek_byte() {
        Some(b' ' | b'\t' | b'\n') => self.advance_byte(),
        Some(b'\r') => {
          self.advance_byte();
          if self.peek_byte() == Some(b'\n') {
            self.advance_byte();
          }
        }
        Some(_) => {
          let remaining = &self.source_bytes()[self.current_byte_offset..];
          let mut matched_len = 0usize;

          for &pattern in inline_comment_starts.iter() {
            if remaining.starts_with(pattern.as_bytes()) {
              matched_len = pattern.len();
              break;
            }
          }

          if matched_len > 0 {
            self.advance_bytes(matched_len);
            self.scan_past_inline_comment();
            continue;
          }

          for &(open, close) in block_comment_delimiters.iter() {
            if remaining.starts_with(open.as_bytes()) {
              matched_len = open.len();
              self.advance_bytes(matched_len);
              self.scan_past_nested_block_comment(open.as_bytes(), close.as_bytes());
              break;
            }
          }

          if matched_len == 0 {
            break;
          }
        }
        None => break,
      }
    }
  }

  /// Scan until a closing delimiter, handling escape bytes
  ///
  /// Emits the given diagnostic if EOF is reached before the delimiter
  ///
  /// INVARIANT: delimiter must not be empty
  #[inline]
  fn scan_until_delimiter(
    &mut self,
    delimiter: &[u8],
    escape_bytes: &BitSet128,
    unterminated_diagnostic: OreqlDiagnosticKind,
  ) {
    debug_assert!(
      !delimiter.is_empty(),
      "scan_until_delimiter() called with empty delimiter"
    );
    let start = self.current_byte_offset as u32;
    let delimiter_len = delimiter.len();

    loop {
      match self.peek_byte() {
        None => {
          self.emit_error(
            unterminated_diagnostic,
            start,
            self.current_byte_offset as u32,
          );
          break;
        }
        Some(byte) => {
          // Check escape
          if escape_bytes.contains(byte) {
            self.advance_byte();
            if self.peek_byte().is_some() {
              self.advance_byte();
            }
            continue;
          }

          // Check delimiter
          if delimiter_len == 1 {
            if byte == delimiter[0] {
              self.advance_byte();
              return;
            }
          } else {
            let remaining = &self.source_bytes()[(self.current_byte_offset)..];
            if remaining.starts_with(delimiter) {
              self.advance_bytes(delimiter_len);
              return;
            }
          }

          self.advance_byte();
        }
      }
    }
  }
}

/// Config for the main token dispatch
pub struct TokenizeConfig<'a> {
  pub single_byte_tokens: &'a SingleByteTokensConfig,
  pub operators: &'a OperatorsConfig,
  pub keywords: &'a KeywordsConfig,
}

impl Default for TokenizeConfig<'static> {
  fn default() -> Self {
    Self {
      single_byte_tokens: SingleByteTokensConfig::default(),
      operators: OperatorsConfig::default(),
      keywords: KeywordsConfig::default(),
    }
  }
}

/// Config for scanning identifiers and keywords
pub struct TokenizeIdentifierConfig<'a> {
  pub keywords: &'a KeywordsConfig,
  pub inline_comment_starts: &'a InlineCommentStartsConfig,
  pub block_comment_delimiters: &'a BlockCommentDelimitersConfig,
}

impl Default for TokenizeIdentifierConfig<'static> {
  fn default() -> Self {
    Self {
      keywords: KeywordsConfig::default(),
      inline_comment_starts: InlineCommentStartsConfig::default(),
      block_comment_delimiters: BlockCommentDelimitersConfig::default(),
    }
  }
}

/// Config for scanning quoted string literals
pub struct TokenizeStringConfig<'a> {
  pub quotes: &'a StringQuotesConfig,
  pub escape_bytes: &'a StringEscapeBytesConfig,
}

/// Config for scanning quoted identifiers
pub struct TokenizeQuotedIdentifierConfig<'a> {
  pub quotes: &'a IdentifierQuotesConfig,
  pub escape_bytes: &'a IdentifierEscapeBytesConfig,
}

/// Config for scanning prefixed strings (bit, hex, raw, etc)
pub struct TokenizePrefixedStringConfig<'a> {
  pub bit_strings: &'a BitStringDelimitersConfig,
  pub hex_strings: &'a HexStringDelimitersConfig,
  pub byte_strings: &'a ByteStringDelimitersConfig,
  pub raw_strings: &'a RawStringDelimitersConfig,
  pub national_strings: &'a NationalStringDelimitersConfig,
  pub unicode_strings: &'a UnicodeStringDelimitersConfig,
  pub heredoc_strings: &'a HeredocDelimitersConfig,
  pub string_escape_bytes: &'a StringEscapeBytesConfig,
  pub byte_string_escape_bytes: &'a ByteStringEscapeBytesConfig,
  pub are_escapes_allowed_in_raw_strings: &'a AreEscapesAllowedInRawStringsConfig,
}

#[cfg(test)]
mod tests;
