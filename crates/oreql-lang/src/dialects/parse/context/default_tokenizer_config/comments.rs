/// Start patterns for **inline** (single-line) comments
///
/// e.g. `["--"]` for `-- comment to end of line`
pub const DEFAULT_INLINE_COMMENT_START_PATTERNS: &[&str] = &["--"];

/// (start, end) delimiters for **block** (multi-line) comments
///
/// e.g. `[("/*", "*/"), ("{#", "#}")]` for `/* block */` or `{# jinja #}`
pub const DEFAULT_BLOCK_COMMENT_DELIMITER_PATTERNS: &[(&str, &str)] = &[("/*", "*/"), ("{#", "#}")];

/// Whether block comments can nest
pub const DEFAULT_ARE_NESTED_COMMENTS_ALLOWED: bool = true;
