mod parse;

use proc_macro::TokenStream;
use quote::quote;

use parse::ConfigInput;

pub fn dialect_config_impl(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as ConfigInput);

  let items = input.items.iter().map(|item| {
    let attrs = &item.attrs;
    let visibility = &item.visibility;
    let name = &item.name;
    let inner_type = &item.inner_type;
    let default_expression = &item.default_expression;

    quote! {
      #(#attrs)*
      #[repr(transparent)]
      #visibility struct #name(#inner_type);

      impl #name {
        /// Create from an inner value
        #[inline]
        #visibility fn new(inner: #inner_type) -> Self {
          Self(inner)
        }

        /// Get the default config
        #[inline]
        #visibility fn default() -> &'static Self {
          use ::std::sync::LazyLock;
          static VALUE: LazyLock<#name> = LazyLock::new(|| #name(#default_expression));
          &VALUE
        }
      }

      impl ::core::ops::Deref for #name {
        type Target = #inner_type;

        #[inline]
        fn deref(&self) -> &Self::Target {
          &self.0
        }
      }
    }
  });

  quote! { #(#items)* }.into()
}
