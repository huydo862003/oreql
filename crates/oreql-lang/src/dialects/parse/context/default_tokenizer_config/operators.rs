use crate::OreqlSyntaxKind;
use OreqlSyntaxKind::*;
use oreql_utils::TrieMap;

oreql_macros::dialect_config! {
  /// Operator patterns mapped to their token kind
  pub struct OperatorsConfig(TrieMap<OreqlSyntaxKind>)
    = TrieMap::from_entries([
      // Single-char
      (&b"+"[..], Plus),
      (b"-", Minus),
      (b"*", Star),
      (b"/", Slash),
      (b"\\", Backslash),
      (b"%", Percent),
      (b"=", Eq),
      (b"<", Lt),
      (b">", Gt),
      (b"!", Exclamation),
      (b"&", Amp),
      (b"|", Pipe),
      (b"^", Caret),
      (b"~", Tilde),
      (b"@", At),
      (b"#", Hash),
      // Comparison
      (b"==", Eq),
      (b">=", Gte),
      (b"<=", Lte),
      (b"<>", Neq),
      (b"!=", Neq),
      (b"<=>", NullSafeEq),
      // Assignment
      (b":=", ColonEq),
      // Double operators
      (b"::", DColon),
      (b"||", DPipe),
      (b"&&", DAmp),
      (b"**", DStar),
      (b"@@", DAt),
      (b"??", DQuestion),
      // Pipe
      (b"|>", PipeGt),
      (b"|/", PipeSlash),
      (b"||/", DPipeSlash),
      // Arrows
      (b"->", Arrow),
      (b"->>", DArrow),
      (b"=>", FatArrow),
      (b"<->", LRArrow),
      // JSON/JSONB operators
      (b"#>", HashArrow),
      (b"#>>", DHashArrow),
      (b"#-", HashDash),
      // Colon operators
      (b":>", ColonGt),
      (b"!:>", ExclamationColonGt),
      (b"?::", QuestionDColon),
      (b"::$", DColonDollar),
      (b"::%", DColonPercent),
      (b"::?", DColonQuestion),
      // Dot operators
      (b".:", DotColon),
      (b".^", DotCaret),
      // Range/GIS operators
      (b"&<", AmpLt),
      (b"&>", AmpGt),
      (b"-|-", Adjacent),
      (b"^@", CaretAt),
      // JSONB containment
      (b"?&", QuestionAmp),
      (b"?|", QuestionPipe),
      // Regex operators
      (b"~*", TildeStar),
      (b"~~~", KeywordGlob),
      (b"~~", KeywordLike),
      (b"~~*", KeywordIlike),
      // Jinja template
      (b"{%", JinjaBlockStart),
      (b"{%+", JinjaBlockStart),
      (b"{%-", JinjaBlockStart),
      (b"%}", JinjaBlockEnd),
      (b"+%}", JinjaBlockEnd),
      (b"-%}", JinjaBlockEnd),
      (b"{{+", JinjaBlockStart),
      (b"{{-", JinjaBlockStart),
      (b"+}}", JinjaBlockEnd),
      (b"-}}", JinjaBlockEnd),
    ]);
}
