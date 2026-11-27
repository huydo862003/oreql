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
