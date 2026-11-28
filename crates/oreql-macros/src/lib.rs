mod diagnostic;
mod dialect_config;
mod multi_arena;
mod syntax_kinds;

use proc_macro::TokenStream;

/// Defines a syntax kind enum with nested group support
///
/// Each `group is_name { ... }` block generates an `is_name()` method
/// Groups can nest arbitrarily
///
/// ```ignore
/// define_syntax_kinds! {
///   #[repr(u16)]
///   pub enum SyntaxKind {
///     group is_token {
///       group is_trivia {
///         Whitespace,
///         Newline,
///       }
///       Eof,
///     }
///     group is_node {
///       group is_statement {
///         Program,
///       }
///     }
///   }
/// }
/// ```
#[proc_macro]
pub fn define_syntax_kinds(input: TokenStream) -> TokenStream {
  syntax_kinds::define_syntax_kinds_impl(input)
}

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
/// // GreenNode(u32) handle, GreenNodeMultiArena SOA container
/// let mut arena = GreenNodeMultiArena::new();
/// let node = arena.push(42u16, 100u32);
/// assert_eq!(node.kind(&arena), 42);
/// ```
#[proc_macro]
pub fn define_multi_arena(input: TokenStream) -> TokenStream {
  multi_arena::define_multi_arena_impl(input)
}

/// Defines a diagnostic kind enum with `#[message]` attributes
///
/// Generates `code_str()` and `message()` methods for each variant
///
/// ```ignore
/// #[oreql_macros::diagnostic_kind]
/// pub enum DiagnosticKind {
///   #[message("unterminated string literal")]
///   UnterminatedString,
///   #[message("unexpected character '{encountered}'")]
///   UnexpectedChar { encountered: char },
/// }
///
/// let diagnostic = DiagnosticKind::UnterminatedString;
/// assert_eq!(diagnostic.code_str(), "unterminated-string");
/// assert_eq!(diagnostic.message(), "unterminated string literal");
/// ```
#[proc_macro_attribute]
pub fn diagnostic_kind(attr: TokenStream, item: TokenStream) -> TokenStream {
  diagnostic::diagnostic_kind_impl(attr, item)
}

/// Declares typed dialect config wrappers with defaults
///
/// ```ignore
/// dialect_config! {
///   pub struct KeywordsConfig(TrieMap<SyntaxKind>) =
///     TrieMap::from_entries([(b"SELECT", KeywordSelect)]);
///
///   pub struct EscapeBytes(ByteLut<bool>) =
///     ByteLut::new().set(b'\'', true);
/// }
///
/// // Type-safe: can't pass EscapeBytes where KeywordsConfig is expected
/// fn scan(keywords: &KeywordsConfig) { ... }
/// scan(KeywordsConfig::default());
/// ```
#[proc_macro]
pub fn dialect_config(input: TokenStream) -> TokenStream {
  dialect_config::dialect_config_impl(input)
}
