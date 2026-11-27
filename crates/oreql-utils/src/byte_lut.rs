/// A const-constructible ASCII byte to value lookup table
///
/// Direct index by byte value, O(1) lookup, no hashing
pub struct ByteLut<V: Copy> {
  table: [Option<V>; 128],
}

impl<V: Copy> ByteLut<V> {
  /// Create an empty lookup table
  pub const fn new() -> Self {
    Self { table: [None; 128] }
  }

  /// Set a mapping, returning the new table
  pub const fn set(mut self, byte: u8, value: V) -> Self {
    self.table[byte as usize] = Some(value);
    self
  }

  /// Look up a byte
  #[inline]
  pub const fn get(&self, byte: u8) -> Option<V> {
    if (byte as usize) < 128 {
      self.table[byte as usize]
    } else {
      None
    }
  }

  /// Check if a byte has a mapping
  #[inline]
  pub const fn contains(&self, byte: u8) -> bool {
    if (byte as usize) < 128 {
      self.table[byte as usize].is_some()
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  static TOKENS: ByteLut<u16> = ByteLut::new()
    .set(b'(', 1)
    .set(b')', 2)
    .set(b'+', 3)
    .set(b'-', 4)
    .set(b'*', 5);

  #[test]
  fn lookup_existing() {
    assert_eq!(TOKENS.get(b'('), Some(1));
    assert_eq!(TOKENS.get(b')'), Some(2));
    assert_eq!(TOKENS.get(b'+'), Some(3));
    assert_eq!(TOKENS.get(b'*'), Some(5));
  }

  #[test]
  fn lookup_missing() {
    assert_eq!(TOKENS.get(b'x'), None);
    assert_eq!(TOKENS.get(b' '), None);
    assert_eq!(TOKENS.get(0xFF), None);
  }

  #[test]
  fn contains_check() {
    assert!(TOKENS.contains(b'('));
    assert!(!TOKENS.contains(b'x'));
    assert!(!TOKENS.contains(0xFF));
  }
}
