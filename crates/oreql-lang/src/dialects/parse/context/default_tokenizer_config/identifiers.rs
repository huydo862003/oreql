use super::DelimiterPair;
use oreql_utils::BitSet128;

oreql_macros::dialect_config! {
  /// Bytes that can appear inside variable names
  ///
  /// e.g. `.insert(b'@').insert(b'$')` to allow `@var` or `$var`
  pub struct VariableAllowedBytesConfig(BitSet128)
    = BitSet128::new();

  /// (open, close) delimiters for **quoted** identifiers
  ///
  /// e.g. `[("\"", "\""), ("[", "]")]` for `"col"` or `[col]`
  pub struct IdentifierQuotesConfig(&'static [DelimiterPair])
    = &[("\"", "\"")];
}
