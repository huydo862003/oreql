use crate::OreqlSyntaxKind;

/// (open, close) delimiters for **bit** string literals
///
/// e.g. `[("b'", "'"), ("B'", "'")]` for `b'0101'`
pub const DEFAULT_BIT_STRING_DELIMITER_PATTERNS: &[(&str, &str)] = &[];

/// (open, close) delimiters for **byte** string literals
///
/// e.g. `[("b'", "'"), ("B'", "'")]` for `b'\x00'`
pub const DEFAULT_BYTE_STRING_DELIMITER_PATTERNS: &[(&str, &str)] = &[];

/// (open, close) delimiters for **hex** string literals
///
/// e.g. `[("x'", "'"), ("X'", "'"), ("0x", "")]` for `x'FF'`
pub const DEFAULT_HEX_STRING_DELIMITER_PATTERNS: &[(&str, &str)] = &[];

/// (open, close) delimiters for **raw** string literals
///
/// e.g. `[("r'", "'"), ("R'", "'")]` for `r'no\escape'`
pub const DEFAULT_RAW_STRING_DELIMITER_PATTERNS: &[(&str, &str)] = &[];

/// (open, close) delimiters for **heredoc** strings
///
/// e.g. `[("$$", "$$")]` for `$$body$$`
pub const DEFAULT_HEREDOC_DELIMITER_PATTERNS: &[(&str, &str)] = &[];

/// (open, close) delimiters for **unicode** strings
///
/// e.g. `[("U&'", "'")]` for `U&'text'`
pub const DEFAULT_UNICODE_STRING_DELIMITER_PATTERNS: &[(&str, &str)] = &[];

/// (open, close) delimiters for **regular** string literals
///
/// e.g. `[("'", "'")]` for `'hello'`
pub const DEFAULT_STRING_QUOTE_PATTERNS: &[(&str, &str)] = &[("'", "'")];

/// Token kind emitted when a heredoc prefix has no matching tag
///
/// e.g. `$$` without a closing `$$` falls back to this kind
pub const DEFAULT_HEREDOC_FALLBACK_KIND: OreqlSyntaxKind = OreqlSyntaxKind::Variable;
