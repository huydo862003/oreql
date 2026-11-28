mod collect;
mod parse;

use proc_macro::TokenStream;
use quote::quote;

use collect::{collect_syntax_groups, collect_syntax_variants};
use parse::SyntaxKindEnumInput;

pub fn define_syntax_kinds_impl(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as SyntaxKindEnumInput);
  let name = &input.name;
  let visibility = &input.visibility;
  let enum_attrs = &input.attrs;

  let mut all_variants = Vec::new();
  let mut all_groups = Vec::new();

  collect_syntax_variants(&input.children, &mut all_variants);
  collect_syntax_groups(&input.children, &mut all_groups);

  let enum_variants = all_variants.iter().map(|(attrs, variant_name)| {
    quote! { #(#attrs)* #variant_name, }
  });

  let group_methods = all_groups.iter().map(|(predicate, attrs, variants)| {
    quote! {
      #(#attrs)*
      #[inline]
      #visibility const fn #predicate(self) -> bool {
        matches!(self, #(Self::#variants)|*)
      }
    }
  });

  quote! {
    #(#enum_attrs)*
    #visibility enum #name {
      #(#enum_variants)*
    }

    impl #name {
      #(#group_methods)*
    }
  }
  .into()
}
