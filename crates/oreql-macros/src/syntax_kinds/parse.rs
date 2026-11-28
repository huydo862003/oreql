use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Ident, Token, Visibility, braced};

/// A leaf variant in the enum
pub struct EnumVariant {
  pub attrs: Vec<Attribute>,
  pub name: Ident,
}

/// A named group that generates an `is_*()` predicate method
pub struct KindGroup {
  pub attrs: Vec<Attribute>,
  pub predicate_name: Ident,
  pub children: Vec<SyntaxKindItem>,
}

/// Either a group or a leaf variant
pub enum SyntaxKindItem {
  Group(KindGroup),
  Variant(EnumVariant),
}

/// The full input to the macro
pub struct SyntaxKindEnumInput {
  pub attrs: Vec<Attribute>,
  pub visibility: Visibility,
  pub name: Ident,
  pub children: Vec<SyntaxKindItem>,
}

impl Parse for SyntaxKindEnumInput {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let attrs = input.call(Attribute::parse_outer)?;
    let visibility: Visibility = input.parse()?;
    input.parse::<Token![enum]>()?;
    let name: Ident = input.parse()?;
    let content;
    braced!(content in input);
    let children = parse_items(&content)?;
    Ok(SyntaxKindEnumInput {
      attrs,
      visibility,
      name,
      children,
    })
  }
}

/// Check if the next token sequence starts with `group`
fn is_group_next(input: ParseStream) -> bool {
  let fork = input.fork();
  let _ = fork.call(Attribute::parse_outer);
  fork.parse::<Ident>().is_ok_and(|ident| ident == "group")
}

/// Parse a list of items (groups and variants) from braced content
fn parse_items(content: ParseStream) -> syn::Result<Vec<SyntaxKindItem>> {
  let mut items = Vec::new();
  while !content.is_empty() {
    if is_group_next(content) {
      items.push(SyntaxKindItem::Group(parse_group(content)?));
    } else {
      let attrs = content.call(Attribute::parse_outer)?;
      let name: Ident = content.parse()?;
      items.push(SyntaxKindItem::Variant(EnumVariant { attrs, name }));
      if !content.is_empty() {
        content.parse::<Token![,]>()?;
      }
    }
  }
  Ok(items)
}

fn parse_group(input: ParseStream) -> syn::Result<KindGroup> {
  let attrs = input.call(Attribute::parse_outer)?;
  let keyword: Ident = input.parse()?;
  if keyword != "group" {
    return Err(syn::Error::new(keyword.span(), "expected `group`"));
  }
  let predicate_name: Ident = input.parse()?;
  let content;
  braced!(content in input);
  let children = parse_items(&content)?;
  Ok(KindGroup {
    attrs,
    predicate_name,
    children,
  })
}
