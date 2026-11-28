pub mod default_tokenizer_config;
pub mod tokenize;

use std::marker::PhantomData;

use crate::{
  GreenTree, GreenTreeBuilder, OreqlDiagnostic, OreqlDiagnosticKind, OreqlStringPool, TextRange,
  dialects::parse::{TokenizeConfig, context::tokenize::PeekResult},
};

use super::DialectParser;

pub struct ParserContext<'a, D: DialectParser> {
  pub diagnostics: Vec<OreqlDiagnostic>,
  string_pool: &'a OreqlStringPool<'a>,
  builder: GreenTreeBuilder,
  current_byte_offset: usize,
  _dialect: PhantomData<D>,
}

impl<'a, D: DialectParser> ParserContext<'a, D> {
  pub fn new(string_pool: &'a OreqlStringPool<'a>) -> Self {
    ParserContext {
      string_pool,
      builder: GreenTreeBuilder::new(),
      diagnostics: Vec::new(),
      current_byte_offset: 0,
      _dialect: PhantomData,
    }
  }

  /// Source bytes
  #[inline]
  pub(crate) fn source_bytes(&self) -> &[u8] {
    self.string_pool.source_text().as_bytes()
  }

  /// Whether we have reached end of source
  #[inline]
  pub(crate) fn is_at_end(&self) -> bool {
    self.current_byte_offset >= self.source_bytes().len()
  }

  /// Current byte without advancing
  ///
  /// Returns `None` if at end
  #[inline]
  pub(crate) fn peek_byte(&self) -> Option<u8> {
    self.source_bytes().get(self.current_byte_offset).copied()
  }

  /// Byte at offset from current position
  #[inline]
  pub(crate) fn peek_byte_at(&self, offset: usize) -> Option<u8> {
    self
      .source_bytes()
      .get(self.current_byte_offset + offset)
      .copied()
  }

  /// Current char without advancing (handles UTF-8)
  #[inline]
  pub(crate) fn peek_char(&self) -> Option<char> {
    let bytes = self.source_bytes();
    if self.current_byte_offset >= bytes.len() {
      return None;
    }
    let remaining = &bytes[self.current_byte_offset..];
    // Safety: source is valid UTF-8 (from &str)
    let remaining_str = unsafe { core::str::from_utf8_unchecked(remaining) };
    remaining_str.chars().next()
  }

  /// Advance by one byte
  ///
  /// INVARIANT: must not be at end
  #[inline]
  pub(crate) fn advance_byte(&mut self) {
    debug_assert!(!self.is_at_end(), "advance_byte called at end of source");
    self.current_byte_offset += 1;
  }

  /// Advance by N bytes
  ///
  /// INVARIANT: must have at least `count` bytes remaining
  #[inline]
  pub(crate) fn advance_bytes(&mut self, count: usize) {
    debug_assert!(
      self.current_byte_offset + count <= self.source_bytes().len(),
      "advance_bytes({count}) would go past end of source"
    );
    self.current_byte_offset += count;
  }

  /// Advance by one char (1-4 bytes depending on UTF-8 encoding)
  ///
  /// No-op if at end
  #[inline]
  pub(crate) fn advance_char(&mut self) {
    if let Some(character) = self.peek_char() {
      self.current_byte_offset += character.len_utf8();
    }
  }

  /// Emit a diagnostic error
  pub(crate) fn emit_error(&mut self, kind: OreqlDiagnosticKind, start: u32, end: u32) {
    self.diagnostics.push(OreqlDiagnostic::error(
      kind,
      TextRange {
        start_byte_offset: start,
        end_byte_offset: end,
      },
    ));
  }

  /// Emit a diagnostic warning
  pub(crate) fn emit_warning(&mut self, kind: OreqlDiagnosticKind, start: u32, end: u32) {
    self.diagnostics.push(OreqlDiagnostic::warning(
      kind,
      TextRange {
        start_byte_offset: start,
        end_byte_offset: end,
      },
    ));
  }

  /// Consume a previously peeked token: advance offset, commit diagnostics, push to builder
  ///
  /// INVARIANT: must be the result of the most recent peek
  #[inline]
  pub fn eat_peeked_token(&mut self, peeked: PeekResult) {
    debug_assert!(
      peeked.string.end_byte_offset as usize >= self.current_byte_offset,
      "eat_peeked_token() called with a token behind the current offset"
    );
    self.current_byte_offset = peeked.string.end_byte_offset as usize;
    self.diagnostics.extend(peeked.pending_diagnostics);
    self.builder.push_token(
      peeked.kind,
      TextRange {
        start_byte_offset: peeked.string.start_byte_offset,
        end_byte_offset: peeked.string.end_byte_offset,
      },
    );
  }

  /// Scan and immediately consume the next token, pushing to builder
  #[inline]
  pub fn eat_next_token(
    &mut self,
    config: &TokenizeConfig,
  ) -> (crate::OreqlSyntaxKind, crate::OreqlString) {
    let (kind, string) = self.scan_next_token(config);
    self.builder.push_token(
      kind,
      crate::TextRange {
        start_byte_offset: string.start_byte_offset,
        end_byte_offset: string.end_byte_offset,
      },
    );
    (kind, string)
  }

  /// Consume and push all whitespace tokens (spaces, tabs) to the builder
  #[inline]
  pub fn consume_whitespace(&mut self, config: &TokenizeConfig) {
    loop {
      let peeked = self.peek_next_token(config);
      if peeked.kind == crate::OreqlSyntaxKind::Whitespace {
        self.builder.push_token(
          peeked.kind,
          crate::TextRange {
            start_byte_offset: peeked.string.start_byte_offset,
            end_byte_offset: peeked.string.end_byte_offset,
          },
        );
        self.eat_peeked_token(peeked);
      } else {
        break;
      }
    }
  }

  /// Consume and push all trivia tokens (whitespace, newlines, comments) to the builder
  #[inline]
  pub fn consume_trivia(&mut self, config: &TokenizeConfig) {
    loop {
      let peeked = self.peek_next_token(config);
      if peeked.kind.is_trivia() {
        self.builder.push_token(
          peeked.kind,
          crate::TextRange {
            start_byte_offset: peeked.string.start_byte_offset,
            end_byte_offset: peeked.string.end_byte_offset,
          },
        );
        self.eat_peeked_token(peeked);
      } else {
        break;
      }
    }
  }

  /// Consume trivia, then eat and return the next significant token
  #[inline]
  pub fn eat_next_significant_token(
    &mut self,
    config: &TokenizeConfig,
  ) -> (crate::OreqlSyntaxKind, crate::OreqlString) {
    self.consume_trivia(config);
    self.eat_next_token(config)
  }

  /// Consume trivia, then peek the next significant token
  #[inline]
  pub fn peek_next_significant_token(&mut self, config: &TokenizeConfig) -> tokenize::PeekResult {
    let saved_offset = self.current_byte_offset;
    let diagnostics_before = self.diagnostics.len();
    self.consume_trivia(config);
    let (kind, string) = self.eat_next_token(config);
    let pending_diagnostics = self.diagnostics.drain(diagnostics_before..).collect();
    self.current_byte_offset = saved_offset;
    tokenize::PeekResult {
      kind,
      string,
      pending_diagnostics,
    }
  }

  pub fn parse(&mut self) -> GreenTree {
    unimplemented!()
  }
}
