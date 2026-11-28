/// A const-constructible ASCII byte set backed by a 128-bit bitset
pub struct BitSet128 {
  bits: [u64; 2],
}

impl BitSet128 {
  /// Create an empty set
  pub const fn new() -> Self {
    Self { bits: [0; 2] }
  }

  /// Insert a byte, returning the new set
  #[inline]
  pub const fn insert(mut self, byte: u8) -> Self {
    if (byte as usize) < 128 {
      let word = (byte >> 6) as usize;
      let bit = byte & 63;
      self.bits[word] |= 1u64 << bit;
    }
    self
  }

  /// Check if a byte is in the set
  #[inline]
  pub const fn contains(&self, byte: u8) -> bool {
    if (byte as usize) < 128 {
      let word = (byte >> 6) as usize;
      let bit = byte & 63;
      (self.bits[word] >> bit) & 1 == 1
    } else {
      false
    }
  }
}

impl Default for BitSet128 {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  static ESCAPE_BYTES: BitSet128 = BitSet128::new().insert(b'\'').insert(b'\\');

  /// Insert and contains
  #[test]
  fn insert_and_contains() {
    assert!(ESCAPE_BYTES.contains(b'\''));
    assert!(ESCAPE_BYTES.contains(b'\\'));
    assert!(!ESCAPE_BYTES.contains(b'x'));
    assert!(!ESCAPE_BYTES.contains(b' '));
  }

  /// Non-ASCII bytes return false
  #[test]
  fn non_ascii_returns_false() {
    assert!(!ESCAPE_BYTES.contains(0x80));
    assert!(!ESCAPE_BYTES.contains(0xFF));
  }
}
