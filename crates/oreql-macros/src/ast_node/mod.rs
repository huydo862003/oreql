mod parse;

use proc_macro::TokenStream;
use quote::{format_ident, quote};

use crate::utils::pascal_to_snake;
use parse::{AstNodeField, parse_fields, parse_groups};

fn generate_accessors<'a>(
  visibility: &'a syn::Visibility,
  fields: &'a [AstNodeField],
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
  fields.iter().map(move |field| {
    let field_name = &field.name;
    let child_kind = &field.kind;
    let nth = field.nth;

    if nth == 0 {
      quote! {
        #[inline]
        #visibility fn #field_name(&self) -> Option<OreqlRedNode> {
          self.node.first_child_of_kind(self.tree, OreqlSyntaxKind::#child_kind)
        }
      }
    } else {
      quote! {
        #[inline]
        #visibility fn #field_name(&self) -> Option<OreqlRedNode> {
          self.node.nth_child_of_kind(self.tree, OreqlSyntaxKind::#child_kind, #nth)
        }
      }
    }
  })
}

/// `#[ast_node]` or `#[ast_node(group1, group2)]`
pub fn ast_node_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
  let groups = parse_groups(attr);
  let item_struct = syn::parse_macro_input!(item as syn::ItemStruct);
  let visibility = &item_struct.vis;
  let name = &item_struct.ident;

  let fields = match parse_fields(&item_struct) {
    Ok(fields) => fields,
    Err(error) => return error,
  };

  let accessors = generate_accessors(visibility, &fields);

  let group_asserts = groups.iter().map(|predicate| {
    let assert_name = format_ident!("_assert_{name}_in_{predicate}");
    quote! {
      const #assert_name: () = assert!(
        OreqlSyntaxKind::#name.#predicate(),
        concat!(stringify!(#name), " is not in group ", stringify!(#predicate))
      );
    }
  });

  quote! {
    #(#group_asserts)*

    #[derive(Clone, Copy)]
    #visibility struct #name<'a> {
      node: OreqlRedNode,
      tree: &'a OreqlRedTree,
    }

    impl<'a> AstNode<'a> for #name<'a> {
      #[inline]
      fn cast(node: OreqlRedNode, tree: &'a OreqlRedTree) -> Option<Self> {
        if node.kind(tree) == OreqlSyntaxKind::#name {
          Some(Self { node, tree })
        } else {
          None
        }
      }

      #[inline]
      fn syntax(&self) -> OreqlRedNode {
        self.node
      }

      #[inline]
      fn tree(&self) -> &'a OreqlRedTree {
        self.tree
      }
    }

    impl<'a> ::core::convert::TryFrom<(OreqlRedNode, &'a OreqlRedTree)> for #name<'a> {
      type Error = ();

      #[inline]
      fn try_from((node, tree): (OreqlRedNode, &'a OreqlRedTree)) -> Result<Self, ()> {
        <Self as AstNode>::cast(node, tree).ok_or(())
      }
    }

    impl<'a> #name<'a> {
      #(#accessors)*
    }
  }
  .into()
}

/// `#[ast_group]` or `#[ast_group(predicate)]`
///
/// Predicate inferred from struct name as `is_{snake_case}` if not provided
pub fn ast_group_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
  let item_struct = syn::parse_macro_input!(item as syn::ItemStruct);
  let visibility = &item_struct.vis;
  let name = &item_struct.ident;

  let fields = match parse_fields(&item_struct) {
    Ok(fields) => fields,
    Err(error) => return error,
  };

  let accessors = generate_accessors(visibility, &fields);

  let groups = parse_groups(attr);
  let predicate = if groups.is_empty() {
    let snake = pascal_to_snake(&name.to_string());
    format_ident!("is_{snake}")
  } else {
    groups[0].clone()
  };
  let assert_name = format_ident!("_assert_{predicate}_exists");

  quote! {
    const #assert_name: bool = OreqlSyntaxKind::Eof.#predicate();

    #[derive(Clone, Copy)]
    #visibility struct #name<'a> {
      node: OreqlRedNode,
      tree: &'a OreqlRedTree,
    }

    impl<'a> AstNode<'a> for #name<'a> {
      #[inline]
      fn cast(node: OreqlRedNode, tree: &'a OreqlRedTree) -> Option<Self> {
        if node.kind(tree).#predicate() {
          Some(Self { node, tree })
        } else {
          None
        }
      }

      #[inline]
      fn syntax(&self) -> OreqlRedNode {
        self.node
      }

      #[inline]
      fn tree(&self) -> &'a OreqlRedTree {
        self.tree
      }
    }

    impl<'a> ::core::convert::TryFrom<(OreqlRedNode, &'a OreqlRedTree)> for #name<'a> {
      type Error = ();

      #[inline]
      fn try_from((node, tree): (OreqlRedNode, &'a OreqlRedTree)) -> Result<Self, ()> {
        <Self as AstNode>::cast(node, tree).ok_or(())
      }
    }

    impl<'a> #name<'a> {
      #(#accessors)*
    }
  }
  .into()
}
