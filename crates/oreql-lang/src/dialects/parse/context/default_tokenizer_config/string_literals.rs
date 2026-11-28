use crate::OreqlSyntaxKind;

oreql_macros::dialect_config! {
  /// (open, close) delimiters for **bit** string literals
  ///
  /// e.g. `[("b'", "'"), ("B'", "'")]` for `b'0101'`
  pub struct BitStringDelimitersConfig(&'static [(&'static str, &'static str)])
    = &[];

  /// (open, close) delimiters for **byte** string literals
  ///
  /// e.g. `[("b'", "'"), ("B'", "'")]` for `b'\x00'`
  pub struct ByteStringDelimitersConfig(&'static [(&'static str, &'static str)])
    = &[];

  /// (open, close) delimiters for **hex** string literals
  ///
  /// e.g. `[("x'", "'"), ("X'", "'"), ("0x", "")]` for `x'FF'`
  pub struct HexStringDelimitersConfig(&'static [(&'static str, &'static str)])
    = &[];

  /// (open, close) delimiters for **raw** string literals
  ///
  /// e.g. `[("r'", "'"), ("R'", "'")]` for `r'no\escape'`
  pub struct RawStringDelimitersConfig(&'static [(&'static str, &'static str)])
    = &[];

  /// (open, close) delimiters for **heredoc** strings
  ///
  /// e.g. `[("$$", "$$")]` for `$$body$$`
  pub struct HeredocDelimitersConfig(&'static [(&'static str, &'static str)])
    = &[];

  /// (open, close) delimiters for **national** strings
  ///
  /// e.g. `[("N'", "'"), ("n'", "'")]` for `N'hello'`
  pub struct NationalStringDelimitersConfig(&'static [(&'static str, &'static str)])
    = &[];

  /// (open, close) delimiters for **unicode** strings
  ///
  /// e.g. `[("U&'", "'")]` for `U&'text'`
  pub struct UnicodeStringDelimitersConfig(&'static [(&'static str, &'static str)])
    = &[];

  /// (open, close) delimiters for **regular** string literals
  ///
  /// e.g. `[("'", "'")]` for `'hello'`
  pub struct StringQuotesConfig(&'static [(&'static str, &'static str)])
    = &[("'", "'")];

  /// Token kind emitted when a heredoc prefix has no matching tag
  ///
  /// e.g. `$$` without a closing `$$` falls back to this kind
  pub struct HeredocFallbackKindConfig(OreqlSyntaxKind)
    = OreqlSyntaxKind::Variable;
}
