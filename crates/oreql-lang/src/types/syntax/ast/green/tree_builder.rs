use crate::types::TextRange;

use super::super::OreqlSyntaxKind;
use super::{ChildrenRange, GreenNodeData, OreqlGreenNode, OreqlGreenNodeMultiArena};

/// Builds a green tree in a single pass
pub struct GreenTreeBuilder {
  arena: OreqlGreenNodeMultiArena,
  /// Stack of (kind, first_child_index_in_arena)
  stack: Vec<(OreqlSyntaxKind, u32)>,
}

pub struct GreenTree {
  pub nodes: OreqlGreenNodeMultiArena,
  pub root: OreqlGreenNode,
}

impl GreenTreeBuilder {
  pub fn new() -> Self {
    Self {
      arena: OreqlGreenNodeMultiArena::new(),
      stack: Vec::new(),
    }
  }

  /// Push a token (leaf node)
  pub fn push_token(&mut self, kind: OreqlSyntaxKind, text_range: TextRange) -> OreqlGreenNode {
    let data = GreenNodeData { text_range };
    self.arena.push(kind, data)
  }

  /// Begin a new internal node at the current arena position
  pub fn start_node(&mut self, kind: OreqlSyntaxKind) {
    self.stack.push((kind, self.arena.len()));
  }

  /// Finish the current internal node
  ///
  /// Children are everything pushed since the matching `start_node`
  pub fn finish_node(&mut self, kind: OreqlSyntaxKind) -> OreqlGreenNode {
    let (started_kind, first_child) = self.stack.pop().expect("no matching start_node");
    debug_assert_eq!(
      kind, started_kind,
      "finish_node kind does not match start_node kind"
    );
    let child_count = self.arena.len() - first_child;
    let data = GreenNodeData {
      children: ChildrenRange {
        first_child,
        child_count,
      },
    };
    self.arena.push(kind, data)
  }

  /// Finish building and return the arena and root handle
  pub fn finish(self) -> GreenTree {
    assert!(self.stack.is_empty(), "unclosed nodes remain");
    let root = OreqlGreenNode::from_index(self.arena.len() - 1);
    GreenTree {
      nodes: self.arena,
      root,
    }
  }
}

impl Default for GreenTreeBuilder {
  fn default() -> Self {
    Self::new()
  }
}
