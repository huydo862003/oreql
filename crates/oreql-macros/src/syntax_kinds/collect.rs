use syn::{Attribute, Ident};

use super::parse::SyntaxKindItem;

/// Recursively flatten all leaf variants for the enum body
pub fn collect_syntax_variants(items: &[SyntaxKindItem], out: &mut Vec<(Vec<Attribute>, Ident)>) {
  for item in items {
    match item {
      SyntaxKindItem::Variant(variant) => {
        out.push((variant.attrs.clone(), variant.name.clone()));
      }
      SyntaxKindItem::Group(group) => {
        collect_syntax_variants(&group.children, out);
      }
    }
  }
}

/// Recursively collect all groups with their flattened variant names
pub fn collect_syntax_groups(
  items: &[SyntaxKindItem],
  out: &mut Vec<(Ident, Vec<Attribute>, Vec<Ident>)>,
) {
  for item in items {
    if let SyntaxKindItem::Group(group) = item {
      let mut variant_names = Vec::new();
      collect_leaf_names(&group.children, &mut variant_names);
      out.push((
        group.predicate_name.clone(),
        group.attrs.clone(),
        variant_names,
      ));
      collect_syntax_groups(&group.children, out);
    }
  }
}

/// Recursively collect just the leaf variant Idents
fn collect_leaf_names(items: &[SyntaxKindItem], out: &mut Vec<Ident>) {
  for item in items {
    match item {
      SyntaxKindItem::Variant(variant) => out.push(variant.name.clone()),
      SyntaxKindItem::Group(group) => collect_leaf_names(&group.children, out),
    }
  }
}
