use crate::TextRange;

#[oreql_macros::diagnostic_kind]
pub enum OreqlDiagnosticKind {
  /* Tokenizer */

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
  pub fn error(kind: OreqlDiagnosticKind, range: TextRange) -> Self {
    Self {
      kind,
      severity: OreqlDiagnosticSeverity::Error,
      range,
    }
  }

  pub fn warning(kind: OreqlDiagnosticKind, range: TextRange) -> Self {
    Self {
      kind,
      severity: OreqlDiagnosticSeverity::Warning,
      range,
    }
  }

  pub fn code_str(&self) -> &'static str {
    self.kind.code_str()
  }

  pub fn message(&self) -> String {
    self.kind.message()
  }
}
