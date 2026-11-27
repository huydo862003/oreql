/// Pool for string data during parsing
pub struct OreqlStringPool<'a> {
  source: &'a str,
  extra: Vec<u8>,
}

/// A reference to a string in the pool
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct OreqlString {
  pub start_byte_offset: u32,
  pub end_byte_offset: u32,
}

impl OreqlString {
  /// Resolve to a byte slice
  #[inline]
  pub fn get<'a>(&self, pool: &'a OreqlStringPool) -> &'a [u8] {
    let source_len = pool.source.len() as u32;
    if self.start_byte_offset < source_len {
      &pool.source.as_bytes()[self.start_byte_offset as usize..self.end_byte_offset as usize]
    } else {
      let start_byte_offset = (self.start_byte_offset - source_len) as usize;
      let end_byte_offset = (self.end_byte_offset - source_len) as usize;
      &pool.extra[start_byte_offset..end_byte_offset]
    }
  }

  /// Resolve to a str slice
  ///
  /// # Safety
  ///
  /// The referenced bytes must be valid UTF-8
  #[inline]
  pub unsafe fn get_str<'a>(&self, pool: &'a OreqlStringPool) -> &'a str {
    unsafe { core::str::from_utf8_unchecked(self.get(pool)) }
  }
}

impl<'a> OreqlStringPool<'a> {
  pub fn new(source: &'a str) -> Self {
    Self {
      source,
      extra: Vec::new(),
    }
  }

  /// Create a reference to a source slice (no copy)
  #[inline]
  pub fn create_string(&self, start_byte_offset: u32, end_byte_offset: u32) -> OreqlString {
    debug_assert!((end_byte_offset as usize) <= self.source.len());
    OreqlString {
      start_byte_offset,
      end_byte_offset,
    }
  }

  /// Push decoded bytes into the extra buffer and return a reference
  pub fn create_extra_string(&mut self, bytes: &[u8]) -> OreqlString {
    let source_len = self.source.len() as u32;
    let start_byte_offset = source_len + self.extra.len() as u32;
    self.extra.extend_from_slice(bytes);
    let end_byte_offset = source_len + self.extra.len() as u32;
    OreqlString {
      start_byte_offset,
      end_byte_offset,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn source_ref_slices_source() {
    let pool = OreqlStringPool::new("let x = 42;");
    let name = pool.create_string(4, 5);
    assert_eq!(name.get(&pool), b"x");
  }

  #[test]
  fn extra_ref_for_decoded_strings() {
    let mut pool = OreqlStringPool::new(r#"let x = "a\nb";"#);
    let decoded = pool.create_extra_string(b"a\nb");
    assert_eq!(decoded.get(&pool), b"a\nb");
  }

  #[test]
  fn source_and_extra_coexist() {
    let mut pool = OreqlStringPool::new("hello world");
    let src = pool.create_string(0, 5);
    let extra = pool.create_extra_string(b"decoded");

    assert_eq!(src.get(&pool), b"hello");
    assert_eq!(extra.get(&pool), b"decoded");
  }

  #[test]
  fn get_str_works() {
    let pool = OreqlStringPool::new("let foo = 1;");
    let name = pool.create_string(4, 7);
    assert_eq!(unsafe { name.get_str(&pool) }, "foo");
  }

  #[test]
  fn multiple_extras() {
    let mut pool = OreqlStringPool::new("src");
    let first = pool.create_extra_string(b"abc");
    let second = pool.create_extra_string(b"def");

    assert_eq!(first.get(&pool), b"abc");
    assert_eq!(second.get(&pool), b"def");
  }
}
