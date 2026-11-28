use crate::TextRange;

#[oreql_macros::diagnostic_kind]
pub enum OreqlDiagnosticKind {
  /* Tokenizer */
  #[message("expected digits after exponent in numeric literal")]
  ExpectedDigitsAfterExponent,
  #[message("unterminated block comment, expected closing `{expected}`")]
  UnterminatedBlockComment { expected: String },
  #[message("unterminated string literal, expected closing `{expected}`")]
  UnterminatedStringLiteral { expected: String },
  #[message("unterminated quoted identifier, expected closing `{expected}`")]
  UnterminatedQuotedIdentifier { expected: String },
  /* Parser */
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OreqlDiagnosticSeverity {
  Error,
  Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OreqlDiagnostic {
  pub kind: OreqlDiagnosticKind,
  pub severity: OreqlDiagnosticSeverity,
  pub range: TextRange,
}

impl OreqlDiagnostic {
  #[inline]
  pub fn error(kind: OreqlDiagnosticKind, range: TextRange) -> Self {
    Self {
      kind,
      severity: OreqlDiagnosticSeverity::Error,
      range,
    }
  }

  #[inline]
  pub fn warning(kind: OreqlDiagnosticKind, range: TextRange) -> Self {
    Self {
      kind,
      severity: OreqlDiagnosticSeverity::Warning,
      range,
    }
  }

  #[inline]
  pub fn code_str(&self) -> &'static str {
    self.kind.code_str()
  }

  #[inline]
  pub fn message(&self) -> String {
    self.kind.message()
  }
}
