use super::*;

/// Simple integer literal
#[test]
fn integer() {
  assert_snapshot!(
    "42",
    r#"(Number "42")
(Eof "")"#
  );
}

/// Decimal with fractional part
#[test]
fn decimal() {
  assert_snapshot!(
    "3.14",
    r#"(Number "3.14")
(Eof "")"#
  );
}

/// Scientific notation with lowercase e
#[test]
fn scientific() {
  assert_snapshot!(
    "1e10",
    r#"(Number "1e10")
(Eof "")"#
  );
}

/// Scientific notation with negative exponent
#[test]
fn scientific_negative_exponent() {
  assert_snapshot!(
    "2.5E-3",
    r#"(Number "2.5E-3")
(Eof "")"#
  );
}

/// Scientific notation with explicit positive exponent
#[test]
fn scientific_positive_exponent() {
  assert_snapshot!(
    "1E+5",
    r#"(Number "1E+5")
(Eof "")"#
  );
}

/// Scientific notation with no digits after sign emits diagnostic
#[test]
fn scientific_no_digits_emits_diagnostic() {
  let (output, diagnostics) = tokenize_and_diagnostic_snapshot::<TestDialect>("1e+");
  assert!(output.starts_with("(Number"), "expected Number token");
  assert!(!diagnostics.is_empty(), "expected diagnostic");
}

/// Underscore digit separators
#[test]
fn underscore_separated() {
  assert_snapshot!(
    "1_000_000",
    r#"(Number "1_000_000")
(Eof "")"#
  );
}

/// Leading zeros preserved
#[test]
fn leading_zero() {
  assert_snapshot!(
    "007",
    r#"(Number "007")
(Eof "")"#
  );
}

/// Zero by itself
#[test]
fn zero() {
  assert_snapshot!(
    "0",
    r#"(Number "0")
(Eof "")"#
  );
}

/// 0x prefix hex literal
#[test]
fn hex_literal() {
  assert_snapshot!(
    "0xFF",
    r#"(HexString "0xFF")
(Eof "")"#
  );
}

/// 0X uppercase prefix also works
#[test]
fn hex_uppercase_prefix() {
  assert_snapshot!(
    "0XAB",
    r#"(HexString "0XAB")
(Eof "")"#
  );
}

/// 0b prefix binary literal
#[test]
fn binary_literal() {
  assert_snapshot!(
    "0b1010",
    r#"(BitString "0b1010")
(Eof "")"#
  );
}

/// 0B uppercase prefix also works
#[test]
fn binary_uppercase_prefix() {
  assert_snapshot!(
    "0B110",
    r#"(BitString "0B110")
(Eof "")"#
  );
}

/// 0x with no hex digits falls back to Identifier
#[test]
fn invalid_hex_fallback() {
  assert_snapshot!(
    "0x",
    r#"(Identifier "0x")
(Eof "")"#
  );
}

/// 0b with no binary digits falls back to Identifier
#[test]
fn invalid_binary_fallback() {
  assert_snapshot!(
    "0b",
    r#"(Identifier "0b")
(Eof "")"#
  );
}

/// Number followed by letter without delimiter splits into two tokens
#[test]
fn then_letter_is_two_tokens() {
  assert_snapshot!(
    "3a",
    r#"(Number "3")
(Identifier "a")
(Eof "")"#
  );
}

/// Minus is a separate operator, not part of the number
#[test]
fn negative_is_operator_then_number() {
  assert_snapshot!(
    "-42",
    r#"(Minus "-")
(Number "42")
(Eof "")"#
  );
}

/// Dot followed by non-digit is not a decimal
#[test]
fn dot_no_fraction_is_not_decimal() {
  assert_snapshot!(
    "1.method",
    r#"(Number "1")
(Dot ".")
(Identifier "method")
(Eof "")"#
  );
}

/// Dot followed by digit makes a decimal
#[test]
fn decimal_proper() {
  assert_snapshot!(
    "1.0",
    r#"(Number "1.0")
(Eof "")"#
  );
}

/// Multiple dots: first decimal consumed, rest are separate tokens
#[test]
fn multiple_dots() {
  assert_snapshot!(
    "1.2.3",
    r#"(Number "1.2")
(Dot ".")
(Number "3")
(Eof "")"#
  );
}

/// Number immediately followed by string quote
#[test]
fn then_string_no_delimiter() {
  assert_snapshot!(
    "3'hello'",
    r#"(Number "3")
(String "'hello'")
(Eof "")"#
  );
}

/// Number immediately followed by quoted identifier
#[test]
fn then_quoted_identifier() {
  assert_snapshot!(
    r#"3"col""#,
    r#"(Number "3")
(Identifier "\"col\"")
(Eof "")"#
  );
}

/// Decimal with underscore in fractional part
#[test]
fn decimal_underscore_fraction() {
  assert_snapshot!(
    "1.000_001",
    r#"(Number "1.000_001")
(Eof "")"#
  );
}

/// Scientific notation on decimal
#[test]
fn decimal_scientific() {
  assert_snapshot!(
    "1.5e2",
    r#"(Number "1.5e2")
(Eof "")"#
  );
}

/// 0 followed by non-hex/binary letter is two tokens
#[test]
fn zero_then_non_prefix_letter() {
  assert_snapshot!(
    "0c",
    r#"(Number "0")
(Identifier "c")
(Eof "")"#
  );
}

/// `1e` at EOF with no exponent digits emits diagnostic
#[test]
fn scientific_no_exponent_at_eof() {
  let (output, diagnostics) = tokenize_and_diagnostic_snapshot::<TestDialect>("1e");
  assert!(output.starts_with("(Number"), "expected Number token");
  assert!(
    !diagnostics.is_empty(),
    "expected diagnostic for missing exponent digits"
  );
}
