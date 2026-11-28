use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Ident, Token, Type, Visibility, braced};

/// A child field declaration on a node variant
#[derive(Clone)]
pub struct NodeField {
  pub name: Ident,
  pub kind: Ident,
}

/// A leaf variant (token or node), optionally with child fields
pub struct EnumVariant {
  pub attrs: Vec<Attribute>,
  pub name: Ident,
  pub fields: Vec<NodeField>,
}

/// A named group
pub struct KindGroup {
  pub attrs: Vec<Attribute>,
  pub predicate_name: Ident,
  pub children: Vec<SyntaxItem>,
}

/// Either a group or a leaf variant
pub enum SyntaxItem {
  Group(KindGroup),
  Variant(EnumVariant),
}

/// The full input to the macro
pub struct SyntaxInput {
  pub attrs: Vec<Attribute>,
  pub visibility: Visibility,
  pub name: Ident,
  pub children: Vec<SyntaxItem>,
}

impl Parse for SyntaxInput {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let attrs = input.call(Attribute::parse_outer)?;
    let visibility: Visibility = input.parse()?;
    input.parse::<Token![enum]>()?;
    let name: Ident = input.parse()?;
    let content;
    braced!(content in input);
    let children = parse_items(&content)?;
    Ok(SyntaxInput {
      attrs,
      visibility,
      name,
      children,
    })
  }
}

/// Peek ahead to check if the next item is a group declaration
fn is_group_next(input: ParseStream) -> bool {
  let fork = input.fork();
  let _ = fork.call(Attribute::parse_outer);
  fork.parse::<Ident>().is_ok_and(|ident| ident == "group")
}

/// Parse a sequence of groups and variants inside braces
fn parse_items(content: ParseStream) -> syn::Result<Vec<SyntaxItem>> {
  let mut items = Vec::new();
  while !content.is_empty() {
    if is_group_next(content) {
      items.push(SyntaxItem::Group(parse_group(content)?));
    } else {
      items.push(SyntaxItem::Variant(parse_variant(content)?));
    }
  }
  Ok(items)
}

/// Parse a variant with optional { field: Type, ... } block
fn parse_variant(input: ParseStream) -> syn::Result<EnumVariant> {
  let attrs = input.call(Attribute::parse_outer)?;
  let name: Ident = input.parse()?;

  let mut fields = Vec::new();

  // Check for { field: Type, ... } block
  if input.peek(syn::token::Brace) {
    let content;
    braced!(content in input);
    while !content.is_empty() {
      let field_name: Ident = content.parse()?;
      content.parse::<Token![:]>()?;
      let field_type: Type = content.parse()?;

      let kind = extract_kind(&field_type);
      fields.push(NodeField {
        name: field_name,
        kind,
      });

      if !content.is_empty() {
        content.parse::<Token![,]>()?;
      }
    }
  }

  // Consume trailing comma
  if !input.is_empty() && input.peek(Token![,]) {
    input.parse::<Token![,]>()?;
  }

  Ok(EnumVariant {
    attrs,
    name,
    fields,
  })
}

/// Parse a `group predicate_name { ... }` block
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

/// Extract the kind ident from a type, unwrapping Option<T> if present
fn extract_kind(ty: &Type) -> Ident {
  match ty {
    Type::Path(path) => {
      let segment = path.path.segments.last().unwrap();
      let name = segment.ident.to_string();

      if name == "Option"
        && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
      {
        return extract_kind(inner);
      }

      segment.ident.clone()
    }
    _ => panic!("expected a path type"),
  }
}
