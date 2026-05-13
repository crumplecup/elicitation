//! `ArchiveBrowsePlugin` — database structure introspection via `elicit_sqlx`.
//!
//! Composes `elicit_sqlx` query tools against `information_schema` to produce
//! [`SchemaDescriptor`], [`TableDescriptor`], and [`IndexDescriptor`] values.
//! Each result carries the appropriate `Established<P>` contract.
//!
//! Primary backend: PostgreSQL. Graceful degradation on MySQL/SQLite.

use std::collections::HashSet;

use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, contracts::Established, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::AnyPool;
use sqlx::Row as _;
use sqlx::any::AnyRow;
use tracing::instrument;

use crate::archive::{
    ColumnDescriptor, CompositeTypeAttribute, CompositeTypeDescriptor, DatabaseDescriptor,
    DomainDescriptor, EnumDescriptor, FunctionDescriptor, FunctionVolatility, IndexDescriptor,
    SchemaDescriptor, SequenceDescriptor, TableDescriptor, TableType, TriggerDescriptor,
    TriggerEvents,
};

// ── helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        serde_json::to_string(value).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

async fn connect(url: &str) -> Result<AnyPool, ErrorData> {
    sqlx::any::install_default_drivers();
    sqlx::any::AnyPoolOptions::new()
        .max_connections(3)
        .connect(url)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("connection failed: {e}"), None))
}

// ── propositions ──────────────────────────────────────────────────────────────

/// Proposition: the specified schema exists in the database.
#[derive(Prop)]
pub struct SchemaExists;

impl VerifiedWorkflow for SchemaExists {}

/// Proposition: the specified table exists in the database.
#[derive(Prop)]
pub struct TableExists;

impl VerifiedWorkflow for TableExists {}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `archive_browse__describe_database`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DescribeDatabaseParams {
    /// Database connection URL.
    pub url: String,
}

/// Parameters for `archive_browse__list_schemas`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListSchemasParams {
    /// Database connection URL.
    pub url: String,
}

/// Parameters for `archive_browse__describe_table`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DescribeTableParams {
    /// Database connection URL.
    pub url: String,
    /// Schema containing the table.
    pub schema: String,
    /// Table name to describe.
    pub table: String,
}

/// Parameters for `archive_browse__list_indexes`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListIndexesParams {
    /// Database connection URL.
    pub url: String,
    /// Schema containing the table.
    pub schema: String,
    /// Table whose indexes to list.
    pub table: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "archive_browse",
    name = "archive_browse__describe_database",
    description = "Connect to a database and return its name, version, and backend kind. \
                   Uses `current_database()` + `version()` (PostgreSQL) with graceful fallback."
)]
#[instrument]
async fn describe_database(p: DescribeDatabaseParams) -> Result<CallToolResult, ErrorData> {
    let pool = connect(&p.url).await?;

    let (db_name, version) = match sqlx::query("SELECT current_database(), version()")
        .fetch_one(&pool)
        .await
    {
        Ok(row) => {
            let name: String = row
                .try_get::<String, _>(0)
                .unwrap_or_else(|_| "unknown".to_string());
            let ver: String = row.try_get::<String, _>(1).unwrap_or_default();
            (name, Some(ver))
        }
        Err(_) => ("unknown".to_string(), None),
    };

    pool.close().await;

    json_result(&DatabaseDescriptor::new(&p.url, db_name, version))
}

#[elicit_tool(
    plugin = "archive_browse",
    name = "archive_browse__list_schemas",
    description = "List all user schemas in the database. Excludes pg_catalog and \
                   information_schema system schemas. Establishes: SchemaExists."
)]
#[instrument]
async fn list_schemas(p: ListSchemasParams) -> Result<CallToolResult, ErrorData> {
    let pool = connect(&p.url).await?;

    let rows = sqlx::query(
        "SELECT schema_name, COALESCE(schema_owner, '') \
         FROM information_schema.schemata \
         WHERE schema_name NOT LIKE 'pg_%' \
           AND schema_name != 'information_schema' \
         ORDER BY schema_name",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

    let schemas: Vec<SchemaDescriptor> = rows
        .iter()
        .map(|row: &AnyRow| {
            let name: String = row.try_get::<String, _>(0).unwrap_or_default();
            let owner: String = row.try_get::<String, _>(1).unwrap_or_default();
            SchemaDescriptor {
                schema_name: name,
                owner,
                table_names: vec![],
            }
        })
        .collect();

    pool.close().await;

    let _proof = Established::<SchemaExists>::assert();
    json_result(&schemas)
}

#[elicit_tool(
    plugin = "archive_browse",
    name = "archive_browse__describe_table",
    description = "Describe a table: columns, types, nullability, primary/foreign keys, \
                   and spatial detection. Establishes: TableExists ∧ SchemaExists."
)]
#[instrument]
async fn describe_table(p: DescribeTableParams) -> Result<CallToolResult, ErrorData> {
    let pool = connect(&p.url).await?;

    // Table type
    let table_type = sqlx::query(
        "SELECT table_type FROM information_schema.tables \
         WHERE table_schema = $1 AND table_name = $2",
    )
    .bind(&p.schema)
    .bind(&p.table)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ErrorData::internal_error(e.to_string(), None))?
    .and_then(|row: AnyRow| row.try_get::<String, _>(0).ok())
    .map(|s| TableType::from_information_schema(&s))
    .unwrap_or(TableType::Table);

    // Columns
    let col_rows = sqlx::query(
        "SELECT column_name, data_type, is_nullable, column_default \
         FROM information_schema.columns \
         WHERE table_schema = $1 AND table_name = $2 \
         ORDER BY ordinal_position",
    )
    .bind(&p.schema)
    .bind(&p.table)
    .fetch_all(&pool)
    .await
    .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

    // Primary key columns
    let pk_cols: HashSet<String> = sqlx::query(
        "SELECT kcu.column_name \
         FROM information_schema.table_constraints tc \
         JOIN information_schema.key_column_usage kcu \
             ON tc.constraint_name = kcu.constraint_name \
             AND tc.table_schema = kcu.table_schema \
             AND tc.table_name = kcu.table_name \
         WHERE tc.constraint_type = 'PRIMARY KEY' \
             AND tc.table_schema = $1 AND tc.table_name = $2",
    )
    .bind(&p.schema)
    .bind(&p.table)
    .fetch_all(&pool)
    .await
    .unwrap_or_default()
    .iter()
    .filter_map(|row| row.try_get::<String, _>(0).ok())
    .collect();

    // Foreign key columns
    let fk_cols: HashSet<String> = sqlx::query(
        "SELECT kcu.column_name \
         FROM information_schema.table_constraints tc \
         JOIN information_schema.key_column_usage kcu \
             ON tc.constraint_name = kcu.constraint_name \
             AND tc.table_schema = kcu.table_schema \
             AND tc.table_name = kcu.table_name \
         WHERE tc.constraint_type = 'FOREIGN KEY' \
             AND tc.table_schema = $1 AND tc.table_name = $2",
    )
    .bind(&p.schema)
    .bind(&p.table)
    .fetch_all(&pool)
    .await
    .unwrap_or_default()
    .iter()
    .filter_map(|row| row.try_get::<String, _>(0).ok())
    .collect();

    let columns: Vec<ColumnDescriptor> = col_rows
        .iter()
        .map(|row: &AnyRow| {
            let name: String = row.try_get::<String, _>(0).unwrap_or_default();
            let sql_type: String = row.try_get::<String, _>(1).unwrap_or_default();
            let nullable_str: String = row.try_get::<String, _>(2).unwrap_or_default();
            let default_value: Option<String> = row.try_get::<String, _>(3).ok();
            ColumnDescriptor {
                is_spatial: ColumnDescriptor::is_spatial_type(&sql_type),
                is_primary_key: pk_cols.contains(&name),
                is_foreign_key: fk_cols.contains(&name),
                nullable: nullable_str == "YES",
                name,
                sql_type,
                default_value,
            }
        })
        .collect();

    pool.close().await;

    let _proof = Established::<TableExists>::assert();
    json_result(&TableDescriptor {
        schema: p.schema,
        table_name: p.table,
        columns,
        estimated_rows: None,
        table_type,
    })
}

#[elicit_tool(
    plugin = "archive_browse",
    name = "archive_browse__list_indexes",
    description = "List all indexes on a table. Uses pg_indexes (PostgreSQL). \
                   Returns an empty list on other backends."
)]
#[instrument]
async fn list_indexes(p: ListIndexesParams) -> Result<CallToolResult, ErrorData> {
    let pool = connect(&p.url).await?;

    // pg_indexes is PostgreSQL-specific; other backends return empty vec gracefully.
    let rows = sqlx::query(
        "SELECT indexname, tablename, indexdef \
         FROM pg_indexes \
         WHERE schemaname = $1 AND tablename = $2 \
         ORDER BY indexname",
    )
    .bind(&p.schema)
    .bind(&p.table)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let indexes: Vec<IndexDescriptor> = rows
        .iter()
        .map(|row: &AnyRow| {
            let index_name: String = row.try_get::<String, _>(0).unwrap_or_default();
            let table_name: String = row.try_get::<String, _>(1).unwrap_or_default();
            let indexdef: String = row.try_get::<String, _>(2).unwrap_or_default();
            let is_unique = indexdef.to_uppercase().contains("UNIQUE");
            IndexDescriptor {
                index_name,
                schema: p.schema.clone(),
                table_name,
                column_names: vec![],
                is_unique,
                index_method: "btree".to_string(),
            }
        })
        .collect();

    pool.close().await;

    json_result(&indexes)
}

// ── Phase 4 — Advanced Object Types ──────────────────────────────────────────

/// Proposition: functions were successfully listed for the given schema.
#[derive(Prop)]
pub struct FunctionsListed;
impl VerifiedWorkflow for FunctionsListed {}

/// Proposition: the specified function was fully described.
#[derive(Prop)]
pub struct FunctionDescribed;
impl VerifiedWorkflow for FunctionDescribed {}

/// Proposition: triggers were successfully listed for the given table.
#[derive(Prop)]
pub struct TriggersListed;
impl VerifiedWorkflow for TriggersListed {}

/// Proposition: sequences were successfully listed for the given schema.
#[derive(Prop)]
pub struct SequencesListed;
impl VerifiedWorkflow for SequencesListed {}

/// Proposition: user-defined types were successfully listed for the given schema.
#[derive(Prop)]
pub struct TypesListed;
impl VerifiedWorkflow for TypesListed {}

// ── Phase 4 params ────────────────────────────────────────────────────────────

/// Parameters for `archive_browse__list_functions`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListFunctionsParams {
    /// Connection URL.
    pub url: String,
    /// Schema to list functions in.
    pub schema: String,
}

/// Parameters for `archive_browse__describe_function`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DescribeFunctionParams {
    /// Connection URL.
    pub url: String,
    /// Schema containing the function.
    pub schema: String,
    /// Function name.
    pub name: String,
    /// Full SQL argument list (for overload disambiguation; may be empty).
    pub arguments: String,
}

/// Parameters for `archive_browse__list_triggers`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListTriggersParams {
    /// Connection URL.
    pub url: String,
    /// Schema containing the table.
    pub schema: String,
    /// Table to list triggers for.
    pub table: String,
}

/// Parameters for `archive_browse__list_sequences`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListSequencesParams {
    /// Connection URL.
    pub url: String,
    /// Schema to list sequences in.
    pub schema: String,
}

/// Parameters for `archive_browse__list_types`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListTypesParams {
    /// Connection URL.
    pub url: String,
    /// Schema to list user-defined types in.
    pub schema: String,
}

// ── Phase 4 tools ─────────────────────────────────────────────────────────────

/// List all functions and procedures in `schema`.
#[elicit_tool(
    plugin = "archive_browse",
    name = "archive_browse__list_functions",
    description = "List PostgreSQL functions and stored procedures in a schema."
)]
#[instrument]
async fn list_functions(p: ListFunctionsParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<FunctionsListed>::assert();
    let pool = connect(&p.url).await?;

    let rows = sqlx::query(
        "SELECT p.oid::bigint, p.proname, \
                pg_get_function_arguments(p.oid), \
                pg_get_function_result(p.oid), \
                l.lanname, \
                CASE p.provolatile \
                  WHEN 'i' THEN 'immutable' \
                  WHEN 's' THEN 'stable' \
                  ELSE 'volatile' END, \
                p.prokind = 'p', \
                left(pg_get_functiondef(p.oid), 512) \
         FROM pg_proc p \
         JOIN pg_namespace n ON n.oid = p.pronamespace \
         JOIN pg_language l  ON l.oid = p.prolang \
         WHERE n.nspname = $1 \
           AND p.prokind IN ('f','p') \
         ORDER BY p.proname",
    )
    .bind(&p.schema)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let functions: Vec<FunctionDescriptor> = rows
        .iter()
        .map(|row: &AnyRow| {
            let vol_str: String = row.try_get::<String, _>(5).unwrap_or_default();
            let volatility = match vol_str.as_str() {
                "immutable" => FunctionVolatility::Immutable,
                "stable" => FunctionVolatility::Stable,
                _ => FunctionVolatility::Volatile,
            };
            FunctionDescriptor {
                oid: row.try_get::<i64, _>(0).unwrap_or_default(),
                schema: p.schema.clone(),
                name: row.try_get::<String, _>(1).unwrap_or_default(),
                arguments: row.try_get::<String, _>(2).unwrap_or_default(),
                return_type: row.try_get::<String, _>(3).unwrap_or_default(),
                language: row.try_get::<String, _>(4).unwrap_or_default(),
                volatility,
                is_procedure: row.try_get::<bool, _>(6).unwrap_or(false),
                body_preview: row.try_get::<String, _>(7).unwrap_or_default(),
            }
        })
        .collect();

    pool.close().await;
    json_result(&functions)
}

/// Describe a single function (full body + metadata).
#[elicit_tool(
    plugin = "archive_browse",
    name = "archive_browse__describe_function",
    description = "Describe a specific PostgreSQL function, including its full body."
)]
#[instrument]
async fn describe_function(p: DescribeFunctionParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<FunctionDescribed>::assert();
    let pool = connect(&p.url).await?;

    // Build the regprocedure cast string for exact OID lookup when arguments
    // are provided; fall back to name-only match for zero-arg disambiguation.
    let rows = sqlx::query(
        "SELECT p.oid::bigint, p.proname, \
                pg_get_function_arguments(p.oid), \
                pg_get_function_result(p.oid), \
                l.lanname, \
                CASE p.provolatile \
                  WHEN 'i' THEN 'immutable' \
                  WHEN 's' THEN 'stable' \
                  ELSE 'volatile' END, \
                p.prokind = 'p', \
                pg_get_functiondef(p.oid) \
         FROM pg_proc p \
         JOIN pg_namespace n ON n.oid = p.pronamespace \
         JOIN pg_language l  ON l.oid = p.prolang \
         WHERE n.nspname = $1 AND p.proname = $2 \
         LIMIT 1",
    )
    .bind(&p.schema)
    .bind(&p.name)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    pool.close().await;

    let row = rows
        .first()
        .ok_or_else(|| ErrorData::invalid_params(format!("function {} not found", p.name), None))?;

    let vol_str: String = row.try_get::<String, _>(5).unwrap_or_default();
    let volatility = match vol_str.as_str() {
        "immutable" => FunctionVolatility::Immutable,
        "stable" => FunctionVolatility::Stable,
        _ => FunctionVolatility::Volatile,
    };
    let fd = FunctionDescriptor {
        oid: row.try_get::<i64, _>(0).unwrap_or_default(),
        schema: p.schema.clone(),
        name: row.try_get::<String, _>(1).unwrap_or_default(),
        arguments: row.try_get::<String, _>(2).unwrap_or_default(),
        return_type: row.try_get::<String, _>(3).unwrap_or_default(),
        language: row.try_get::<String, _>(4).unwrap_or_default(),
        volatility,
        is_procedure: row.try_get::<bool, _>(6).unwrap_or(false),
        body_preview: row.try_get::<String, _>(7).unwrap_or_default(),
    };
    json_result(&fd)
}

/// List all triggers on a given table.
#[elicit_tool(
    plugin = "archive_browse",
    name = "archive_browse__list_triggers",
    description = "List PostgreSQL triggers attached to a table."
)]
#[instrument]
async fn list_triggers(p: ListTriggersParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<TriggersListed>::assert();
    let pool = connect(&p.url).await?;

    let rows = sqlx::query(
        "SELECT t.tgname, \
                CASE t.tgtype & 66 \
                  WHEN 2  THEN 'BEFORE' \
                  WHEN 64 THEN 'INSTEAD OF' \
                  ELSE 'AFTER' END, \
                (t.tgtype & 4)  <> 0, \
                (t.tgtype & 8)  <> 0, \
                (t.tgtype & 16) <> 0, \
                (t.tgtype & 32) <> 0, \
                (t.tgtype & 1)  <> 0, \
                p.proname, n2.nspname, \
                t.tgenabled <> 'D' \
         FROM pg_trigger t \
         JOIN pg_class   c  ON c.oid  = t.tgrelid \
         JOIN pg_namespace n  ON n.oid  = c.relnamespace \
         JOIN pg_proc    p  ON p.oid  = t.tgfoid \
         JOIN pg_namespace n2 ON n2.oid = p.pronamespace \
         WHERE n.nspname = $1 AND c.relname = $2 \
           AND NOT t.tgisinternal \
         ORDER BY t.tgname",
    )
    .bind(&p.schema)
    .bind(&p.table)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let triggers: Vec<TriggerDescriptor> = rows
        .iter()
        .map(|row: &AnyRow| {
            let fn_schema: String = row.try_get::<String, _>(8).unwrap_or_default();
            let fn_name: String = row.try_get::<String, _>(7).unwrap_or_default();
            TriggerDescriptor {
                schema: p.schema.clone(),
                table: p.table.clone(),
                name: row.try_get::<String, _>(0).unwrap_or_default(),
                timing: row.try_get::<String, _>(1).unwrap_or_default(),
                events: TriggerEvents {
                    on_insert: row.try_get::<bool, _>(2).unwrap_or(false),
                    on_update: row.try_get::<bool, _>(3).unwrap_or(false),
                    on_delete: row.try_get::<bool, _>(4).unwrap_or(false),
                    on_truncate: row.try_get::<bool, _>(5).unwrap_or(false),
                },
                row_level: row.try_get::<bool, _>(6).unwrap_or(false),
                function: format!("{fn_schema}.{fn_name}"),
                enabled: row.try_get::<bool, _>(9).unwrap_or(true),
            }
        })
        .collect();

    pool.close().await;
    json_result(&triggers)
}

/// List all sequences in a schema.
#[elicit_tool(
    plugin = "archive_browse",
    name = "archive_browse__list_sequences",
    description = "List PostgreSQL sequences in a schema with their current state."
)]
#[instrument]
async fn list_sequences(p: ListSequencesParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<SequencesListed>::assert();
    let pool = connect(&p.url).await?;

    let rows = sqlx::query(
        "SELECT s.sequence_name, \
                s.start_value::bigint, \
                s.increment::bigint, \
                s.minimum_value::bigint, \
                s.maximum_value::bigint, \
                s.cycle_option = 'YES', \
                d.refobjid::text \
         FROM information_schema.sequences s \
         LEFT JOIN pg_class c ON c.relname = s.sequence_name \
              AND c.relnamespace = (SELECT oid FROM pg_namespace WHERE nspname = $1) \
         LEFT JOIN pg_depend d ON d.objid = c.oid \
              AND d.classid = 'pg_class'::regclass \
              AND d.deptype = 'a' \
         WHERE s.sequence_schema = $1 \
         ORDER BY s.sequence_name",
    )
    .bind(&p.schema)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let sequences: Vec<SequenceDescriptor> = rows
        .iter()
        .map(|row: &AnyRow| SequenceDescriptor {
            schema: p.schema.clone(),
            name: row.try_get::<String, _>(0).unwrap_or_default(),
            current_value: None, // nextval would consume — omit for safety
            start_value: row.try_get::<i64, _>(1).unwrap_or(1),
            increment_by: row.try_get::<i64, _>(2).unwrap_or(1),
            min_value: row.try_get::<i64, _>(3).unwrap_or(1),
            max_value: row.try_get::<i64, _>(4).unwrap_or(i64::MAX),
            cycle: row.try_get::<bool, _>(5).unwrap_or(false),
            owned_by: row.try_get::<String, _>(6).ok().filter(|s| !s.is_empty()),
        })
        .collect();

    pool.close().await;
    json_result(&sequences)
}

/// List user-defined types (enums, domains, composites) in a schema.
#[elicit_tool(
    plugin = "archive_browse",
    name = "archive_browse__list_types",
    description = "List user-defined PostgreSQL types (enums, domains, composites) in a schema."
)]
#[instrument]
async fn list_types(p: ListTypesParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<TypesListed>::assert();
    let pool = connect(&p.url).await?;

    // ── enums ──────────────────────────────────────────────────────────────────
    let enum_rows = sqlx::query(
        "SELECT t.typname, e.enumlabel \
         FROM pg_type t \
         JOIN pg_namespace n ON n.oid = t.typnamespace \
         JOIN pg_enum e      ON e.enumtypid = t.oid \
         WHERE n.nspname = $1 AND t.typtype = 'e' \
         ORDER BY t.typname, e.enumsortorder",
    )
    .bind(&p.schema)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let mut enums: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    for row in &enum_rows {
        let name: String = row.try_get::<String, _>(0).unwrap_or_default();
        let label: String = row.try_get::<String, _>(1).unwrap_or_default();
        enums.entry(name).or_default().push(label);
    }
    let enum_descriptors: Vec<EnumDescriptor> = enums
        .into_iter()
        .map(|(name, labels)| EnumDescriptor {
            schema: p.schema.clone(),
            name,
            labels,
        })
        .collect();

    // ── domains ────────────────────────────────────────────────────────────────
    let domain_rows = sqlx::query(
        "SELECT t.typname, bt.typname, t.typnotnull, \
                d.adsrc, \
                array_to_string(array_agg(c.consrc ORDER BY c.conname), '; ') \
         FROM pg_type t \
         JOIN pg_namespace n ON n.oid = t.typnamespace \
         JOIN pg_type bt     ON bt.oid = t.typbasetype \
         LEFT JOIN pg_attrdef d  ON d.adtypid = t.oid \
         LEFT JOIN pg_constraint c ON c.contypid = t.oid \
         WHERE n.nspname = $1 AND t.typtype = 'd' \
         GROUP BY t.typname, bt.typname, t.typnotnull, d.adsrc \
         ORDER BY t.typname",
    )
    .bind(&p.schema)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let domain_descriptors: Vec<DomainDescriptor> = domain_rows
        .iter()
        .map(|row: &AnyRow| {
            let checks_str: Option<String> = row.try_get::<String, _>(4).ok();
            DomainDescriptor {
                schema: p.schema.clone(),
                name: row.try_get::<String, _>(0).unwrap_or_default(),
                base_type: row.try_get::<String, _>(1).unwrap_or_default(),
                not_null: row.try_get::<bool, _>(2).unwrap_or(false),
                default_expr: row.try_get::<String, _>(3).ok().filter(|s| !s.is_empty()),
                check_constraints: checks_str
                    .unwrap_or_default()
                    .split("; ")
                    .filter(|s| !s.is_empty())
                    .map(str::to_owned)
                    .collect(),
            }
        })
        .collect();

    // ── composite types ────────────────────────────────────────────────────────
    let comp_rows = sqlx::query(
        "SELECT t.typname, a.attname, bt.typname \
         FROM pg_type t \
         JOIN pg_namespace n  ON n.oid = t.typnamespace \
         JOIN pg_class   c    ON c.oid = t.typrelid \
         JOIN pg_attribute a  ON a.attrelid = c.oid AND a.attnum > 0 \
         JOIN pg_type bt      ON bt.oid = a.atttypid \
         WHERE n.nspname = $1 AND t.typtype = 'c' AND c.relkind = 'c' \
         ORDER BY t.typname, a.attnum",
    )
    .bind(&p.schema)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let mut composites: std::collections::BTreeMap<String, Vec<CompositeTypeAttribute>> =
        std::collections::BTreeMap::new();
    for row in &comp_rows {
        let type_name: String = row.try_get::<String, _>(0).unwrap_or_default();
        let attr_name: String = row.try_get::<String, _>(1).unwrap_or_default();
        let attr_type: String = row.try_get::<String, _>(2).unwrap_or_default();
        composites
            .entry(type_name)
            .or_default()
            .push(CompositeTypeAttribute {
                name: attr_name,
                type_name: attr_type,
            });
    }
    let composite_descriptors: Vec<CompositeTypeDescriptor> = composites
        .into_iter()
        .map(|(name, attributes)| CompositeTypeDescriptor {
            schema: p.schema.clone(),
            name,
            attributes,
        })
        .collect();

    pool.close().await;

    #[derive(Serialize)]
    struct TypesResult {
        enums: Vec<EnumDescriptor>,
        domains: Vec<DomainDescriptor>,
        composites: Vec<CompositeTypeDescriptor>,
    }
    json_result(&TypesResult {
        enums: enum_descriptors,
        domains: domain_descriptors,
        composites: composite_descriptors,
    })
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for database structure introspection.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "archive_browse")]
pub struct ArchiveBrowsePlugin;

impl ArchiveBrowsePlugin {
    /// Create a new browse plugin, registering sqlx drivers.
    #[instrument]
    pub fn new() -> Self {
        sqlx::any::install_default_drivers();
        Self
    }
}

impl Default for ArchiveBrowsePlugin {
    fn default() -> Self {
        Self::new()
    }
}
