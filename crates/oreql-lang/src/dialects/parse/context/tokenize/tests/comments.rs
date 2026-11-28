use super::*;

/// Inline comment followed by content on next line
#[test]
fn inline() {
  assert_snapshot!(
    "-- comment\n42",
    r#"(LineComment "-- comment")
(Newline "\n")
(Number "42")
(Eof "")"#
  );
}

/// Inline comment at end of file (no trailing newline)
#[test]
fn inline_at_eof() {
  assert_snapshot!(
    "-- comment",
    r#"(LineComment "-- comment")
(Eof "")"#
  );
}

/// Inline comment without space after dashes
#[test]
fn inline_no_space() {
  assert_snapshot!(
    "--comment",
    r#"(LineComment "--comment")
(Eof "")"#
  );
}

/// Inline comment with nothing after dashes
#[test]
fn inline_empty() {
  assert_snapshot!(
    "--",
    r#"(LineComment "--")
(Eof "")"#
  );
}

/// Simple block comment
#[test]
fn block() {
  assert_snapshot!(
    "/* comment */",
    r#"(BlockComment "/* comment */")
(Eof "")"#
  );
}

/// Block comment with nested block comment
#[test]
fn block_nested() {
  assert_snapshot!(
    "/* /* nested */ */",
    r#"(BlockComment "/* /* nested */ */")
(Eof "")"#
  );
}

/// Empty block comment
#[test]
fn block_empty() {
  assert_snapshot!(
    "/**/",
    r#"(BlockComment "/**/")
(Eof "")"#
  );
}

/// Block comment spanning multiple lines
#[test]
fn block_with_newlines() {
  assert_snapshot!(
    "/* line1\nline2 */",
    r#"(BlockComment "/* line1\nline2 */")
(Eof "")"#
  );
}

/// Unterminated block comment emits diagnostic
#[test]
fn block_unterminated_emits_diagnostic() {
  let (output, diagnostics) = tokenize_and_diagnostic_snapshot::<TestDialect>("/* unterminated");
  assert!(
    output.starts_with("(BlockComment"),
    "expected BlockComment token"
  );
  assert!(!diagnostics.is_empty(), "expected diagnostic");
}

/// Jinja-style block comment
#[test]
fn jinja() {
  assert_snapshot!(
    "{# jinja #}",
    r#"(BlockComment "{# jinja #}")
(Eof "")"#
  );
}

/// Multiple consecutive inline comments
#[test]
fn multiple_consecutive() {
  assert_snapshot!(
    "-- first\n-- second",
    r#"(LineComment "-- first")
(Newline "\n")
(LineComment "-- second")
(Eof "")"#
  );
}

/// Block comment followed by inline comment
#[test]
fn block_then_inline() {
  assert_snapshot!(
    "/* block */ -- inline",
    r#"(BlockComment "/* block */")
(Whitespace " ")
(LineComment "-- inline")
(Eof "")"#
  );
}

/// Three levels of nested block comments
#[test]
fn block_nested_three_deep() {
  assert_snapshot!(
    "/* /* /* deep */ */ */",
    r#"(BlockComment "/* /* /* deep */ */ */")
(Eof "")"#
  );
}

/// Unterminated jinja comment emits diagnostic
#[test]
fn jinja_unterminated_emits_diagnostic() {
  let (output, diagnostics) = tokenize_and_diagnostic_snapshot::<TestDialect>("{# no close");
  assert!(
    output.starts_with("(BlockComment"),
    "expected BlockComment token"
  );
  assert!(!diagnostics.is_empty(), "expected diagnostic");
}

/// Inline comment immediately followed by newline (empty body)
#[test]
fn inline_empty_then_newline() {
  assert_snapshot!(
    "--\n42",
    r#"(LineComment "--")
(Newline "\n")
(Number "42")
(Eof "")"#
  );
}

/// `/*/` opens a block comment (not `/*` + `/`), closed by next `*/`
#[test]
fn block_comment_tricky_open() {
  assert_snapshot!(
    "1 /*/2 */",
    r#"(Number "1")
(Whitespace " ")
(BlockComment "/*/2 */")
(Eof "")"#
  );
}

/// Block comment between two numbers
#[test]
fn block_comment_between_numbers() {
  assert_snapshot!(
    "1/* */2",
    r#"(Number "1")
(BlockComment "/* */")
(Number "2")
(Eof "")"#
  );
}

/// Block comment between two identifiers
#[test]
fn block_comment_between_identifiers() {
  assert_snapshot!(
    "a/**/b",
    r#"(Identifier "a")
(BlockComment "/**/")
(Identifier "b")
(Eof "")"#
  );
}
