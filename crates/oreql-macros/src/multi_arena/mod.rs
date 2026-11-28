mod parse;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};

use parse::MultiArenaInput;

pub fn define_multi_arena_impl(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as MultiArenaInput);
  let visibility = &input.visibility;
  let name = &input.name;
  let attrs = &input.attrs;
  let arena_name = format_ident!("{}MultiArena", name, span = Span::call_site());

  let field_names: Vec<_> = input.fields.iter().map(|field| &field.name).collect();
  let field_types: Vec<_> = input.fields.iter().map(|field| &field.field_type).collect();
  let first_field_name = &input.fields[0].name;

  // Generate per-field methods on the handle
  let handle_field_methods = input.fields.iter().map(|field| {
    let field_name = &field.name;
    let field_type = &field.field_type;
    let unchecked_name = format_ident!("{}_unchecked", field_name);
    let set_name = format_ident!("set_{}", field_name);

    quote! {
      /// Read a value by node handle
      #[inline]
      #visibility fn #field_name(&self, arena: &#arena_name) -> #field_type {
        assert!(self.0 < arena.length, "index out of bounds");
        unsafe { *arena.#field_name.as_ptr().add(self.0 as usize) }
      }

      /// Read a value without bounds checking
      ///
      /// # Safety
      ///
      /// Handle must refer to a valid entry in this arena
      #[inline]
      #visibility unsafe fn #unchecked_name(&self, arena: &#arena_name) -> #field_type {
        unsafe { *arena.#field_name.as_ptr().add(self.0 as usize) }
      }

      /// Write a value to an existing row
      #[inline]
      #visibility fn #set_name(&self, arena: &mut #arena_name, value: #field_type) {
        assert!(self.0 < arena.length, "index out of bounds");
        unsafe { ::core::ptr::write(arena.#field_name.as_ptr().add(self.0 as usize), value) }
      }
    }
  });

  // Generate per-field methods on the arena
  let arena_field_methods = input.fields.iter().map(|field| {
    let field_name = &field.name;
    let field_type = &field.field_type;
    let unchecked_name = format_ident!("{}_unchecked", field_name);
    let set_name = format_ident!("set_{}", field_name);
    let slice_name = format_ident!("{}_slice", field_name);
    let slice_mut_name = format_ident!("{}_slice_mut", field_name);
    let spare_slice_mut_name = format_ident!("{}_spare_slice_mut", field_name);

    quote! {
      /// Read a value by node handle
      #[inline]
      #visibility fn #field_name(&self, node: #name) -> #field_type {
        assert!(node.0 < self.length, "index out of bounds");
        unsafe { *self.#field_name.as_ptr().add(node.0 as usize) }
      }

      /// Read a value without bounds checking
      ///
      /// # Safety
      ///
      /// `node` must refer to a valid entry in this arena
      #[inline]
      #visibility unsafe fn #unchecked_name(&self, node: #name) -> #field_type {
        unsafe { *self.#field_name.as_ptr().add(node.0 as usize) }
      }

      /// Write a value to an existing row
      #[inline]
      #visibility fn #set_name(&mut self, node: #name, value: #field_type) {
        assert!(node.0 < self.length, "index out of bounds");
        unsafe { ::core::ptr::write(self.#field_name.as_ptr().add(node.0 as usize), value) }
      }

      /// Immutable slice view of the column
      #[inline]
      #visibility fn #slice_name(&self) -> &[#field_type] {
        if self.length == 0 { return &[]; }
        unsafe { ::core::slice::from_raw_parts(self.#field_name.as_ptr(), self.length as usize) }
      }

      /// Mutable slice view of the column
      #[inline]
      #visibility fn #slice_mut_name(&mut self) -> &mut [#field_type] {
        if self.length == 0 { return &mut []; }
        unsafe { ::core::slice::from_raw_parts_mut(self.#field_name.as_ptr(), self.length as usize) }
      }

      /// Mutable slice of spare capacity (between length and capacity)
      ///
      /// # Safety
      ///
      /// Contents are uninitialized. Caller must initialize before
      /// calling set_len to include these entries.
      #[inline]
      #visibility unsafe fn #spare_slice_mut_name(&mut self) -> &mut [::core::mem::MaybeUninit<#field_type>] {
        let start = self.length as usize;
        let spare = (self.capacity - self.length) as usize;
        unsafe {
          ::core::slice::from_raw_parts_mut(
            self.#field_name.as_ptr().add(start) as *mut ::core::mem::MaybeUninit<#field_type>,
            spare,
          )
        }
      }
    }
  });

  // Generate per-field realloc logic
  let realloc_fields = input.fields.iter().map(|field| {
    let field_name = &field.name;
    let field_type = &field.field_type;
    quote! {
      {
        let new_layout = ::core::alloc::Layout::array::<#field_type>(new_capacity_usize).expect("layout overflow");
        let new_ptr = if self.capacity == 0 {
          unsafe { ::std::alloc::alloc(new_layout) }
        } else {
          let old_layout = ::core::alloc::Layout::array::<#field_type>(old_capacity_usize).expect("layout overflow");
          unsafe { ::std::alloc::realloc(self.#field_name.as_ptr() as *mut u8, old_layout, new_layout.size()) }
        };
        self.#field_name = ::core::ptr::NonNull::new(new_ptr as *mut #field_type).expect("allocation failed");
      }
    }
  });

  // Generate per-field drop logic
  let drop_fields = input.fields.iter().map(|field| {
    let field_name = &field.name;
    let field_type = &field.field_type;
    quote! {
      #[allow(dropping_copy_types)]
      if ::core::mem::needs_drop::<#field_type>() {
        for index in (start as usize)..(end as usize) {
          unsafe { ::core::ptr::drop_in_place(self.#field_name.as_ptr().add(index)); }
        }
      }
    }
  });

  // Generate per-field dealloc logic
  let dealloc_fields = input.fields.iter().map(|field| {
    let field_name = &field.name;
    let field_type = &field.field_type;
    quote! {
      {
        let layout = ::core::alloc::Layout::array::<#field_type>(self.capacity as usize).expect("layout overflow");
        unsafe { ::std::alloc::dealloc(self.#field_name.as_ptr() as *mut u8, layout) };
      }
    }
  });

  // Generate push write statements
  let push_writes = input.fields.iter().map(|field| {
    let field_name = &field.name;
    quote! {
      ::core::ptr::write(self.#field_name.as_ptr().add(index as usize), #field_name);
    }
  });

  // Generate extend_from_slices params and body
  let extend_params: Vec<_> = input
    .fields
    .iter()
    .map(|field| {
      let field_name = &field.name;
      let field_type = &field.field_type;
      quote! { #field_name: &[#field_type] }
    })
    .collect();

  let extend_asserts: Vec<_> = input
    .fields
    .iter()
    .skip(1)
    .map(|field| {
      let field_name = &field.name;
      let first_name = &input.fields[0].name;
      quote! {
        assert_eq!(#field_name.len(), #first_name.len(), "all slices must have the same length");
      }
    })
    .collect();

  let extend_copies: Vec<_> = input
    .fields
    .iter()
    .map(|field| {
      let field_name = &field.name;
      quote! {
        ::core::ptr::copy_nonoverlapping(
          #field_name.as_ptr(),
          self.#field_name.as_ptr().add(start as usize),
          count,
        );
      }
    })
    .collect();

  quote! {
    #(#attrs)*
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    #[repr(transparent)]
    #visibility struct #name(u32);

    #[allow(non_snake_case)]
    impl #name {
      /// Create a handle from a raw arena index
      #[inline]
      #visibility fn from_index(index: u32) -> Self {
        Self(index)
      }

      /// Get the raw arena index
      #[inline]
      #visibility fn index(&self) -> u32 {
        self.0
      }

      #(#handle_field_methods)*
    }

    #[repr(C)]
    #visibility struct #arena_name {
      #(#field_names: ::core::ptr::NonNull<#field_types>,)*
      length: u32,
      capacity: u32,
    }

    #[allow(non_snake_case)]
    impl #arena_name {
      #visibility fn new() -> Self {
        Self {
          #(#field_names: ::core::ptr::NonNull::dangling(),)*
          length: 0,
          capacity: 0,
        }
      }

      /// Create with pre-allocated capacity
      #visibility fn with_capacity(capacity: u32) -> Self {
        let mut arena = Self::new();
        if capacity > 0 {
          arena.realloc_to(capacity);
        }
        arena
      }

      /// Number of rows
      #[inline]
      #visibility fn len(&self) -> u32 {
        self.length
      }

      /// Whether the arena is empty
      #[inline]
      #visibility fn is_empty(&self) -> bool {
        self.length == 0
      }

      /// Current capacity in rows
      #[inline]
      #visibility fn capacity(&self) -> u32 {
        self.capacity
      }

      /// Reserve space for at least `additional` more rows
      ///
      /// Does nothing if there is already enough capacity
      #visibility fn reserve(&mut self, additional: u32) {
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
      #visibility fn reserve_exact(&mut self, additional: u32) {
        let required = self.length.checked_add(additional).expect("capacity overflow");
        if required > self.capacity {
          self.realloc_to(required);
        }
      }

      /// Push a full row and return the node handle
      #[inline]
      #visibility fn push(&mut self, #(#field_names: #field_types),*) -> #name {
        if self.length >= self.capacity {
          self.reserve(1);
        }
        let index = self.length;
        unsafe {
          #(#push_writes)*
        }
        self.length += 1;
        #name(index)
      }

      /// Batch-push rows from per-column slices
      ///
      /// All slices must have the same length
      /// Returns the handle of the first pushed row
      #visibility fn extend_from_slices(&mut self, #(#extend_params),*) -> #name {
        let count = #first_field_name.len();
        #(#extend_asserts)*
        self.reserve(count as u32);
        let start = self.length;
        unsafe {
          #(#extend_copies)*
        }
        self.length += count as u32;
        #name(start)
      }

      /// Set length without bounds checking
      ///
      /// # Safety
      ///
      /// New length must be <= capacity, and all entries up to new_length must be initialized
      #[inline]
      #visibility unsafe fn set_len(&mut self, new_length: u32) {
        debug_assert!(new_length <= self.capacity, "set_len exceeds capacity");
        self.length = new_length;
      }

      /// Reset length to zero without deallocating
      #visibility fn clear(&mut self) {
        self.drop_range(0, self.length);
        self.length = 0;
      }

      /// Truncate to a previous length
      ///
      /// Drops elements beyond `new_length`
      #visibility fn truncate(&mut self, new_length: u32) {
        assert!(new_length <= self.length, "truncate length exceeds current length");
        if new_length < self.length {
          self.drop_range(new_length, self.length);
          self.length = new_length;
        }
      }

      /// Save the current position and return an RAII guard
      ///
      /// When dropped, the arena truncates back to the saved position
      #visibility fn scratch_space(&mut self) -> ScratchGuard<'_> {
        let saved = self.length;
        ScratchGuard { arena: self, saved_length: saved }
      }

      #(#arena_field_methods)*

      fn realloc_to(&mut self, new_capacity: u32) {
        let new_capacity_usize = new_capacity as usize;
        let old_capacity_usize = self.capacity as usize;
        #(#realloc_fields)*
        self.capacity = new_capacity;
      }

      fn drop_range(&mut self, start: u32, end: u32) {
        if start >= end { return; }
        #(#drop_fields)*
      }
    }

    impl Drop for #arena_name {
      fn drop(&mut self) {
        self.drop_range(0, self.length);
        if self.capacity > 0 {
          #(#dealloc_fields)*
        }
      }
    }

    unsafe impl Send for #arena_name where #(#field_types: Send),* {}
    unsafe impl Sync for #arena_name where #(#field_types: Sync),* {}

    /// RAII guard that truncates the arena to a saved position on drop
    #visibility struct ScratchGuard<'arena> {
      arena: &'arena mut #arena_name,
      saved_length: u32,
    }

    impl ::core::ops::Deref for ScratchGuard<'_> {
      type Target = #arena_name;
      #[inline]
      fn deref(&self) -> &Self::Target { self.arena }
    }

    impl ::core::ops::DerefMut for ScratchGuard<'_> {
      #[inline]
      fn deref_mut(&mut self) -> &mut Self::Target { self.arena }
    }

    impl Drop for ScratchGuard<'_> {
      fn drop(&mut self) {
        self.arena.truncate(self.saved_length);
      }
    }
  }
  .into()
}
