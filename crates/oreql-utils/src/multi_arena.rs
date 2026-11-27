use std::ops::{Deref, DerefMut};

/// Defines a node handle and its corresponding SOA arena
///
/// ```ignore
/// define_multi_arena! {
///   pub struct GreenNode {
///     kind: u16,
///     offset: u32,
///   }
/// }
///
/// // GreenNode is a u32 newtype (index into the arena)
/// // GreenNodeMultiArena is the SOA container
/// let mut arena = GreenNodeMultiArena::new();
/// let node: GreenNode = arena.push(42u16, 100u32);
/// assert_eq!(arena.kind(node), 42);
/// assert_eq!(arena.offset(node), 100);
/// ```
#[macro_export]
macro_rules! define_multi_arena {
  (
    $(#[$meta:meta])*
    $vis:vis struct $Name:ident {
      $($field:ident : $T:ty),+ $(,)?
    }
  ) => {
    $(#[$meta])*
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    $vis struct $Name(u32);

    ::paste::paste! {
      impl $Name {
        $(
          /// Read a value by node handle
          #[inline]
          $vis fn $field(&self, arena: &[<$Name MultiArena>]) -> $T {
            assert!(self.0 < arena.length, "index out of bounds");
            unsafe { *arena.$field.as_ptr().add(self.0 as usize) }
          }

          /// Read a value without bounds checking
          ///
          /// # Safety
          ///
          /// Handle must refer to a valid entry in this arena
          #[inline]
          $vis unsafe fn [<$field _unchecked>](&self, arena: &[<$Name MultiArena>]) -> $T {
            unsafe { *arena.$field.as_ptr().add(self.0 as usize) }
          }

          /// Write a value to an existing row
          #[inline]
          $vis fn [<set_ $field>](&self, arena: &mut [<$Name MultiArena>], value: $T) {
            assert!(self.0 < arena.length, "index out of bounds");
            unsafe { ::core::ptr::write(arena.$field.as_ptr().add(self.0 as usize), value) }
          }
        )+
      }

      $vis struct [<$Name MultiArena>] {
        $($field: ::core::ptr::NonNull<$T>,)+
        length: u32,
        capacity: u32,
      }

      impl [<$Name MultiArena>] {
        $vis fn new() -> Self {
          Self {
            $($field: ::core::ptr::NonNull::dangling(),)+
            length: 0,
            capacity: 0,
          }
        }

        /// Create with pre-allocated capacity
        $vis fn with_capacity(capacity: u32) -> Self {
          let mut arena = Self::new();
          if capacity > 0 {
            arena.realloc_to(capacity);
          }
          arena
        }

        /// Number of rows
        #[inline]
        $vis fn len(&self) -> u32 {
          self.length
        }

        /// Whether the arena is empty
        #[inline]
        $vis fn is_empty(&self) -> bool {
          self.length == 0
        }

        /// Current capacity in rows
        #[inline]
        $vis fn capacity(&self) -> u32 {
          self.capacity
        }

        /// Reserve space for at least `additional` more rows
        ///
        /// Does nothing if there is already enough capacity
        $vis fn reserve(&mut self, additional: u32) {
          let required = self.length.checked_add(additional).expect("capacity overflow");
          if required > self.capacity {
            let new_capacity = ::core::cmp::max(
              if self.capacity == 0 { 8 } else { self.capacity.checked_mul(2).expect("capacity overflow") },
              required,
            );
            self.realloc_to(new_capacity);
          }
        }

        /// Reserve exact capacity for `additional` more rows
        ///
        /// Unlike `reserve`, this will not over-allocate
        $vis fn reserve_exact(&mut self, additional: u32) {
          let required = self.length.checked_add(additional).expect("capacity overflow");
          if required > self.capacity {
            self.realloc_to(required);
          }
        }

        /// Push a full row and return the node handle
        #[inline]
        $vis fn push(&mut self, $($field: $T),+) -> $Name {
          if self.length >= self.capacity {
            self.reserve(1);
          }
          let index = self.length;
          unsafe {
            $(::core::ptr::write(self.$field.as_ptr().add(index as usize), $field);)+
          }
          self.length += 1;
          $Name(index)
        }

        /// Reset length to zero without deallocating
        $vis fn clear(&mut self) {
          self.drop_range(0, self.length);
          self.length = 0;
        }

        /// Save the current position and return an RAII guard
        ///
        /// When dropped, the arena resets back to the saved position
        $vis fn scratch_space(&mut self) -> $crate::ScratchGuard<'_, [<$Name MultiArena>]> {
          let saved = self.length;
          $crate::ScratchGuard { arena: self, saved_length: saved }
        }

        $(
          /// Read a value by node handle
          #[inline]
          $vis fn $field(&self, node: $Name) -> $T {
            assert!(node.0 < self.length, "index out of bounds");
            unsafe { *self.$field.as_ptr().add(node.0 as usize) }
          }

          /// Read a value without bounds checking
          ///
          /// # Safety
          ///
          /// `node` must refer to a valid entry in this arena
          #[inline]
          $vis unsafe fn [<$field _unchecked>](&self, node: $Name) -> $T {
            unsafe { *self.$field.as_ptr().add(node.0 as usize) }
          }

          /// Write a value to an existing row
          #[inline]
          $vis fn [<set_ $field>](&mut self, node: $Name, value: $T) {
            assert!(node.0 < self.length, "index out of bounds");
            unsafe { ::core::ptr::write(self.$field.as_ptr().add(node.0 as usize), value) }
          }

          /// Immutable slice view of the column
          #[inline]
          $vis fn [<$field _slice>](&self) -> &[$T] {
            if self.length == 0 { return &[]; }
            unsafe { ::core::slice::from_raw_parts(self.$field.as_ptr(), self.length as usize) }
          }

          /// Mutable slice view of the column
          #[inline]
          $vis fn [<$field _slice_mut>](&mut self) -> &mut [$T] {
            if self.length == 0 { return &mut []; }
            unsafe { ::core::slice::from_raw_parts_mut(self.$field.as_ptr(), self.length as usize) }
          }
        )+

        fn realloc_to(&mut self, new_capacity: u32) {
          let new_capacity_usize = new_capacity as usize;
          let old_capacity_usize = self.capacity as usize;

          $(
            {
              let new_layout = ::core::alloc::Layout::array::<$T>(new_capacity_usize).expect("layout overflow");
              let new_ptr = if self.capacity == 0 {
                unsafe { ::std::alloc::alloc(new_layout) }
              } else {
                let old_layout = ::core::alloc::Layout::array::<$T>(old_capacity_usize).expect("layout overflow");
                unsafe { ::std::alloc::realloc(self.$field.as_ptr() as *mut u8, old_layout, new_layout.size()) }
              };
              self.$field = ::core::ptr::NonNull::new(new_ptr as *mut $T).expect("allocation failed");
            }
          )+

          self.capacity = new_capacity;
        }

        fn drop_range(&mut self, start: u32, end: u32) {
          if start >= end { return; }
          $(
            #[allow(dropping_copy_types)]
            if ::core::mem::needs_drop::<$T>() {
              for index in (start as usize)..(end as usize) {
                unsafe { ::core::ptr::drop_in_place(self.$field.as_ptr().add(index)); }
              }
            }
          )+
        }
      }

      impl $crate::MultiArena for [<$Name MultiArena>] {
        fn reset(&mut self, length: u32) {
          assert!(length <= self.length, "reset length exceeds current length");
          if length < self.length {
            self.drop_range(length, self.length);
            self.length = length;
          }
        }
      }

      impl Drop for [<$Name MultiArena>] {
        fn drop(&mut self) {
          self.drop_range(0, self.length);
          if self.capacity > 0 {
            $(
              {
                let layout = ::core::alloc::Layout::array::<$T>(self.capacity as usize).expect("layout overflow");
                unsafe { ::std::alloc::dealloc(self.$field.as_ptr() as *mut u8, layout) };
              }
            )+
          }
        }
      }

      unsafe impl Send for [<$Name MultiArena>] where $($T: Send),+ {}
      unsafe impl Sync for [<$Name MultiArena>] where $($T: Sync),+ {}
    }
  };
}

/// Trait used by ScratchGuard to reset any MultiArena
pub trait MultiArena {
  fn reset(&mut self, length: u32);
}

/// A guard that resets the arena to a saved position on drop
pub struct ScratchGuard<'arena, T: MultiArena> {
  pub arena: &'arena mut T,
  pub saved_length: u32,
}

impl<T: MultiArena> Deref for ScratchGuard<'_, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.arena
  }
}

impl<T: MultiArena> DerefMut for ScratchGuard<'_, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.arena
  }
}

impl<T: MultiArena> Drop for ScratchGuard<'_, T> {
  fn drop(&mut self) {
    self.arena.reset(self.saved_length);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  define_multi_arena! {
    struct TestNode {
      ids: u32,
      values: f64,
      flags: u8,
    }
  }

  #[test]
  fn push_returns_handle() {
    let mut arena = TestNodeMultiArena::new();
    let node0 = arena.push(1, 10.0, b'a');
    let node1 = arena.push(2, 20.0, b'b');
    let node2 = arena.push(3, 30.0, b'c');

    assert_eq!(arena.len(), 3);
    assert_eq!(node0.ids(&arena), 1);
    assert_eq!(node2.ids(&arena), 3);
    assert_eq!(node1.values(&arena), 20.0);
    assert_eq!(node2.flags(&arena), b'c');
  }

  #[test]
  fn handle_is_u32_newtype() {
    let mut arena = TestNodeMultiArena::new();
    let node = arena.push(42, 3.14, b'x');
    assert_eq!(node, TestNode(0));
  }

  #[test]
  fn slices() {
    let mut arena = TestNodeMultiArena::new();
    for index in 0..100u32 {
      arena.push(index, index as f64, 0);
    }

    let slice = arena.ids_slice();
    assert_eq!(slice.len(), 100);
    assert_eq!(slice[99], 99);

    let vslice = arena.values_slice();
    assert_eq!(vslice[50], 50.0);
  }

  #[test]
  fn set_value() {
    let mut arena = TestNodeMultiArena::new();
    let node = arena.push(0, 0.0, 0);
    node.set_ids(&mut arena, 42);
    assert_eq!(node.ids(&arena), 42);
  }

  #[test]
  fn clear_and_reuse() {
    let mut arena = TestNodeMultiArena::new();
    arena.push(1, 1.0, 1);
    arena.clear();
    assert_eq!(arena.len(), 0);
    assert!(arena.capacity() > 0);

    let node = arena.push(99, 99.0, 99);
    assert_eq!(node.ids(&arena), 99);
  }

  #[test]
  fn grows_correctly() {
    let mut arena = TestNodeMultiArena::new();
    let mut handles = Vec::new();
    for index in 0..1000u32 {
      handles.push(arena.push(index, index as f64, (index % 256) as u8));
    }
    assert_eq!(arena.len(), 1000);
    for (index, handle) in handles.iter().enumerate() {
      assert_eq!(handle.ids(&arena), index as u32);
    }
  }

  #[test]
  fn unchecked_access() {
    let mut arena = TestNodeMultiArena::new();
    let node = arena.push(7, 3.14, b'x');
    unsafe {
      assert_eq!(node.ids_unchecked(&arena), 7);
      assert_eq!(node.values_unchecked(&arena), 3.14);
      assert_eq!(node.flags_unchecked(&arena), b'x');
    }
  }

  #[test]
  fn reserve_avoids_realloc() {
    let mut arena = TestNodeMultiArena::new();
    arena.reserve(100);
    assert!(arena.capacity() >= 100);

    let capacity_before = arena.capacity();
    for index in 0..100u32 {
      arena.push(index, index as f64, 0);
    }
    assert_eq!(arena.capacity(), capacity_before);
  }

  #[test]
  fn reserve_exact_no_overalloc() {
    let mut arena = TestNodeMultiArena::new();
    arena.reserve_exact(10);
    assert_eq!(arena.capacity(), 10);
  }

  #[test]
  fn with_capacity_preallocates() {
    let arena = TestNodeMultiArena::with_capacity(64);
    assert!(arena.capacity() >= 64);
    assert_eq!(arena.len(), 0);
  }

  #[test]
  fn scratch_space_restores_on_drop() {
    let mut arena = TestNodeMultiArena::new();
    arena.push(1, 10.0, b'a');
    arena.push(2, 20.0, b'b');

    {
      let mut scratch = arena.scratch_space();
      scratch.push(3, 30.0, b'c');
      scratch.push(4, 40.0, b'd');
      assert_eq!(scratch.len(), 4);
      assert_eq!(TestNode(3).ids(&scratch), 4);
    }

    assert_eq!(arena.len(), 2);
    assert_eq!(TestNode(0).ids(&arena), 1);
    assert_eq!(TestNode(1).ids(&arena), 2);
  }
}
