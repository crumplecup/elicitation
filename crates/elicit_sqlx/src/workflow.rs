//! `SqlxWorkflowPlugin` — phrase-level SQL tool compositions with verified contracts.
//!
//! While the driver-specific plugins (`SqlxPgPlugin`, `SqlxSqlitePlugin`,
//! `SqlxMySqlPlugin`) provide the **letters** of the alphabet (connect, execute,
//! fetch, begin, commit…), this plugin provides **words**: each tool is a
//! meaningful verb with explicit contract documentation and internally
//! proof-carrying implementations.
//!
//! # Contracts and Propositions
//!
//! Every tool documents its **assumptions** and the **propositions it establishes**
//! on success.  The Rust implementation carries those proofs internally via
//! [`elicitation::contracts::Established`] — zero-cost `PhantomData` markers
//! that disappear at compile time.
//!
//! Example contract chain for a transactional insert:
//!
//! ```text
//! connect(url)          → establishes DbConnected
//!     ↓
//! begin(pool_id)        → establishes TransactionOpen
//!     ↓
//! tx_execute(tx_id, sql)→ establishes QueryExecuted
//!     ↓
//! commit(tx_id)         → establishes TransactionCommitted
//! ```
//!
//! # Driver support
//!
//! Uses the `Any` driver backend (via `sqlx::any::install_default_drivers()`),
//! so the same 13 tools work against Postgres, SQLite, and MySQL.  The
//! connection URL determines the backend at runtime.
//!
//! # Tool namespace: `sqlx_workflow__*`
//!
//! | Tool | Establishes | Description |
//! |---|---|---|
//! | `connect` | `DbConnected` | Open pool, return `pool_id` |
//! | `disconnect` | — | Close and remove pool |
//! | `execute` | `QueryExecuted` | Execute SQL → `QueryResultData` |
//! | `fetch_all` | `RowsFetched` | SELECT → `Vec<RowData>` |
//! | `fetch_one` | `RowsFetched` | SELECT → first `RowData` |
//! | `fetch_optional` | — | SELECT → `Option<RowData>` |
//! | `begin` | `TransactionOpen` | Start transaction → `tx_id` |
//! | `commit` | `TransactionCommitted` | Commit transaction |
//! | `rollback` | `TransactionRolledBack` | Roll back transaction |
//! | `tx_execute` | `QueryExecuted` | Execute within transaction |
//! | `tx_fetch_all` | `RowsFetched` | SELECT within transaction |
//! | `tx_fetch_one` | `RowsFetched` | SELECT (first) within transaction |
//! | `tx_fetch_optional` | — | SELECT (optional) within transaction |

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::contracts::{And, Established, Prop, both};
use elicitation::{ColumnEntry, ColumnValue, RowData};
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::any::AnyRow;
use sqlx::{Column as _, Row as _, TypeInfo as _};
use tokio::sync::Mutex;
use tracing::instrument;
use uuid::Uuid;

use crate::QueryResultData;

// ── Propositions ─────────────────────────────────────────────────────────────

/// Proposition: a named pool was successfully created and is ready to accept queries.
pub struct DbConnected;
impl Prop for DbConnected {}

/// Proposition: a SQL statement completed and `rows_affected` is known.
pub struct QueryExecuted;
impl Prop for QueryExecuted {}

/// Proposition: a SELECT returned ≥ 0 rows without error.
pub struct RowsFetched;
impl Prop for RowsFetched {}

/// Proposition: a transaction was started and is uncommitted.
pub struct TransactionOpen;
impl Prop for TransactionOpen {}

/// Proposition: a transaction was successfully committed.
pub struct TransactionCommitted;
impl Prop for TransactionCommitted {}

/// Proposition: a transaction was successfully rolled back.
pub struct TransactionRolledBack;
impl Prop for TransactionRolledBack {}

/// Composite: a connection was made and a query was executed.
pub type ConnectedAndExecuted = And<DbConnected, QueryExecuted>;

/// Composite: a connection was made, a transaction was opened and committed.
pub type FullCommit = And<DbConnected, And<TransactionOpen, TransactionCommitted>>;

// ── Registry types ────────────────────────────────────────────────────────────

type AnyPool = sqlx::AnyPool;
type AnyTransaction = sqlx::Transaction<'static, sqlx::Any>;
type PoolRegistry = Arc<Mutex<HashMap<Uuid, AnyPool>>>;
type TxRegistry = Arc<Mutex<HashMap<Uuid, AnyTransaction>>>;

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `sqlx_workflow__connect`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WfConnectParams {
    /// Database URL. Examples: `postgres://user:pass@host/db`, `sqlite::memory:`,
    /// `mysql://user:pass@host/db`.
    pub database_url: String,
    /// Maximum connections in the pool (default: `sqlx` default, usually 10).
    /// Set to `1` when using `sqlite::memory:` to share the same in-memory DB
    /// across all pool calls.
    #[serde(default)]
    pub max_connections: Option<u32>,
}

/// Parameters for tools that reference a pool by ID.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WfPoolIdParams {
    /// UUID returned by `sqlx_workflow__connect`.
    pub pool_id: Uuid,
}

/// Parameters for pool-level SQL execution.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WfPoolSqlParams {
    /// UUID returned by `sqlx_workflow__connect`.
    pub pool_id: Uuid,
    /// SQL statement (may include positional parameters).
    pub sql: String,
    /// Optional positional arguments. JSON bools, numbers, strings, and null
    /// are mapped to native driver bindings.
    #[serde(default)]
    pub args: Vec<serde_json::Value>,
}

/// Parameters for transaction-level SQL execution.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WfTxSqlParams {
    /// UUID returned by `sqlx_workflow__begin`.
    pub tx_id: Uuid,
    /// SQL statement (may include positional parameters).
    pub sql: String,
    /// Optional positional arguments.
    #[serde(default)]
    pub args: Vec<serde_json::Value>,
}

/// Parameters for `sqlx_workflow__begin`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WfBeginParams {
    /// UUID returned by `sqlx_workflow__connect`.
    pub pool_id: Uuid,
}

/// Parameters for `sqlx_workflow__commit` and `sqlx_workflow__rollback`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WfTxIdParams {
    /// UUID returned by `sqlx_workflow__begin`.
    pub tx_id: Uuid,
}

// ── Result structs ────────────────────────────────────────────────────────────

/// Returned by `sqlx_workflow__connect`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct WfConnectResult {
    /// Use this handle in subsequent pool and transaction calls.
    pub pool_id: Uuid,
    /// Contract established: `DbConnected`.
    pub contract: &'static str,
}

/// Returned by `sqlx_workflow__begin`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct WfBeginResult {
    /// Use this handle in `tx_execute`, `tx_fetch_*`, `commit`, `rollback`.
    pub tx_id: Uuid,
    /// Contract established: `TransactionOpen`.
    pub contract: &'static str,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn parse_args<T: for<'de> Deserialize<'de>>(
    params: &CallToolRequestParams,
) -> Result<T, ErrorData> {
    let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
    serde_json::from_value(value).map_err(|e| ErrorData::invalid_params(e.to_string(), None))
}

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

/// Decode an [`AnyRow`] into a serializable [`RowData`] using `try_get` dispatch.
fn decode_any_row(row: &AnyRow) -> RowData {
    let columns = row
        .columns()
        .iter()
        .enumerate()
        .map(|(i, col)| {
            let name = col.name().to_string();
            let value = match col.type_info().name() {
                "BOOL" | "BOOLEAN" => row
                    .try_get::<bool, _>(i)
                    .map(ColumnValue::Bool)
                    .unwrap_or(ColumnValue::Null),
                "INT2" | "SMALLINT" | "SMALLSERIAL" | "TINYINT" => row
                    .try_get::<i16, _>(i)
                    .map(ColumnValue::SmallInt)
                    .unwrap_or(ColumnValue::Null),
                "INT" | "INT4" | "INTEGER" | "SERIAL" | "MEDIUMINT" => row
                    .try_get::<i32, _>(i)
                    .map(ColumnValue::Integer)
                    .unwrap_or(ColumnValue::Null),
                "INT8" | "BIGINT" | "BIGSERIAL" => row
                    .try_get::<i64, _>(i)
                    .map(ColumnValue::BigInt)
                    .unwrap_or(ColumnValue::Null),
                "FLOAT" | "FLOAT4" | "REAL" => row
                    .try_get::<f32, _>(i)
                    .map(ColumnValue::Real)
                    .unwrap_or(ColumnValue::Null),
                "FLOAT8" | "DOUBLE" | "DOUBLE PRECISION" => row
                    .try_get::<f64, _>(i)
                    .map(ColumnValue::Double)
                    .unwrap_or(ColumnValue::Null),
                "BLOB" | "BYTEA" => row
                    .try_get::<Vec<u8>, _>(i)
                    .map(ColumnValue::Blob)
                    .unwrap_or(ColumnValue::Null),
                _ => row
                    .try_get::<String, _>(i)
                    .map(ColumnValue::Text)
                    .unwrap_or(ColumnValue::Null),
            };
            ColumnEntry::new(name, value)
        })
        .collect();
    RowData::new(columns)
}

/// Bind JSON args into `sqlx::any::AnyArguments`.
fn any_args_from_json(args: &[serde_json::Value]) -> sqlx::any::AnyArguments<'static> {
    use sqlx::Arguments as _;
    let mut out = sqlx::any::AnyArguments::default();
    for val in args {
        match val {
            serde_json::Value::Bool(b) => out.add(*b).expect("bind bool"),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    out.add(i).expect("bind i64");
                } else if let Some(f) = n.as_f64() {
                    out.add(f).expect("bind f64");
                } else {
                    out.add(Option::<String>::None).expect("bind null");
                }
            }
            serde_json::Value::String(s) => out.add(s.clone()).expect("bind string"),
            _ => out.add(Option::<String>::None).expect("bind null"),
        }
    }
    out
}

// ── Internal contract-carrying helpers ───────────────────────────────────────

/// Connect to a database, store pool in registry.
///
/// Returns `(pool_id, Established<DbConnected>)` on success.
async fn connect_inner(
    url: &str,
    max_connections: Option<u32>,
    pools: &PoolRegistry,
) -> Result<(Uuid, Established<DbConnected>), CallToolResult> {
    sqlx::any::install_default_drivers();
    let mut opts = sqlx::any::AnyPoolOptions::new();
    if let Some(n) = max_connections {
        opts = opts.max_connections(n);
    }
    let pool = opts.connect(url).await.map_err(|e| {
        CallToolResult::error(vec![Content::text(format!(
            "DbConnected not established: {e}"
        ))])
    })?;
    let id = Uuid::new_v4();
    pools.lock().await.insert(id, pool);
    let proof: Established<DbConnected> = Established::assert();
    Ok((id, proof))
}

/// Execute a SQL statement against a pool.
///
/// Returns `(QueryResultData, Established<QueryExecuted>)` on success.
async fn execute_inner(
    pool_id: Uuid,
    sql: &str,
    args: &[serde_json::Value],
    pools: &PoolRegistry,
) -> Result<(QueryResultData, Established<QueryExecuted>), CallToolResult> {
    let result = {
        let guard = pools.lock().await;
        let pool = guard.get(&pool_id).ok_or_else(|| {
            CallToolResult::error(vec![Content::text(
                "QueryExecuted not established: pool_id not found",
            )])
        })?;
        if args.is_empty() {
            sqlx::query(sql).execute(pool).await
        } else {
            sqlx::query_with(sql, any_args_from_json(args))
                .execute(pool)
                .await
        }
    }
    .map_err(|e| {
        CallToolResult::error(vec![Content::text(format!(
            "QueryExecuted not established: {e}"
        ))])
    })?;
    let proof: Established<QueryExecuted> = Established::assert();
    Ok((
        QueryResultData {
            rows_affected: result.rows_affected(),
            last_insert_id: result.last_insert_id(),
        },
        proof,
    ))
}

/// Execute a SELECT against a pool; return all rows.
///
/// Returns `(Vec<RowData>, Established<RowsFetched>)` on success.
async fn fetch_all_inner(
    pool_id: Uuid,
    sql: &str,
    args: &[serde_json::Value],
    pools: &PoolRegistry,
) -> Result<(Vec<RowData>, Established<RowsFetched>), CallToolResult> {
    let rows = {
        let guard = pools.lock().await;
        let pool = guard.get(&pool_id).ok_or_else(|| {
            CallToolResult::error(vec![Content::text(
                "RowsFetched not established: pool_id not found",
            )])
        })?;
        if args.is_empty() {
            sqlx::query(sql).fetch_all(pool).await
        } else {
            sqlx::query_with(sql, any_args_from_json(args))
                .fetch_all(pool)
                .await
        }
    }
    .map_err(|e| {
        CallToolResult::error(vec![Content::text(format!(
            "RowsFetched not established: {e}"
        ))])
    })?;
    let data: Vec<RowData> = rows.iter().map(decode_any_row).collect();
    let proof: Established<RowsFetched> = Established::assert();
    Ok((data, proof))
}

/// Execute a SELECT; return exactly one row or error.
async fn fetch_one_inner(
    pool_id: Uuid,
    sql: &str,
    args: &[serde_json::Value],
    pools: &PoolRegistry,
) -> Result<(RowData, Established<RowsFetched>), CallToolResult> {
    let row = {
        let guard = pools.lock().await;
        let pool = guard.get(&pool_id).ok_or_else(|| {
            CallToolResult::error(vec![Content::text(
                "RowsFetched not established: pool_id not found",
            )])
        })?;
        if args.is_empty() {
            sqlx::query(sql).fetch_one(pool).await
        } else {
            sqlx::query_with(sql, any_args_from_json(args))
                .fetch_one(pool)
                .await
        }
    }
    .map_err(|e| {
        CallToolResult::error(vec![Content::text(format!(
            "RowsFetched not established: {e}"
        ))])
    })?;
    let proof: Established<RowsFetched> = Established::assert();
    Ok((decode_any_row(&row), proof))
}

/// Execute a SELECT; return first row or None.
async fn fetch_optional_inner(
    pool_id: Uuid,
    sql: &str,
    args: &[serde_json::Value],
    pools: &PoolRegistry,
) -> Result<Option<RowData>, CallToolResult> {
    let maybe = {
        let guard = pools.lock().await;
        let pool = guard
            .get(&pool_id)
            .ok_or_else(|| CallToolResult::error(vec![Content::text("pool_id not found")]))?;
        if args.is_empty() {
            sqlx::query(sql).fetch_optional(pool).await
        } else {
            sqlx::query_with(sql, any_args_from_json(args))
                .fetch_optional(pool)
                .await
        }
    }
    .map_err(|e| {
        CallToolResult::error(vec![Content::text(format!("fetch_optional failed: {e}"))])
    })?;
    Ok(maybe.as_ref().map(decode_any_row))
}

/// Begin a transaction against a pool.
///
/// Returns `(tx_id, Established<TransactionOpen>)` on success.
async fn begin_inner(
    pool_id: Uuid,
    pools: &PoolRegistry,
    txs: &TxRegistry,
) -> Result<(Uuid, Established<TransactionOpen>), CallToolResult> {
    let tx = {
        let guard = pools.lock().await;
        let pool = guard.get(&pool_id).ok_or_else(|| {
            CallToolResult::error(vec![Content::text(
                "TransactionOpen not established: pool_id not found",
            )])
        })?;
        pool.begin().await.map_err(|e| {
            CallToolResult::error(vec![Content::text(format!(
                "TransactionOpen not established: {e}"
            ))])
        })?
    };
    let id = Uuid::new_v4();
    txs.lock().await.insert(id, tx);
    let proof: Established<TransactionOpen> = Established::assert();
    Ok((id, proof))
}

/// Execute SQL within a transaction.
async fn tx_execute_inner(
    tx_id: Uuid,
    sql: &str,
    args: &[serde_json::Value],
    txs: &TxRegistry,
) -> Result<(QueryResultData, Established<QueryExecuted>), CallToolResult> {
    let mut guard = txs.lock().await;
    let tx = guard.get_mut(&tx_id).ok_or_else(|| {
        CallToolResult::error(vec![Content::text(
            "QueryExecuted not established: tx_id not found",
        )])
    })?;
    let result = if args.is_empty() {
        sqlx::query(sql).execute(&mut **tx).await
    } else {
        sqlx::query_with(sql, any_args_from_json(args))
            .execute(&mut **tx)
            .await
    }
    .map_err(|e| {
        CallToolResult::error(vec![Content::text(format!(
            "QueryExecuted not established: {e}"
        ))])
    })?;
    let proof: Established<QueryExecuted> = Established::assert();
    Ok((
        QueryResultData {
            rows_affected: result.rows_affected(),
            last_insert_id: result.last_insert_id(),
        },
        proof,
    ))
}

/// Fetch all rows within a transaction.
async fn tx_fetch_all_inner(
    tx_id: Uuid,
    sql: &str,
    args: &[serde_json::Value],
    txs: &TxRegistry,
) -> Result<(Vec<RowData>, Established<RowsFetched>), CallToolResult> {
    let mut guard = txs.lock().await;
    let tx = guard.get_mut(&tx_id).ok_or_else(|| {
        CallToolResult::error(vec![Content::text(
            "RowsFetched not established: tx_id not found",
        )])
    })?;
    let rows = if args.is_empty() {
        sqlx::query(sql).fetch_all(&mut **tx).await
    } else {
        sqlx::query_with(sql, any_args_from_json(args))
            .fetch_all(&mut **tx)
            .await
    }
    .map_err(|e| {
        CallToolResult::error(vec![Content::text(format!(
            "RowsFetched not established: {e}"
        ))])
    })?;
    let data: Vec<RowData> = rows.iter().map(decode_any_row).collect();
    let proof: Established<RowsFetched> = Established::assert();
    Ok((data, proof))
}

/// Fetch first row within a transaction.
async fn tx_fetch_one_inner(
    tx_id: Uuid,
    sql: &str,
    args: &[serde_json::Value],
    txs: &TxRegistry,
) -> Result<(RowData, Established<RowsFetched>), CallToolResult> {
    let mut guard = txs.lock().await;
    let tx = guard.get_mut(&tx_id).ok_or_else(|| {
        CallToolResult::error(vec![Content::text(
            "RowsFetched not established: tx_id not found",
        )])
    })?;
    let row = if args.is_empty() {
        sqlx::query(sql).fetch_one(&mut **tx).await
    } else {
        sqlx::query_with(sql, any_args_from_json(args))
            .fetch_one(&mut **tx)
            .await
    }
    .map_err(|e| {
        CallToolResult::error(vec![Content::text(format!(
            "RowsFetched not established: {e}"
        ))])
    })?;
    let proof: Established<RowsFetched> = Established::assert();
    Ok((decode_any_row(&row), proof))
}

/// Fetch optional row within a transaction.
async fn tx_fetch_optional_inner(
    tx_id: Uuid,
    sql: &str,
    args: &[serde_json::Value],
    txs: &TxRegistry,
) -> Result<Option<RowData>, CallToolResult> {
    let mut guard = txs.lock().await;
    let tx = guard
        .get_mut(&tx_id)
        .ok_or_else(|| CallToolResult::error(vec![Content::text("tx_id not found")]))?;
    let maybe = if args.is_empty() {
        sqlx::query(sql).fetch_optional(&mut **tx).await
    } else {
        sqlx::query_with(sql, any_args_from_json(args))
            .fetch_optional(&mut **tx)
            .await
    }
    .map_err(|e| {
        CallToolResult::error(vec![Content::text(format!(
            "tx_fetch_optional failed: {e}"
        ))])
    })?;
    Ok(maybe.as_ref().map(decode_any_row))
}

/// Commit a transaction.
async fn commit_inner(
    tx_id: Uuid,
    txs: &TxRegistry,
) -> Result<Established<TransactionCommitted>, CallToolResult> {
    let tx = txs.lock().await.remove(&tx_id).ok_or_else(|| {
        CallToolResult::error(vec![Content::text(
            "TransactionCommitted not established: tx_id not found",
        )])
    })?;
    tx.commit().await.map_err(|e| {
        CallToolResult::error(vec![Content::text(format!(
            "TransactionCommitted not established: {e}"
        ))])
    })?;
    Ok(Established::assert())
}

/// Roll back a transaction.
async fn rollback_inner(
    tx_id: Uuid,
    txs: &TxRegistry,
) -> Result<Established<TransactionRolledBack>, CallToolResult> {
    let tx = txs.lock().await.remove(&tx_id).ok_or_else(|| {
        CallToolResult::error(vec![Content::text(
            "TransactionRolledBack not established: tx_id not found",
        )])
    })?;
    tx.rollback().await.map_err(|e| {
        CallToolResult::error(vec![Content::text(format!(
            "TransactionRolledBack not established: {e}"
        ))])
    })?;
    Ok(Established::assert())
}

// ── Plugin struct ─────────────────────────────────────────────────────────────

/// Any-driver MCP plugin for verified SQL workflows.
///
/// Works with Postgres, SQLite, and MySQL — the connection URL determines the
/// backend.  Maintains named pools and open transactions between tool calls;
/// create once and wire into a [`PluginRegistry`][elicitation::PluginRegistry].
///
/// Every tool documents the propositions it establishes.  Internal helpers
/// return `Established<P>` proof markers that compile away to nothing but
/// machine-check the correctness of the implementation.
pub struct SqlxWorkflowPlugin {
    pools: PoolRegistry,
    txs: TxRegistry,
}

impl SqlxWorkflowPlugin {
    /// Create a new plugin with no open pools or transactions.
    pub fn new() -> Self {
        Self {
            pools: Arc::new(Mutex::new(HashMap::new())),
            txs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Open a new connection pool.  Returns `(pool_id, Established<DbConnected>)`.
    pub async fn connect(&self, url: &str) -> Result<(Uuid, Established<DbConnected>), String> {
        self.connect_with(url, None).await
    }

    /// Open a new connection pool with explicit pool options.
    pub async fn connect_with(
        &self,
        url: &str,
        max_connections: Option<u32>,
    ) -> Result<(Uuid, Established<DbConnected>), String> {
        connect_inner(url, max_connections, &self.pools)
            .await
            .map_err(tool_result_to_string)
    }

    /// Close and remove a pool.
    pub async fn disconnect(&self, pool_id: Uuid) -> Result<(), String> {
        let pool = self
            .pools
            .lock()
            .await
            .remove(&pool_id)
            .ok_or("pool_id not found")?;
        pool.close().await;
        Ok(())
    }

    /// Execute a non-returning SQL statement against a pool.
    pub async fn execute(
        &self,
        pool_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<(QueryResultData, Established<QueryExecuted>), String> {
        execute_inner(pool_id, sql, args, &self.pools)
            .await
            .map_err(tool_result_to_string)
    }

    /// Execute a SELECT and return all rows as [`RowData`].
    pub async fn fetch_all_data(
        &self,
        pool_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<(Vec<RowData>, Established<RowsFetched>), String> {
        fetch_all_inner(pool_id, sql, args, &self.pools)
            .await
            .map_err(tool_result_to_string)
    }

    /// Execute a SELECT and return the first row as [`RowData`].
    pub async fn fetch_one_data(
        &self,
        pool_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<(RowData, Established<RowsFetched>), String> {
        fetch_one_inner(pool_id, sql, args, &self.pools)
            .await
            .map_err(tool_result_to_string)
    }

    /// Execute a SELECT and return the first row or `None`.
    pub async fn fetch_optional_data(
        &self,
        pool_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<Option<RowData>, String> {
        fetch_optional_inner(pool_id, sql, args, &self.pools)
            .await
            .map_err(tool_result_to_string)
    }

    /// Begin a transaction.  Returns `(tx_id, Established<TransactionOpen>)`.
    pub async fn begin(
        &self,
        pool_id: Uuid,
    ) -> Result<(Uuid, Established<TransactionOpen>), String> {
        begin_inner(pool_id, &self.pools, &self.txs)
            .await
            .map_err(tool_result_to_string)
    }

    /// Commit a transaction.
    pub async fn commit(&self, tx_id: Uuid) -> Result<Established<TransactionCommitted>, String> {
        commit_inner(tx_id, &self.txs)
            .await
            .map_err(tool_result_to_string)
    }

    /// Roll back a transaction.
    pub async fn rollback(
        &self,
        tx_id: Uuid,
    ) -> Result<Established<TransactionRolledBack>, String> {
        rollback_inner(tx_id, &self.txs)
            .await
            .map_err(tool_result_to_string)
    }

    /// Execute SQL within a transaction.
    pub async fn tx_execute(
        &self,
        tx_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<(QueryResultData, Established<QueryExecuted>), String> {
        tx_execute_inner(tx_id, sql, args, &self.txs)
            .await
            .map_err(tool_result_to_string)
    }

    /// SELECT all rows within a transaction.
    pub async fn tx_fetch_all_data(
        &self,
        tx_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<(Vec<RowData>, Established<RowsFetched>), String> {
        tx_fetch_all_inner(tx_id, sql, args, &self.txs)
            .await
            .map_err(tool_result_to_string)
    }

    /// SELECT first row within a transaction.
    pub async fn tx_fetch_one_data(
        &self,
        tx_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<(RowData, Established<RowsFetched>), String> {
        tx_fetch_one_inner(tx_id, sql, args, &self.txs)
            .await
            .map_err(tool_result_to_string)
    }

    /// SELECT optional first row within a transaction.
    pub async fn tx_fetch_optional_data(
        &self,
        tx_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<Option<RowData>, String> {
        tx_fetch_optional_inner(tx_id, sql, args, &self.txs)
            .await
            .map_err(tool_result_to_string)
    }
}

/// Extract a text message from a `CallToolResult` error variant.
fn tool_result_to_string(r: CallToolResult) -> String {
    r.content
        .first()
        .and_then(|c| c.as_text())
        .map(|t| t.text.clone())
        .unwrap_or_else(|| "tool error".to_string())
}

impl Default for SqlxWorkflowPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for SqlxWorkflowPlugin {
    fn name(&self) -> &'static str {
        "sqlx_workflow"
    }

    fn list_tools(&self) -> Vec<Tool> {
        fn tool(name: &'static str, desc: &'static str) -> Tool {
            Tool::new(name, desc, Arc::new(Default::default()))
        }
        vec![
            tool(
                "sqlx_workflow__connect",
                "Connect to any SQL database (Postgres, SQLite, MySQL) via URL. \
                 Assumes: URL is well-formed and the database is reachable. \
                 Establishes: DbConnected — pool stored by returned pool_id.",
            ),
            tool(
                "sqlx_workflow__disconnect",
                "Close and remove a named pool. \
                 Assumes: pool_id was returned by sqlx_workflow__connect.",
            ),
            tool(
                "sqlx_workflow__execute",
                "Execute a non-returning SQL statement (INSERT, UPDATE, DELETE, DDL). \
                 Assumes: DbConnected (pool_id valid). \
                 Establishes: QueryExecuted — rows_affected is accurate.",
            ),
            tool(
                "sqlx_workflow__fetch_all",
                "Execute a SELECT and return all rows. \
                 Assumes: DbConnected (pool_id valid). \
                 Establishes: RowsFetched — returned Vec contains every matching row.",
            ),
            tool(
                "sqlx_workflow__fetch_one",
                "Execute a SELECT and return exactly the first row; errors if none found. \
                 Assumes: DbConnected (pool_id valid); at least one row exists. \
                 Establishes: RowsFetched.",
            ),
            tool(
                "sqlx_workflow__fetch_optional",
                "Execute a SELECT and return the first row or null. \
                 Assumes: DbConnected (pool_id valid).",
            ),
            tool(
                "sqlx_workflow__begin",
                "Start a database transaction. \
                 Assumes: DbConnected (pool_id valid). \
                 Establishes: TransactionOpen — tx stored by returned tx_id.",
            ),
            tool(
                "sqlx_workflow__commit",
                "Commit an open transaction. \
                 Assumes: TransactionOpen (tx_id valid). \
                 Establishes: TransactionCommitted — all changes are durable.",
            ),
            tool(
                "sqlx_workflow__rollback",
                "Roll back an open transaction. \
                 Assumes: TransactionOpen (tx_id valid). \
                 Establishes: TransactionRolledBack — all changes since begin are undone.",
            ),
            tool(
                "sqlx_workflow__tx_execute",
                "Execute a non-returning SQL statement within an open transaction. \
                 Assumes: TransactionOpen (tx_id valid). \
                 Establishes: QueryExecuted.",
            ),
            tool(
                "sqlx_workflow__tx_fetch_all",
                "SELECT all rows within an open transaction. \
                 Assumes: TransactionOpen (tx_id valid). \
                 Establishes: RowsFetched.",
            ),
            tool(
                "sqlx_workflow__tx_fetch_one",
                "SELECT first row within an open transaction; errors if none found. \
                 Assumes: TransactionOpen (tx_id valid); at least one row exists. \
                 Establishes: RowsFetched.",
            ),
            tool(
                "sqlx_workflow__tx_fetch_optional",
                "SELECT first row (or null) within an open transaction. \
                 Assumes: TransactionOpen (tx_id valid).",
            ),
        ]
    }

    #[instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        Box::pin(async move {
            let verb = params.name.strip_prefix("sqlx_workflow__").ok_or_else(|| {
                ErrorData::invalid_params(format!("unknown tool: {}", params.name), None)
            })?;

            match verb {
                // ── Pool management ───────────────────────────────────────────
                "connect" => {
                    let p: WfConnectParams = parse_args(&params)?;
                    let (pool_id, _proof) =
                        connect_inner(&p.database_url, p.max_connections, &self.pools)
                            .await
                            .map_err(err_from_result)?;
                    Ok(json_result(&WfConnectResult {
                        pool_id,
                        contract: "DbConnected",
                    }))
                }

                "disconnect" => {
                    let p: WfPoolIdParams = parse_args(&params)?;
                    let pool = self
                        .pools
                        .lock()
                        .await
                        .remove(&p.pool_id)
                        .ok_or_else(|| ErrorData::invalid_params("pool_id not found", None))?;
                    pool.close().await;
                    Ok(CallToolResult::success(vec![Content::text(
                        r#"{"ok":true}"#,
                    )]))
                }

                // ── Direct pool queries ───────────────────────────────────────
                "execute" => {
                    let p: WfPoolSqlParams = parse_args(&params)?;
                    let (result, _proof) = execute_inner(p.pool_id, &p.sql, &p.args, &self.pools)
                        .await
                        .map_err(err_from_result)?;
                    Ok(json_result(&result))
                }

                "fetch_all" => {
                    let p: WfPoolSqlParams = parse_args(&params)?;
                    let (rows, _proof) = fetch_all_inner(p.pool_id, &p.sql, &p.args, &self.pools)
                        .await
                        .map_err(err_from_result)?;
                    Ok(json_result(&rows))
                }

                "fetch_one" => {
                    let p: WfPoolSqlParams = parse_args(&params)?;
                    let (row, _proof) = fetch_one_inner(p.pool_id, &p.sql, &p.args, &self.pools)
                        .await
                        .map_err(err_from_result)?;
                    Ok(json_result(&row))
                }

                "fetch_optional" => {
                    let p: WfPoolSqlParams = parse_args(&params)?;
                    let maybe = fetch_optional_inner(p.pool_id, &p.sql, &p.args, &self.pools)
                        .await
                        .map_err(err_from_result)?;
                    Ok(json_result(&maybe))
                }

                // ── Transactions ──────────────────────────────────────────────
                "begin" => {
                    let p: WfBeginParams = parse_args(&params)?;
                    let (tx_id, _proof) = begin_inner(p.pool_id, &self.pools, &self.txs)
                        .await
                        .map_err(err_from_result)?;
                    Ok(json_result(&WfBeginResult {
                        tx_id,
                        contract: "TransactionOpen",
                    }))
                }

                "commit" => {
                    let p: WfTxIdParams = parse_args(&params)?;
                    let _proof = commit_inner(p.tx_id, &self.txs)
                        .await
                        .map_err(err_from_result)?;
                    Ok(CallToolResult::success(vec![Content::text(
                        r#"{"ok":true,"contract":"TransactionCommitted"}"#,
                    )]))
                }

                "rollback" => {
                    let p: WfTxIdParams = parse_args(&params)?;
                    let _proof = rollback_inner(p.tx_id, &self.txs)
                        .await
                        .map_err(err_from_result)?;
                    Ok(CallToolResult::success(vec![Content::text(
                        r#"{"ok":true,"contract":"TransactionRolledBack"}"#,
                    )]))
                }

                // ── Transaction queries ───────────────────────────────────────
                "tx_execute" => {
                    let p: WfTxSqlParams = parse_args(&params)?;
                    let (result, _proof) = tx_execute_inner(p.tx_id, &p.sql, &p.args, &self.txs)
                        .await
                        .map_err(err_from_result)?;
                    Ok(json_result(&result))
                }

                "tx_fetch_all" => {
                    let p: WfTxSqlParams = parse_args(&params)?;
                    let (rows, _proof) = tx_fetch_all_inner(p.tx_id, &p.sql, &p.args, &self.txs)
                        .await
                        .map_err(err_from_result)?;
                    Ok(json_result(&rows))
                }

                "tx_fetch_one" => {
                    let p: WfTxSqlParams = parse_args(&params)?;
                    let (row, _proof) = tx_fetch_one_inner(p.tx_id, &p.sql, &p.args, &self.txs)
                        .await
                        .map_err(err_from_result)?;
                    Ok(json_result(&row))
                }

                "tx_fetch_optional" => {
                    let p: WfTxSqlParams = parse_args(&params)?;
                    let maybe = tx_fetch_optional_inner(p.tx_id, &p.sql, &p.args, &self.txs)
                        .await
                        .map_err(err_from_result)?;
                    Ok(json_result(&maybe))
                }

                other => Err(ErrorData::invalid_params(
                    format!("unknown sqlx_workflow tool: {other}"),
                    None,
                )),
            }
        })
    }
}

/// Convert a `CallToolResult` (error variant) into `ErrorData` for propagation.
fn err_from_result(r: CallToolResult) -> ErrorData {
    let msg = r
        .content
        .first()
        .and_then(|c| c.as_text())
        .map(|t| t.text.clone())
        .unwrap_or_else(|| "tool error".to_string());
    ErrorData::internal_error(msg, None)
}

// ── Proposition combinators (re-exported for callers) ────────────────────────

/// Combine `DbConnected` + `QueryExecuted` into the composite proof.
pub fn connected_and_executed(
    db: Established<DbConnected>,
    qe: Established<QueryExecuted>,
) -> Established<ConnectedAndExecuted> {
    both(db, qe)
}

/// Combine `DbConnected` + `TransactionOpen` + `TransactionCommitted` into `FullCommit`.
pub fn full_commit(
    db: Established<DbConnected>,
    tx_open: Established<TransactionOpen>,
    committed: Established<TransactionCommitted>,
) -> Established<FullCommit> {
    both(db, both(tx_open, committed))
}
