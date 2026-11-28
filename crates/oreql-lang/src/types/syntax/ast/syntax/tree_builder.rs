use crate::types::TextRange;
use crate::types::syntax::ast::OreqlSyntaxKind;

use super::node::NO_PARENT;
use super::{ChildrenSpan, OreqlSyntaxNode, OreqlSyntaxNodeMultiArena, SyntaxNodeData, SyntaxTree};

pub struct SyntaxTreeBuilder {
  arena: OreqlSyntaxNodeMultiArena,
  children: Vec<u32>,
  /// Scratch stack of direct child indices
  scratch: Vec<u32>,
  /// Stack of (kind, scratch_start) for open nodes
  stack: Vec<(OreqlSyntaxKind, usize)>,
}

impl SyntaxTreeBuilder {
  pub fn new() -> Self {
    Self {
      arena: OreqlSyntaxNodeMultiArena::new(),
      children: Vec::new(),
      scratch: Vec::new(),
      stack: Vec::new(),
    }
  }

  /// Push a token (leaf node)
  #[inline]
  pub fn push_token(&mut self, kind: OreqlSyntaxKind, text_range: TextRange) -> OreqlSyntaxNode {
    let data = SyntaxNodeData { text_range };
    let node = self.arena.push(kind, data, NO_PARENT);
    self.scratch.push(node.index());
    node
  }

  /// Begin a new internal node
  #[inline]
  pub fn start_node(&mut self, kind: OreqlSyntaxKind) {
    self.stack.push((kind, self.scratch.len()));
  }

  /// Finish the current internal node
  #[inline]
  pub fn finish_node(&mut self, kind: OreqlSyntaxKind) -> OreqlSyntaxNode {
    let (started_kind, scratch_start) = self.stack.pop().expect("no matching start_node");
    debug_assert_eq!(
      kind, started_kind,
      "finish_node kind does not match start_node kind"
    );

    // Flush direct children from scratch to children vector
    let children_start = self.children.len() as u32;
    let children_length = (self.scratch.len() - scratch_start) as u32;
    self
      .children
      .extend_from_slice(&self.scratch[scratch_start..]);
    self.scratch.truncate(scratch_start);

    // Push the internal node
    let data = SyntaxNodeData {
      children_span: ChildrenSpan {
        start: children_start,
        length: children_length,
      },
    };
    let node = self.arena.push(kind, data, NO_PARENT);

    // Set parent for all direct children
    let node_index = node.index();
    for position in children_start..(children_start + children_length) {
      let child_index = self.children[position as usize];
      let child = OreqlSyntaxNode::from_index(child_index);
      child.set__parent(&mut self.arena, node_index);
    }

    // This node is now a direct child of its parent
    self.scratch.push(node_index);
    node
  }

  /// Finish building and return the syntax tree
  pub fn finish(self) -> SyntaxTree {
    assert!(self.stack.is_empty(), "unclosed nodes remain");
    let root = OreqlSyntaxNode::from_index(self.arena.len() - 1);
    SyntaxTree {
      nodes: self.arena,
      children: self.children,
      root,
    }
  }
}

impl Default for SyntaxTreeBuilder {
  fn default() -> Self {
    Self::new()
  }
}
