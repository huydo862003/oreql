use super::*;

/// Comparison operators
#[test]
fn comparison_operators() {
  assert_snapshot!(
    ">=",
    r#"(Gte ">=")
(Eof "")"#
  );
  assert_snapshot!(
    "<=",
    r#"(Lte "<=")
(Eof "")"#
  );
  assert_snapshot!(
    "<>",
    r#"(Neq "<>")
(Eof "")"#
  );
  assert_snapshot!(
    "!=",
    r#"(Neq "!=")
(Eof "")"#
  );
}

/// Arrow operators
#[test]
fn arrow_operators() {
  assert_snapshot!(
    "->",
    r#"(Arrow "->")
(Eof "")"#
  );
  assert_snapshot!(
    "->>",
    r#"(DArrow "->>")
(Eof "")"#
  );
  assert_snapshot!(
    "=>",
    r#"(FatArrow "=>")
(Eof "")"#
  );
}

/// Double pipe concatenation
#[test]
fn pipe_operators() {
  assert_snapshot!(
    "||",
    r#"(DPipe "||")
(Eof "")"#
  );
}

/// Trie picks the longest matching operator
#[test]
fn longest_match_operator() {
  assert_snapshot!(
    "->>",
    r#"(DArrow "->>")
(Eof "")"#
  );
}

/// Two operators back to back without space
#[test]
fn consecutive_operators() {
  assert_snapshot!(
    ">=<=",
    r#"(Gte ">=")
(Lte "<=")
(Eof "")"#
  );
}

/// Plus operator before number
#[test]
fn plus_before_number() {
  assert_snapshot!(
    "+42",
    r#"(Plus "+")
(Number "42")
(Eof "")"#
  );
}
