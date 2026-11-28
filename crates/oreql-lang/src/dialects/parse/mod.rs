mod context;

use crate::{GreenTree, OreqlStringPool};

pub use context::ParserContext;
pub use context::default_tokenizer_config;

pub trait DialectParser {
  fn parse<'a>(string_pool: &'a mut OreqlStringPool<'a>) -> GreenTree;

  /* Method overrides */
}
