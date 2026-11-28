use proc_macro::TokenStream;
use quote::format_ident;
use syn::{Ident, Token, punctuated::Punctuated};

pub struct AstNodeField {
  pub name: Ident,
  pub kind: Ident,
  pub nth: usize,
}

pub fn parse_fields(item_struct: &syn::ItemStruct) -> Result<Vec<AstNodeField>, TokenStream> {
  let fields_raw: Vec<(Ident, Ident)> = match &item_struct.fields {
    syn::Fields::Named(named) => named
      .named
      .iter()
      .map(|field| {
        let name = field.ident.clone().unwrap();
        let kind = extract_kind(&field.ty);
        (name, kind)
      })
      .collect(),
    syn::Fields::Unit => Vec::new(),
    _ => {
      return Err(
        syn::Error::new_spanned(
          &item_struct.ident,
          "ast_node expects named fields or unit struct",
        )
        .to_compile_error()
        .into(),
      );
    }
  };

  let mut kind_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
  let mut fields = Vec::with_capacity(fields_raw.len());

  for (name, kind) in fields_raw {
    let key = kind.to_string();
    let nth = *kind_counts.get(&key).unwrap_or(&0);
    kind_counts.insert(key, nth + 1);
    fields.push(AstNodeField { name, kind, nth });
  }

  Ok(fields)
}

/// Extract kind ident from type, unwrapping Option<T> if present
fn extract_kind(ty: &syn::Type) -> Ident {
  match ty {
    syn::Type::Path(path) => {
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

/// Normalize group name: prepend `is_` if not already present
fn normalize_group(raw: Ident) -> Ident {
  if raw.to_string().starts_with("is_") {
    raw
  } else {
    format_ident!("is_{raw}")
  }
}

/// Parse comma-separated group names from attribute
pub fn parse_groups(attr: TokenStream) -> Vec<Ident> {
  if attr.is_empty() {
    return Vec::new();
  }
  let parsed =
    syn::parse::Parser::parse(Punctuated::<Ident, Token![,]>::parse_terminated, attr).unwrap();
  parsed.into_iter().map(normalize_group).collect()
}
