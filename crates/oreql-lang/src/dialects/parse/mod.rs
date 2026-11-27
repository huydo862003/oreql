mod context;

use crate::{GreenTree, OreqlStringPool};

pub use context::ParserContext;

pub trait DialectParser {
  fn parse<'a>(string_pool: &'a mut OreqlStringPool<'a>) -> GreenTree {
    let mut parser: ParserContext<'_, Self> = ParserContext::new(string_pool);
    parser.parse()
  }
}
