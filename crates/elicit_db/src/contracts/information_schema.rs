//! `INFORMATION_SCHEMA` introspection propositions.
//!
//! Source: ISO/IEC 9075-11:2023 — SQL/Schemata (Information and Definition Schemas).
//! All §references are to ISO/IEC 9075-11 unless stated otherwise.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
            }
        };
    }

    // -- SCHEMATA view (§SCHEMATA) --

    /// The queried schema exists in the `INFORMATION_SCHEMA.SCHEMATA` view.
    ///
    /// Source: ISO/IEC 9075-11 §SCHEMATA view
    pub struct SchemaExists;
    structural_prop!(SchemaExists, "SchemaExists");

    /// The schema has a default character set declared in `SCHEMATA`.
    ///
    /// Source: ISO/IEC 9075-11 §SCHEMATA view — `DEFAULT_CHARACTER_SET_NAME`
    pub struct SchemaDefaultCharacterSetDeclared;
    structural_prop!(
        SchemaDefaultCharacterSetDeclared,
        "SchemaDefaultCharacterSetDeclared"
    );

    /// The schema has a default collation declared in `SCHEMATA`.
    ///
    /// Source: ISO/IEC 9075-11 §SCHEMATA view — `DEFAULT_COLLATION_NAME`
    pub struct SchemaDefaultCollationDeclared;
    structural_prop!(
        SchemaDefaultCollationDeclared,
        "SchemaDefaultCollationDeclared"
    );

    /// The schema has a SQL path declaration recorded in `SCHEMATA`.
    ///
    /// Source: ISO/IEC 9075-11 §SCHEMATA view — `SQL_PATH`
    pub struct SchemaSqlPathDeclared;
    structural_prop!(SchemaSqlPathDeclared, "SchemaSqlPathDeclared");

    // -- TABLES view (§TABLES) --

    /// The queried table exists in `INFORMATION_SCHEMA.TABLES`.
    ///
    /// Source: ISO/IEC 9075-11 §TABLES view
    pub struct TableExists;
    structural_prop!(TableExists, "TableExists");

    /// A `TABLE_TYPE='BASE TABLE'` entry exists in `INFORMATION_SCHEMA.TABLES`.
    ///
    /// Source: ISO/IEC 9075-11 §TABLES view — `TABLE_TYPE`
    pub struct BaseTableExists;
    structural_prop!(BaseTableExists, "BaseTableExists");

    /// A `TABLE_TYPE='VIEW'` entry exists in `INFORMATION_SCHEMA.TABLES`.
    ///
    /// Source: ISO/IEC 9075-11 §TABLES view — `TABLE_TYPE`
    pub struct ViewTableExists;
    structural_prop!(ViewTableExists, "ViewTableExists");

    /// A `TABLE_TYPE='FOREIGN'` entry exists in `INFORMATION_SCHEMA.TABLES` (SQL/MED).
    ///
    /// Source: ISO/IEC 9075-11 §TABLES view — `TABLE_TYPE`
    pub struct ForeignTableExists;
    structural_prop!(ForeignTableExists, "ForeignTableExists");

    /// `IS_INSERTABLE_INTO='YES'` is recorded for the table in `INFORMATION_SCHEMA.TABLES`.
    ///
    /// Source: ISO/IEC 9075-11 §TABLES view — `IS_INSERTABLE_INTO`
    pub struct TableIsInsertableInto;
    structural_prop!(TableIsInsertableInto, "TableIsInsertableInto");

    /// An `ON COMMIT` action is declared for a temporary table in `INFORMATION_SCHEMA.TABLES`.
    ///
    /// Source: ISO/IEC 9075-11 §TABLES view — `COMMIT_ACTION`
    pub struct TableCommitActionDeclared;
    structural_prop!(TableCommitActionDeclared, "TableCommitActionDeclared");

    // -- COLUMNS view (§COLUMNS) --

    /// The queried column exists in `INFORMATION_SCHEMA.COLUMNS`.
    ///
    /// Source: ISO/IEC 9075-11 §COLUMNS view
    pub struct ColumnExists;
    structural_prop!(ColumnExists, "ColumnExists");

    /// `COLUMN.DATA_TYPE` is recorded correctly in `INFORMATION_SCHEMA.COLUMNS`.
    ///
    /// Source: ISO/IEC 9075-11 §COLUMNS view — `DATA_TYPE`
    pub struct ColumnDataTypeDeclared;
    structural_prop!(ColumnDataTypeDeclared, "ColumnDataTypeDeclared");

    /// `COLUMN.IS_NULLABLE='YES'` is recorded in `INFORMATION_SCHEMA.COLUMNS`.
    ///
    /// Source: ISO/IEC 9075-11 §COLUMNS view — `IS_NULLABLE`
    pub struct ColumnIsNullable;
    structural_prop!(ColumnIsNullable, "ColumnIsNullable");

    /// `COLUMN.IS_NULLABLE='NO'` is recorded in `INFORMATION_SCHEMA.COLUMNS`.
    ///
    /// Source: ISO/IEC 9075-11 §COLUMNS view — `IS_NULLABLE`
    pub struct ColumnIsNotNullable;
    structural_prop!(ColumnIsNotNullable, "ColumnIsNotNullable");

    /// `COLUMN.COLUMN_DEFAULT` is non-null in `INFORMATION_SCHEMA.COLUMNS`.
    ///
    /// Source: ISO/IEC 9075-11 §COLUMNS view — `COLUMN_DEFAULT`
    pub struct ColumnDefaultValueDeclared;
    structural_prop!(ColumnDefaultValueDeclared, "ColumnDefaultValueDeclared");

    /// `CHARACTER_MAXIMUM_LENGTH` is set for character columns in `INFORMATION_SCHEMA.COLUMNS`.
    ///
    /// Source: ISO/IEC 9075-11 §COLUMNS view — `CHARACTER_MAXIMUM_LENGTH`
    pub struct ColumnCharacterMaximumLengthSet;
    structural_prop!(
        ColumnCharacterMaximumLengthSet,
        "ColumnCharacterMaximumLengthSet"
    );

    /// `NUMERIC_PRECISION` is set for numeric columns in `INFORMATION_SCHEMA.COLUMNS`.
    ///
    /// Source: ISO/IEC 9075-11 §COLUMNS view — `NUMERIC_PRECISION`
    pub struct ColumnNumericPrecisionSet;
    structural_prop!(ColumnNumericPrecisionSet, "ColumnNumericPrecisionSet");

    /// `DATETIME_PRECISION` is set for datetime columns in `INFORMATION_SCHEMA.COLUMNS`.
    ///
    /// Source: ISO/IEC 9075-11 §COLUMNS view — `DATETIME_PRECISION`
    pub struct ColumnDatetimePrecisionSet;
    structural_prop!(ColumnDatetimePrecisionSet, "ColumnDatetimePrecisionSet");

    /// `ORDINAL_POSITION` matches the expected column order in `INFORMATION_SCHEMA.COLUMNS`.
    ///
    /// Source: ISO/IEC 9075-11 §COLUMNS view — `ORDINAL_POSITION`
    pub struct ColumnOrdinalPositionCorrect;
    structural_prop!(ColumnOrdinalPositionCorrect, "ColumnOrdinalPositionCorrect");

    /// A generation expression is recorded for the column in `INFORMATION_SCHEMA.COLUMNS`.
    ///
    /// Source: ISO/IEC 9075-11 §COLUMNS view — `GENERATION_EXPRESSION`
    pub struct ColumnGenerationExpressionDeclared;
    structural_prop!(
        ColumnGenerationExpressionDeclared,
        "ColumnGenerationExpressionDeclared"
    );

    // -- VIEWS view (§VIEWS) --

    /// The view exists in `INFORMATION_SCHEMA.VIEWS`.
    ///
    /// Source: ISO/IEC 9075-11 §VIEWS view
    pub struct ViewExists;
    structural_prop!(ViewExists, "ViewExists");

    /// `VIEW_DEFINITION` is accessible (not NULL) in `INFORMATION_SCHEMA.VIEWS`.
    ///
    /// Source: ISO/IEC 9075-11 §VIEWS view — `VIEW_DEFINITION`
    pub struct ViewDefinitionAccessible;
    structural_prop!(ViewDefinitionAccessible, "ViewDefinitionAccessible");

    /// `IS_UPDATABLE='YES'` is recorded for the view in `INFORMATION_SCHEMA.VIEWS`.
    ///
    /// Source: ISO/IEC 9075-11 §VIEWS view — `IS_UPDATABLE`
    pub struct ViewIsUpdatable;
    structural_prop!(ViewIsUpdatable, "ViewIsUpdatable");

    /// `CHECK_OPTION` is `'CASCADED'` or `'LOCAL'` in `INFORMATION_SCHEMA.VIEWS`.
    ///
    /// Source: ISO/IEC 9075-11 §VIEWS view — `CHECK_OPTION`
    pub struct ViewCheckOptionDeclared;
    structural_prop!(ViewCheckOptionDeclared, "ViewCheckOptionDeclared");

    // -- TABLE_CONSTRAINTS view (§TABLE_CONSTRAINTS) --

    /// A constraint entry exists in `INFORMATION_SCHEMA.TABLE_CONSTRAINTS`.
    ///
    /// Source: ISO/IEC 9075-11 §TABLE_CONSTRAINTS view
    pub struct TableConstraintExists;
    structural_prop!(TableConstraintExists, "TableConstraintExists");

    /// A `CONSTRAINT_TYPE='PRIMARY KEY'` entry exists in `INFORMATION_SCHEMA.TABLE_CONSTRAINTS`.
    ///
    /// Source: ISO/IEC 9075-11 §TABLE_CONSTRAINTS view — `CONSTRAINT_TYPE`
    pub struct PrimaryKeyConstraintRecorded;
    structural_prop!(PrimaryKeyConstraintRecorded, "PrimaryKeyConstraintRecorded");

    /// A `CONSTRAINT_TYPE='UNIQUE'` entry exists in `INFORMATION_SCHEMA.TABLE_CONSTRAINTS`.
    ///
    /// Source: ISO/IEC 9075-11 §TABLE_CONSTRAINTS view — `CONSTRAINT_TYPE`
    pub struct UniqueConstraintRecorded;
    structural_prop!(UniqueConstraintRecorded, "UniqueConstraintRecorded");

    /// A `CONSTRAINT_TYPE='FOREIGN KEY'` entry exists in `INFORMATION_SCHEMA.TABLE_CONSTRAINTS`.
    ///
    /// Source: ISO/IEC 9075-11 §TABLE_CONSTRAINTS view — `CONSTRAINT_TYPE`
    pub struct ForeignKeyConstraintRecorded;
    structural_prop!(ForeignKeyConstraintRecorded, "ForeignKeyConstraintRecorded");

    /// A `CONSTRAINT_TYPE='CHECK'` entry exists in `INFORMATION_SCHEMA.TABLE_CONSTRAINTS`.
    ///
    /// Source: ISO/IEC 9075-11 §TABLE_CONSTRAINTS view — `CONSTRAINT_TYPE`
    pub struct CheckConstraintRecorded;
    structural_prop!(CheckConstraintRecorded, "CheckConstraintRecorded");

    /// `IS_ENFORCED='YES'` is recorded for the constraint in `INFORMATION_SCHEMA.TABLE_CONSTRAINTS`.
    ///
    /// Source: ISO/IEC 9075-11 §TABLE_CONSTRAINTS view — `IS_ENFORCED`
    pub struct ConstraintEnforced;
    structural_prop!(ConstraintEnforced, "ConstraintEnforced");

    // -- REFERENTIAL_CONSTRAINTS view (§REFERENTIAL_CONSTRAINTS) --

    /// A foreign key entry exists in `INFORMATION_SCHEMA.REFERENTIAL_CONSTRAINTS`.
    ///
    /// Source: ISO/IEC 9075-11 §REFERENTIAL_CONSTRAINTS view
    pub struct ForeignKeyExists;
    structural_prop!(ForeignKeyExists, "ForeignKeyExists");

    /// `UPDATE_RULE` is set in `INFORMATION_SCHEMA.REFERENTIAL_CONSTRAINTS`.
    ///
    /// Source: ISO/IEC 9075-11 §REFERENTIAL_CONSTRAINTS view — `UPDATE_RULE`
    pub struct ReferentialConstraintUpdateRuleDeclared;
    structural_prop!(
        ReferentialConstraintUpdateRuleDeclared,
        "ReferentialConstraintUpdateRuleDeclared"
    );

    /// `DELETE_RULE` is set in `INFORMATION_SCHEMA.REFERENTIAL_CONSTRAINTS`.
    ///
    /// Source: ISO/IEC 9075-11 §REFERENTIAL_CONSTRAINTS view — `DELETE_RULE`
    pub struct ReferentialConstraintDeleteRuleDeclared;
    structural_prop!(
        ReferentialConstraintDeleteRuleDeclared,
        "ReferentialConstraintDeleteRuleDeclared"
    );

    /// `MATCH_OPTION` is set (`NONE`, `FULL`, or `PARTIAL`) in `INFORMATION_SCHEMA.REFERENTIAL_CONSTRAINTS`.
    ///
    /// Source: ISO/IEC 9075-11 §REFERENTIAL_CONSTRAINTS view — `MATCH_OPTION`
    pub struct ReferentialConstraintMatchOptionDeclared;
    structural_prop!(
        ReferentialConstraintMatchOptionDeclared,
        "ReferentialConstraintMatchOptionDeclared"
    );

    // -- KEY_COLUMN_USAGE view (§KEY_COLUMN_USAGE) --

    /// A key column entry exists in `INFORMATION_SCHEMA.KEY_COLUMN_USAGE`.
    ///
    /// Source: ISO/IEC 9075-11 §KEY_COLUMN_USAGE view
    pub struct KeyColumnUsageRecorded;
    structural_prop!(KeyColumnUsageRecorded, "KeyColumnUsageRecorded");

    /// `ORDINAL_POSITION` in `KEY_COLUMN_USAGE` matches the expected key column position.
    ///
    /// Source: ISO/IEC 9075-11 §KEY_COLUMN_USAGE view — `ORDINAL_POSITION`
    pub struct PrimaryKeyColumnOrdinalCorrect;
    structural_prop!(
        PrimaryKeyColumnOrdinalCorrect,
        "PrimaryKeyColumnOrdinalCorrect"
    );

    /// `POSITION_IN_UNIQUE_CONSTRAINT` correctly maps the FK column to the referenced PK column.
    ///
    /// Source: ISO/IEC 9075-11 §KEY_COLUMN_USAGE view — `POSITION_IN_UNIQUE_CONSTRAINT`
    pub struct ForeignKeyColumnPositionMapped;
    structural_prop!(
        ForeignKeyColumnPositionMapped,
        "ForeignKeyColumnPositionMapped"
    );

    // -- CHECK_CONSTRAINTS view (§CHECK_CONSTRAINTS) --

    /// `CHECK_CLAUSE` is recorded in `INFORMATION_SCHEMA.CHECK_CONSTRAINTS`.
    ///
    /// Source: ISO/IEC 9075-11 §CHECK_CONSTRAINTS view — `CHECK_CLAUSE`
    pub struct CheckConstraintClauseDeclared;
    structural_prop!(
        CheckConstraintClauseDeclared,
        "CheckConstraintClauseDeclared"
    );

    /// The check constraint represents a `NOT NULL` constraint.
    ///
    /// Source: ISO/IEC 9075-11 §CHECK_CONSTRAINTS view — `CHECK_CLAUSE`
    pub struct CheckConstraintNotNullClause;
    structural_prop!(CheckConstraintNotNullClause, "CheckConstraintNotNullClause");

    // -- Privileges views (§TABLE_PRIVILEGES, §COLUMN_PRIVILEGES, §USAGE_PRIVILEGES) --

    /// An entry exists in `INFORMATION_SCHEMA.TABLE_PRIVILEGES` for the grantee and table.
    ///
    /// Source: ISO/IEC 9075-11 §TABLE_PRIVILEGES view
    pub struct TablePrivilegeRecorded;
    structural_prop!(TablePrivilegeRecorded, "TablePrivilegeRecorded");

    /// An entry exists in `INFORMATION_SCHEMA.COLUMN_PRIVILEGES` for the grantee and column.
    ///
    /// Source: ISO/IEC 9075-11 §COLUMN_PRIVILEGES view
    pub struct ColumnPrivilegeRecorded;
    structural_prop!(ColumnPrivilegeRecorded, "ColumnPrivilegeRecorded");

    /// An entry exists in `INFORMATION_SCHEMA.USAGE_PRIVILEGES` for a domain, sequence, or type.
    ///
    /// Source: ISO/IEC 9075-11 §USAGE_PRIVILEGES view
    pub struct UsagePrivilegeRecorded;
    structural_prop!(UsagePrivilegeRecorded, "UsagePrivilegeRecorded");

    // -- ROUTINES view (§ROUTINES) --

    /// An entry exists in `INFORMATION_SCHEMA.ROUTINES`.
    ///
    /// Source: ISO/IEC 9075-11 §ROUTINES view
    pub struct RoutineExists;
    structural_prop!(RoutineExists, "RoutineExists");

    /// `ROUTINE_TYPE='FUNCTION'` is recorded in `INFORMATION_SCHEMA.ROUTINES`.
    ///
    /// Source: ISO/IEC 9075-11 §ROUTINES view — `ROUTINE_TYPE`
    pub struct RoutineIsFunction;
    structural_prop!(RoutineIsFunction, "RoutineIsFunction");

    /// `ROUTINE_TYPE='PROCEDURE'` is recorded in `INFORMATION_SCHEMA.ROUTINES`.
    ///
    /// Source: ISO/IEC 9075-11 §ROUTINES view — `ROUTINE_TYPE`
    pub struct RoutineIsProcedure;
    structural_prop!(RoutineIsProcedure, "RoutineIsProcedure");

    /// `DATA_TYPE` for the function return value is declared in `INFORMATION_SCHEMA.ROUTINES`.
    ///
    /// Source: ISO/IEC 9075-11 §ROUTINES view — `DATA_TYPE`
    pub struct RoutineDataTypeDeclared;
    structural_prop!(RoutineDataTypeDeclared, "RoutineDataTypeDeclared");

    /// `SQL_DATA_ACCESS` is declared for the routine in `INFORMATION_SCHEMA.ROUTINES`.
    ///
    /// Source: ISO/IEC 9075-11 §ROUTINES view — `SQL_DATA_ACCESS`
    pub struct RoutineSqlDataAccessDeclared;
    structural_prop!(RoutineSqlDataAccessDeclared, "RoutineSqlDataAccessDeclared");

    // -- TRIGGERS view (§TRIGGERS) --

    /// An entry exists in `INFORMATION_SCHEMA.TRIGGERS`.
    ///
    /// Source: ISO/IEC 9075-11 §TRIGGERS view
    pub struct TriggerExistsInSchema;
    structural_prop!(TriggerExistsInSchema, "TriggerExistsInSchema");

    /// `EVENT_MANIPULATION` matches the declared event (`INSERT`, `UPDATE`, or `DELETE`).
    ///
    /// Source: ISO/IEC 9075-11 §TRIGGERS view — `EVENT_MANIPULATION`
    pub struct TriggerEventManipulationCorrect;
    structural_prop!(
        TriggerEventManipulationCorrect,
        "TriggerEventManipulationCorrect"
    );

    /// `ACTION_TIMING` matches `BEFORE`, `AFTER`, or `INSTEAD OF`.
    ///
    /// Source: ISO/IEC 9075-11 §TRIGGERS view — `ACTION_TIMING`
    pub struct TriggerTimingCorrect;
    structural_prop!(TriggerTimingCorrect, "TriggerTimingCorrect");

    // -- DOMAINS view (§DOMAINS) --

    /// An entry exists in `INFORMATION_SCHEMA.DOMAINS`.
    ///
    /// Source: ISO/IEC 9075-11 §DOMAINS view
    pub struct DomainExists;
    structural_prop!(DomainExists, "DomainExists");

    /// `DATA_TYPE` is declared for the domain in `INFORMATION_SCHEMA.DOMAINS`.
    ///
    /// Source: ISO/IEC 9075-11 §DOMAINS view — `DATA_TYPE`
    pub struct DomainDataTypeDeclared;
    structural_prop!(DomainDataTypeDeclared, "DomainDataTypeDeclared");

    /// A domain constraint entry exists in `INFORMATION_SCHEMA.DOMAIN_CONSTRAINTS`.
    ///
    /// Source: ISO/IEC 9075-11 §DOMAIN_CONSTRAINTS view
    pub struct DomainConstraintRecorded;
    structural_prop!(DomainConstraintRecorded, "DomainConstraintRecorded");

    // -- CHARACTER_SETS view (§CHARACTER_SETS) --

    /// An entry exists in `INFORMATION_SCHEMA.CHARACTER_SETS`.
    ///
    /// Source: ISO/IEC 9075-11 §CHARACTER_SETS view
    pub struct CharacterSetExists;
    structural_prop!(CharacterSetExists, "CharacterSetExists");

    // -- COLLATIONS view (§COLLATIONS) --

    /// An entry exists in `INFORMATION_SCHEMA.COLLATIONS`.
    ///
    /// Source: ISO/IEC 9075-11 §COLLATIONS view
    pub struct CollationExists;
    structural_prop!(CollationExists, "CollationExists");

    // -- SEQUENCES view (§SEQUENCES) --

    /// An entry exists in `INFORMATION_SCHEMA.SEQUENCES`.
    ///
    /// Source: ISO/IEC 9075-11 §SEQUENCES view
    pub struct SequenceExists;
    structural_prop!(SequenceExists, "SequenceExists");
}

pub use emit_impls::{
    BaseTableExists, CharacterSetExists, CheckConstraintClauseDeclared,
    CheckConstraintNotNullClause, CheckConstraintRecorded, CollationExists,
    ColumnCharacterMaximumLengthSet, ColumnDataTypeDeclared, ColumnDatetimePrecisionSet,
    ColumnDefaultValueDeclared, ColumnExists, ColumnGenerationExpressionDeclared,
    ColumnIsNotNullable, ColumnIsNullable, ColumnNumericPrecisionSet, ColumnOrdinalPositionCorrect,
    ColumnPrivilegeRecorded, ConstraintEnforced, DomainConstraintRecorded, DomainDataTypeDeclared,
    DomainExists, ForeignKeyColumnPositionMapped, ForeignKeyConstraintRecorded, ForeignKeyExists,
    ForeignTableExists, KeyColumnUsageRecorded, PrimaryKeyColumnOrdinalCorrect,
    PrimaryKeyConstraintRecorded, ReferentialConstraintDeleteRuleDeclared,
    ReferentialConstraintMatchOptionDeclared, ReferentialConstraintUpdateRuleDeclared,
    RoutineDataTypeDeclared, RoutineExists, RoutineIsFunction, RoutineIsProcedure,
    RoutineSqlDataAccessDeclared, SchemaDefaultCharacterSetDeclared,
    SchemaDefaultCollationDeclared, SchemaExists, SchemaSqlPathDeclared, SequenceExists,
    TableCommitActionDeclared, TableConstraintExists, TableExists, TableIsInsertableInto,
    TablePrivilegeRecorded, TriggerEventManipulationCorrect, TriggerExistsInSchema,
    TriggerTimingCorrect, UniqueConstraintRecorded, UsagePrivilegeRecorded,
    ViewCheckOptionDeclared, ViewDefinitionAccessible, ViewExists, ViewIsUpdatable,
    ViewTableExists,
};
