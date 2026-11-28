use syn::{Attribute, Ident};

use super::parse::{NodeField, SyntaxItem};

/// Collected info about a group for codegen
pub struct GroupInfo {
  pub predicate_name: Ident,
  pub attrs: Vec<Attribute>,
  pub variant_names: Vec<Ident>,
  pub parent_predicates: Vec<Ident>,
}

/// Collected info about a node variant for codegen
pub struct NodeInfo {
  pub name: Ident,
  pub fields: Vec<NodeField>,
  /// Group predicates this node belongs to, excluding is_node
  pub group_predicates: Vec<Ident>,
}

/// All collected data from one traversal
pub struct CollectedSyntax {
  pub variants: Vec<(Vec<Attribute>, Ident)>,
  pub groups: Vec<GroupInfo>,
  pub nodes: Vec<NodeInfo>,
}

/// Walk the syntax tree once to collect enum variants, groups, and node info
pub fn collect(items: &[SyntaxItem]) -> CollectedSyntax {
  let mut result = CollectedSyntax {
    variants: Vec::new(),
    groups: Vec::new(),
    nodes: Vec::new(),
  };
  collect_recursive(items, &[], false, &mut result);
  result
}

fn collect_recursive(
  items: &[SyntaxItem],
  parent_predicates: &[Ident],
  is_node: bool,
  out: &mut CollectedSyntax,
) {
  for item in items {
    match item {
      SyntaxItem::Variant(variant) => {
        out
          .variants
          .push((variant.attrs.clone(), variant.name.clone()));
        if is_node {
          out.nodes.push(NodeInfo {
            name: variant.name.clone(),
            fields: variant.fields.clone(),
            group_predicates: parent_predicates.to_vec(),
          });
        }
      }
      SyntaxItem::Group(group) => {
        let mut variant_names = Vec::new();
        collect_leaf_names(&group.children, &mut variant_names);
        out.groups.push(GroupInfo {
          predicate_name: group.predicate_name.clone(),
          attrs: group.attrs.clone(),
          variant_names,
          parent_predicates: parent_predicates.to_vec(),
        });

        let is_node_group = is_node || group.predicate_name == "is_node";
        let mut child_parents = parent_predicates.to_vec();
        child_parents.push(group.predicate_name.clone());
        collect_recursive(&group.children, &child_parents, is_node_group, out);
      }
    }
  }
}

/// Flatten all variant names under a group for the matches!() predicate
fn collect_leaf_names(items: &[SyntaxItem], out: &mut Vec<Ident>) {
  for item in items {
    match item {
      SyntaxItem::Variant(variant) => out.push(variant.name.clone()),
      SyntaxItem::Group(group) => collect_leaf_names(&group.children, out),
    }
  }
}
