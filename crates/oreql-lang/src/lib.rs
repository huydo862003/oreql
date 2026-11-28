extern crate self as oreql_lang;

pub mod dialects;
pub mod types;

pub use types::TextRange;
pub use types::{
  AstNode, ChildrenSpan, OreqlDiagnostic, OreqlDiagnosticKind, OreqlDiagnosticSeverity,
  OreqlString, OreqlStringPool, OreqlSyntaxKind, OreqlSyntaxNode, OreqlSyntaxNodeMultiArena,
  SyntaxNodeChildrenIterator, SyntaxNodeData, SyntaxTree, SyntaxTreeBuilder,
};
