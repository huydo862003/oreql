use super::OreqlSyntaxKind;
#[cfg(test)]
use super::syntax::SyntaxTreeBuilder;
use super::syntax::{OreqlSyntaxNode, SyntaxTree};

/// Trait for typed AST node wrappers
pub trait AstNode<'a>: Sized + Copy {
  /// Try to cast a syntax node to this typed node
  fn cast(node: OreqlSyntaxNode, tree: &'a SyntaxTree) -> Option<Self>;

  /// Underlying syntax node
  fn syntax(&self) -> OreqlSyntaxNode;

  /// The syntax tree this node belongs to
  fn tree(&self) -> &'a SyntaxTree;
}

// Group nodes

#[oreql_macros::ast_group]
pub struct Statement {}

#[oreql_macros::ast_group]
pub struct DmlStatement {}

#[oreql_macros::ast_group]
pub struct DdlStatement {}

#[oreql_macros::ast_group]
pub struct DclStatement {}

#[oreql_macros::ast_group]
pub struct TclStatement {}

#[oreql_macros::ast_group]
pub struct Expression {}

#[oreql_macros::ast_group]
pub struct QualifiedExpression {}

#[oreql_macros::ast_group]
pub struct CallExpression {}

#[oreql_macros::ast_group]
pub struct LiteralNode {}

#[oreql_macros::ast_group]
pub struct Reference {}

#[oreql_macros::ast_group]
pub struct TableExpression {}

#[oreql_macros::ast_group]
pub struct SetOperation {}

#[oreql_macros::ast_group]
pub struct Clause {}

#[oreql_macros::ast_group]
pub struct Token {}

#[oreql_macros::ast_group]
pub struct Trivia {}

// DML statements

#[oreql_macros::ast_node(dml_statement)]
pub struct SelectStatement {
  with_clause: Option<WithClause>,
  select_clause: SelectClause,
  from_clause: Option<FromClause>,
  where_clause: Option<WhereClause>,
  group_by_clause: Option<GroupByClause>,
  having_clause: Option<HavingClause>,
  window_clause: Option<WindowClause>,
  qualify_clause: Option<QualifyClause>,
  order_by_clause: Option<OrderByClause>,
  limit_clause: Option<LimitClause>,
  offset_clause: Option<OffsetClause>,
  fetch_clause: Option<FetchClause>,
  for_update_clause: Option<ForUpdateClause>,
}

#[oreql_macros::ast_node(dml_statement)]
pub struct InsertStatement {
  into_clause: Option<IntoClause>,
  select_clause: Option<SelectClause>,
  values_clause: Option<ValuesClause>,
  returning_clause: Option<ReturningClause>,
  on_conflict_clause: Option<OnConflictClause>,
  on_duplicate_key_clause: Option<OnDuplicateKeyClause>,
}

#[oreql_macros::ast_node(dml_statement)]
pub struct UpdateStatement {
  set_clause: SetClause,
  from_clause: Option<FromClause>,
  where_clause: Option<WhereClause>,
  order_by_clause: Option<OrderByClause>,
  limit_clause: Option<LimitClause>,
  returning_clause: Option<ReturningClause>,
}

#[oreql_macros::ast_node(dml_statement)]
pub struct DeleteStatement {
  from_clause: Option<FromClause>,
  where_clause: Option<WhereClause>,
  order_by_clause: Option<OrderByClause>,
  limit_clause: Option<LimitClause>,
  returning_clause: Option<ReturningClause>,
}

#[oreql_macros::ast_node(dml_statement)]
pub struct MergeStatement {
  merge_when_clause: Option<MergeWhenClause>,
  merge_insert_clause: Option<MergeInsertClause>,
}

// DDL statements

#[oreql_macros::ast_node(ddl_statement)]
pub struct CreateStatement {}

#[oreql_macros::ast_node(ddl_statement)]
pub struct AlterStatement {}

#[oreql_macros::ast_node(ddl_statement)]
pub struct DropStatement {}

#[oreql_macros::ast_node(ddl_statement)]
pub struct TruncateStatement {}

// DCL statements

#[oreql_macros::ast_node(dcl_statement)]
pub struct GrantStatement {}

#[oreql_macros::ast_node(dcl_statement)]
pub struct RevokeStatement {}

// TCL statements

#[oreql_macros::ast_node(tcl_statement)]
pub struct BeginStatement {}

#[oreql_macros::ast_node(tcl_statement)]
pub struct CommitStatement {}

#[oreql_macros::ast_node(tcl_statement)]
pub struct RollbackStatement {}

// Other statements

#[oreql_macros::ast_node(statement)]
pub struct ExplainStatement {}

#[oreql_macros::ast_node(statement)]
pub struct SetStatement {}

#[oreql_macros::ast_node(statement)]
pub struct ShowStatement {}

#[oreql_macros::ast_node(statement)]
pub struct UseStatement {}

#[oreql_macros::ast_node(statement)]
pub struct ExecuteStatement {}

#[oreql_macros::ast_node(statement)]
pub struct CallStatement {}

#[oreql_macros::ast_node(statement)]
pub struct DescribeStatement {}

#[oreql_macros::ast_node(statement)]
pub struct AnalyzeStatement {}

// Clauses

#[oreql_macros::ast_node(clause)]
pub struct SelectClause {}

#[oreql_macros::ast_node(clause)]
pub struct FromClause {}

#[oreql_macros::ast_node(clause)]
pub struct WhereClause {}

#[oreql_macros::ast_node(clause)]
pub struct GroupByClause {}

#[oreql_macros::ast_node(clause)]
pub struct HavingClause {}

#[oreql_macros::ast_node(clause)]
pub struct OrderByClause {}

#[oreql_macros::ast_node(clause)]
pub struct LimitClause {}

#[oreql_macros::ast_node(clause)]
pub struct OffsetClause {}

#[oreql_macros::ast_node(clause)]
pub struct FetchClause {}

#[oreql_macros::ast_node(clause)]
pub struct WindowClause {}

#[oreql_macros::ast_node(clause)]
pub struct QualifyClause {}

#[oreql_macros::ast_node(clause)]
pub struct IntoClause {}

#[oreql_macros::ast_node(clause)]
pub struct SetClause {}

#[oreql_macros::ast_node(clause)]
pub struct ValuesClause {}

#[oreql_macros::ast_node(clause)]
pub struct ReturningClause {}

#[oreql_macros::ast_node(clause)]
pub struct WithClause {}

#[oreql_macros::ast_node(clause)]
pub struct CommonTableExpressionClause {}

#[oreql_macros::ast_node(clause)]
pub struct ForUpdateClause {}

#[oreql_macros::ast_node(clause)]
pub struct OnConflictClause {}

#[oreql_macros::ast_node(clause)]
pub struct OnDuplicateKeyClause {}

#[oreql_macros::ast_node(clause)]
pub struct MergeWhenClause {}

#[oreql_macros::ast_node(clause)]
pub struct MergeInsertClause {}

#[oreql_macros::ast_node(clause)]
pub struct WhenClause {}

#[oreql_macros::ast_node(clause)]
pub struct ElseClause {}

#[oreql_macros::ast_node(clause)]
pub struct PartitionByClause {}

#[oreql_macros::ast_node(clause)]
pub struct JoinOnClause {}

#[oreql_macros::ast_node(clause)]
pub struct JoinUsingClause {}

// Expressions

#[oreql_macros::ast_node(expression)]
pub struct CaseExpression {
  keyword_case: KeywordCase,
  keyword_end: KeywordEnd,
}

#[oreql_macros::ast_node(expression)]
pub struct BinExpression {}

#[oreql_macros::ast_node(expression)]
pub struct UnaryExpression {}

#[oreql_macros::ast_node(expression)]
pub struct ParenExpression {}

#[oreql_macros::ast_node(expression)]
pub struct BetweenExpression {}

#[oreql_macros::ast_node(expression)]
pub struct InExpression {}

#[oreql_macros::ast_node(expression)]
pub struct IsExpression {}

#[oreql_macros::ast_node(expression)]
pub struct LikeExpression {
  keyword_escape: Option<KeywordEscape>,
}

#[oreql_macros::ast_node(expression)]
pub struct ExistsExpression {}

#[oreql_macros::ast_node(expression)]
pub struct SubqueryExpression {}

#[oreql_macros::ast_node(expression)]
pub struct BracketExpression {}

#[oreql_macros::ast_node(expression)]
pub struct DotExpression {}

#[oreql_macros::ast_node(expression)]
pub struct CastExpression {
  data_type: DataType,
}

#[oreql_macros::ast_node(expression)]
pub struct TryCastExpression {
  data_type: DataType,
}

#[oreql_macros::ast_node(expression)]
pub struct TypeCastExpression {
  data_type: DataType,
}

// Qualified expressions

#[oreql_macros::ast_node(qualified_expression)]
pub struct AliasExpression {
  keyword_as: Option<KeywordAs>,
}

#[oreql_macros::ast_node(qualified_expression)]
pub struct ExtractExpression {}

#[oreql_macros::ast_node(qualified_expression)]
pub struct SubstringExpression {}

#[oreql_macros::ast_node(qualified_expression)]
pub struct TrimExpression {}

#[oreql_macros::ast_node(qualified_expression)]
pub struct OverlayExpression {}

#[oreql_macros::ast_node(qualified_expression)]
pub struct PositionExpression {}

#[oreql_macros::ast_node(qualified_expression)]
pub struct CollateExpression {}

#[oreql_macros::ast_node(qualified_expression)]
pub struct AtTimeZoneExpression {}

// Function calls

#[oreql_macros::ast_node(call_expression)]
pub struct FunctionCall {}

#[oreql_macros::ast_node(call_expression)]
pub struct AggregateFunctionCall {}

#[oreql_macros::ast_node(call_expression)]
pub struct WindowFunctionCall {
  window_specification: Option<WindowSpecification>,
}

// Table expressions

#[oreql_macros::ast_node(table_expression)]
pub struct JoinExpression {
  join_on_clause: Option<JoinOnClause>,
  join_using_clause: Option<JoinUsingClause>,
}

#[oreql_macros::ast_node(table_expression)]
pub struct LateralExpression {}

#[oreql_macros::ast_node(table_expression)]
pub struct UnnestExpression {}

// Qualifiers

#[oreql_macros::ast_node(qualifier)]
pub struct WindowSpecification {
  partition_by_clause: Option<PartitionByClause>,
  order_by_clause: Option<OrderByClause>,
  window_frame: Option<WindowFrame>,
}

#[oreql_macros::ast_node(qualifier)]
pub struct WindowFrame {}

#[oreql_macros::ast_node(qualifier)]
pub struct OrderByItem {}

// DDL nodes

#[oreql_macros::ast_node(ddl_node)]
pub struct ColumnDefinition {
  data_type: Option<DataType>,
}

#[oreql_macros::ast_node(ddl_node)]
pub struct DataType {
  data_type_parameter: Option<DataTypeParameter>,
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::TextRange;

  /// Cast root to typed node and access children
  #[test]
  fn select_statement_typed_access() {
    let mut builder = SyntaxTreeBuilder::new();
    builder.start_node(OreqlSyntaxKind::SelectStatement);
    builder.start_node(OreqlSyntaxKind::SelectClause);
    builder.push_token(
      OreqlSyntaxKind::KeywordSelect,
      TextRange {
        start_byte_offset: 0,
        end_byte_offset: 6,
      },
    );
    builder.finish_node(OreqlSyntaxKind::SelectClause);
    builder.start_node(OreqlSyntaxKind::FromClause);
    builder.push_token(
      OreqlSyntaxKind::KeywordFrom,
      TextRange {
        start_byte_offset: 7,
        end_byte_offset: 11,
      },
    );
    builder.finish_node(OreqlSyntaxKind::FromClause);
    builder.start_node(OreqlSyntaxKind::WhereClause);
    builder.push_token(
      OreqlSyntaxKind::KeywordWhere,
      TextRange {
        start_byte_offset: 14,
        end_byte_offset: 19,
      },
    );
    builder.finish_node(OreqlSyntaxKind::WhereClause);
    builder.finish_node(OreqlSyntaxKind::SelectStatement);
    let tree = builder.finish();

    let root = tree.root;
    let select = SelectStatement::cast(root, &tree).unwrap();

    assert!(select.select_clause().is_some());
    assert!(select.from_clause().is_some());
    assert!(select.where_clause().is_some());
    assert!(select.having_clause().is_none());
  }

  /// Cast fails for wrong kind
  #[test]
  fn cast_wrong_kind_returns_none() {
    let mut builder = SyntaxTreeBuilder::new();
    builder.start_node(OreqlSyntaxKind::InsertStatement);
    builder.finish_node(OreqlSyntaxKind::InsertStatement);
    let tree = builder.finish();

    let root = tree.root;
    assert!(SelectStatement::cast(root, &tree).is_none());
    assert!(InsertStatement::cast(root, &tree).is_some());
  }

  /// Group node casts using predicate
  #[test]
  fn group_node_cast() {
    let mut builder = SyntaxTreeBuilder::new();
    builder.start_node(OreqlSyntaxKind::SelectStatement);
    builder.finish_node(OreqlSyntaxKind::SelectStatement);
    let tree = builder.finish();

    let root = tree.root;
    assert!(Statement::cast(root, &tree).is_some());
    assert!(DmlStatement::cast(root, &tree).is_some());
    assert!(Expression::cast(root, &tree).is_none());
  }
}
