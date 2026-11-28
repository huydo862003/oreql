use oreql_utils::BitSet128;

oreql_macros::dialect_config! {
  /// Bytes that act as escape characters inside **strings**
  pub struct StringEscapeBytesConfig(BitSet128)
    = BitSet128::new().insert(b'\'');

  /// Bytes that act as escape characters inside **byte strings**
  pub struct ByteStringEscapeBytesConfig(BitSet128)
    = BitSet128::new().insert(b'\'');

  /// Bytes that act as escape characters inside **identifiers**
  pub struct IdentifierEscapeBytesConfig(BitSet128)
    = BitSet128::new();

  /// Bytes that can follow an escape character
  ///
  /// e.g. `.insert(b'n').insert(b't')` for `\n`, `\t`
  pub struct EscapeFollowBytesConfig(BitSet128)
    = BitSet128::new();

  /// Whether escape characters work inside raw strings
  pub struct AreEscapesAllowedInRawStringsConfig(bool) = true;
}
