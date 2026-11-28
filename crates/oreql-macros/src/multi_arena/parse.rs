use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Ident, Token, Type, Visibility, braced};

pub struct ArenaField {
  pub name: Ident,
  pub field_type: Type,
}

pub struct MultiArenaInput {
  pub attrs: Vec<Attribute>,
  pub visibility: Visibility,
  pub name: Ident,
  pub fields: Vec<ArenaField>,
}

impl Parse for MultiArenaInput {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let attrs = input.call(Attribute::parse_outer)?;
    let visibility: Visibility = input.parse()?;
    input.parse::<Token![struct]>()?;
    let name: Ident = input.parse()?;

    let content;
    braced!(content in input);

    let mut fields = Vec::new();
    while !content.is_empty() {
      let field_name: Ident = content.parse()?;
      content.parse::<Token![:]>()?;
      let field_type: Type = content.parse()?;
      fields.push(ArenaField {
        name: field_name,
        field_type,
      });
      if !content.is_empty() {
        content.parse::<Token![,]>()?;
      }
    }

    Ok(MultiArenaInput {
      attrs,
      visibility,
      name,
      fields,
    })
  }
}
