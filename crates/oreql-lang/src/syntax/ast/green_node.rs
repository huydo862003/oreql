use crate::types::TextRange;

use super::OreqlSyntaxKind;

oreql_utils::define_multi_arena! {
  pub struct OreqlGreenNode {
    kind: OreqlSyntaxKind,
    text_span: TextRange,
  }
}
