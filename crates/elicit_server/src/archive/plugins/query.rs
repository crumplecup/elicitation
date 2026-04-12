//! `ArchiveQueryPlugin` — SQL execution and DataFrame framing.
//!
//! Executes arbitrary SQL and returns typed [`QueryResult`] values containing
//! column metadata and row data using `elicit_db` value types.

use elicit_db::{DbRow, DbRows, DbValue};
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, contracts::Established, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::AnyPool;
use sqlx::any::AnyRow;
use sqlx::{Column as _, Row as _, TypeInfo as _};
use tracing::instrument;

use crate::archive::{ColumnDescriptor, QueryResult};

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

/// Decode a single column value from an `AnyRow`.
fn decode_value(row: &AnyRow, i: usize, type_name: &str) -> DbValue {
    match type_name {
        "BOOL" | "BOOLEAN" => row
            .try_get::<bool, _>(i)
            .map(DbValue::Bool)
            .unwrap_or(DbValue::Null),
        "INT2" | "SMALLINT" | "SMALLSERIAL" | "TINYINT" => row
            .try_get::<i16, _>(i)
            .map(|v| DbValue::Int(i64::from(v)))
            .unwrap_or(DbValue::Null),
        "INT" | "INT4" | "INTEGER" | "SERIAL" | "MEDIUMINT" => row
            .try_get::<i32, _>(i)
            .map(|v| DbValue::Int(i64::from(v)))
            .unwrap_or(DbValue::Null),
        "INT8" | "BIGINT" | "BIGSERIAL" => row
            .try_get::<i64, _>(i)
            .map(DbValue::Int)
            .unwrap_or(DbValue::Null),
        "FLOAT" | "FLOAT4" | "REAL" => row
            .try_get::<f32, _>(i)
            .map(|v| DbValue::Float(f64::from(v)))
            .unwrap_or(DbValue::Null),
        "FLOAT8" | "DOUBLE" | "DOUBLE PRECISION" => row
            .try_get::<f64, _>(i)
            .map(DbValue::Float)
            .unwrap_or(DbValue::Null),
        "BLOB" | "BYTEA" => row
            .try_get::<Vec<u8>, _>(i)
            .map(DbValue::Bytes)
            .unwrap_or(DbValue::Null),
        _ => row
            .try_get::<String, _>(i)
            .map(DbValue::Text)
            .unwrap_or(DbValue::Null),
    }
}

/// Decode a `Vec<AnyRow>` into column descriptors + `DbRows`.
fn decode_rows(rows: &[AnyRow]) -> (Vec<ColumnDescriptor>, DbRows) {
    if rows.is_empty() {
        return (
            vec![],
            DbRows {
                rows: vec![],
                affected: 0,
            },
        );
    }

    let col_descs: Vec<ColumnDescriptor> = rows[0]
        .columns()
        .iter()
        .map(|col| {
            let sql_type = col.type_info().name().to_string();
            ColumnDescriptor {
                is_spatial: ColumnDescriptor::is_spatial_type(&sql_type),
                name: col.name().to_string(),
                sql_type,
                nullable: true,
                is_primary_key: false,
                is_foreign_key: false,
                default_value: None,
            }
        })
        .collect();

    let db_rows: Vec<DbRow> = rows
        .iter()
        .map(|row| {
            let cols: Vec<(String, DbValue)> = row
                .columns()
                .iter()
                .enumerate()
                .map(|(i, col)| {
                    let name = col.name().to_string();
                    let value = decode_value(row, i, col.type_info().name());
                    (name, value)
                })
                .collect();
            DbRow(cols)
        })
        .collect();

    let affected = db_rows.len() as u64;
    (
        col_descs,
        DbRows {
            rows: db_rows,
            affected,
        },
    )
}

// ── propositions ──────────────────────────────────────────────────────────────

/// Proposition: a SQL query completed without error.
#[derive(Prop)]
pub struct QueryExecuted;

impl VerifiedWorkflow for QueryExecuted {}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `archive_query__execute`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteQueryParams {
    /// Database connection URL.
    pub url: String,
    /// SQL statement to execute.
    pub sql: String,
}

/// Parameters for `archive_query__preview_table`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PreviewTableParams {
    /// Database connection URL.
    pub url: String,
    /// Schema containing the table.
    pub schema: String,
    /// Table to preview.
    pub table: String,
    /// Maximum rows to return (default: 100).
    pub limit: Option<u32>,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "archive_query",
    name = "archive_query__execute",
    description = "Execute an arbitrary SQL SELECT and return a QueryResult with column metadata \
                   and typed row data. Establishes: QueryExecuted."
)]
#[instrument]
async fn execute(p: ExecuteQueryParams) -> Result<CallToolResult, ErrorData> {
    let pool = connect(&p.url).await?;

    let rows = sqlx::query(&p.sql)
        .fetch_all(&pool)
        .await
        .map_err(|e| ErrorData::internal_error(format!("query failed: {e}"), None))?;

    let (col_descs, db_rows) = decode_rows(&rows);

    pool.close().await;

    let _proof = Established::<QueryExecuted>::assert();
    json_result(&QueryResult::new(col_descs, db_rows))
}

#[elicit_tool(
    plugin = "archive_query",
    name = "archive_query__preview_table",
    description = "Preview the first N rows of a table. Returns a QueryResult with column \
                   metadata and typed row data. Default limit: 100."
)]
#[instrument]
async fn preview_table(p: PreviewTableParams) -> Result<CallToolResult, ErrorData> {
    let pool = connect(&p.url).await?;

    let limit = p.limit.unwrap_or(100);
    let sql = format!(
        r#"SELECT * FROM "{}"."{}" LIMIT {}"#,
        p.schema.replace('"', ""),
        p.table.replace('"', ""),
        limit
    );

    let rows = sqlx::query(&sql)
        .fetch_all(&pool)
        .await
        .map_err(|e| ErrorData::internal_error(format!("preview failed: {e}"), None))?;

    let (col_descs, db_rows) = decode_rows(&rows);

    pool.close().await;

    json_result(&QueryResult::new(col_descs, db_rows))
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for SQL query execution.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "archive_query")]
pub struct ArchiveQueryPlugin;

impl ArchiveQueryPlugin {
    /// Create a new query plugin, registering sqlx drivers.
    #[instrument]
    pub fn new() -> Self {
        sqlx::any::install_default_drivers();
        Self
    }
}

impl Default for ArchiveQueryPlugin {
    fn default() -> Self {
        Self::new()
    }
}

// ── Direct async helpers (used by frontend fetch tasks) ───────────────────────

/// Fetch up to `limit` rows from a table directly, without going through the
/// MCP tool layer.  Returns a `QueryResult` or an error string.
///
/// Used by the ratatui and egui frontend background fetch tasks.
pub async fn preview_table_direct(
    url: &str,
    schema: &str,
    table: &str,
    limit: u32,
) -> Result<crate::archive::QueryResult, String> {
    sqlx::any::install_default_drivers();
    let pool = sqlx::any::AnyPoolOptions::new()
        .max_connections(3)
        .connect(url)
        .await
        .map_err(|e| format!("connection failed: {e}"))?;

    let effective_limit = if limit == 0 { 200 } else { limit };
    let sql = format!(
        r#"SELECT * FROM "{}"."{}" LIMIT {}"#,
        schema.replace('"', ""),
        table.replace('"', ""),
        effective_limit
    );

    let rows = sqlx::query(&sql)
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("query failed: {e}"))?;

    pool.close().await;
    let (col_descs, db_rows) = decode_rows(&rows);
    Ok(crate::archive::QueryResult::new(col_descs, db_rows))
}

/// Execute arbitrary SQL directly, without going through the MCP tool layer.
///
/// Used by the SQL editor in the frontend fetch tasks.
pub async fn execute_sql_direct(
    url: &str,
    sql: &str,
) -> Result<crate::archive::QueryResult, String> {
    sqlx::any::install_default_drivers();
    let pool = sqlx::any::AnyPoolOptions::new()
        .max_connections(3)
        .connect(url)
        .await
        .map_err(|e| format!("connection failed: {e}"))?;

    let rows = sqlx::query(sql)
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("query failed: {e}"))?;

    pool.close().await;
    let (col_descs, db_rows) = decode_rows(&rows);
    Ok(crate::archive::QueryResult::new(col_descs, db_rows))
}
