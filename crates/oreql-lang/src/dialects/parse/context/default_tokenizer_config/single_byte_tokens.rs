use crate::OreqlSyntaxKind;
use OreqlSyntaxKind::*;
use oreql_utils::ByteLut;

/// Maps single ASCII bytes to their token kind for the lexer
pub const DEFAULT_SINGLE_BYTE_TOKENS: ByteLut<OreqlSyntaxKind> = ByteLut::new()
  .set(b'(', LParen)
  .set(b')', RParen)
  .set(b'[', LBracket)
  .set(b']', RBracket)
  .set(b'{', LBrace)
  .set(b'}', RBrace)
  .set(b'&', Amp)
  .set(b'^', Caret)
  .set(b':', Colon)
  .set(b',', Comma)
  .set(b'.', Dot)
  .set(b'-', Minus)
  .set(b'=', Eq)
  .set(b'>', Gt)
  .set(b'<', Lt)
  .set(b'%', Percent)
  .set(b'!', Exclamation)
  .set(b'|', Pipe)
  .set(b'+', Plus)
  .set(b';', Semicolon)
  .set(b'/', Slash)
  .set(b'\\', Backslash)
  .set(b'*', Star)
  .set(b'~', Tilde)
  .set(b'?', Question)
  .set(b'@', At)
  .set(b'#', Hash);
