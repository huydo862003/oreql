mod context;

use crate::{OreqlString, OreqlStringPool, OreqlSyntaxKind, SyntaxTree};

pub use context::ParserContext;
pub use context::default_tokenizer_config;
pub use context::tokenize::{
  TokenizeConfig, TokenizeIdentifierConfig, TokenizePrefixedStringConfig,
  TokenizeQuotedIdentifierConfig, TokenizeStringConfig,
};

use context::default_tokenizer_config::*;

pub trait DialectParser: Sized {
  /* Entrypoint - Don't override */
  fn parse<'a>(_string_pool: &'a OreqlStringPool<'a>) -> SyntaxTree {
    unimplemented!()
  }

  /* Tokenizer methods - override to customize dialect behavior */

  fn tokenize_next(context: &mut ParserContext<'_, Self>) -> (OreqlSyntaxKind, OreqlString) {
    context.eat_next_token(&TokenizeConfig {
      single_byte_tokens: SingleByteTokensConfig::default(),
      operators: OperatorsConfig::default(),
      keywords: KeywordsConfig::default(),
    })
  }

  fn tokenize_whitespace(context: &mut ParserContext<'_, Self>) -> (OreqlSyntaxKind, OreqlString) {
    context.tokenize_whitespace()
  }

  fn tokenize_newline(context: &mut ParserContext<'_, Self>) -> (OreqlSyntaxKind, OreqlString) {
    context.tokenize_newline()
  }

  fn tokenize_number_lit(context: &mut ParserContext<'_, Self>) -> (OreqlSyntaxKind, OreqlString) {
    context.tokenize_number_lit()
  }

  fn tokenize_identifier_or_keyword(
    context: &mut ParserContext<'_, Self>,
  ) -> (OreqlSyntaxKind, OreqlString) {
    context.tokenize_identifier_or_keyword(&TokenizeIdentifierConfig::default())
  }

  fn tokenize_string_lit(context: &mut ParserContext<'_, Self>) -> (OreqlSyntaxKind, OreqlString) {
    context.tokenize_string_lit(&TokenizeStringConfig {
      quotes: StringQuotesConfig::default(),
      escape_bytes: StringEscapeBytesConfig::default(),
    })
  }

  fn tokenize_quoted_identifier(
    context: &mut ParserContext<'_, Self>,
  ) -> (OreqlSyntaxKind, OreqlString) {
    context.tokenize_quoted_identifier(&TokenizeQuotedIdentifierConfig {
      quotes: IdentifierQuotesConfig::default(),
      escape_bytes: IdentifierEscapeBytesConfig::default(),
    })
  }

  fn tokenize_inline_comment(
    context: &mut ParserContext<'_, Self>,
  ) -> (OreqlSyntaxKind, OreqlString) {
    context.tokenize_inline_comment(InlineCommentStartsConfig::default())
  }

  fn tokenize_block_comment(
    context: &mut ParserContext<'_, Self>,
    open: &[u8],
    close: &[u8],
  ) -> (OreqlSyntaxKind, OreqlString) {
    context.tokenize_nested_block_comment(open, close)
  }

  fn tokenize_operator(
    context: &mut ParserContext<'_, Self>,
  ) -> Option<(OreqlSyntaxKind, OreqlString)> {
    context.tokenize_operator(OperatorsConfig::default())
  }

  fn tokenize_prefixed_string(
    context: &mut ParserContext<'_, Self>,
  ) -> Option<(OreqlSyntaxKind, OreqlString)> {
    context.tokenize_prefixed_string(&TokenizePrefixedStringConfig {
      bit_strings: BitStringDelimitersConfig::default(),
      hex_strings: HexStringDelimitersConfig::default(),
      byte_strings: ByteStringDelimitersConfig::default(),
      raw_strings: RawStringDelimitersConfig::default(),
      national_strings: NationalStringDelimitersConfig::default(),
      unicode_strings: UnicodeStringDelimitersConfig::default(),
      heredoc_strings: HeredocDelimitersConfig::default(),
      string_escape_bytes: StringEscapeBytesConfig::default(),
      byte_string_escape_bytes: ByteStringEscapeBytesConfig::default(),
      are_escapes_allowed_in_raw_strings: AreEscapesAllowedInRawStringsConfig::default(),
    })
  }

  fn tokenize_variable(context: &mut ParserContext<'_, Self>) -> (OreqlSyntaxKind, OreqlString) {
    context.tokenize_variable(VariableAllowedBytesConfig::default())
  }
}
