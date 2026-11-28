use super::*;
use oreql_utils::ByteLut;

/* Regular string literals */

/// Basic single-quoted string
#[test]
fn single_quoted() {
  assert_snapshot!(
    "'hello'",
    r#"(String "'hello'")
(Eof "")"#
  );
}

/// Empty string literal
#[test]
fn empty() {
  assert_snapshot!(
    "''",
    r#"(String "''")
(Eof "")"#
  );
}

/// String containing spaces
#[test]
fn with_spaces() {
  assert_snapshot!(
    "'hello world'",
    r#"(String "'hello world'")
(Eof "")"#
  );
}

/// String containing digits
#[test]
fn with_numbers() {
  assert_snapshot!(
    "'abc123'",
    r#"(String "'abc123'")
(Eof "")"#
  );
}

/// String containing special ASCII characters
#[test]
fn with_special_characters() {
  assert_snapshot!(
    "'@#$%'",
    r#"(String "'@#$%'")
(Eof "")"#
  );
}

/// Newline inside string is part of the string, not a separate token
#[test]
fn with_newline_inside() {
  assert_snapshot!(
    "'line1\nline2'",
    r#"(String "'line1\nline2'")
(Eof "")"#
  );
}

/// Tab inside string is part of the string
#[test]
fn with_tab_inside() {
  assert_snapshot!(
    "'col\there'",
    r#"(String "'col\there'")
(Eof "")"#
  );
}

/// CRLF inside string is part of the string
#[test]
fn with_crlf_inside() {
  assert_snapshot!(
    "'line1\r\nline2'",
    r#"(String "'line1\r\nline2'")
(Eof "")"#
  );
}

/// Multi-byte UTF-8 characters inside string
#[test]
fn unicode_content() {
  assert_snapshot!(
    "'caf\u{00e9}'",
    r#"(String "'café'")
(Eof "")"#
  );
}

/// With doubled-quote escape, `'a' 'b'` is one string: `'` escapes `'`
#[test]
fn consecutive_with_doubled_escape() {
  assert_snapshot!(
    "'a' 'b'",
    r#"(String "'a' 'b'")
(Eof "")"#
  );
}

/* Escape handling (default: doubled quote) */

/// Doubled single quote acts as escape
#[test]
fn doubled_quote_escape() {
  assert_snapshot!(
    "'it''s'",
    r#"(String "'it''s'")
(Eof "")"#
  );
}

/// Multiple doubled quotes in a row
#[test]
fn multiple_escaped_quotes() {
  assert_snapshot!(
    "'it''''s'",
    r#"(String "'it''''s'")
(Eof "")"#
  );
}

/// String containing only escaped quotes
#[test]
fn only_escaped_quotes() {
  assert_snapshot!(
    "''''",
    r#"(String "''''")
(Eof "")"#
  );
}

/* Backslash escape dialect */

struct BackslashEscapeDialect;
impl DialectParser for BackslashEscapeDialect {
  fn tokenize_string_lit(
    context: &mut ParserContext<'_, Self>,
  ) -> (OreqlSyntaxKind, crate::OreqlString) {
    static ESCAPE_CONFIG: StringEscapeBytesConfig =
      StringEscapeBytesConfig::new(ByteLut::new().set(b'\'', true).set(b'\\', true));
    context.tokenize_string_lit(&TokenizeStringConfig {
      quotes: StringQuotesConfig::default(),
      escape_bytes: &ESCAPE_CONFIG,
    })
  }
}

/// Backslash before n keeps both bytes in the string
#[test]
fn backslash_escape_newline() {
  let output = tokenize_snapshot::<BackslashEscapeDialect>("'hello\\nworld'");
  assert_eq!(
    output,
    r#"(String "'hello\\nworld'")
(Eof "")"#
  );
}

/// Backslash before single quote prevents string termination
#[test]
fn backslash_escaped_quote() {
  let output = tokenize_snapshot::<BackslashEscapeDialect>("'it\\'s'");
  assert_eq!(
    output,
    r#"(String "'it\\'s'")
(Eof "")"#
  );
}

/* Unterminated strings */

/// Unterminated string emits diagnostic
#[test]
fn unterminated_emits_diagnostic() {
  let (output, diagnostics) = tokenize_and_diagnostic_snapshot::<TestDialect>("'unterminated");
  assert!(output.starts_with("(String"), "expected String token");
  assert!(!diagnostics.is_empty(), "expected diagnostic");
}

/// Escape consumes a quote but string still unterminated at EOF
#[test]
fn unterminated_with_escape() {
  let (output, diagnostics) = tokenize_and_diagnostic_snapshot::<TestDialect>("'it''s");
  assert!(output.starts_with("(String"), "expected String token");
  assert!(
    !diagnostics.is_empty(),
    "expected diagnostic for unterminated string"
  );
}

/// Backslash escape at very end of input consumes the backslash, then hits EOF
#[test]
fn backslash_escape_at_eof() {
  // In backslash-escape dialect: `'hello\` - backslash consumes itself, peek is None, then EOF
  let output = tokenize_snapshot::<BackslashEscapeDialect>("'hello\\");
  assert!(output.starts_with("(String"), "expected String token");
}

/* Prefixed strings */

struct PrefixedStringDialect;
impl DialectParser for PrefixedStringDialect {
  fn tokenize_prefixed_string(
    context: &mut ParserContext<'_, Self>,
  ) -> Option<(OreqlSyntaxKind, crate::OreqlString)> {
    static BIT: BitStringDelimitersConfig =
      BitStringDelimitersConfig::new(&[("b'", "'"), ("B'", "'")]);
    static HEX: HexStringDelimitersConfig =
      HexStringDelimitersConfig::new(&[("x'", "'"), ("X'", "'")]);
    static BYTE: ByteStringDelimitersConfig = ByteStringDelimitersConfig::new(&[("b'", "'")]);
    static RAW: RawStringDelimitersConfig =
      RawStringDelimitersConfig::new(&[("r'", "'"), ("R'", "'")]);
    static NATIONAL: NationalStringDelimitersConfig =
      NationalStringDelimitersConfig::new(&[("N'", "'"), ("n'", "'")]);
    static UNICODE: UnicodeStringDelimitersConfig =
      UnicodeStringDelimitersConfig::new(&[("U&'", "'")]);
    static HEREDOC: HeredocDelimitersConfig = HeredocDelimitersConfig::new(&[("$$", "$$")]);
    static RAW_ESCAPES: AreEscapesAllowedInRawStringsConfig =
      AreEscapesAllowedInRawStringsConfig::new(false);

    context.tokenize_prefixed_string(&TokenizePrefixedStringConfig {
      bit_strings: &BIT,
      hex_strings: &HEX,
      byte_strings: &BYTE,
      raw_strings: &RAW,
      national_strings: &NATIONAL,
      unicode_strings: &UNICODE,
      heredoc_strings: &HEREDOC,
      string_escape_bytes: StringEscapeBytesConfig::default(),
      byte_string_escape_bytes: ByteStringEscapeBytesConfig::default(),
      are_escapes_allowed_in_raw_strings: &RAW_ESCAPES,
    })
  }
}

macro_rules! assert_prefixed {
  ($input:expr, $expected:expr) => {
    let input = $input;
    let expected = $expected;
    let output = tokenize_snapshot::<PrefixedStringDialect>(input);
    assert_eq!(output, expected, "\ninput: {input:?}");
  };
}

/// b'...' and B'...' produce BitString
#[test]
fn prefixed_bit_string() {
  assert_prefixed!(
    "b'0101'",
    r#"(BitString "b'0101'")
(Eof "")"#
  );
  assert_prefixed!(
    "B'1100'",
    r#"(BitString "B'1100'")
(Eof "")"#
  );
}

/// x'...' and X'...' produce HexString
#[test]
fn prefixed_hex_string() {
  assert_prefixed!(
    "x'FF'",
    r#"(HexString "x'FF'")
(Eof "")"#
  );
  assert_prefixed!(
    "X'00AB'",
    r#"(HexString "X'00AB'")
(Eof "")"#
  );
}

/// N'...' and n'...' produce NationalString
#[test]
fn prefixed_national_string() {
  assert_prefixed!(
    "N'hello'",
    r#"(NationalString "N'hello'")
(Eof "")"#
  );
  assert_prefixed!(
    "n'world'",
    r#"(NationalString "n'world'")
(Eof "")"#
  );
}

/// U&'...' produces UnicodeString
#[test]
fn prefixed_unicode_string() {
  assert_prefixed!(
    "U&'text'",
    r#"(UnicodeString "U&'text'")
(Eof "")"#
  );
}

/// r'...' and R'...' produce RawString with escapes disabled
#[test]
fn prefixed_raw_string() {
  assert_prefixed!(
    "r'no\\escape'",
    r#"(RawString "r'no\\escape'")
(Eof "")"#
  );
  assert_prefixed!(
    "R'path'",
    r#"(RawString "R'path'")
(Eof "")"#
  );
}

/// Empty prefixed bit string
#[test]
fn prefixed_empty_bit_string() {
  assert_prefixed!(
    "b''",
    r#"(BitString "b''")
(Eof "")"#
  );
}

/// No matching prefix falls through to identifier
#[test]
fn prefixed_no_match_falls_through_to_identifier() {
  assert_prefixed!(
    "count",
    r#"(Identifier "count")
(Eof "")"#
  );
}
