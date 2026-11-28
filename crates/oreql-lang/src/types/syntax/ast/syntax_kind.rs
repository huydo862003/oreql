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
        // &<
        AmpLt,
        // &>
        AmpGt,
        // ??
        DQuestion,
        // ?::
        QuestionDColon,
        // ?&
        QuestionAmp,
        // ?|
        QuestionPipe,
        // ~*
        TildeStar,
        // |/
        PipeSlash,
        // ||/
        DPipeSlash,
        // ^@
        CaretAt,
        // .:
        DotColon,
        // .^
        DotCaret,
        // ::$
        DColonDollar,
        // ::%
        DColonPercent,
        // ::?
        DColonQuestion,
        // :>
        ColonGt,
        // !:>
        ExclamationColonGt,
        // #-
        HashDash,
        // -|-
        Adjacent,
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
        Variable,
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
      //
      // WARNING: multi-word keywords (e.g. KeywordGroupBy, KeywordOrderBy)
      // have a text span that includes the whitespace between words, so the
      // token may contain trivia (spaces, tabs)
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
        // ISNULL
        KeywordIsnull,
        // SETTINGS
        KeywordSettings,
        // SQL SECURITY
        KeywordSqlSecurity,
        // CLUSTER BY
        KeywordClusterBy,
        // LOCALTIME
        KeywordLocaltime,
        // LOCALTIMESTAMP
        KeywordLocaltimestamp,
        // NAMESPACE
        KeywordNamespace,
        // CURRENT_SCHEMA
        KeywordCurrentSchema,
        // CURRENT_CATALOG
        KeywordCurrentCatalog,
        // CURRENT_ROLE
        KeywordCurrentRole,
        // CURRENT_DATETIME
        KeywordCurrentDatetime,
      }

      // Jinja template tokens
      // {%  {%+  {%-
      JinjaBlockStart,
      // %}  +%}  -%}
      JinjaBlockEnd,

      // end of file
      Eof,
      // error token
      Error,
    }

    group is_node {
      // top-level program
      Program,

      // Statements
      group is_statement {
        // Data manipulation: SELECT, INSERT, UPDATE, DELETE, MERGE
        group is_dml_statement {
          // SELECT ...
          SelectStatement,
          // INSERT INTO ... VALUES ...
          InsertStatement,
          // UPDATE ... SET ...
          UpdateStatement,
          // DELETE FROM ...
          DeleteStatement,
          // MERGE INTO ...
          MergeStatement,
        }

        // Data definition: CREATE, ALTER, DROP, TRUNCATE
        group is_ddl_statement {
          // CREATE TABLE/VIEW/INDEX ...
          CreateStatement,
          // ALTER TABLE/VIEW ...
          AlterStatement,
          // DROP TABLE/VIEW/INDEX ...
          DropStatement,
          // TRUNCATE TABLE ...
          TruncateStatement,
        }

        // Data control: GRANT, REVOKE
        group is_dcl_statement {
          // GRANT ... TO ...
          GrantStatement,
          // REVOKE ... FROM ...
          RevokeStatement,
        }

        // Transaction control: BEGIN, COMMIT, ROLLBACK
        group is_tcl_statement {
          // BEGIN ... END
          BeginStatement,
          // COMMIT
          CommitStatement,
          // ROLLBACK
          RollbackStatement,
        }

        // Other statements
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
        // EXECUTE/EXEC proc
        ExecuteStatement,
        // CALL procedure(args)
        CallStatement,
        // COPY ... INTO ...
        CopyStatement,
        // LOAD DATA ...
        LoadStatement,
        // DESCRIBE/DESC table
        DescribeStatement,
        // PRAGMA ...
        PragmaStatement,
        // ANALYZE TABLE t
        AnalyzeStatement,
        // COMMENT ON TABLE t IS 'text'
        CommentStatement,
        // REFRESH MATERIALIZED VIEW
        RefreshStatement,
        // LOCK TABLE t
        LockStatement,
        // VACUUM
        VacuumStatement,
        // KILL query_id
        KillStatement,
        // CACHE TABLE t
        CacheStatement,
        // UNCACHE TABLE t
        UncacheStatement,
      }

      // Clauses: structural parts of statements
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
        // name AS (SELECT ...) inside WITH
        CommonTableExpressionClause,
        // FOR UPDATE / FOR SHARE / FOR NO KEY UPDATE
        ForUpdateClause,
        // CONNECT BY PRIOR parent_id = id (Oracle)
        ConnectByClause,
        // START WITH id = 1 (Oracle)
        StartWithClause,
        // ON CONFLICT (col) DO UPDATE SET ...
        OnConflictClause,
        // ON DUPLICATE KEY UPDATE col = val (MySQL)
        OnDuplicateKeyClause,
        // SELECT * EXCEPT(col) (BigQuery)
        SelectExceptClause,
        // SELECT * REPLACE(expr AS col) (BigQuery)
        SelectReplaceClause,
        // MATCH_RECOGNIZE (...)
        MatchRecognizeClause,
        // SETTINGS key = val (ClickHouse)
        SettingsClause,
        // ELSE expr in CASE
        ElseClause,
      }

      // Expressions: anything that produces a value
      group is_expression {
        // a + b, x AND y
        BinExpression,
        // -x, NOT y, PRIOR col
        UnaryExpression,
        // (expr)
        ParenExpression,
        // col BETWEEN 1 AND 10
        BetweenExpression,
        // col IN (1, 2, 3)
        InExpression,
        // col IS NULL, col IS DISTINCT FROM val
        IsExpression,
        // col LIKE 'pattern'
        LikeExpression,
        // col ILIKE 'pattern'
        IlikeExpression,
        // col RLIKE 'pattern'
        RlikeExpression,
        // col SIMILAR TO 'pattern'
        SimilarToExpression,
        // col GLOB 'pattern'
        GlobExpression,
        // (a, b) OVERLAPS (c, d)
        OverlapsExpression,
        // CASE WHEN ... THEN ... END
        CaseExpression,
        // CAST(x AS INT)
        CastExpression,
        // TRY_CAST(x AS INT)
        TryCastExpression,
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
        // :: type cast
        TypeCastExpression,
        // placeholder: ?, $1, :name
        PlaceholderExpression,
        // x -> x + 1
        LambdaExpression,
        // (1, 2, 3)
        TupleExpression,
        // ROW(1, 2)
        RowExpression,
        // ARRAY[1, 2, 3]
        ArrayExpression,
        // MAP { 'a': 1, 'b': 2 }
        MapExpression,
        // STRUCT(1 AS a, 2 AS b)
        StructExpression,
        // JSON path: $.key
        JsonPathExpression,

        // Expressions modified by a qualifier (keyword syntax)
        group is_qualified_expression {
          // expr AS name
          AliasExpression,
          // fn() FILTER (WHERE ...)
          FilterExpression,
          // fn() WITHIN GROUP (ORDER BY ...)
          WithinGroupExpression,
          // _utf8 'hello' (charset introducer)
          IntroducerExpression,
          // expr COLLATE utf8mb4
          CollateExpression,
          // expr AT TIME ZONE 'UTC'
          AtTimeZoneExpression,
          // fn() IGNORE NULLS
          IgnoreNullsExpression,
          // fn() RESPECT NULLS
          RespectNullsExpression,
          // expr FORMAT JSON
          FormatExpression,
          // IF(cond, true, false)
          IfExpression,
          // CONVERT(expr, type) or CONVERT(expr USING charset)
          ConvertExpression,
          // EXTRACT(YEAR FROM date)
          ExtractExpression,
          // SUBSTRING(str FROM 1 FOR 3)
          SubstringExpression,
          // TRIM(BOTH ' ' FROM str)
          TrimExpression,
          // OVERLAY(str PLACING repl FROM pos FOR len)
          OverlayExpression,
          // POSITION(substr IN str)
          PositionExpression,
          // col > ANY (SELECT ...), col = ALL (SELECT ...)
          SubqueryPredicateExpression,
          // MATCH(col) AGAINST('text' IN BOOLEAN MODE)
          MatchAgainstExpression,
          // GROUP_CONCAT(col SEPARATOR ',')
          GroupConcatExpression,
        }

        // Function invocations
        group is_function_call {
          // fn(args)
          FunctionCall,
          // COUNT(DISTINCT x)
          AggregateFunctionCall,
          // ROW_NUMBER() OVER (...)
          WindowFunctionCall,
        }

        // Literal values
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
          // TIME '12:30:00'
          TimeLiteral,
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
          // my_func
          FunctionReference,
          // my_type
          TypeReference,
        }
      }

      // Table expressions: produce a table, not a scalar value
      group is_table_expression {
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
        // CROSS APPLY / OUTER APPLY (SQL Server)
        ApplyExpression,
        // VALUES (1,2),(3,4) as table source
        ValuesExpression,
        // TABLE(fn()) table function
        TableFunctionExpression,
        // MATCH_RECOGNIZE (...) (Oracle)
        MatchRecognizeExpression,
        // table AS alias
        TableAliasExpression,
      }

      // Set operations: combine query results
      group is_set_operation {
        // SELECT ... UNION SELECT ...
        UnionExpression,
        // SELECT ... INTERSECT SELECT ...
        IntersectExpression,
        // SELECT ... EXCEPT SELECT ...
        ExceptExpression,
      }

      // DDL structural nodes
      group is_ddl_node {
        // col INT NOT NULL
        ColumnDefinition,
        // INT, VARCHAR(255)
        DataType,
        // INT(11), DECIMAL(10, 2) - type parameters
        DataTypeParameter,
        // INDEX idx_name (col1, col2)
        IndexDefinition,
        // PARTITION BY RANGE (col) (...)
        PartitionDefinition,

        // Column-level constraints
        group is_column_constraint {
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
          // NULL
          NullConstraint,
          // DEFAULT expr
          DefaultConstraint,
          // AUTO_INCREMENT / AUTOINCREMENT / SERIAL
          AutoIncrementConstraint,
          // GENERATED ALWAYS AS (expr) STORED/VIRTUAL
          GeneratedConstraint,
          // GENERATED ALWAYS AS IDENTITY
          IdentityConstraint,
          // COMMENT 'text'
          CommentConstraint,
          // COLLATE utf8
          CollateConstraint,
          // REFERENCES table(col) ON DELETE CASCADE
          ReferencesConstraint,
          // CONSTRAINT name ...
          NamedConstraint,
          // ON UPDATE CURRENT_TIMESTAMP
          OnUpdateConstraint,
          // CHARACTER SET utf8
          CharacterSetConstraint,
          // ENCODING AUTO
          EncodingConstraint,
          // INVISIBLE / VISIBLE
          VisibilityConstraint,
        }

        // Table-level properties
        group is_table_property {
          // ENGINE = InnoDB
          EngineProperty,
          // CHARACTER SET = utf8mb4
          CharacterSetProperty,
          // COLLATE = utf8mb4_unicode_ci
          CollateProperty,
          // AUTO_INCREMENT = 1000
          AutoIncrementProperty,
          // COMMENT = 'text'
          CommentProperty,
          // ROW_FORMAT = DYNAMIC
          RowFormatProperty,
          // TABLESPACE ts_name
          TablespaceProperty,
          // PARTITION BY ...
          PartitionByProperty,
          // CLUSTER BY (col)
          ClusterByProperty,
          // DISTRIBUTED BY (col)
          DistributedByProperty,
          // SORT BY (col)
          SortByProperty,
          // STORED AS PARQUET
          StoredAsProperty,
          // LOCATION '/path'
          LocationProperty,
          // TBLPROPERTIES ('key'='val')
          TablePropertiesProperty,
          // WITH (option = val)
          WithProperty,
        }
      }

      // Grouping modifiers for GROUP BY
      group is_grouping_modifier {
        // GROUPING SETS ((a), (b))
        GroupingSets,
        // CUBE(a, b)
        Cube,
        // ROLLUP(a, b)
        Rollup,
      }

      // Structural modifiers attached to expressions
      group is_qualifier {
        // OVER (PARTITION BY a ORDER BY b ...)
        WindowSpecification,
        // ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
        WindowFrame,
        // UNBOUNDED PRECEDING, CURRENT ROW, 5 FOLLOWING
        WindowFrameBound,
        // EXCLUDE CURRENT ROW / EXCLUDE GROUP / EXCLUDE TIES
        WindowExclude,
        // col ASC NULLS FIRST
        OrderByItem,
        // DISTINCT / ALL modifier on function args
        SetQuantifier,
      }

      // command string after SHOW/EXPLAIN
      CommandNode,

      // Error recovery
      group is_error {
        // error recovery node
        ErrorNode,
      }
    }
  }
}
