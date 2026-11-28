/// Result of looking up a key in the trie
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrieResult<V: Copy> {
  /// Key exists in the trie, with its value
  Found(V),
  /// Key is a valid prefix but not a complete key
  Prefix,
  /// No path exists for this key
  NotFound,
}

/// A trie set with no associated values
pub type TrieSet = TrieMap<()>;

impl TrieSet {
  /// Build a set trie from an iterator of byte string keys
  pub fn from_keys<'a>(keys: impl IntoIterator<Item = &'a [u8]>) -> Self {
    Self::from_entries(keys.into_iter().map(|key| (key, ())))
  }
}

/// A compact trie mapping byte string keys to values
///
/// Built once from key-value pairs via `from_entries`
pub struct TrieMap<V: Copy> {
  nodes: Vec<TrieNode<V>>,
  edges: Vec<TrieEdge>,
}

#[derive(Clone, Copy)]
struct TrieNode<V: Copy> {
  edge_start: u32,
  edge_count: u16,
  value: Option<V>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TrieEdge {
  byte: u8,
  target: u32,
}

impl<V: Copy> TrieMap<V> {
  /// Build a trie from key-value pairs
  pub fn from_entries<'a>(entries: impl IntoIterator<Item = (&'a [u8], V)>) -> Self {
    let mut build_nodes: Vec<(Vec<TrieEdge>, Option<V>)> = vec![(Vec::new(), None)];

    for (key, value) in entries {
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
          build_nodes.push((Vec::new(), None));
          node_index = new_index;
        }
      }
      build_nodes[node_index as usize].1 = Some(value);
    }

    let mut nodes = Vec::with_capacity(build_nodes.len());
    let mut edges = Vec::new();

    for (children, value) in &mut build_nodes {
      let edge_start = edges.len() as u32;
      children.sort_unstable_by_key(|edge| edge.byte);
      let edge_count = children.len() as u16;
      edges.append(children);
      nodes.push(TrieNode {
        edge_start,
        edge_count,
        value: *value,
      });
    }

    TrieMap { nodes, edges }
  }

  /// Look up a key, returning Found(value), Prefix, or NotFound
  pub fn lookup(&self, key: &[u8]) -> TrieResult<V> {
    let mut node_index = 0usize;

    for &byte in key {
      match self.find_child(node_index, byte) {
        Some(child) => node_index = child,
        None => return TrieResult::NotFound,
      }
    }

    match unsafe { self.nodes.get_unchecked(node_index) }.value {
      Some(value) => TrieResult::Found(value),
      None => TrieResult::Prefix,
    }
  }

  /// Get the value for a key, or None
  pub fn get(&self, key: &[u8]) -> Option<V> {
    match self.lookup(key) {
      TrieResult::Found(value) => Some(value),
      _ => None,
    }
  }

  /// Check if a key exists
  pub fn contains(&self, key: &[u8]) -> bool {
    matches!(self.lookup(key), TrieResult::Found(_))
  }

  /// Walk one byte from the given node for incremental matching
  pub fn step(&self, node_index: usize, byte: u8) -> Option<usize> {
    self.find_child(node_index, byte)
  }

  /// Get the value at a node (from incremental walking)
  pub fn value_at(&self, node_index: usize) -> Option<V> {
    unsafe { self.nodes.get_unchecked(node_index) }.value
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

  fn test_trie() -> TrieMap<u16> {
    TrieMap::from_entries([
      (&b"SELECT"[..], 1u16),
      (b"SET", 2),
      (b"FROM", 3),
      (b"WHERE", 4),
      (b"GROUP BY", 5),
    ])
  }

  #[test]
  fn found() {
    let trie = test_trie();
    assert_eq!(trie.lookup(b"SELECT"), TrieResult::Found(1));
    assert_eq!(trie.lookup(b"SET"), TrieResult::Found(2));
    assert_eq!(trie.lookup(b"FROM"), TrieResult::Found(3));
    assert_eq!(trie.lookup(b"WHERE"), TrieResult::Found(4));
    assert_eq!(trie.lookup(b"GROUP BY"), TrieResult::Found(5));
  }

  #[test]
  fn get_value() {
    let trie = test_trie();
    assert_eq!(trie.get(b"SELECT"), Some(1));
    assert_eq!(trie.get(b"FROM"), Some(3));
    assert_eq!(trie.get(b"MISSING"), None);
    assert_eq!(trie.get(b"SE"), None);
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
    assert_eq!(trie.lookup(b"SET"), TrieResult::Found(2));
    assert_eq!(trie.lookup(b"SELECT"), TrieResult::Found(1));
  }

  #[test]
  fn contains_check() {
    let trie = test_trie();
    assert!(trie.contains(b"SELECT"));
    assert!(!trie.contains(b"SE"));
    assert!(!trie.contains(b"INSERT"));
  }

  #[test]
  fn single_entry() {
    let trie = TrieMap::from_entries([(&b"A"[..], 42u16)]);
    assert_eq!(trie.get(b"A"), Some(42));
    assert_eq!(trie.get(b"B"), None);
    assert_eq!(trie.get(b"AB"), None);
  }
}
