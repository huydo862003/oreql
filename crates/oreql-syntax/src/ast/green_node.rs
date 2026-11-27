use std::ops::Deref;

use oreql_types::TextRange;

use crate::OreqlSyntaxKind;

/// Index to the pool
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct OreqlGreenNode(u32);

impl OreqlGreenNode {
  #[inline]
  pub fn kind(&self, pool: &OreqlGreenNodePool) -> OreqlSyntaxKind {
    *pool
      .kinds
      .get(self.0 as usize)
      .expect("Out-of-bound index to OreqlSyntaxTokenPool's kinds")
  }

  #[inline]
  pub unsafe fn kind_unchecked(&self, pool: &OreqlGreenNodePool) -> OreqlSyntaxKind {
    unsafe { *pool.kinds.get_unchecked(self.0 as usize) }
  }

  #[inline]
  pub fn text_span(&self, pool: &OreqlGreenNodePool) -> TextRange {
    *pool
      .text_spans
      .get(self.0 as usize)
      .expect("Out-of-bound index to OreqlSyntaxTokenPool's text_spans")
  }

  #[inline]
  pub unsafe fn text_span_unchecked(&self, pool: &OreqlGreenNodePool) -> TextRange {
    unsafe { *pool.text_spans.get_unchecked(self.0 as usize) }
  }
}

/// The pool for green nodes
pub struct OreqlGreenNodePool {
  kinds: Vec<OreqlSyntaxKind>,
  text_spans: Vec<TextRange>,
  index: u32,
}

impl OreqlGreenNodePool {
  #[inline]
  pub fn push(&mut self, kind: OreqlSyntaxKind, text_span: TextRange) {
    let len = self.kinds.len();
    let index = self.index;
    self.index += 1;

    if index as usize >= len {
      self.kinds.push(kind);
      self.text_spans.push(text_span);
    } else {
      self.kinds[index as usize] = kind;
      self.text_spans[index as usize] = text_span;
    }
  }

  #[inline]
  pub fn scratch_space<'a>(&'a mut self) -> impl Deref<Target = u32> + 'a {
    let index = self.index;
    return IndexGuard { pool: self, index };
  }

  #[inline]
  pub fn clear(&mut self) {
    self.index = 0;
  }
}

struct IndexGuard<'a> {
  pool: &'a mut OreqlGreenNodePool,
  index: u32,
}

impl<'a> Deref for IndexGuard<'a> {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.index
  }
}

impl<'a> Drop for IndexGuard<'a> {
  fn drop(&mut self) {
    self.pool.index = self.index;
  }
}
