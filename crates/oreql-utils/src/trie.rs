/// Result of looking up a key in the trie
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrieResult {
  /// Key exists in the trie
  Found,
  /// Key is a valid prefix but not a complete key
  Prefix,
  /// No path exists for this key
  NotFound,
}

/// A compact trie for byte string lookup
///
/// Built once from a set of keys via `from_keys`
pub struct Trie {
  nodes: Vec<TrieNode>,
  edges: Vec<TrieEdge>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TrieNode {
  edge_start: u32,
  edge_count: u16,
  is_terminal: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TrieEdge {
  byte: u8,
  target: u32,
}

impl Trie {
  /// Build a trie from an iterator of byte string keys
  pub fn from_keys<'a>(keys: impl IntoIterator<Item = &'a [u8]>) -> Self {
    let mut build_nodes: Vec<(Vec<TrieEdge>, bool)> = vec![(Vec::new(), false)];

    for key in keys {
      let mut node_index = 0u32;
      for &byte in key {
        let children = &build_nodes[node_index as usize].0;
        let found = children.iter().position(|edge| edge.byte == byte);

        if let Some(position) = found {
          node_index = children[position].target;
        } else {
          let new_index = build_nodes.len() as u32;
          build_nodes[node_index as usize].0.push(TrieEdge {
            byte,
            target: new_index,
          });
          build_nodes.push((Vec::new(), false));
          node_index = new_index;
        }
      }
      build_nodes[node_index as usize].1 = true;
    }

    let mut nodes = Vec::with_capacity(build_nodes.len());
    let mut edges = Vec::new();

    for (children, is_terminal) in &mut build_nodes {
      let edge_start = edges.len() as u32;
      children.sort_unstable_by_key(|edge| edge.byte);
      let edge_count = children.len() as u16;
      edges.append(children);
      nodes.push(TrieNode {
        edge_start,
        edge_count,
        is_terminal: *is_terminal,
      });
    }

    Trie { nodes, edges }
  }

  /// Look up a key
  #[inline]
  pub fn lookup(&self, key: &[u8]) -> TrieResult {
    let mut node_index = 0usize;

    for &byte in key {
      match self.find_child(node_index, byte) {
        Some(child) => node_index = child,
        None => return TrieResult::NotFound,
      }
    }

    if unsafe { self.nodes.get_unchecked(node_index) }.is_terminal {
      TrieResult::Found
    } else {
      TrieResult::Prefix
    }
  }

  #[inline]
  fn find_child(&self, node_index: usize, byte: u8) -> Option<usize> {
    let node = unsafe { self.nodes.get_unchecked(node_index) };
    let start = node.edge_start as usize;
    let count = node.edge_count as usize;
    let edge_slice = unsafe { self.edges.get_unchecked(start..start + count) };

    if count <= 8 {
      for edge in edge_slice {
        if edge.byte == byte {
          return Some(edge.target as usize);
        }
        if edge.byte > byte {
          return None;
        }
      }
      None
    } else {
      edge_slice
        .binary_search_by_key(&byte, |edge| edge.byte)
        .ok()
        .map(|index| unsafe { edge_slice.get_unchecked(index) }.target as usize)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_trie() -> Trie {
    Trie::from_keys([&b"SELECT"[..], b"SET", b"FROM", b"WHERE", b"GROUP BY"])
  }

  #[test]
  fn found() {
    let trie = test_trie();
    assert_eq!(trie.lookup(b"SELECT"), TrieResult::Found);
    assert_eq!(trie.lookup(b"SET"), TrieResult::Found);
    assert_eq!(trie.lookup(b"FROM"), TrieResult::Found);
    assert_eq!(trie.lookup(b"WHERE"), TrieResult::Found);
    assert_eq!(trie.lookup(b"GROUP BY"), TrieResult::Found);
  }

  #[test]
  fn not_found() {
    let trie = test_trie();
    assert_eq!(trie.lookup(b"SELECTS"), TrieResult::NotFound);
    assert_eq!(trie.lookup(b"FROMM"), TrieResult::NotFound);
    assert_eq!(trie.lookup(b"INSERT"), TrieResult::NotFound);
  }

  #[test]
  fn prefix() {
    let trie = test_trie();
    assert_eq!(trie.lookup(b"SE"), TrieResult::Prefix);
    assert_eq!(trie.lookup(b"GROUP"), TrieResult::Prefix);
    assert_eq!(trie.lookup(b""), TrieResult::Prefix);
  }

  #[test]
  fn prefix_sharing() {
    let trie = test_trie();
    assert_eq!(trie.lookup(b"SE"), TrieResult::Prefix);
    assert_eq!(trie.lookup(b"SET"), TrieResult::Found);
    assert_eq!(trie.lookup(b"SELECT"), TrieResult::Found);
  }

  #[test]
  fn single_key() {
    let trie = Trie::from_keys([&b"A"[..]]);
    assert_eq!(trie.lookup(b"A"), TrieResult::Found);
    assert_eq!(trie.lookup(b"B"), TrieResult::NotFound);
    assert_eq!(trie.lookup(b"AB"), TrieResult::NotFound);
  }
}
