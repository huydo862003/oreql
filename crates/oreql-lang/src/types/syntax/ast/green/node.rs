use crate::types::TextRange;

use super::super::OreqlSyntaxKind;

oreql_macros::define_multi_arena! {
  pub struct OreqlGreenNode {
    kind: OreqlSyntaxKind,
    data: GreenNodeData,
  }
}

/// Data payload for a green node
///
/// For tokens, this is a text range
/// For internal nodes, this is a children range
#[derive(Clone, Copy)]
#[repr(C)]
pub union GreenNodeData {
  pub text_range: TextRange,
  pub children: ChildrenRange,
}

impl OreqlGreenNode {
  /// Get children range without checking the kind
  ///
  /// # Safety
  ///
  /// Must be an internal node
  pub unsafe fn node_children_range_unchecked(
    &self,
    arena: &OreqlGreenNodeMultiArena,
  ) -> ChildrenRange {
    unsafe { self.data(arena).children }
  }

  /// Get text range of an internal node from its children
  ///
  /// # Safety
  ///
  /// Must be an internal node
  #[inline]
  pub unsafe fn node_text_range_unchecked(&self, arena: &OreqlGreenNodeMultiArena) -> TextRange {
    let children = unsafe { self.data(arena).children };
    if children.child_count == 0 {
      return TextRange::default();
    }
    let first = OreqlGreenNode::from_index(children.first_child);
    let last = OreqlGreenNode::from_index(children.first_child + children.child_count - 1);
    TextRange {
      start_byte_offset: first.text_span(arena).start_byte_offset,
      end_byte_offset: last.text_span(arena).end_byte_offset,
    }
  }

  /// Get text range without checking the kind
  ///
  /// # Safety
  ///
  /// Must be a token
  pub unsafe fn token_text_range_unchecked(&self, arena: &OreqlGreenNodeMultiArena) -> TextRange {
    unsafe { self.data(arena).text_range }
  }

  /// Text span covering this node
  #[inline]
  pub fn text_span(&self, arena: &OreqlGreenNodeMultiArena) -> TextRange {
    if self.kind(arena).is_token() {
      return unsafe { self.data(arena).text_range };
    }
    let children = unsafe { self.data(arena).children };
    if children.child_count == 0 {
      return TextRange::default();
    }
    let first = OreqlGreenNode::from_index(children.first_child);
    let last = OreqlGreenNode::from_index(children.first_child + children.child_count - 1);
    TextRange {
      start_byte_offset: first.text_span(arena).start_byte_offset,
      end_byte_offset: last.text_span(arena).end_byte_offset,
    }
  }

  /// Iterate direct children in the arena
  pub fn children(&self, arena: &OreqlGreenNodeMultiArena) -> GreenNodeChildrenIter {
    if self.kind(arena).is_node() {
      let range = unsafe { self.data(arena).children };
      GreenNodeChildrenIter {
        current: range.first_child,
        end: range.first_child + range.child_count,
      }
    } else {
      GreenNodeChildrenIter { current: 0, end: 0 }
    }
  }
}

/// Iterator over a green node's direct children
pub struct GreenNodeChildrenIter {
  current: u32,
  end: u32,
}

impl Iterator for GreenNodeChildrenIter {
  type Item = OreqlGreenNode;

  #[inline]
  fn next(&mut self) -> Option<OreqlGreenNode> {
    if self.current >= self.end {
      return None;
    }
    let handle = OreqlGreenNode::from_index(self.current);
    self.current += 1;
    Some(handle)
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    let remaining = (self.end - self.current) as usize;
    (remaining, Some(remaining))
  }
}

impl ExactSizeIterator for GreenNodeChildrenIter {}

/// Range of children in the arena for internal nodes
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct ChildrenRange {
  pub first_child: u32,
  pub child_count: u32,
}
