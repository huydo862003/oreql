pub mod default_tokenizer_config;

use crate::{GreenTree, GreenTreeBuilder, OreqlStringPool};

pub struct ParserContext<'a> {
  pub string_pool: &'a mut OreqlStringPool<'a>,
  pub builder: GreenTreeBuilder,
}

impl<'a> ParserContext<'a> {
  pub fn new(string_pool: &'a mut OreqlStringPool<'a>) -> Self {
    ParserContext {
      string_pool,
      builder: GreenTreeBuilder::new(),
    }
  }

  pub fn parse(&mut self) -> GreenTree {
    unimplemented!()
  }
}
