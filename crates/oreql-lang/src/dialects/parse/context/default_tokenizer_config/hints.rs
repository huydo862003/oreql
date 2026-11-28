use crate::OreqlSyntaxKind;

oreql_macros::dialect_config! {
  /// (start, end) delimiters for **hint** comments
  ///
  /// e.g. `&[("/*+", "*/")]` for `/*+ hint */`
  pub struct HintCommentDelimitersConfig(&'static [(&'static str, &'static str)])
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
