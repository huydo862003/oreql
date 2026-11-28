pub mod default_tokenizer_config;
mod tokenize;

use crate::{
  GreenTree, GreenTreeBuilder, OreqlDiagnostic, OreqlDiagnosticKind, OreqlStringPool, TextRange,
};

pub struct ParserContext<'a> {
  pub string_pool: &'a OreqlStringPool<'a>,
  pub builder: GreenTreeBuilder,
  pub diagnostics: Vec<OreqlDiagnostic>,
  current_byte_offset: usize,
}

impl<'a> ParserContext<'a> {
  pub fn new(string_pool: &'a OreqlStringPool<'a>) -> Self {
    ParserContext {
      string_pool,
      builder: GreenTreeBuilder::new(),
      current_byte_offset: 0,
    }
  }

  pub fn parse(&mut self) -> GreenTree {
    unimplemented!()
  }
}
