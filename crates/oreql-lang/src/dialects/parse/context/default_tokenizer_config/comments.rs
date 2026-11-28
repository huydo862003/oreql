use super::DelimiterPair;

oreql_macros::dialect_config! {
  /// Start patterns for **inline** (single-line) comments
  ///
  /// e.g. `["--"]` for `-- comment to end of line`
  pub struct InlineCommentStartsConfig(&'static [&'static str])
    = &["--"];

  /// (start, end) delimiters for **block** (multi-line) comments
  ///
  /// e.g. `[("/*", "*/"), ("{#", "#}")]` for `/* block */` or `{# jinja #}`
  pub struct BlockCommentDelimitersConfig(&'static [DelimiterPair])
    = &[("/*", "*/"), ("{#", "#}")];
}
