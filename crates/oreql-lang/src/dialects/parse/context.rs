use std::marker::PhantomData;

use crate::{GreenTree, GreenTreeBuilder, OreqlStringPool, dialects::parse::DialectParser};

pub struct ParserContext<'a, D: DialectParser + ?Sized> {
  pub string_pool: &'a mut OreqlStringPool<'a>,
  pub builder: GreenTreeBuilder,
  _marker: PhantomData<D>,
}

// Define parse option on D to enable monomorphization optimization
impl<'a, D: DialectParser + ?Sized> ParserContext<'a, D> {
  pub fn new(string_pool: &'a mut OreqlStringPool<'a>) -> Self {
    ParserContext {
      string_pool,
      builder: GreenTreeBuilder::new(),
      _marker: PhantomData::default(),
    }
  }

  pub fn parse(&mut self) -> GreenTree {
    unimplemented!()
  }
}
