use super::*;

/// Spaces, newline, then more spaces
#[test]
fn whitespace_and_newlines() {
  assert_snapshot!(
    "  \n  ",
    r#"(Whitespace "  ")
(Newline "\n")
(Whitespace "  ")
(Eof "")"#
  );
}

/// CRLF is a single newline token
#[test]
fn crlf_is_single_newline() {
  assert_snapshot!(
    "\r\n",
    r#"(Newline "\r\n")
(Eof "")"#
  );
}

/// Bare CR without LF
#[test]
fn bare_cr_is_newline() {
  assert_snapshot!(
    "\r",
    r#"(Newline "\r")
(Eof "")"#
  );
}

/// Each LF is a separate newline token
#[test]
fn multiple_newlines() {
  assert_snapshot!(
    "\n\n",
    r#"(Newline "\n")
(Newline "\n")
(Eof "")"#
  );
}

/// Tabs are whitespace
#[test]
fn tabs_as_whitespace() {
  assert_snapshot!(
    "\t\t",
    r#"(Whitespace "\t\t")
(Eof "")"#
  );
}

/// Spaces and tabs merge into one whitespace token
#[test]
fn mixed_spaces_and_tabs() {
  assert_snapshot!(
    " \t ",
    r#"(Whitespace " \t ")
(Eof "")"#
  );
}

/// Parentheses
#[test]
fn parens() {
  assert_snapshot!(
    "()",
    r#"(LParen "(")
(RParen ")")
(Eof "")"#
  );
}

/// Comma and semicolon punctuation
#[test]
fn comma_semicolon() {
  assert_snapshot!(
    ",;",
    r#"(Comma ",")
(Semicolon ";")
(Eof "")"#
  );
}
