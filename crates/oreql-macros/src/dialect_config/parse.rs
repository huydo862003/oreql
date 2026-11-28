use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Expr, Ident, Token, Type, Visibility};

pub struct ConfigItem {
  pub attrs: Vec<Attribute>,
  pub visibility: Visibility,
  pub name: Ident,
  pub inner_type: Type,
  pub default_expression: Expr,
}

pub struct ConfigInput {
  pub items: Vec<ConfigItem>,
}

impl Parse for ConfigInput {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut items = Vec::new();
    while !input.is_empty() {
      let attrs = input.call(Attribute::parse_outer)?;
      let visibility: Visibility = input.parse()?;
      input.parse::<Token![struct]>()?;
      let name: Ident = input.parse()?;

      let content;
      syn::parenthesized!(content in input);
      let inner_type: Type = content.parse()?;

      input.parse::<Token![=]>()?;
      let default_expression: Expr = input.parse()?;
      input.parse::<Token![;]>()?;

      items.push(ConfigItem {
        attrs,
        visibility,
        name,
        inner_type,
        default_expression,
      });
    }
    Ok(ConfigInput { items })
  }
}
