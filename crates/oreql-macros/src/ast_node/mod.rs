use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, parse_macro_input};

struct AstNodeField {
  name: Ident,
  kind: Ident,
}

pub fn ast_node_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
  let kind = parse_macro_input!(attr as Ident);
  let item_struct = parse_macro_input!(item as syn::ItemStruct);

  let visibility = &item_struct.vis;
  let name = &item_struct.ident;

  let fields: Vec<AstNodeField> = match &item_struct.fields {
    syn::Fields::Named(named) => named
      .named
      .iter()
      .map(|field| AstNodeField {
        name: field.ident.clone().unwrap(),
        kind: match &field.ty {
          syn::Type::Path(path) => path.path.segments.last().unwrap().ident.clone(),
          _ => panic!("expected a path type"),
        },
      })
      .collect(),
    syn::Fields::Unit => Vec::new(),
    _ => {
      return syn::Error::new_spanned(name, "ast_node expects named fields or unit struct")
        .to_compile_error()
        .into();
    }
  };

  let accessors = fields.iter().map(|field| {
    let field_name = &field.name;
    let child_kind = &field.kind;

    quote! {
      #[inline]
      #visibility fn #field_name(&self) -> Option<OreqlRedNode> {
        self.node.first_child_of_kind(self.tree, OreqlSyntaxKind::#child_kind)
      }
    }
  });

  quote! {
    #[derive(Clone, Copy)]
    #visibility struct #name<'a> {
      node: OreqlRedNode,
      tree: &'a OreqlRedTree,
    }

    impl<'a> AstNode<'a> for #name<'a> {
      #[inline]
      fn cast(node: OreqlRedNode, tree: &'a OreqlRedTree) -> Option<Self> {
        if node.kind(tree) == OreqlSyntaxKind::#kind {
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

    impl<'a> #name<'a> {
      #(#accessors)*
    }
  }
  .into()
}
