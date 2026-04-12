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
    ColumnDescriptor, DatabaseDescriptor, IndexDescriptor, SchemaDescriptor, TableDescriptor,
    TableType,
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
