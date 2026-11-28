use crate::OreqlSyntaxKind;
use OreqlSyntaxKind::*;
use oreql_utils::{BitSet128, ByteLut, TrieMap};

oreql_macros::dialect_config! {
  /// Single-byte tokens for **JSON path** tokenization
  ///
  /// Reduced set compared to SQL: only path-relevant characters
  pub struct JsonPathSingleByteTokensConfig(ByteLut<OreqlSyntaxKind>) = ByteLut::new()
    .set(b'(', LParen)
    .set(b')', RParen)
    .set(b'[', LBracket)
    .set(b']', RBracket)
    .set(b':', Colon)
    .set(b',', Comma)
    .set(b'-', Minus)
    .set(b'.', Dot)
    .set(b'?', Question)
    .set(b'@', At)
    .set(b'$', Dollar)
    .set(b'*', Star);

  /// Operators for **JSON path** tokenization
  ///
  /// Only `..` (recursive descent) is a multi-char operator
  pub struct JsonPathOperatorsConfig(TrieMap<OreqlSyntaxKind>) = TrieMap::from_entries([
    (&b".."[..], Dot),
  ]);

  /// Escape bytes for **JSON path** strings
  pub struct JsonPathStringEscapeBytesConfig(BitSet128) =
    BitSet128::new().insert(b'\\');

  /// Escape bytes for **JSON path** identifiers
  pub struct JsonPathIdentifierEscapeBytesConfig(BitSet128) =
    BitSet128::new().insert(b'\\');
}
