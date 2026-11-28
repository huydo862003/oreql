use crate::types::TextRange;
use crate::types::syntax::ast::OreqlSyntaxKind;

oreql_macros::define_multi_arena! {
  /// Syntax node: kind + data + parent
  pub struct OreqlSyntaxNode {
    _kind: OreqlSyntaxKind,
    _data: SyntaxNodeData,
    _parent: u32,
  }
}

pub(super) const NO_PARENT: u32 = u32::MAX;

/// Data payload for a syntax node
///
/// - Tokens: text range (byte offsets into source)
/// - Internal nodes: children span (range into children index vector)
#[derive(Clone, Copy)]
#[repr(C)]
pub union SyntaxNodeData {
  pub text_range: TextRange,
  pub children_span: ChildrenSpan,
}

/// Span into the children index vector
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct ChildrenSpan {
  pub start: u32,
  pub length: u32,
}

/// Complete syntax tree: node arena + children index vector
pub struct SyntaxTree {
  pub nodes: OreqlSyntaxNodeMultiArena,
  pub children: Vec<u32>,
  pub root: OreqlSyntaxNode,
}

impl OreqlSyntaxNode {
  /// Syntax kind
  #[inline]
  pub fn kind(self, tree: &SyntaxTree) -> OreqlSyntaxKind {
    self._kind(&tree.nodes)
  }

  /// Whether this is a token (leaf)
  #[inline]
  pub fn is_token(self, tree: &SyntaxTree) -> bool {
    self.kind(tree).is_token()
  }

  /// Whether this is an internal node
  #[inline]
  pub fn is_node(self, tree: &SyntaxTree) -> bool {
    self.kind(tree).is_node()
  }

  /// Text range for tokens
  ///
  /// INVARIANT: must be a token
  #[inline]
  pub fn text_range(self, tree: &SyntaxTree) -> TextRange {
    debug_assert!(self.is_token(tree), "text_range called on internal node");
    unsafe { self._data(&tree.nodes).text_range }
  }

  /// Text span covering this node (tokens or internal)
  ///
  /// - For tokens, returns the stored text range directly
  /// - For internal nodes, walks to the leftmost and rightmost leaf tokens
  #[inline]
  pub fn text_span(self, tree: &SyntaxTree) -> TextRange {
    if self.is_token(tree) {
      return unsafe { self._data(&tree.nodes).text_range };
    }
    let span = unsafe { self._data(&tree.nodes).children_span };
    if span.length == 0 {
      return TextRange::default();
    }
    // Walk to leftmost leaf
    let mut first = OreqlSyntaxNode::from_index(tree.children[span.start as usize]);
    while first.is_node(tree) {
      let child_span = unsafe { first._data(&tree.nodes).children_span };
      if child_span.length == 0 {
        return TextRange::default();
      }
      first = OreqlSyntaxNode::from_index(tree.children[child_span.start as usize]);
    }
    // Walk to rightmost leaf
    let mut last =
      OreqlSyntaxNode::from_index(tree.children[(span.start + span.length - 1) as usize]);
    while last.is_node(tree) {
      let child_span = unsafe { last._data(&tree.nodes).children_span };
      if child_span.length == 0 {
        return TextRange::default();
      }
      last = OreqlSyntaxNode::from_index(
        tree.children[(child_span.start + child_span.length - 1) as usize],
      );
    }
    TextRange {
      start_byte_offset: unsafe { first._data(&tree.nodes).text_range.start_byte_offset },
      end_byte_offset: unsafe { last._data(&tree.nodes).text_range.end_byte_offset },
    }
  }

  /// Whether this is the root
  #[inline]
  pub fn is_root(self, tree: &SyntaxTree) -> bool {
    self._parent(&tree.nodes) == NO_PARENT
  }

  /// Parent node
  #[inline]
  pub fn parent_node(self, tree: &SyntaxTree) -> Option<OreqlSyntaxNode> {
    let parent_index = self._parent(&tree.nodes);
    if parent_index == NO_PARENT {
      return None;
    }
    Some(OreqlSyntaxNode::from_index(parent_index))
  }

  /// Iterator over direct children
  #[inline]
  pub fn children(self, tree: &SyntaxTree) -> SyntaxNodeChildrenIterator<'_> {
    if self.is_token(tree) {
      return SyntaxNodeChildrenIterator {
        children: &[],
        position: 0,
      };
    }
    let span = unsafe { self._data(&tree.nodes).children_span };
    let start = span.start as usize;
    let end = start + span.length as usize;
    SyntaxNodeChildrenIterator {
      children: &tree.children[start..end],
      position: 0,
    }
  }

  /// Number of direct children
  #[inline]
  pub fn child_count(self, tree: &SyntaxTree) -> usize {
    if self.is_token(tree) {
      return 0;
    }
    unsafe { self._data(&tree.nodes).children_span.length as usize }
  }

  /// Get the nth direct child
  #[inline]
  pub fn nth_child(self, tree: &SyntaxTree, index: usize) -> Option<OreqlSyntaxNode> {
    if self.is_token(tree) {
      return None;
    }
    let span = unsafe { self._data(&tree.nodes).children_span };
    if index >= span.length as usize {
      return None;
    }
    let child_index = tree.children[span.start as usize + index];
    Some(OreqlSyntaxNode::from_index(child_index))
  }

  /// First child
  #[inline]
  pub fn first_child(self, tree: &SyntaxTree) -> Option<OreqlSyntaxNode> {
    self.nth_child(tree, 0)
  }

  /// Last child
  #[inline]
  pub fn last_child(self, tree: &SyntaxTree) -> Option<OreqlSyntaxNode> {
    let count = self.child_count(tree);
    if count == 0 {
      return None;
    }
    self.nth_child(tree, count - 1)
  }

  /// First direct child with the given syntax kind
  #[inline]
  pub fn first_child_of_kind(
    self,
    tree: &SyntaxTree,
    target: OreqlSyntaxKind,
  ) -> Option<OreqlSyntaxNode> {
    self.children(tree).find(|child| child.kind(tree) == target)
  }

  /// Nth direct child with the given syntax kind (0-indexed)
  pub fn nth_child_of_kind(
    self,
    tree: &SyntaxTree,
    target: OreqlSyntaxKind,
    index: usize,
  ) -> Option<OreqlSyntaxNode> {
    self
      .children(tree)
      .filter(|child| child.kind(tree) == target)
      .nth(index)
  }
}

/// Iterator over direct children of a syntax node
pub struct SyntaxNodeChildrenIterator<'a> {
  children: &'a [u32],
  position: usize,
}

impl Iterator for SyntaxNodeChildrenIterator<'_> {
  type Item = OreqlSyntaxNode;

  #[inline]
  fn next(&mut self) -> Option<OreqlSyntaxNode> {
    if self.position >= self.children.len() {
      return None;
    }
    let index = self.children[self.position];
    self.position += 1;
    Some(OreqlSyntaxNode::from_index(index))
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    let remaining = self.children.len() - self.position;
    (remaining, Some(remaining))
  }
}

impl ExactSizeIterator for SyntaxNodeChildrenIterator<'_> {}

#[cfg(test)]
mod tests {
  use super::super::SyntaxTreeBuilder;
  use super::*;

  /// Flat tree: root with 3 token children
  #[test]
  fn flat_children() {
    let mut builder = SyntaxTreeBuilder::new();
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
    let tree = builder.finish();

    let root = tree.root;
    assert_eq!(root.kind(&tree), OreqlSyntaxKind::Program);
    assert!(root.is_root(&tree));
    assert_eq!(root.child_count(&tree), 3);

    let first = root.first_child(&tree).unwrap();
    assert_eq!(first.kind(&tree), OreqlSyntaxKind::KeywordSelect);
    assert_eq!(first.parent_node(&tree).unwrap().index(), root.index());

    let last = root.last_child(&tree).unwrap();
    assert_eq!(last.kind(&tree), OreqlSyntaxKind::Number);
  }

  /// Nested tree: root -> statement -> token
  #[test]
  fn nested_children() {
    let mut builder = SyntaxTreeBuilder::new();
    builder.start_node(OreqlSyntaxKind::Program);
    builder.start_node(OreqlSyntaxKind::SelectStatement);
    builder.push_token(
      OreqlSyntaxKind::KeywordSelect,
      TextRange {
        start_byte_offset: 0,
        end_byte_offset: 6,
      },
    );
    builder.finish_node(OreqlSyntaxKind::SelectStatement);
    builder.finish_node(OreqlSyntaxKind::Program);
    let tree = builder.finish();

    let root = tree.root;
    assert_eq!(root.child_count(&tree), 1);

    let statement = root.first_child(&tree).unwrap();
    assert_eq!(statement.kind(&tree), OreqlSyntaxKind::SelectStatement);
    assert_eq!(statement.child_count(&tree), 1);

    let keyword = statement.first_child(&tree).unwrap();
    assert_eq!(keyword.kind(&tree), OreqlSyntaxKind::KeywordSelect);
    assert_eq!(
      keyword.parent_node(&tree).unwrap().index(),
      statement.index()
    );
    assert_eq!(statement.parent_node(&tree).unwrap().index(), root.index());
  }

  /// Text span of internal node spans its children
  #[test]
  fn internal_node_text_span() {
    let mut builder = SyntaxTreeBuilder::new();
    builder.start_node(OreqlSyntaxKind::Program);
    builder.push_token(
      OreqlSyntaxKind::Number,
      TextRange {
        start_byte_offset: 0,
        end_byte_offset: 2,
      },
    );
    builder.push_token(
      OreqlSyntaxKind::Number,
      TextRange {
        start_byte_offset: 3,
        end_byte_offset: 5,
      },
    );
    builder.finish_node(OreqlSyntaxKind::Program);
    let tree = builder.finish();

    let span = tree.root.text_span(&tree);
    assert_eq!(span.start_byte_offset, 0);
    assert_eq!(span.end_byte_offset, 5);
  }

  /// first_child_of_kind finds correct child
  #[test]
  fn find_child_of_kind() {
    let mut builder = SyntaxTreeBuilder::new();
    builder.start_node(OreqlSyntaxKind::Program);
    builder.push_token(
      OreqlSyntaxKind::Whitespace,
      TextRange {
        start_byte_offset: 0,
        end_byte_offset: 1,
      },
    );
    builder.push_token(
      OreqlSyntaxKind::Number,
      TextRange {
        start_byte_offset: 1,
        end_byte_offset: 2,
      },
    );
    builder.finish_node(OreqlSyntaxKind::Program);
    let tree = builder.finish();

    let found = tree
      .root
      .first_child_of_kind(&tree, OreqlSyntaxKind::Number);
    assert!(found.is_some());
    assert_eq!(found.unwrap().kind(&tree), OreqlSyntaxKind::Number);

    let not_found = tree
      .root
      .first_child_of_kind(&tree, OreqlSyntaxKind::String);
    assert!(not_found.is_none());
  }
}
