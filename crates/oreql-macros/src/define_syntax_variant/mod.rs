mod collect;
mod parse;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;

use crate::utils::snake_to_pascal;
use collect::{GroupInfo, NodeInfo, collect};
use parse::SyntaxInput;

/// Generate the SyntaxKind enum, group predicates, and typed wrappers
pub fn define_syntax_variant_impl(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as SyntaxInput);
  let enum_name = &input.name;
  let visibility = &input.visibility;
  let enum_attrs = &input.attrs;

  let collected = collect(&input.children);

  let enum_variants = collected.variants.iter().map(|(attrs, name)| {
    quote! { #(#attrs)* #name, }
  });

  let group_predicates = collected.groups.iter().map(|group| {
    let predicate = &group.predicate_name;
    let attrs = &group.attrs;
    let variants = &group.variant_names;
    quote! {
      #(#attrs)*
      #[inline]
      #visibility const fn #predicate(self) -> bool {
        matches!(self, #(Self::#variants)|*)
      }
    }
  });

  let group_structs = collected
    .groups
    .iter()
    .map(|group| generate_group_struct(visibility, group));
  let node_structs = collected
    .nodes
    .iter()
    .map(|node| generate_node_struct(enum_name, visibility, node));

  quote! {
    #(#enum_attrs)*
    #visibility enum #enum_name {
      #(#enum_variants)*
    }

    impl #enum_name {
      #(#group_predicates)*
    }

    #(#group_structs)*
    #(#node_structs)*
  }
  .into()
}

/// Generate a typed wrapper struct for a group with AstNode impl and parent conversions
fn generate_group_struct(
  visibility: &syn::Visibility,
  group: &GroupInfo,
) -> proc_macro2::TokenStream {
  let predicate = &group.predicate_name;
  let struct_name = predicate_to_struct_name(predicate);
  let attrs = &group.attrs;

  let parent_conversions = group.parent_predicates.iter().map(|parent_pred| {
    let parent_struct = predicate_to_struct_name(parent_pred);
    quote! {
      impl<'a> ::core::convert::From<#struct_name<'a>> for #parent_struct<'a> {
        #[inline]
        fn from(node: #struct_name<'a>) -> Self {
          Self { node: node.node, tree: node.tree }
        }
      }

      impl<'a> ::core::convert::TryFrom<#parent_struct<'a>> for #struct_name<'a> {
        type Error = ();

        #[inline]
        fn try_from(node: #parent_struct<'a>) -> Result<Self, ()> {
          <Self as ::oreql_lang::AstNode>::cast(node.node, node.tree).ok_or(())
        }
      }
    }
  });

  quote! {
    #(#attrs)*
    #[derive(Clone, Copy)]
    #visibility struct #struct_name<'a> {
      node: ::oreql_lang::OreqlSyntaxNode,
      tree: &'a ::oreql_lang::SyntaxTree,
    }

    impl<'a> #struct_name<'a> {
      #[inline]
      #visibility fn from_syntax(tree: &'a ::oreql_lang::SyntaxTree, node: ::oreql_lang::OreqlSyntaxNode) -> Option<Self> {
        if node.kind(tree).#predicate() {
          Some(Self { node, tree })
        } else {
          None
        }
      }
    }

    impl<'a> ::oreql_lang::AstNode<'a> for #struct_name<'a> {
      #[inline]
      fn cast(node: ::oreql_lang::OreqlSyntaxNode, tree: &'a ::oreql_lang::SyntaxTree) -> Option<Self> {
        Self::from_syntax(tree, node)
      }

      #[inline]
      fn syntax(&self) -> ::oreql_lang::OreqlSyntaxNode {
        self.node
      }

      #[inline]
      fn tree(&self) -> &'a ::oreql_lang::SyntaxTree {
        self.tree
      }
    }

    #(#parent_conversions)*
  }
}

/// Generate a typed wrapper struct for a node variant with child accessors and AstNode impl
fn generate_node_struct(
  enum_name: &syn::Ident,
  visibility: &syn::Visibility,
  node: &NodeInfo,
) -> proc_macro2::TokenStream {
  let name = &node.name;

  let mut kind_counts: HashMap<String, usize> = HashMap::new();
  let accessors = node.fields.iter().map(|field| {
    let field_name = &field.name;
    let child_kind = &field.kind;
    let key = child_kind.to_string();
    let nth = *kind_counts.get(&key).unwrap_or(&0);
    kind_counts.insert(key, nth + 1);

    if nth == 0 {
      quote! {
        #[inline]
        #visibility fn #field_name(&self) -> Option<::oreql_lang::OreqlSyntaxNode> {
          self.node.first_child_of_kind(self.tree, #enum_name::#child_kind)
        }
      }
    } else {
      quote! {
        #[inline]
        #visibility fn #field_name(&self) -> Option<::oreql_lang::OreqlSyntaxNode> {
          self.node.nth_child_of_kind(self.tree, #enum_name::#child_kind, #nth)
        }
      }
    }
  });

  // Generate From for each group this node belongs to
  let group_conversions = node.group_predicates.iter().map(|pred| {
    let group_struct = predicate_to_struct_name(pred);
    quote! {
      impl<'a> ::core::convert::From<#name<'a>> for #group_struct<'a> {
        #[inline]
        fn from(node: #name<'a>) -> Self {
          Self { node: node.node, tree: node.tree }
        }
      }
    }
  });

  quote! {
    #[derive(Clone, Copy)]
    #visibility struct #name<'a> {
      node: ::oreql_lang::OreqlSyntaxNode,
      tree: &'a ::oreql_lang::SyntaxTree,
    }

    impl<'a> #name<'a> {
      #[inline]
      #visibility fn from_syntax(tree: &'a ::oreql_lang::SyntaxTree, node: ::oreql_lang::OreqlSyntaxNode) -> Option<Self> {
        if node.kind(tree) == #enum_name::#name {
          Some(Self { node, tree })
        } else {
          None
        }
      }

      #(#accessors)*
    }

    impl<'a> ::oreql_lang::AstNode<'a> for #name<'a> {
      #[inline]
      fn cast(node: ::oreql_lang::OreqlSyntaxNode, tree: &'a ::oreql_lang::SyntaxTree) -> Option<Self> {
        Self::from_syntax(tree, node)
      }

      #[inline]
      fn syntax(&self) -> ::oreql_lang::OreqlSyntaxNode {
        self.node
      }

      #[inline]
      fn tree(&self) -> &'a ::oreql_lang::SyntaxTree {
        self.tree
      }
    }

    #(#group_conversions)*
  }
}

/// Convert a group predicate name (e.g. is_statement) to a PascalCase struct name (Statement)
fn predicate_to_struct_name(predicate: &syn::Ident) -> syn::Ident {
  let snake = predicate.to_string();
  let stripped = snake.strip_prefix("is_").unwrap_or(&snake);
  format_ident!("{}", snake_to_pascal(stripped))
}
