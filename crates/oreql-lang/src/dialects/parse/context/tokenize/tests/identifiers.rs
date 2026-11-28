use super::*;

/// Plain identifier
#[test]
fn simple() {
  assert_snapshot!(
    "my_col",
    r#"(Identifier "my_col")
(Eof "")"#
  );
}

/// Identifier starting with underscore
#[test]
fn starts_with_underscore() {
  assert_snapshot!(
    "_private",
    r#"(Identifier "_private")
(Eof "")"#
  );
}

/// Single underscore is a valid identifier
#[test]
fn single_underscore() {
  assert_snapshot!(
    "_",
    r#"(Identifier "_")
(Eof "")"#
  );
}

/// Double underscore identifier
#[test]
fn double_underscore() {
  assert_snapshot!(
    "__",
    r#"(Identifier "__")
(Eof "")"#
  );
}

/// Underscore followed by digit
#[test]
fn underscore_digit() {
  assert_snapshot!(
    "_1",
    r#"(Identifier "_1")
(Eof "")"#
  );
}

/// Identifier containing digits
#[test]
fn with_digits() {
  assert_snapshot!(
    "col1",
    r#"(Identifier "col1")
(Eof "")"#
  );
}

/// Keywords are case-insensitive
#[test]
fn keyword_case_insensitive() {
  assert_snapshot!(
    "SELECT",
    r#"(KeywordSelect "SELECT")
(Eof "")"#
  );
  assert_snapshot!(
    "select",
    r#"(KeywordSelect "select")
(Eof "")"#
  );
}

/// Mixed case keyword still matches
#[test]
fn keyword_mixed_case() {
  assert_snapshot!(
    "SeLeCt",
    r#"(KeywordSelect "SeLeCt")
(Eof "")"#
  );
}

/// Compound keyword with single space
#[test]
fn keyword_compound_group_by() {
  assert_snapshot!(
    "GROUP BY",
    r#"(KeywordGroupBy "GROUP BY")
(Eof "")"#
  );
}

/// Compound keyword with extra whitespace between words
#[test]
fn keyword_compound_extra_spaces() {
  assert_snapshot!(
    "ORDER  BY",
    r#"(KeywordOrderBy "ORDER  BY")
(Eof "")"#
  );
}

/// Two-word compound keyword
#[test]
fn keyword_compound_primary_key() {
  assert_snapshot!(
    "PRIMARY KEY",
    r#"(KeywordPrimaryKey "PRIMARY KEY")
(Eof "")"#
  );
}

/// GROUP alone is not a keyword, only GROUP BY is
#[test]
fn compound_keyword_prefix_alone_is_identifier() {
  assert_snapshot!(
    "GROUP x",
    r#"(Identifier "GROUP")
(Whitespace " ")
(Identifier "x")
(Eof "")"#
  );
}

/// Double-quoted identifier
#[test]
fn quoted_double() {
  assert_snapshot!(
    r#""my col""#,
    r#"(Identifier "\"my col\"")
(Eof "")"#
  );
}

/// Empty double-quoted identifier
#[test]
fn quoted_empty() {
  assert_snapshot!(
    r#""""#,
    r#"(Identifier "\"\"")
(Eof "")"#
  );
}

/// Double-quoted identifier containing spaces
#[test]
fn quoted_with_spaces() {
  assert_snapshot!(
    r#""my column""#,
    r#"(Identifier "\"my column\"")
(Eof "")"#
  );
}

/// Dot-qualified identifier
#[test]
fn dot_qualified() {
  assert_snapshot!(
    "a.b",
    r#"(Identifier "a")
(Dot ".")
(Identifier "b")
(Eof "")"#
  );
}

/// Star after dot for SELECT a.*
#[test]
fn star_qualified() {
  assert_snapshot!(
    "a.*",
    r#"(Identifier "a")
(Dot ".")
(Star "*")
(Eof "")"#
  );
}

/// Identifier immediately followed by string quote
#[test]
fn then_string_no_delimiter() {
  assert_snapshot!(
    "col'value'",
    r#"(Identifier "col")
(String "'value'")
(Eof "")"#
  );
}

/// Keyword immediately followed by string without space
#[test]
fn keyword_then_string_no_space() {
  assert_snapshot!(
    "SELECT'hello'",
    r#"(KeywordSelect "SELECT")
(String "'hello'")
(Eof "")"#
  );
}

/// Keyword immediately followed by quoted identifier
#[test]
fn keyword_then_quoted_identifier() {
  assert_snapshot!(
    r#"SELECT"col""#,
    r#"(KeywordSelect "SELECT")
(Identifier "\"col\"")
(Eof "")"#
  );
}

/// Keyword followed by alphanumeric char becomes identifier
#[test]
fn keyword_suffixed_with_char_is_identifier() {
  assert_snapshot!(
    "SELECTx",
    r#"(Identifier "SELECTx")
(Eof "")"#
  );
}

/// Keyword followed by underscore becomes identifier
#[test]
fn keyword_suffixed_with_underscore_is_identifier() {
  assert_snapshot!(
    "SELECT_all",
    r#"(Identifier "SELECT_all")
(Eof "")"#
  );
}

/// Keyword followed by digit becomes identifier
#[test]
fn keyword_suffixed_with_digit_is_identifier() {
  assert_snapshot!(
    "FROM1",
    r#"(Identifier "FROM1")
(Eof "")"#
  );
}

/// Compound keyword with tab between words
#[test]
fn keyword_compound_with_tab() {
  assert_snapshot!(
    "GROUP\tBY",
    r#"(KeywordGroupBy "GROUP\tBY")
(Eof "")"#
  );
}

/// Compound keyword with mixed whitespace between words
#[test]
fn keyword_compound_with_mixed_whitespace() {
  assert_snapshot!(
    "ORDER \t BY",
    r#"(KeywordOrderBy "ORDER \t BY")
(Eof "")"#
  );
}

/// Compound keyword with newline between words still matches
#[test]
fn keyword_compound_split_by_newline() {
  assert_snapshot!(
    "GROUP\nBY",
    r#"(KeywordGroupBy "GROUP\nBY")
(Eof "")"#
  );
}

/// Compound keyword with block comment between words still matches
#[test]
fn keyword_compound_split_by_comment() {
  assert_snapshot!(
    "GROUP /* comment */ BY",
    r#"(KeywordGroupBy "GROUP /* comment */ BY")
(Eof "")"#
  );
}

/// Compound keyword with inline comment between words still matches
#[test]
fn keyword_compound_split_by_inline_comment() {
  assert_snapshot!(
    "GROUP -- comment\nBY",
    r#"(KeywordGroupBy "GROUP -- comment\nBY")
(Eof "")"#
  );
}

/// Identifier longer than 128 bytes skips keyword lookup
#[test]
fn very_long_identifier() {
  let long = "a".repeat(200);
  let output = tokenize_snapshot::<TestDialect>(&long);
  assert_eq!(output, format!(r#"(Identifier "{long}")
(Eof "")"#));
}

/// Compound keyword prefix at EOF produces identifier
#[test]
fn compound_keyword_prefix_at_eof() {
  assert_snapshot!(
    "GROUP",
    r#"(Identifier "GROUP")
(Eof "")"#
  );
}

/// Compound keyword prefix followed by EOF after whitespace
#[test]
fn compound_keyword_prefix_whitespace_then_eof() {
  assert_snapshot!(
    "GROUP ",
    r#"(Identifier "GROUP")
(Whitespace " ")
(Eof "")"#
  );
}
