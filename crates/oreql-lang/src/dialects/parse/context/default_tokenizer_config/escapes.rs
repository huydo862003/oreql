use oreql_utils::ByteLut;

oreql_macros::dialect_config! {
  /// Bytes that act as escape characters inside **strings**
  pub struct StringEscapeBytesConfig(ByteLut<bool>)
    = ByteLut::new().set(b'\'', true);

  /// Bytes that act as escape characters inside **byte strings**
  pub struct ByteStringEscapeBytesConfig(ByteLut<bool>)
    = ByteLut::new().set(b'\'', true);

  /// Bytes that act as escape characters inside **identifiers**
  pub struct IdentifierEscapeBytesConfig(ByteLut<bool>)
    = ByteLut::new();

  /// Bytes that can follow an escape character
  ///
  /// e.g. `.set(b'n', true).set(b't', true)` for `\n`, `\t`
  pub struct EscapeFollowBytesConfig(ByteLut<bool>)
    = ByteLut::new();

  /// Whether escape characters work inside raw strings
  pub struct AreEscapesAllowedInRawStringsConfig(bool) = true;
}
