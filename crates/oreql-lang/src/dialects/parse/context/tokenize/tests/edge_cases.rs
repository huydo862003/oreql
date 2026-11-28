use super::*;

/// Empty input produces only EOF
#[test]
fn empty_source() {
  assert_snapshot!("", r#"(Eof "")"#);
}

/// Only whitespace
#[test]
fn only_whitespace() {
  assert_snapshot!(
    "   ",
    r#"(Whitespace "   ")
(Eof "")"#
  );
}

/// Only a line comment
#[test]
fn only_line_comment() {
  assert_snapshot!(
    "-- just a comment",
    r#"(LineComment "-- just a comment")
(Eof "")"#
  );
}

/// Only a block comment
#[test]
fn only_block_comment() {
  assert_snapshot!(
    "/* only this */",
    r#"(BlockComment "/* only this */")
(Eof "")"#
  );
}

/// Two statements separated by semicolon
#[test]
fn multiple_statements() {
  assert_snapshot!(
    "SELECT 1; SELECT 2",
    r#"(KeywordSelect "SELECT")
(Whitespace " ")
(Number "1")
(Semicolon ";")
(Whitespace " ")
(KeywordSelect "SELECT")
(Whitespace " ")
(Number "2")
(Eof "")"#
  );
}

/// Full SELECT statement
#[test]
fn select_statement() {
  assert_snapshot!(
    "SELECT a, b FROM t",
    r#"(KeywordSelect "SELECT")
(Whitespace " ")
(Identifier "a")
(Comma ",")
(Whitespace " ")
(Identifier "b")
(Whitespace " ")
(KeywordFrom "FROM")
(Whitespace " ")
(Identifier "t")
(Eof "")"#
  );
}

/// WHERE clause with comparison
#[test]
fn where_clause() {
  assert_snapshot!(
    "WHERE x > 1",
    r#"(KeywordWhere "WHERE")
(Whitespace " ")
(Identifier "x")
(Whitespace " ")
(Gt ">")
(Whitespace " ")
(Number "1")
(Eof "")"#
  );
}

/* Peek and eat */

/// Peeking does not advance the offset
#[test]
fn peek_does_not_consume() {
  let pool = crate::OreqlStringPool::new("SELECT");
  let mut context = super::super::ParserContext::<TestDialect>::new(&pool);
  let config = super::super::TokenizeConfig::default();
  let peeked = context.peek_next_token(&config);
  assert_eq!(peeked.kind, OreqlSyntaxKind::KeywordSelect);
  let (kind, _) = context.eat_next_token(&config);
  assert_eq!(kind, OreqlSyntaxKind::KeywordSelect);
}

/// Eating a peeked token advances past it
#[test]
fn eat_peeked_advances_past_token() {
  let pool = crate::OreqlStringPool::new("SELECT FROM");
  let mut context = super::super::ParserContext::<TestDialect>::new(&pool);
  let config = super::super::TokenizeConfig::default();
  let peeked = context.peek_next_token(&config);
  assert_eq!(peeked.kind, OreqlSyntaxKind::KeywordSelect);
  context.eat_peeked_token(peeked);
  let (kind, _) = context.eat_next_token(&config);
  assert_eq!(kind, OreqlSyntaxKind::Whitespace);
  let (kind, _) = context.eat_next_token(&config);
  assert_eq!(kind, OreqlSyntaxKind::KeywordFrom);
}

/* Token boundary transitions */

/// With doubled-quote escape, `'hello'world` is one string
/// because `'w` is treated as an escaped quote + following char
#[test]
fn string_then_identifier_with_doubled_escape() {
  let (output, diagnostics) = tokenize_and_diagnostic_snapshot::<TestDialect>("'hello'world");
  assert!(output.starts_with("(String"), "expected String token");
  assert!(
    !diagnostics.is_empty(),
    "unterminated because no closing quote"
  );
}

/// Non-ASCII character produces Error token via advance_char
#[test]
fn non_ascii_character() {
  assert_snapshot!(
    "\u{1F600}",
    r#"(Error "😀")
(Eof "")"#
  );
}

/// Single `-` is an operator, not the start of `--` comment
#[test]
fn single_dash_is_operator() {
  assert_snapshot!(
    "- 1",
    r#"(Minus "-")
(Whitespace " ")
(Number "1")
(Eof "")"#
  );
}

/// Single `/` is an operator, not the start of `/*` comment
#[test]
fn single_slash_is_operator() {
  assert_snapshot!(
    "/ 1",
    r#"(Slash "/")
(Whitespace " ")
(Number "1")
(Eof "")"#
  );
}

/// `{` without `#` is not a jinja comment
#[test]
fn lone_brace_is_not_jinja() {
  assert_snapshot!(
    "{ 1",
    r#"(LBrace "{")
(Whitespace " ")
(Number "1")
(Eof "")"#
  );
}
