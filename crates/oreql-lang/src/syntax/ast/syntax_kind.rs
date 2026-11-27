#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum OreqlSyntaxKind {
  // Tokens
  Eof,

  // Marker: all variants above are tokens, all below are nodes
  _NodeStart,

  // Nodes
  Program,
}

impl OreqlSyntaxKind {
  #[inline]
  pub fn is_token(self) -> bool {
    (self as u16) < (Self::_NodeStart as u16)
  }

  #[inline]
  pub fn is_node(self) -> bool {
    (self as u16) > (Self::_NodeStart as u16)
  }
}
