#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct TextRange {
  pub start_byte_offset: u32,
  pub end_byte_offset: u32,
}
