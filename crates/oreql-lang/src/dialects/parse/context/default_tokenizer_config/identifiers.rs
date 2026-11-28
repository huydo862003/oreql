/// (open, close) delimiters for **quoted** identifiers
///
/// e.g. `[("\"", "\""), ("[", "]")]` for `"col"` or `[col]`
pub const DEFAULT_IDENTIFIER_QUOTE_PATTERNS: &[(&str, &str)] = &[("\"", "\"")];

/// Bytes that can appear inside variable names
///
/// e.g. `&[b'@', b'$']` to allow `@var` or `$var`
pub const DEFAULT_WITHIN_VARIABLE_ALLOWED_BYTES: &[u8] = &[];
