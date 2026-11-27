oreql_macros::define_syntax_kinds! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  #[repr(u16)]
  pub enum OreqlSyntaxKind {
    group is_token {
      // Whitespace, newlines, comments
      group is_trivia {
        // ' ', '\t'
        Whitespace,
        // '\n', '\r\n'
        Newline,
        // -- line comment
        LineComment,
        // /* block comment */
        BlockComment,
      }

      // Delimiters and separators
      group is_punctuation {
        // (
        LParen,
        // )
        RParen,
        // [
        LBracket,
        // ]
        RBracket,
        // {
        LBrace,
        // }
        RBrace,
        // ,
        Comma,
        // .
        Dot,
        // ;
        Semicolon,
        // :
        Colon,
        // ::
        DColon,
        // ?
        Question,
      }

      // Arithmetic, comparison, logical, bitwise operators
      group is_operator {
        // +
        Plus,
        // -
        Minus,
        // *
        Star,
        // /
        Slash,
        // \
        Backslash,
        // %
        Percent,
        // **
        DStar,
        // =
        Eq,
        // != or <>
        Neq,
        // <
        Lt,
        // <=
        Lte,
        // >
        Gt,
        // >=
        Gte,
        // <=>
        NullSafeEq,
        // :=
        ColonEq,
        // &
        Amp,
        // &&
        DAmp,
        // |
        Pipe,
        // ||
        DPipe,
        // |>
        PipeGt,
        // ^
        Caret,
        // ~
        Tilde,
        // ->
        Arrow,
        // ->>
        DArrow,
        // =>
        FatArrow,
        // #>
        HashArrow,
        // #>>
        DHashArrow,
        // <->
        LRArrow,
        // @
        At,
        // @@
        DAt,
        // #
        Hash,
        // $
        Dollar,
        // !
        Exclamation,
      }

      // Literal value tokens
      group is_literal {
        // 42, 3.14, 1e10
        Number,
        // 'hello', "world"
        String,
        // b'0101'
        BitString,
        // x'FF'
        HexString,
        // b'bytes'
        ByteString,
        // N'unicode'
        NationalString,
        // r'raw\nstring'
        RawString,
        // $$heredoc$$
        HeredocString,
        // U&'unicode'
        UnicodeString,
      }

      // Names and references
      group is_identifier {
        // my_table, "quoted_name", `backtick`
        Identifier,
        // @param
        Parameter,
        // @@global_var
        SessionParameter,
        // unresolved name
        Var,
      }

      // SQL type name keywords
      group is_type_keyword {
        // BIT
        KeywordBit,
        // BOOLEAN, BOOL
        KeywordBoolean,
        // TINYINT, INT1
        KeywordTinyint,
        // SMALLINT, INT2
        KeywordSmallint,
        // MEDIUMINT
        KeywordMediumint,
        // INT, INTEGER
        KeywordInt,
        // BIGINT, INT8
        KeywordBigint,
        // INT128
        KeywordInt128,
        // INT256
        KeywordInt256,
        // UINT
        KeywordUint,
        // UINT128
        KeywordUint128,
        // UINT256
        KeywordUint256,
        // FLOAT, REAL
        KeywordFloat,
        // DOUBLE, DOUBLE PRECISION
        KeywordDouble,
        // DECIMAL, NUMERIC, NUMBER
        KeywordDecimal,
        // BIGDECIMAL, BIGNUMERIC
        KeywordBigdecimal,
        // CHAR, CHARACTER
        KeywordChar,
        // NCHAR
        KeywordNchar,
        // VARCHAR
        KeywordVarchar,
        // NVARCHAR
        KeywordNvarchar,
        // TEXT, STRING, CLOB
        KeywordText,
        // BINARY
        KeywordBinary,
        // VARBINARY, BYTEA, BLOB
        KeywordVarbinary,
        // JSON
        KeywordJson,
        // JSONB
        KeywordJsonb,
        // DATE
        KeywordDate,
        // TIME
        KeywordTime,
        // TIMETZ
        KeywordTimetz,
        // TIMESTAMP
        KeywordTimestamp,
        // TIMESTAMPTZ
        KeywordTimestamptz,
        // TIMESTAMPLTZ
        KeywordTimestampltz,
        // TIMESTAMPNTZ
        KeywordTimestampntz,
        // DATETIME
        KeywordDatetime,
        // INTERVAL
        KeywordInterval,
        // UUID
        KeywordUuid,
        // XML
        KeywordXml,
        // SERIAL
        KeywordSerial,
        // BIGSERIAL
        KeywordBigserial,
        // MONEY
        KeywordMoney,
        // GEOMETRY
        KeywordGeometry,
        // GEOGRAPHY
        KeywordGeography,
        // HSTORE
        KeywordHstore,
        // INET
        KeywordInet,
        // ENUM
        KeywordEnum,
        // VARIANT
        KeywordVariant,
        // OBJECT
        KeywordObject,
        // ARRAY
        KeywordArray,
        // MAP
        KeywordMap,
        // STRUCT
        KeywordStruct,
        // NULLABLE
        KeywordNullable,
        // VECTOR
        KeywordVector,
        // SUPER
        KeywordSuper,
        // VOID
        KeywordVoid,
        // UNKNOWN
        KeywordUnknown,
      }

      // SQL language keywords
      group is_keyword {
        // ALL
        KeywordAll,
        // ALTER
        KeywordAlter,
        // AND
        KeywordAnd,
        // ANTI
        KeywordAnti,
        // ANY
        KeywordAny,
        // APPLY
        KeywordApply,
        // AS
        KeywordAs,
        // ASC
        KeywordAsc,
        // ASOF
        KeywordAsof,
        // BEGIN
        KeywordBegin,
        // BETWEEN
        KeywordBetween,
        // BY
        KeywordBy,
        // CACHE
        KeywordCache,
        // CASE
        KeywordCase,
        // CAST
        KeywordCast,
        // CHARACTER SET
        KeywordCharacterSet,
        // CHECK
        KeywordCheck,
        // COLLATE
        KeywordCollate,
        // COLUMN
        KeywordColumn,
        // COMMENT
        KeywordComment,
        // COMMIT
        KeywordCommit,
        // CONNECT BY
        KeywordConnectBy,
        // CONSTRAINT
        KeywordConstraint,
        // COPY
        KeywordCopy,
        // CREATE
        KeywordCreate,
        // CROSS
        KeywordCross,
        // CUBE
        KeywordCube,
        // CURRENT_DATE
        KeywordCurrentDate,
        // CURRENT_TIME
        KeywordCurrentTime,
        // CURRENT_TIMESTAMP
        KeywordCurrentTimestamp,
        // CURRENT_USER
        KeywordCurrentUser,
        // DATABASE
        KeywordDatabase,
        // DECLARE
        KeywordDeclare,
        // DEFAULT
        KeywordDefault,
        // DELETE
        KeywordDelete,
        // DESC
        KeywordDesc,
        // DESCRIBE
        KeywordDescribe,
        // DISTINCT
        KeywordDistinct,
        // DISTRIBUTE BY
        KeywordDistributeBy,
        // DIV
        KeywordDiv,
        // DROP
        KeywordDrop,
        // ELSE
        KeywordElse,
        // END
        KeywordEnd,
        // ESCAPE
        KeywordEscape,
        // EXCEPT
        KeywordExcept,
        // EXECUTE
        KeywordExecute,
        // EXISTS
        KeywordExists,
        // EXPLAIN
        KeywordExplain,
        // FALSE
        KeywordFalse,
        // FETCH
        KeywordFetch,
        // FILE
        KeywordFile,
        // FILTER
        KeywordFilter,
        // FIRST
        KeywordFirst,
        // FOR
        KeywordFor,
        // FOREIGN KEY
        KeywordForeignKey,
        // FORMAT
        KeywordFormat,
        // FROM
        KeywordFrom,
        // FULL
        KeywordFull,
        // FUNCTION
        KeywordFunction,
        // GLOB
        KeywordGlob,
        // GLOBAL
        KeywordGlobal,
        // GRANT
        KeywordGrant,
        // GROUP BY
        KeywordGroupBy,
        // GROUPING SETS
        KeywordGroupingSets,
        // HAVING
        KeywordHaving,
        // IF
        KeywordIf,
        // IGNORE
        KeywordIgnore,
        // ILIKE
        KeywordIlike,
        // IN
        KeywordIn,
        // INDEX
        KeywordIndex,
        // INNER
        KeywordInner,
        // INSERT
        KeywordInsert,
        // INTERSECT
        KeywordIntersect,
        // INTO
        KeywordInto,
        // IS
        KeywordIs,
        // JOIN
        KeywordJoin,
        // KEEP
        KeywordKeep,
        // KEY
        KeywordKey,
        // KILL
        KeywordKill,
        // LANGUAGE
        KeywordLanguage,
        // LATERAL
        KeywordLateral,
        // LEFT
        KeywordLeft,
        // LIKE
        KeywordLike,
        // LIMIT
        KeywordLimit,
        // LOAD
        KeywordLoad,
        // LOCAL
        KeywordLocal,
        // LOCK
        KeywordLock,
        // MATCH
        KeywordMatch,
        // MERGE
        KeywordMerge,
        // MOD
        KeywordMod,
        // NATURAL
        KeywordNatural,
        // NEXT
        KeywordNext,
        // NOT
        KeywordNot,
        // NOTNULL
        KeywordNotnull,
        // NULL
        KeywordNull,
        // OFFSET
        KeywordOffset,
        // ON
        KeywordOn,
        // ONLY
        KeywordOnly,
        // OPERATOR
        KeywordOperator,
        // OR
        KeywordOr,
        // ORDER BY
        KeywordOrderBy,
        // ORDINALITY
        KeywordOrdinality,
        // OUT
        KeywordOut,
        // OUTER
        KeywordOuter,
        // OVER
        KeywordOver,
        // OVERLAPS
        KeywordOverlaps,
        // OVERWRITE
        KeywordOverwrite,
        // PARTITION
        KeywordPartition,
        // PARTITION BY
        KeywordPartitionBy,
        // PERCENT
        KeywordPercent,
        // PIVOT
        KeywordPivot,
        // PRAGMA
        KeywordPragma,
        // PRIMARY KEY
        KeywordPrimaryKey,
        // PROCEDURE
        KeywordProcedure,
        // QUALIFY
        KeywordQualify,
        // RANGE
        KeywordRange,
        // RECURSIVE
        KeywordRecursive,
        // REFERENCES
        KeywordReferences,
        // RENAME
        KeywordRename,
        // REPLACE
        KeywordReplace,
        // RETURNING
        KeywordReturning,
        // REVOKE
        KeywordRevoke,
        // RIGHT
        KeywordRight,
        // RLIKE
        KeywordRlike,
        // ROLLBACK
        KeywordRollback,
        // ROLLUP
        KeywordRollup,
        // ROW
        KeywordRow,
        // ROWS
        KeywordRows,
        // SCHEMA
        KeywordSchema,
        // SELECT
        KeywordSelect,
        // SEMI
        KeywordSemi,
        // SEQUENCE
        KeywordSequence,
        // SESSION
        KeywordSession,
        // SESSION_USER
        KeywordSessionUser,
        // SET
        KeywordSet,
        // SHOW
        KeywordShow,
        // SIMILAR TO
        KeywordSimilarTo,
        // SOME
        KeywordSome,
        // SORT BY
        KeywordSortBy,
        // START WITH
        KeywordStartWith,
        // STRAIGHT_JOIN
        KeywordStraightJoin,
        // TABLE
        KeywordTable,
        // TABLESAMPLE
        KeywordTableSample,
        // TEMPORARY, TEMP
        KeywordTemporary,
        // THEN
        KeywordThen,
        // TOP
        KeywordTop,
        // TRIGGER
        KeywordTrigger,
        // TRUE
        KeywordTrue,
        // TRUNCATE
        KeywordTruncate,
        // TYPE
        KeywordType,
        // UNCACHE
        KeywordUncache,
        // UNION
        KeywordUnion,
        // UNIQUE
        KeywordUnique,
        // UNNEST
        KeywordUnnest,
        // UNPIVOT
        KeywordUnpivot,
        // UPDATE
        KeywordUpdate,
        // USE
        KeywordUse,
        // USING
        KeywordUsing,
        // VALUES
        KeywordValues,
        // VIEW
        KeywordView,
        // VOLATILE
        KeywordVolatile,
        // WHEN
        KeywordWhen,
        // WHERE
        KeywordWhere,
        // WINDOW
        KeywordWindow,
        // WITH
        KeywordWith,
        // XOR
        KeywordXor,
        // ANALYZE
        KeywordAnalyze,
      }

      // end of file
      Eof,
      // error token
      Error,
    }

    group is_node {
      // Top-level SQL statements
      group is_statement {
        // top-level program
        Program,
        // SELECT ...
        SelectStatement,
        // INSERT INTO ... VALUES ...
        InsertStatement,
        // UPDATE ... SET ...
        UpdateStatement,
        // DELETE FROM ...
        DeleteStatement,
        // CREATE TABLE/VIEW/INDEX ...
        CreateStatement,
        // ALTER TABLE/VIEW ...
        AlterStatement,
        // DROP TABLE/VIEW/INDEX ...
        DropStatement,
        // TRUNCATE TABLE ...
        TruncateStatement,
        // MERGE INTO ...
        MergeStatement,
        // BEGIN ... END
        BeginStatement,
        // COMMIT
        CommitStatement,
        // ROLLBACK
        RollbackStatement,
        // GRANT ... TO ...
        GrantStatement,
        // REVOKE ... FROM ...
        RevokeStatement,
        // EXPLAIN SELECT ...
        ExplainStatement,
        // DECLARE @x INT
        DeclareStatement,
        // SET @x = 1
        SetStatement,
        // SHOW TABLES
        ShowStatement,
        // USE database
        UseStatement,
        // EXECUTE proc
        ExecuteStatement,
        // COPY ... INTO ...
        CopyStatement,
        // LOAD DATA ...
        LoadStatement,
        // DESCRIBE table
        DescribeStatement,
        // PRAGMA ...
        PragmaStatement,
      }

      // Subordinate clauses within statements
      group is_clause {
        // SELECT a, b
        SelectClause,
        // FROM table
        FromClause,
        // WHERE cond
        WhereClause,
        // GROUP BY col
        GroupByClause,
        // HAVING cond
        HavingClause,
        // ORDER BY col ASC
        OrderByClause,
        // LIMIT 10
        LimitClause,
        // OFFSET 5
        OffsetClause,
        // FETCH FIRST 10 ROWS ONLY
        FetchClause,
        // WINDOW w AS (...)
        WindowClause,
        // QUALIFY row_number() > 1
        QualifyClause,
        // INTO target
        IntoClause,
        // SET col = val
        SetClause,
        // VALUES (1, 2), (3, 4)
        ValuesClause,
        // RETURNING *
        ReturningClause,
        // WITH cte AS (...)
        WithClause,
        // PARTITION BY col
        PartitionByClause,
        // WHEN cond THEN result
        WhenClause,
        // ON condition
        JoinOnClause,
        // USING (col)
        JoinUsingClause,
        // WHEN MATCHED THEN UPDATE ...
        MergeWhenClause,
        // WHEN NOT MATCHED THEN INSERT ...
        MergeInsertClause,
      }

      // Value-producing expressions
      group is_expression {
        // a + b, x AND y
        BinExpression,
        // -x, NOT y
        UnaryExpression,
        // (expr)
        ParenExpression,
        // col BETWEEN 1 AND 10
        BetweenExpression,
        // col IN (1, 2, 3)
        InExpression,
        // col IS NULL
        IsExpression,
        // col LIKE 'pattern'
        LikeExpression,
        // col ILIKE 'pattern'
        IlikeExpression,
        // col RLIKE 'pattern'
        RlikeExpression,
        // col SIMILAR TO 'pattern'
        SimilarToExpression,
        // CASE WHEN ... THEN ... END
        CaseExpression,
        // CAST(x AS INT)
        CastExpression,
        // TRY_CAST(x AS INT)
        TryCastExpression,
        // EXTRACT(YEAR FROM date)
        ExtractExpression,
        // SUBSTRING(str FROM 1 FOR 3)
        SubstringExpression,
        // TRIM(BOTH ' ' FROM str)
        TrimExpression,
        // EXISTS (SELECT ...)
        ExistsExpression,
        // (SELECT ...)
        SubqueryExpression,
        // array[1]
        BracketExpression,
        // db.schema.table
        DotExpression,
        // *
        StarExpression,
        // table.*
        QualifiedStarExpression,
        // fn(args) FILTER (WHERE cond)
        FilterExpression,
        // fn(x ORDER BY y)
        WithinGroupExpression,
        // t1 AS alias
        AliasExpression,
        // ... JOIN ... ON ...
        JoinExpression,
        // LATERAL (...)
        LateralExpression,
        // UNNEST(array)
        UnnestExpression,
        // PIVOT (...)
        PivotExpression,
        // UNPIVOT (...)
        UnpivotExpression,
        // TABLESAMPLE (10 PERCENT)
        TableSampleExpression,
        // name AS (SELECT ...)
        CteExpression,
        // SELECT ... UNION SELECT ...
        UnionExpression,
        // SELECT ... INTERSECT SELECT ...
        IntersectExpression,
        // SELECT ... EXCEPT SELECT ...
        ExceptExpression,
        // x -> x + 1
        LambdaExpression,
        // GROUPING SETS ((a), (b))
        GroupingSetsExpression,
        // CUBE(a, b)
        CubeExpression,
        // ROLLUP(a, b)
        RollupExpression,
        // (1, 2, 3)
        TupleExpression,
        // ROW(1, 2)
        RowExpression,
        // ARRAY[1, 2, 3]
        ArrayExpression,
        // JSON path: $.key
        JsonPathExpression,
        // :: type cast
        TypeCastExpression,
        // placeholder: ?, $1, :name
        PlaceholderExpression,
        // command string after SHOW/EXPLAIN
        CommandExpression,
      }

      // Function invocation nodes
      group is_function_call {
        // fn(args)
        FunctionCall,
        // COUNT(DISTINCT x)
        AggregateFunctionCall,
        // ROW_NUMBER() OVER (...)
        WindowFunctionCall,
      }

      // Literal value nodes in the tree
      group is_literal_node {
        // 42, 3.14
        NumberLiteral,
        // 'hello'
        StringLiteral,
        // TRUE, FALSE
        BooleanLiteral,
        // NULL
        NullLiteral,
        // INTERVAL '1' DAY
        IntervalLiteral,
        // DATE '2024-01-01'
        DateLiteral,
        // TIMESTAMP '2024-01-01 00:00:00'
        TimestampLiteral,
      }

      // Named object references
      group is_reference {
        // my_col
        ColumnReference,
        // my_table
        TableReference,
        // my_schema
        SchemaReference,
        // my_db
        DatabaseReference,
      }

      // Window specification nodes
      group is_window {
        // (PARTITION BY a ORDER BY b ROWS BETWEEN ...)
        WindowSpec,
        // ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
        WindowFrame,
      }

      // Data definition language nodes
      group is_ddl {
        // col INT NOT NULL
        ColumnDefinition,
        // INT, VARCHAR(255)
        DataType,
        // PRIMARY KEY (id)
        PrimaryKeyConstraint,
        // FOREIGN KEY (col) REFERENCES ...
        ForeignKeyConstraint,
        // UNIQUE (col)
        UniqueConstraint,
        // CHECK (expr)
        CheckConstraint,
        // NOT NULL
        NotNullConstraint,
        // DEFAULT expr
        DefaultConstraint,
        // AUTO_INCREMENT
        AutoIncrementConstraint,
        // COMMENT 'text'
        CommentConstraint,
        // COLLATE utf8
        CollateConstraint,
      }

      // col ASC NULLS FIRST
      OrderByItem,

      // Error recovery
      group is_error {
        // error recovery node
        ErrorNode,
      }
    }
  }
}
