mod ast_node;
mod diagnostic;
mod dialect_config;
mod multi_arena;
mod syntax_kinds;
mod utils;

use proc_macro::TokenStream;

/// Defines a syntax kind enum with nested group support
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

/// Defines a typed AST node for a single SyntaxKind variant
///
/// Struct name must match the SyntaxKind variant exactly.
/// Fields declare child accessors using SyntaxKind variant names.
///
/// ```ignore
/// #[oreql_macros::ast_node]
/// pub struct SelectStatement {
///   select_clause: SelectClause,
///   from_clause: FromClause,
/// }
/// ```
#[proc_macro_attribute]
pub fn ast_node(attr: TokenStream, item: TokenStream) -> TokenStream {
  ast_node::ast_node_impl(attr, item)
}

/// Defines a typed AST node for a SyntaxKind group
///
/// ```ignore
/// #[oreql_macros::ast_group(is_expression)]
/// pub struct Expression {}
///
/// // Generates Expression<'a> with:
/// // - AstNode impl (cast using kind.is_expression())
/// // - TryFrom<(OreqlRedNode, &OreqlRedTree)>
/// ```
#[proc_macro_attribute]
pub fn ast_group(attr: TokenStream, item: TokenStream) -> TokenStream {
  ast_node::ast_group_impl(attr, item)
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
