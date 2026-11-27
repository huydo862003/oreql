#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct TextRange {
  pub start_byte_offset: u32,
  pub end_byte_offset: u32,
}
