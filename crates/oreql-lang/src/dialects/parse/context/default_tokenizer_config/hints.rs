use crate::OreqlSyntaxKind;

/// (start, end) delimiters for **hint** comments
///
/// e.g. `&[("/*+", "*/")]` for `/*+ hint */`
pub const DEFAULT_HINT_COMMENT_DELIMITER_PATTERNS: &[(&str, &str)] = &[];

/// Token kinds that can appear before a hint comment
pub const DEFAULT_HINT_PRECEDING_ALLOWED_TOKENS: &[OreqlSyntaxKind] = &[
  OreqlSyntaxKind::KeywordSelect,
  OreqlSyntaxKind::KeywordInsert,
  OreqlSyntaxKind::KeywordUpdate,
  OreqlSyntaxKind::KeywordDelete,
];
