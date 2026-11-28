/// Bytes that act as escape characters inside **strings**
pub const DEFAULT_STRING_ESCAPE_BYTES: &[u8] = &[b'\''];

/// Bytes that act as escape characters inside **byte strings**
pub const DEFAULT_BYTE_STRING_ESCAPE_BYTES: &[u8] = &[b'\''];

/// Bytes that act as escape characters inside **identifiers**
pub const DEFAULT_IDENTIFIER_ESCAPE_BYTES: &[u8] = &[];

/// Bytes that can follow an escape character
///
/// e.g. `&[b'n', b't', b'r', b'\\']` for `\n`, `\t`, `\r`
pub const DEFAULT_ESCAPE_FOLLOW_ALLOWED_BYTES: &[u8] = &[];

/// Whether escape characters work inside raw strings
pub const DEFAULT_ARE_ESCAPES_ALLOWED_IN_RAW_STRINGS: bool = true;
