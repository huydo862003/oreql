use crate::types::TextRange;

use super::super::OreqlSyntaxKind;

oreql_utils::define_multi_arena! {
  pub struct OreqlGreenNode {
    kind: OreqlSyntaxKind,
    data: GreenNodeData,
  }
}

impl OreqlGreenNode {
  /// Get children range without checking the kind
  ///
  /// # Safety
  ///
  /// Must be an internal node
  #[inline]
  pub unsafe fn node_children_range_unchecked(&self, arena: &OreqlGreenNodeMultiArena) -> ChildrenRange {
    unsafe { self.data(arena).children }
  }

  /// Get text range of an internal node from its children
  ///
  /// # Safety
  ///
  /// Must be an internal node
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
  #[inline]
  pub unsafe fn token_text_range_unchecked(&self, arena: &OreqlGreenNodeMultiArena) -> TextRange {
    unsafe { self.data(arena).text_range }
  }

  /// Text span covering this node
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
  #[inline]
  pub fn children(&self, arena: &OreqlGreenNodeMultiArena) -> GreenNodeChildren {
    if self.kind(arena).is_node() {
      let range = unsafe { self.data(arena).children };
      GreenNodeChildren {
        current: range.first_child,
        end: range.first_child + range.child_count,
      }
    } else {
      GreenNodeChildren { current: 0, end: 0 }
    }
  }
}

/// Iterator over a green node's direct children
pub struct GreenNodeChildren {
  current: u32,
  end: u32,
}

impl Iterator for GreenNodeChildren {
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

impl ExactSizeIterator for GreenNodeChildren {}

/// Range of children in the arena for internal nodes
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct ChildrenRange {
  pub first_child: u32,
  pub child_count: u32,
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

impl Default for GreenNodeData {
  fn default() -> Self {
    Self {
      text_range: TextRange::default(),
    }
  }
}

impl GreenNodeData {
  #[inline]
  pub fn from_text_range(text_range: TextRange) -> Self {
    Self { text_range }
  }

  #[inline]
  pub fn from_children(children: ChildrenRange) -> Self {
    Self { children }
  }
}
