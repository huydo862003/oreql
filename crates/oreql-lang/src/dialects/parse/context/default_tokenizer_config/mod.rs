mod comments;
mod escapes;
mod hints;
mod identifiers;
mod json_path;
mod keywords;
mod operators;
mod single_byte_tokens;
mod string_literals;
mod template_tokens;

/// (open, close) delimiter pair for string/comment/identifier quoting
pub type DelimiterPair = (&'static str, &'static str);

pub use comments::*;
pub use escapes::*;
pub use hints::*;
pub use identifiers::*;
pub use json_path::*;
pub use keywords::*;
pub use operators::*;
pub use single_byte_tokens::*;
pub use string_literals::*;
