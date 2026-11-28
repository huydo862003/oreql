mod comments;
mod edge_cases;
mod identifiers;
mod numbers;
mod operators;
mod strings;
mod trivia;

use super::*;
use crate::OreqlStringPool;
use crate::dialects::parse::DialectParser;

struct TestDialect;
impl DialectParser for TestDialect {}

fn tokenize_snapshot<D: DialectParser>(source: &str) -> String {
  let pool = OreqlStringPool::new(source);
  let mut context = ParserContext::<D>::new(&pool);
  let config = TokenizeConfig::default();
  let mut lines = Vec::new();
  loop {
    let (kind, string) = context.eat_next_token(&config);
    let text = std::str::from_utf8(string.get(&pool)).unwrap();
    lines.push(format!("({kind:?} {text:?})"));
    if kind == OreqlSyntaxKind::Eof {
      break;
    }
  }
  lines.join("\n")
}

fn tokenize_and_diagnostic_snapshot<D: DialectParser>(source: &str) -> (String, Vec<String>) {
  let pool = OreqlStringPool::new(source);
  let mut context = ParserContext::<D>::new(&pool);
  let config = TokenizeConfig::default();
  let mut lines = Vec::new();
  loop {
    let (kind, string) = context.eat_next_token(&config);
    let text = std::str::from_utf8(string.get(&pool)).unwrap();
    lines.push(format!("({kind:?} {text:?})"));
    if kind == OreqlSyntaxKind::Eof {
      break;
    }
  }
  let diagnostics = context
    .diagnostics
    .iter()
    .map(|diagnostic| diagnostic.message())
    .collect();
  (lines.join("\n"), diagnostics)
}

macro_rules! assert_snapshot {
  ($input:expr, $expected:expr) => {
    let input = $input;
    let expected = $expected;
    let output = tokenize_snapshot::<TestDialect>(input);
    assert_eq!(output, expected, "\ninput: {input:?}");
  };
}

pub(super) use assert_snapshot;
