use super::DelimiterPair;
use crate::OreqlSyntaxKind;

oreql_macros::dialect_config! {
  /// (start, end) delimiters for **hint** comments
  ///
  /// e.g. `&[("/*+", "*/")]` for `/*+ hint */`
  pub struct HintCommentDelimitersConfig(&'static [DelimiterPair])
    = &[];

  /// Token kinds that can appear before a hint comment
  pub struct HintPrecedingTokensConfig(&'static [OreqlSyntaxKind])
    = &[
      OreqlSyntaxKind::KeywordSelect,
      OreqlSyntaxKind::KeywordInsert,
      OreqlSyntaxKind::KeywordUpdate,
      OreqlSyntaxKind::KeywordDelete,
    ];
}
