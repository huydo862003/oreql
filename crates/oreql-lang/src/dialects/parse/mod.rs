mod context;

use crate::{GreenTree, OreqlStringPool, OreqlSyntaxKind};
use OreqlSyntaxKind::*;
use oreql_utils::ByteLut;

pub use context::ParserContext;

pub trait DialectParser {
  /* Entrypoint */

  fn parse<'a>(string_pool: &'a mut OreqlStringPool<'a>) -> GreenTree {
    let mut parser: ParserContext<'_, Self> = ParserContext::new(string_pool);
    parser.parse()
  }


  /* Tokenizer config */

  /** Single-byte tokens */

  /// Maps single ASCII bytes to their token kind for the lexer
  const SINGLE_BYTE_TOKENS: ByteLut<OreqlSyntaxKind> = ByteLut::new()
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

  /** String literals */

  /// (open, close) delimiters for **bit** string literals
  ///
  /// e.g. `[("b'", "'"), ("B'", "'")]` for `b'0101'`
  const BIT_STRING_DELIMITER_PATTERNS: &[(&str, &str)] = &[];

  /// (open, close) delimiters for **byte** string literals
  ///
  /// e.g. `[("b'", "'"), ("B'", "'")]` for `b'\x00'`
  const BYTE_STRING_DELIMITER_PATTERNS: &[(&str, &str)] = &[];

  /// (open, close) delimiters for **hex** string literals
  ///
  /// e.g. `[("x'", "'"), ("X'", "'"), ("0x", "")]` for `x'FF'`
  const HEX_STRING_DELIMITER_PATTERNS: &[(&str, &str)] = &[];

  /// (open, close) delimiters for **raw** string literals
  ///
  /// e.g. `[("r'", "'"), ("R'", "'")]` for `r'no\escape'`
  const RAW_STRING_DELIMITER_PATTERNS: &[(&str, &str)] = &[];

  /// (open, close) delimiters for **heredoc** strings
  ///
  /// e.g. `[("$$", "$$")]` for `$$body$$`
  const HEREDOC_DELIMITER_PATTERNS: &[(&str, &str)] = &[];

  /// (open, close) delimiters for **unicode** strings
  ///
  /// e.g. `[("U&'", "'")]` for `U&'text'`
  const UNICODE_STRING_DELIMITER_PATTERNS: &[(&str, &str)] = &[];

  /// (open, close) delimiters for **regular** string literals
  ///
  /// e.g. `[("'", "'")]` for `'hello'`
  const STRING_QUOTE_PATTERNS: &[(&str, &str)] = &[("'", "'")];

  /// Token kind emitted when a heredoc prefix has no matching tag
  ///
  /// e.g. `$$` without a closing `$$` falls back to this kind
  const HEREDOC_FALLBACK_KIND: OreqlSyntaxKind = Variable;

  /** Identifiers */

  /// (open, close) delimiters for **quoted** identifiers
  ///
  /// e.g. `[("\"", "\""), ("[", "]")]` for `"col"` or `[col]`
  const IDENTIFIER_QUOTE_PATTERNS: &[(&str, &str)] = &[("\"", "\"")];

  /// Bytes that can appear inside variable names
  ///
  /// e.g. `&[b'@', b'$']` to allow `@var` or `$var`
  const WITHIN_VARIABLE_ALLOWED_BYTES: &[u8] = &[];

  /** Escape characters */

  /// Bytes that act as escape characters inside **strings**
  const STRING_ESCAPE_BYTES: &[u8] = &[b'\''];

  /// Bytes that act as escape characters inside **byte strings**
  const BYTE_STRING_ESCAPE_BYTES: &[u8] = &[b'\''];

  /// Bytes that act as escape characters inside **identifiers**
  const IDENTIFIER_ESCAPE_BYTES: &[u8] = &[];

  /// Bytes that can follow an escape character
  ///
  /// e.g. `&[b'n', b't', b'r', b'\\']` for `\n`, `\t`, `\r`
  const ESCAPE_FOLLOW_ALLOWED_BYTES: &[u8] = &[];

  /// Whether escape characters work inside raw strings
  const ARE_ESCAPES_ALLOWED_IN_RAW_STRINGS: bool = true;

  /** Comments */

  /// Start patterns for **inline** (single-line) comments
  ///
  /// e.g. `["--"]` for `-- comment to end of line`
  const INLINE_COMMENT_START_PATTERNS: &[&str] = &["--"];

  /// (start, end) delimiters for **block** (multi-line) comments
  ///
  /// e.g. `[("/*", "*/"), ("{#", "#}")]` for `/* block */` or `{# jinja #}`
  const BLOCK_COMMENT_DELIMITER_PATTERNS: &[(&str, &str)] = &[("/*", "*/"), ("{#", "#}")];

  /// Whether block comments can nest
  const ARE_NESTED_COMMENTS_ALLOWED: bool = true;

  /** Hints */

  /// (start, end) delimiters for **hint** comments
  ///
  /// e.g. `&[("/*+", "*/")]` for `/*+ hint */`
  const HINT_COMMENT_DELIMITER_PATTERNS: &[(&str, &str)] = &[];

  /// Token kinds that can appear before a hint comment
  const HINT_PRECEDING_ALLOWED_TOKENS: &[OreqlSyntaxKind] =
    &[KeywordSelect, KeywordInsert, KeywordUpdate, KeywordDelete];

  /* Method overrides */
}
