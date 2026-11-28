use std::cell::UnsafeCell;

use crate::types::TextRange;
use crate::types::syntax::ast::OreqlSyntaxKind;
use crate::types::syntax::ast::green::{
  GreenNodeChildrenIter, GreenTree, OreqlGreenNode, OreqlGreenNodeMultiArena,
};

oreql_macros::define_multi_arena! {
  /// Red node: parent red index + child position + green index
  ///
  /// Lazily populated as the tree is navigated top-down
  pub struct OreqlRedNode {
    _parent: u32,
    _child_index: u32,
    _green: u32,
  }
}

const NO_PARENT: u32 = u32::MAX;

/// Red tree: owns a green tree + lazily growing red arena
pub struct OreqlRedTree {
  green: GreenTree,
  /* Safety: this is private & cannot be shared across multiple threads */
  red: UnsafeCell<OreqlRedNodeMultiArena>,
}

impl OreqlRedTree {
  pub fn new(green: GreenTree) -> Self {
    let mut red = OreqlRedNodeMultiArena::new();
    red.push(NO_PARENT, 0, green.root.index());
    Self {
      green,
      red: UnsafeCell::new(red),
    }
  }

  /// Root red node
  #[inline]
  pub fn root(&self) -> OreqlRedNode {
    OreqlRedNode::from_index(0)
  }

  #[inline]
  fn green_arena(&self) -> &OreqlGreenNodeMultiArena {
    &self.green.nodes
  }

  #[inline]
  fn red_arena(&self) -> &OreqlRedNodeMultiArena {
    unsafe { &*self.red.get() }
  }

  #[inline]
  fn red_arena_mut(&self) -> &mut OreqlRedNodeMultiArena {
    unsafe { &mut *self.red.get() }
  }
}

impl OreqlRedNode {
  /// Green node handle
  #[inline]
  pub fn green(self, tree: &OreqlRedTree) -> OreqlGreenNode {
    OreqlGreenNode::from_index(self._green(tree.red_arena()))
  }

  /// Syntax kind
  #[inline]
  pub fn kind(self, tree: &OreqlRedTree) -> OreqlSyntaxKind {
    self.green(tree).kind(tree.green_arena())
  }

  /// Text span
  #[inline]
  pub fn text_span(self, tree: &OreqlRedTree) -> TextRange {
    self.green(tree).text_span(tree.green_arena())
  }

  /// Whether this is the root
  #[inline]
  pub fn is_root(self, tree: &OreqlRedTree) -> bool {
    self._parent(tree.red_arena()) == NO_PARENT
  }

  /// Parent node
  ///
  /// Returns None for root
  #[inline]
  pub fn parent(self, tree: &OreqlRedTree) -> Option<OreqlRedNode> {
    let parent_index = self._parent(tree.red_arena());
    if parent_index == NO_PARENT {
      return None;
    }
    Some(OreqlRedNode::from_index(parent_index))
  }

  /// Index of this node within its parent's children
  #[inline]
  pub fn child_position(self, tree: &OreqlRedTree) -> u32 {
    self._child_index(tree.red_arena())
  }

  /// Iterator over direct children
  ///
  /// Pushes each child into the red arena on iteration
  #[inline]
  pub fn children(self, tree: &OreqlRedTree) -> RedNodeChildrenIterator<'_> {
    RedNodeChildrenIterator {
      tree,
      parent_red: self.index(),
      inner: self.green(tree).children(tree.green_arena()),
      position: 0,
    }
  }

  /// Number of direct children
  #[inline]
  pub fn child_count(self, tree: &OreqlRedTree) -> usize {
    self.green(tree).children(tree.green_arena()).len()
  }

  /// Get the nth child
  pub fn nth_child(self, tree: &OreqlRedTree, index: usize) -> Option<OreqlRedNode> {
    self.children(tree).nth(index)
  }

  /// First child
  #[inline]
  pub fn first_child(self, tree: &OreqlRedTree) -> Option<OreqlRedNode> {
    self.children(tree).next()
  }

  /// Last child
  #[inline]
  pub fn last_child(self, tree: &OreqlRedTree) -> Option<OreqlRedNode> {
    self.children(tree).last()
  }

  /// Next sibling
  #[inline]
  pub fn next_sibling(self, tree: &OreqlRedTree) -> Option<OreqlRedNode> {
    let parent = self.parent(tree)?;
    parent.nth_child(tree, self.child_position(tree) as usize + 1)
  }

  /// Previous sibling
  #[inline]
  pub fn previous_sibling(self, tree: &OreqlRedTree) -> Option<OreqlRedNode> {
    let position = self.child_position(tree);
    if position == 0 {
      return None;
    }
    let parent = self.parent(tree)?;
    parent.nth_child(tree, position as usize - 1)
  }

  /// Whether this is a token (leaf)
  #[inline]
  pub fn is_token(self, tree: &OreqlRedTree) -> bool {
    self.kind(tree).is_token()
  }

  /// Whether this is an internal node
  #[inline]
  pub fn is_node(self, tree: &OreqlRedTree) -> bool {
    self.kind(tree).is_node()
  }
}

/// Iterator over red node children
pub struct RedNodeChildrenIterator<'a> {
  tree: &'a OreqlRedTree,
  parent_red: u32,
  inner: GreenNodeChildrenIter,
  position: u32,
}

impl Iterator for RedNodeChildrenIterator<'_> {
  type Item = OreqlRedNode;

  #[inline]
  fn next(&mut self) -> Option<OreqlRedNode> {
    let green = self.inner.next()?;
    let index = self.position;
    self.position += 1;
    let red = self
      .tree
      .red_arena_mut()
      .push(self.parent_red, index, green.index());
    Some(red)
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    self.inner.size_hint()
  }
}

impl ExactSizeIterator for RedNodeChildrenIterator<'_> {}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::syntax::ast::green::GreenTreeBuilder;

  /// Navigate parent and children on a flat tree
  #[test]
  fn parent_child_navigation() {
    let mut builder = GreenTreeBuilder::new();
    builder.start_node(OreqlSyntaxKind::Program);
    builder.push_token(
      OreqlSyntaxKind::KeywordSelect,
      TextRange {
        start_byte_offset: 0,
        end_byte_offset: 6,
      },
    );
    builder.push_token(
      OreqlSyntaxKind::Whitespace,
      TextRange {
        start_byte_offset: 6,
        end_byte_offset: 7,
      },
    );
    builder.push_token(
      OreqlSyntaxKind::Number,
      TextRange {
        start_byte_offset: 7,
        end_byte_offset: 8,
      },
    );
    builder.finish_node(OreqlSyntaxKind::Program);
    let tree = OreqlRedTree::new(builder.finish());

    let root = tree.root();
    assert_eq!(root.kind(&tree), OreqlSyntaxKind::Program);
    assert!(root.is_root(&tree));
    assert!(root.parent(&tree).is_none());
    assert_eq!(root.child_count(&tree), 3);

    let first = root.first_child(&tree).unwrap();
    assert_eq!(first.kind(&tree), OreqlSyntaxKind::KeywordSelect);
    assert_eq!(first.child_position(&tree), 0);
    assert_eq!(
      first.parent(&tree).unwrap().kind(&tree),
      OreqlSyntaxKind::Program
    );

    let second = first.next_sibling(&tree).unwrap();
    assert_eq!(second.kind(&tree), OreqlSyntaxKind::Whitespace);
    assert_eq!(second.child_position(&tree), 1);

    let third = second.next_sibling(&tree).unwrap();
    assert_eq!(third.kind(&tree), OreqlSyntaxKind::Number);
    assert_eq!(third.child_position(&tree), 2);
    assert!(third.next_sibling(&tree).is_none());

    let back = third.previous_sibling(&tree).unwrap();
    assert_eq!(back.kind(&tree), OreqlSyntaxKind::Whitespace);
  }

  /// Empty root
  #[test]
  fn root_has_no_parent() {
    let mut builder = GreenTreeBuilder::new();
    builder.start_node(OreqlSyntaxKind::Program);
    builder.finish_node(OreqlSyntaxKind::Program);
    let tree = OreqlRedTree::new(builder.finish());

    let root = tree.root();
    assert!(root.is_root(&tree));
    assert!(root.parent(&tree).is_none());
    assert_eq!(root.child_count(&tree), 0);
  }

  /// Text spans accessible through red nodes
  #[test]
  fn text_span_through_red() {
    let mut builder = GreenTreeBuilder::new();
    builder.start_node(OreqlSyntaxKind::Program);
    builder.push_token(
      OreqlSyntaxKind::Number,
      TextRange {
        start_byte_offset: 0,
        end_byte_offset: 2,
      },
    );
    builder.finish_node(OreqlSyntaxKind::Program);
    let tree = OreqlRedTree::new(builder.finish());

    let token = tree.root().first_child(&tree).unwrap();
    let span = token.text_span(&tree);
    assert_eq!(span.start_byte_offset, 0);
    assert_eq!(span.end_byte_offset, 2);
  }
}
