use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
  Data, DeriveInput, Expr, Fields, Lit, Meta, Token, parse_macro_input, punctuated::Punctuated,
};

pub fn diagnostic_kind_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let mut input = parse_macro_input!(item as DeriveInput);
  let name = &input.ident;

  let data = match &input.data {
    Data::Enum(data) => data,
    _ => {
      return syn::Error::new_spanned(name, "diagnostic_kind can only be used on enums")
        .to_compile_error()
        .into();
    }
  };

  let mut message_arms = Vec::new();
  let mut code_string_arms = Vec::new();

  for variant in &data.variants {
    let variant_name = &variant.ident;
    let kebab = pascal_to_kebab(&variant_name.to_string());

    let message_attribute = variant
      .attrs
      .iter()
      .find(|attr| attr.path().is_ident("message"));

    let Some(attribute) = message_attribute else {
      return syn::Error::new_spanned(variant_name, "missing #[message(\"...\")] attribute")
        .to_compile_error()
        .into();
    };

    let message_body = parse_message_attr(attribute, &variant.fields);

    match &variant.fields {
      Fields::Named(_) => {
        code_string_arms.push(quote! { #name::#variant_name { .. } => #kebab });
      }
      Fields::Unnamed(_) => {
        code_string_arms.push(quote! { #name::#variant_name(..) => #kebab });
      }
      Fields::Unit => {
        code_string_arms.push(quote! { #name::#variant_name => #kebab });
      }
    }

    match &variant.fields {
      Fields::Named(fields) => {
        let field_names: Vec<_> = fields.named.iter().map(|field| &field.ident).collect();
        message_arms.push(quote! {
          #name::#variant_name { #(#field_names,)* } => { #message_body }
        });
      }
      Fields::Unit => {
        message_arms.push(quote! { #name::#variant_name => { #message_body } });
      }
      Fields::Unnamed(_) => {
        message_arms.push(quote! { #name::#variant_name(..) => { #message_body } });
      }
    }
  }

  // Strip #[message] attributes from variants
  if let Data::Enum(ref mut data) = input.data {
    for variant in &mut data.variants {
      variant
        .attrs
        .retain(|attribute| !attribute.path().is_ident("message"));
    }
  }

  let attrs = &input.attrs;
  let visibility = &input.vis;
  let variants = match &input.data {
    Data::Enum(data) => &data.variants,
    _ => unreachable!(),
  };

  let output = quote! {
    #(#attrs)*
    #[derive(Debug, Clone, PartialEq, Eq)]
    #visibility enum #name {
      #variants
    }

    impl #name {
      /// Kebab-case error code for LSP/CLI output
      pub fn code_str(&self) -> &'static str {
        match self {
          #(#code_string_arms,)*
        }
      }

      /// Human-readable error message
      #[allow(unused_variables)]
      pub fn message(&self) -> String {
        match self {
          #(#message_arms)*
        }
      }
    }
  };

  output.into()
}

fn parse_message_attr(attr: &syn::Attribute, fields: &Fields) -> TokenStream2 {
  let Meta::List(list) = &attr.meta else {
    return quote! { format!("") };
  };

  let args: Punctuated<Expr, Token![,]> = match list.parse_args_with(Punctuated::parse_terminated) {
    Ok(args) => args,
    Err(error) => return error.to_compile_error(),
  };

  let mut iter = args.iter();
  let Some(first) = iter.next() else {
    return quote! { format!("") };
  };

  let Expr::Lit(lit) = first else {
    return quote! { format!("") };
  };
  let Lit::Str(string) = &lit.lit else {
    return quote! { format!("") };
  };
  let template = string.value();

  let extra_args: Vec<_> = iter.collect();

  if !extra_args.is_empty() {
    return quote! { format!(#template, #(#extra_args),*) };
  }

  match fields {
    Fields::Named(named) => {
      let used_fields: Vec<_> = named
        .named
        .iter()
        .filter(|field| {
          let field_name = field.ident.as_ref().unwrap().to_string();
          template.contains(&format!("{{{field_name}}}"))
            || template.contains(&format!("{{{field_name}:"))
        })
        .map(|field| &field.ident)
        .collect();

      if used_fields.is_empty() {
        quote! { format!(#template) }
      } else {
        quote! { format!(#template, #(#used_fields = #used_fields),*) }
      }
    }
    _ => quote! { format!(#template) },
  }
}

fn pascal_to_kebab(name: &str) -> String {
  let mut result = String::new();
  let mut chars = name.chars().peekable();
  while let Some(character) = chars.next() {
    if character.is_uppercase() {
      if !result.is_empty() {
        let next_is_lower = chars.peek().is_some_and(|next| next.is_lowercase());
        let previous_is_lower_or_digit = result
          .ends_with(|previous: char| previous.is_ascii_lowercase() || previous.is_ascii_digit());
        if previous_is_lower_or_digit || next_is_lower {
          result.push('-');
        }
      }
      result.push(character.to_ascii_lowercase());
    } else {
      result.push(character);
    }
  }
  result
}
