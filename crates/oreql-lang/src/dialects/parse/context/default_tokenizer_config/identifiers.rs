use oreql_utils::ByteLut;

oreql_macros::dialect_config! {
  /// Bytes that can appear inside variable names
  ///
  /// e.g. `.set(b'@', true).set(b'$', true)` to allow `@var` or `$var`
  pub struct VariableAllowedBytesConfig(ByteLut<bool>)
    = ByteLut::new();

  /// (open, close) delimiters for **quoted** identifiers
  ///
  /// e.g. `[("\"", "\""), ("[", "]")]` for `"col"` or `[col]`
  pub struct IdentifierQuotesConfig(&'static [(&'static str, &'static str)])
    = &[("\"", "\"")];
}
