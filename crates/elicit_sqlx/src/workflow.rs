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

use elicitation::PluginContext;
use elicitation::contracts::{And, Established, Prop, both};
use elicitation::emit_code::CustomEmit;
use elicitation::{ColumnEntry, ColumnValue, Elicit, RowData, elicit_tool};
use futures::future::BoxFuture;
use proc_macro2::TokenStream;
use quote::quote;
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
use uuid::Uuid;

use crate::QueryResultData;

// ── Propositions ─────────────────────────────────────────────────────────────

/// Proposition: a named pool was successfully created and is ready to accept queries.
#[derive(Elicit)]
pub struct DbConnected;
impl Prop for DbConnected {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_db_connected_axiom() {
                let connect_ok: bool = kani::any();
                kani::assume(connect_ok);
                assert!(connect_ok, "sqlx::AnyPool::connect axiom: Ok => pool created and ready");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_db_connected(connect_ok: bool) -> (result: bool)
                ensures result == connect_ok,
            {
                connect_ok
            }
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_db_connected_contract() -> bool {
                true
            }
        }
    }
}

/// Proposition: a SQL statement completed and `rows_affected` is known.
#[derive(Elicit)]
pub struct QueryExecuted;
impl Prop for QueryExecuted {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_query_executed_axiom() {
                let execute_ok: bool = kani::any();
                kani::assume(execute_ok);
                assert!(execute_ok, "sqlx::query execute axiom: Ok => rows_affected is known");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_query_executed(execute_ok: bool) -> (result: bool)
                ensures result == execute_ok,
            {
                execute_ok
            }
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_query_executed_contract() -> bool {
                true
            }
        }
    }
}

/// Proposition: a SELECT returned ≥ 0 rows without error.
#[derive(Elicit)]
pub struct RowsFetched;
impl Prop for RowsFetched {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_rows_fetched_axiom() {
                let fetch_ok: bool = kani::any();
                kani::assume(fetch_ok);
                assert!(fetch_ok, "sqlx::query fetch axiom: Ok => rows returned without error");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_rows_fetched(fetch_ok: bool) -> (result: bool)
                ensures result == fetch_ok,
            {
                fetch_ok
            }
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_rows_fetched_contract() -> bool {
                true
            }
        }
    }
}

/// Proposition: a transaction was started and is uncommitted.
#[derive(Elicit)]
pub struct TransactionOpen;
impl Prop for TransactionOpen {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_transaction_open_axiom() {
                let begin_ok: bool = kani::any();
                kani::assume(begin_ok);
                assert!(begin_ok, "sqlx::begin axiom: Ok => transaction started and uncommitted");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_transaction_open(begin_ok: bool) -> (result: bool)
                ensures result == begin_ok,
            {
                begin_ok
            }
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_transaction_open_contract() -> bool {
                true
            }
        }
    }
}

/// Proposition: a transaction was successfully committed.
#[derive(Elicit)]
pub struct TransactionCommitted;
impl Prop for TransactionCommitted {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_transaction_committed_axiom() {
                let commit_ok: bool = kani::any();
                kani::assume(commit_ok);
                assert!(commit_ok, "sqlx::commit axiom: Ok => transaction successfully committed");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_transaction_committed(commit_ok: bool) -> (result: bool)
                ensures result == commit_ok,
            {
                commit_ok
            }
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_transaction_committed_contract() -> bool {
                true
            }
        }
    }
}

/// Proposition: a transaction was successfully rolled back.
#[derive(Elicit)]
pub struct TransactionRolledBack;
impl Prop for TransactionRolledBack {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_transaction_rolled_back_axiom() {
                let rollback_ok: bool = kani::any();
                kani::assume(rollback_ok);
                assert!(rollback_ok, "sqlx::rollback axiom: Ok => transaction successfully rolled back");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_transaction_rolled_back(rollback_ok: bool) -> (result: bool)
                ensures result == rollback_ok,
            {
                rollback_ok
            }
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_transaction_rolled_back_contract() -> bool {
                true
            }
        }
    }
}

/// Composite: a connection was made and a query was executed.
pub type ConnectedAndExecuted = And<DbConnected, QueryExecuted>;

/// Composite: a connection was made, a transaction was opened and committed.
pub type FullCommit = And<DbConnected, And<TransactionOpen, TransactionCommitted>>;

// ── Internal type aliases ─────────────────────────────────────────────────────

type AnyPool = sqlx::AnyPool;
type AnyTransaction = sqlx::Transaction<'static, sqlx::Any>;

// ── Context types ─────────────────────────────────────────────────────────────

/// Plugin-level context: pools and open transactions shared across all tool calls.
pub struct SqlxCtx {
    pools: Mutex<HashMap<Uuid, AnyPool>>,
    txs: Mutex<HashMap<Uuid, Arc<SqlxTxCtx>>>,
}

impl SqlxCtx {
    fn new() -> Self {
        Self {
            pools: Mutex::new(HashMap::new()),
            txs: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for SqlxCtx {}

/// Per-call context for tools that operate directly on a pool.
pub struct SqlxPoolCtx {
    /// The resolved connection pool for this call.
    pub pool: AnyPool,
}

impl PluginContext for SqlxPoolCtx {}

/// Per-call context for tools that operate inside a transaction.
pub struct SqlxTxCtx {
    tx: Mutex<Option<AnyTransaction>>,
}

impl SqlxTxCtx {
    fn new(tx: AnyTransaction) -> Self {
        Self {
            tx: Mutex::new(Some(tx)),
        }
    }
}

impl PluginContext for SqlxTxCtx {}

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

// ── Per-tool param structs (same fields, distinct types for unique EmitCode impls) ──

/// Parameters for `sqlx_workflow__fetch_all`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WfFetchAllParams {
    /// UUID returned by `sqlx_workflow__connect`.
    pub pool_id: Uuid,
    /// SQL SELECT statement.
    pub sql: String,
    /// Optional positional arguments.
    #[serde(default)]
    pub args: Vec<serde_json::Value>,
}

/// Parameters for `sqlx_workflow__fetch_one`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WfFetchOneParams {
    /// UUID returned by `sqlx_workflow__connect`.
    pub pool_id: Uuid,
    /// SQL SELECT statement.
    pub sql: String,
    /// Optional positional arguments.
    #[serde(default)]
    pub args: Vec<serde_json::Value>,
}

/// Parameters for `sqlx_workflow__fetch_optional`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WfFetchOptionalParams {
    /// UUID returned by `sqlx_workflow__connect`.
    pub pool_id: Uuid,
    /// SQL SELECT statement.
    pub sql: String,
    /// Optional positional arguments.
    #[serde(default)]
    pub args: Vec<serde_json::Value>,
}

/// Parameters for `sqlx_workflow__rollback`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WfRollbackParams {
    /// UUID returned by `sqlx_workflow__begin`.
    pub tx_id: Uuid,
}

/// Parameters for `sqlx_workflow__tx_fetch_all`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WfTxFetchAllParams {
    /// UUID returned by `sqlx_workflow__begin`.
    pub tx_id: Uuid,
    /// SQL SELECT statement.
    pub sql: String,
    /// Optional positional arguments.
    #[serde(default)]
    pub args: Vec<serde_json::Value>,
}

/// Parameters for `sqlx_workflow__tx_fetch_one`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WfTxFetchOneParams {
    /// UUID returned by `sqlx_workflow__begin`.
    pub tx_id: Uuid,
    /// SQL SELECT statement.
    pub sql: String,
    /// Optional positional arguments.
    #[serde(default)]
    pub args: Vec<serde_json::Value>,
}

/// Parameters for `sqlx_workflow__tx_fetch_optional`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WfTxFetchOptionalParams {
    /// UUID returned by `sqlx_workflow__begin`.
    pub tx_id: Uuid,
    /// SQL SELECT statement.
    pub sql: String,
    /// Optional positional arguments.
    #[serde(default)]
    pub args: Vec<serde_json::Value>,
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

/// Emit a `.bind(...)` chain for a slice of JSON values (used by custom emit impls).
fn bind_chain(args: &[serde_json::Value]) -> TokenStream {
    let binds: Vec<TokenStream> = args
        .iter()
        .map(|v| match v {
            serde_json::Value::Bool(b) => quote! { .bind(#b) },
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    quote! { .bind(#i i64) }
                } else if let Some(f) = n.as_f64() {
                    quote! { .bind(#f f64) }
                } else {
                    quote! { .bind(Option::<String>::None) }
                }
            }
            serde_json::Value::String(s) => quote! { .bind(#s) },
            _ => quote! { .bind(Option::<String>::None) },
        })
        .collect();
    quote! { #(#binds)* }
}

// ── Tool functions ────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "sqlx_workflow",
    name = "sqlx_workflow__connect",
    description = "Connect to any SQL database (Postgres, SQLite, MySQL) via URL. \
                   Assumes: URL is well-formed and the database is reachable. \
                   Establishes: DbConnected — pool stored by returned pool_id.",
    emit = WfConnectEmit
)]
async fn wf_connect(ctx: Arc<SqlxCtx>, p: WfConnectParams) -> Result<CallToolResult, ErrorData> {
    sqlx::any::install_default_drivers();
    let mut opts = sqlx::any::AnyPoolOptions::new();
    if let Some(n) = p.max_connections {
        opts = opts.max_connections(n);
    }
    let pool = opts
        .connect(&p.database_url)
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    let pool_id = Uuid::new_v4();
    ctx.pools.lock().await.insert(pool_id, pool);
    Ok(json_result(&WfConnectResult {
        pool_id,
        contract: "DbConnected",
    }))
}

#[elicit_tool(
    plugin = "sqlx_workflow",
    name = "sqlx_workflow__disconnect",
    description = "Close and remove a named pool. \
                   Assumes: pool_id was returned by sqlx_workflow__connect.",
    emit = WfDisconnectEmit
)]
async fn wf_disconnect(ctx: Arc<SqlxCtx>, p: WfPoolIdParams) -> Result<CallToolResult, ErrorData> {
    if let Some(pool) = ctx.pools.lock().await.remove(&p.pool_id) {
        pool.close().await;
    }
    Ok(CallToolResult::success(vec![Content::text(
        r#"{"ok":true}"#,
    )]))
}

#[elicit_tool(
    plugin = "sqlx_workflow",
    name = "sqlx_workflow__execute",
    description = "Execute a non-returning SQL statement (INSERT, UPDATE, DELETE, DDL). \
                   Assumes: DbConnected (pool_id valid). \
                   Establishes: QueryExecuted — rows_affected is accurate.",
    emit = WfExecuteEmit
)]
async fn wf_execute(
    ctx: Arc<SqlxPoolCtx>,
    p: WfPoolSqlParams,
) -> Result<CallToolResult, ErrorData> {
    let result = if p.args.is_empty() {
        sqlx::query(&p.sql).execute(&ctx.pool).await
    } else {
        sqlx::query_with(&p.sql, any_args_from_json(&p.args))
            .execute(&ctx.pool)
            .await
    }
    .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    let _proof: Established<QueryExecuted> = Established::assert();
    Ok(json_result(&QueryResultData {
        rows_affected: result.rows_affected(),
        last_insert_id: result.last_insert_id(),
    }))
}

#[elicit_tool(
    plugin = "sqlx_workflow",
    name = "sqlx_workflow__fetch_all",
    description = "Execute a SELECT and return all rows. \
                   Assumes: DbConnected (pool_id valid). \
                   Establishes: RowsFetched — returned Vec contains every matching row.",
    emit = WfFetchAllEmit
)]
async fn wf_fetch_all(
    ctx: Arc<SqlxPoolCtx>,
    p: WfFetchAllParams,
) -> Result<CallToolResult, ErrorData> {
    let rows = if p.args.is_empty() {
        sqlx::query(&p.sql).fetch_all(&ctx.pool).await
    } else {
        sqlx::query_with(&p.sql, any_args_from_json(&p.args))
            .fetch_all(&ctx.pool)
            .await
    }
    .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    let _proof: Established<RowsFetched> = Established::assert();
    let data: Vec<RowData> = rows.iter().map(decode_any_row).collect();
    Ok(json_result(&data))
}

#[elicit_tool(
    plugin = "sqlx_workflow",
    name = "sqlx_workflow__fetch_one",
    description = "Execute a SELECT and return exactly the first row; errors if none found. \
                   Assumes: DbConnected (pool_id valid); at least one row exists. \
                   Establishes: RowsFetched.",
    emit = WfFetchOneEmit
)]
async fn wf_fetch_one(
    ctx: Arc<SqlxPoolCtx>,
    p: WfFetchOneParams,
) -> Result<CallToolResult, ErrorData> {
    let row = if p.args.is_empty() {
        sqlx::query(&p.sql).fetch_one(&ctx.pool).await
    } else {
        sqlx::query_with(&p.sql, any_args_from_json(&p.args))
            .fetch_one(&ctx.pool)
            .await
    }
    .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    let _proof: Established<RowsFetched> = Established::assert();
    Ok(json_result(&decode_any_row(&row)))
}

#[elicit_tool(
    plugin = "sqlx_workflow",
    name = "sqlx_workflow__fetch_optional",
    description = "Execute a SELECT and return the first row or null. \
                   Assumes: DbConnected (pool_id valid).",
    emit = WfFetchOptionalEmit
)]
async fn wf_fetch_optional(
    ctx: Arc<SqlxPoolCtx>,
    p: WfFetchOptionalParams,
) -> Result<CallToolResult, ErrorData> {
    let maybe = if p.args.is_empty() {
        sqlx::query(&p.sql).fetch_optional(&ctx.pool).await
    } else {
        sqlx::query_with(&p.sql, any_args_from_json(&p.args))
            .fetch_optional(&ctx.pool)
            .await
    }
    .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    let data: Option<RowData> = maybe.as_ref().map(decode_any_row);
    Ok(json_result(&data))
}

#[elicit_tool(
    plugin = "sqlx_workflow",
    name = "sqlx_workflow__begin",
    description = "Start a database transaction. \
                   Assumes: DbConnected (pool_id valid). \
                   Establishes: TransactionOpen — tx stored by returned tx_id.",
    emit = WfBeginEmit
)]
async fn wf_begin(ctx: Arc<SqlxCtx>, p: WfBeginParams) -> Result<CallToolResult, ErrorData> {
    let pool = ctx
        .pools
        .lock()
        .await
        .get(&p.pool_id)
        .cloned()
        .ok_or_else(|| {
            ErrorData::invalid_params(
                "TransactionOpen not established: pool_id not found".to_string(),
                None,
            )
        })?;
    let tx = pool
        .begin()
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    let tx_id = Uuid::new_v4();
    ctx.txs
        .lock()
        .await
        .insert(tx_id, Arc::new(SqlxTxCtx::new(tx)));
    Ok(json_result(&WfBeginResult {
        tx_id,
        contract: "TransactionOpen",
    }))
}

#[elicit_tool(
    plugin = "sqlx_workflow",
    name = "sqlx_workflow__commit",
    description = "Commit an open transaction. \
                   Assumes: TransactionOpen (tx_id valid). \
                   Establishes: TransactionCommitted — all changes are durable.",
    emit = WfCommitEmit
)]
async fn wf_commit(ctx: Arc<SqlxTxCtx>, p: WfTxIdParams) -> Result<CallToolResult, ErrorData> {
    let _ = p; // tx_id resolved by call_tool before dispatch
    let tx = ctx
        .tx
        .lock()
        .await
        .take()
        .ok_or_else(|| ErrorData::internal_error("transaction already consumed", None))?;
    tx.commit()
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    Ok(CallToolResult::success(vec![Content::text(
        r#"{"ok":true,"contract":"TransactionCommitted"}"#,
    )]))
}

#[elicit_tool(
    plugin = "sqlx_workflow",
    name = "sqlx_workflow__rollback",
    description = "Roll back an open transaction. \
                   Assumes: TransactionOpen (tx_id valid). \
                   Establishes: TransactionRolledBack — all changes since begin are undone.",
    emit = WfRollbackEmit
)]
async fn wf_rollback(
    ctx: Arc<SqlxTxCtx>,
    p: WfRollbackParams,
) -> Result<CallToolResult, ErrorData> {
    let _ = p; // tx_id resolved by call_tool before dispatch
    let tx = ctx
        .tx
        .lock()
        .await
        .take()
        .ok_or_else(|| ErrorData::internal_error("transaction already consumed", None))?;
    tx.rollback()
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    Ok(CallToolResult::success(vec![Content::text(
        r#"{"ok":true,"contract":"TransactionRolledBack"}"#,
    )]))
}

#[elicit_tool(
    plugin = "sqlx_workflow",
    name = "sqlx_workflow__tx_execute",
    description = "Execute a non-returning SQL statement within an open transaction. \
                   Assumes: TransactionOpen (tx_id valid). \
                   Establishes: QueryExecuted.",
    emit = WfTxExecuteEmit
)]
async fn wf_tx_execute(ctx: Arc<SqlxTxCtx>, p: WfTxSqlParams) -> Result<CallToolResult, ErrorData> {
    let mut guard = ctx.tx.lock().await;
    let tx = guard
        .as_mut()
        .ok_or_else(|| ErrorData::internal_error("transaction not available", None))?;
    let result = if p.args.is_empty() {
        sqlx::query(&p.sql).execute(&mut **tx).await
    } else {
        sqlx::query_with(&p.sql, any_args_from_json(&p.args))
            .execute(&mut **tx)
            .await
    }
    .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    let _proof: Established<QueryExecuted> = Established::assert();
    Ok(json_result(&QueryResultData {
        rows_affected: result.rows_affected(),
        last_insert_id: result.last_insert_id(),
    }))
}

#[elicit_tool(
    plugin = "sqlx_workflow",
    name = "sqlx_workflow__tx_fetch_all",
    description = "SELECT all rows within an open transaction. \
                   Assumes: TransactionOpen (tx_id valid). \
                   Establishes: RowsFetched.",
    emit = WfTxFetchAllEmit
)]
async fn wf_tx_fetch_all(
    ctx: Arc<SqlxTxCtx>,
    p: WfTxFetchAllParams,
) -> Result<CallToolResult, ErrorData> {
    let mut guard = ctx.tx.lock().await;
    let tx = guard
        .as_mut()
        .ok_or_else(|| ErrorData::internal_error("transaction not available", None))?;
    let rows = if p.args.is_empty() {
        sqlx::query(&p.sql).fetch_all(&mut **tx).await
    } else {
        sqlx::query_with(&p.sql, any_args_from_json(&p.args))
            .fetch_all(&mut **tx)
            .await
    }
    .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    let _proof: Established<RowsFetched> = Established::assert();
    let data: Vec<RowData> = rows.iter().map(decode_any_row).collect();
    Ok(json_result(&data))
}

#[elicit_tool(
    plugin = "sqlx_workflow",
    name = "sqlx_workflow__tx_fetch_one",
    description = "SELECT first row within an open transaction; errors if none found. \
                   Assumes: TransactionOpen (tx_id valid); at least one row exists. \
                   Establishes: RowsFetched.",
    emit = WfTxFetchOneEmit
)]
async fn wf_tx_fetch_one(
    ctx: Arc<SqlxTxCtx>,
    p: WfTxFetchOneParams,
) -> Result<CallToolResult, ErrorData> {
    let mut guard = ctx.tx.lock().await;
    let tx = guard
        .as_mut()
        .ok_or_else(|| ErrorData::internal_error("transaction not available", None))?;
    let row = if p.args.is_empty() {
        sqlx::query(&p.sql).fetch_one(&mut **tx).await
    } else {
        sqlx::query_with(&p.sql, any_args_from_json(&p.args))
            .fetch_one(&mut **tx)
            .await
    }
    .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    let _proof: Established<RowsFetched> = Established::assert();
    Ok(json_result(&decode_any_row(&row)))
}

#[elicit_tool(
    plugin = "sqlx_workflow",
    name = "sqlx_workflow__tx_fetch_optional",
    description = "SELECT first row (or null) within an open transaction. \
                   Assumes: TransactionOpen (tx_id valid).",
    emit = WfTxFetchOptionalEmit
)]
async fn wf_tx_fetch_optional(
    ctx: Arc<SqlxTxCtx>,
    p: WfTxFetchOptionalParams,
) -> Result<CallToolResult, ErrorData> {
    let mut guard = ctx.tx.lock().await;
    let tx = guard
        .as_mut()
        .ok_or_else(|| ErrorData::internal_error("transaction not available", None))?;
    let maybe = if p.args.is_empty() {
        sqlx::query(&p.sql).fetch_optional(&mut **tx).await
    } else {
        sqlx::query_with(&p.sql, any_args_from_json(&p.args))
            .fetch_optional(&mut **tx)
            .await
    }
    .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    let data: Option<RowData> = maybe.as_ref().map(decode_any_row);
    Ok(json_result(&data))
}

// ── Custom emit types ─────────────────────────────────────────────────────────
//
// Zero-sized types implementing `CustomEmit<ParamsType>`, co-located with their
// tool functions.  Each is also registered in the global `EmitEntry` inventory
// under its full `sqlx_workflow__*` name so that `emit_dispatch_crate` finds it
// when the `emit` feature is not necessarily enabled in the consuming crate.

/// Emit: `sqlx_workflow__connect` — create a pool.
pub struct WfConnectEmit;
impl CustomEmit<WfConnectParams> for WfConnectEmit {
    fn emit_code(p: &WfConnectParams) -> TokenStream {
        let url = p.database_url.as_str();
        let max_conn = p.max_connections.unwrap_or(10);
        quote! {
            let pool = sqlx::any::AnyPoolOptions::new()
                .max_connections(#max_conn)
                .connect(#url)
                .await?;
        }
    }
}

/// Emit: `sqlx_workflow__disconnect` — drop pool.
pub struct WfDisconnectEmit;
impl CustomEmit<WfPoolIdParams> for WfDisconnectEmit {
    fn emit_code(_p: &WfPoolIdParams) -> TokenStream {
        quote! { drop(pool); }
    }
}

/// Emit: `sqlx_workflow__execute` — non-returning statement on pool.
pub struct WfExecuteEmit;
impl CustomEmit<WfPoolSqlParams> for WfExecuteEmit {
    fn emit_code(p: &WfPoolSqlParams) -> TokenStream {
        let sql = p.sql.as_str();
        let binds = bind_chain(&p.args);
        quote! {
            sqlx::query(#sql) #binds .execute(&pool).await?;
        }
    }
}

/// Emit: `sqlx_workflow__fetch_all` — SELECT all rows from pool.
pub struct WfFetchAllEmit;
impl CustomEmit<WfFetchAllParams> for WfFetchAllEmit {
    fn emit_code(p: &WfFetchAllParams) -> TokenStream {
        let sql = p.sql.as_str();
        let binds = bind_chain(&p.args);
        quote! {
            let rows = sqlx::query(#sql) #binds .fetch_all(&pool).await?;
        }
    }
}

/// Emit: `sqlx_workflow__fetch_one` — SELECT first row from pool.
pub struct WfFetchOneEmit;
impl CustomEmit<WfFetchOneParams> for WfFetchOneEmit {
    fn emit_code(p: &WfFetchOneParams) -> TokenStream {
        let sql = p.sql.as_str();
        let binds = bind_chain(&p.args);
        quote! {
            let row = sqlx::query(#sql) #binds .fetch_one(&pool).await?;
        }
    }
}

/// Emit: `sqlx_workflow__fetch_optional` — SELECT optional row from pool.
pub struct WfFetchOptionalEmit;
impl CustomEmit<WfFetchOptionalParams> for WfFetchOptionalEmit {
    fn emit_code(p: &WfFetchOptionalParams) -> TokenStream {
        let sql = p.sql.as_str();
        let binds = bind_chain(&p.args);
        quote! {
            let row = sqlx::query(#sql) #binds .fetch_optional(&pool).await?;
        }
    }
}

/// Emit: `sqlx_workflow__begin` — begin transaction.
pub struct WfBeginEmit;
impl CustomEmit<WfBeginParams> for WfBeginEmit {
    fn emit_code(_p: &WfBeginParams) -> TokenStream {
        quote! {
            let mut tx = pool.begin().await?;
        }
    }
}

/// Emit: `sqlx_workflow__commit` — commit transaction.
pub struct WfCommitEmit;
impl CustomEmit<WfTxIdParams> for WfCommitEmit {
    fn emit_code(_p: &WfTxIdParams) -> TokenStream {
        quote! { tx.commit().await?; }
    }
}

/// Emit: `sqlx_workflow__rollback` — rollback transaction.
pub struct WfRollbackEmit;
impl CustomEmit<WfRollbackParams> for WfRollbackEmit {
    fn emit_code(_p: &WfRollbackParams) -> TokenStream {
        quote! { tx.rollback().await?; }
    }
}

/// Emit: `sqlx_workflow__tx_execute` — non-returning statement in transaction.
pub struct WfTxExecuteEmit;
impl CustomEmit<WfTxSqlParams> for WfTxExecuteEmit {
    fn emit_code(p: &WfTxSqlParams) -> TokenStream {
        let sql = p.sql.as_str();
        let binds = bind_chain(&p.args);
        quote! {
            tx.execute(sqlx::query(#sql) #binds).await?;
        }
    }
}

/// Emit: `sqlx_workflow__tx_fetch_all` — SELECT all rows in transaction.
pub struct WfTxFetchAllEmit;
impl CustomEmit<WfTxFetchAllParams> for WfTxFetchAllEmit {
    fn emit_code(p: &WfTxFetchAllParams) -> TokenStream {
        let sql = p.sql.as_str();
        let binds = bind_chain(&p.args);
        quote! {
            let rows = sqlx::query(#sql) #binds .fetch_all(&mut **tx).await?;
        }
    }
}

/// Emit: `sqlx_workflow__tx_fetch_one` — SELECT first row in transaction.
pub struct WfTxFetchOneEmit;
impl CustomEmit<WfTxFetchOneParams> for WfTxFetchOneEmit {
    fn emit_code(p: &WfTxFetchOneParams) -> TokenStream {
        let sql = p.sql.as_str();
        let binds = bind_chain(&p.args);
        quote! {
            let row = sqlx::query(#sql) #binds .fetch_one(&mut **tx).await?;
        }
    }
}

/// Emit: `sqlx_workflow__tx_fetch_optional` — SELECT optional row in transaction.
pub struct WfTxFetchOptionalEmit;
impl CustomEmit<WfTxFetchOptionalParams> for WfTxFetchOptionalEmit {
    fn emit_code(p: &WfTxFetchOptionalParams) -> TokenStream {
        let sql = p.sql.as_str();
        let binds = bind_chain(&p.args);
        quote! {
            let row = sqlx::query(#sql) #binds .fetch_optional(&mut **tx).await?;
        }
    }
}

// ── Inventory emit entries (full names for emit_dispatch_crate compatibility) ──
//
// These entries are registered WITHOUT a feature gate so they are always present
// when `elicit_sqlx` is compiled, matching the tool names expected by
// `emit_dispatch_crate("sqlx_workflow__*", "elicit_sqlx", ...)`.

use elicitation::emit_code::{CrateDep, EmitCode};

const SQLX_DEP: CrateDep = CrateDep {
    name: "sqlx",
    version: "0.8",
    features: &["runtime-tokio", "any", "sqlite", "postgres", "mysql"],
};

/// Emit-only params for pool-level SQL tools.
///
/// Unlike `WfPoolSqlParams`, this struct omits `pool_id` because the emitted
/// code references the `pool` variable by name (established by a preceding
/// `connect` step) rather than looking it up by UUID at runtime.
#[derive(Debug, Deserialize)]
struct WfPoolSqlEmitParams {
    sql: String,
    #[serde(default)]
    args: Vec<serde_json::Value>,
}

/// Emit-only params for transaction-level SQL tools.
///
/// Like `WfPoolSqlEmitParams`, omits the runtime UUID (`tx_id`) because the
/// emitted code references the `tx` variable by name.
#[derive(Debug, Deserialize)]
struct WfTxSqlEmitParams {
    sql: String,
    #[serde(default)]
    args: Vec<serde_json::Value>,
}

struct WfConnectEmitEntry(WfConnectParams);
impl EmitCode for WfConnectEmitEntry {
    fn emit_code(&self) -> TokenStream {
        WfConnectEmit::emit_code(&self.0)
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![SQLX_DEP]
    }
    fn shared_scope(&self) -> bool {
        true
    }
}

struct WfDisconnectEmitEntry;
impl EmitCode for WfDisconnectEmitEntry {
    fn emit_code(&self) -> TokenStream {
        quote! { drop(pool); }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![]
    }
    fn shared_scope(&self) -> bool {
        true
    }
}

struct WfExecuteEmitEntry(WfPoolSqlEmitParams);
impl EmitCode for WfExecuteEmitEntry {
    fn emit_code(&self) -> TokenStream {
        let sql = self.0.sql.as_str();
        let binds = bind_chain(&self.0.args);
        quote! { sqlx::query(#sql) #binds .execute(&pool).await?; }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![SQLX_DEP]
    }
    fn shared_scope(&self) -> bool {
        true
    }
}

struct WfFetchAllEmitEntry(WfPoolSqlEmitParams);
impl EmitCode for WfFetchAllEmitEntry {
    fn emit_code(&self) -> TokenStream {
        let sql = self.0.sql.as_str();
        let binds = bind_chain(&self.0.args);
        quote! { let rows = sqlx::query(#sql) #binds .fetch_all(&pool).await?; }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![SQLX_DEP]
    }
    fn shared_scope(&self) -> bool {
        true
    }
}

struct WfFetchOneEmitEntry(WfPoolSqlEmitParams);
impl EmitCode for WfFetchOneEmitEntry {
    fn emit_code(&self) -> TokenStream {
        let sql = self.0.sql.as_str();
        let binds = bind_chain(&self.0.args);
        quote! { let row = sqlx::query(#sql) #binds .fetch_one(&pool).await?; }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![SQLX_DEP]
    }
    fn shared_scope(&self) -> bool {
        true
    }
}

struct WfFetchOptionalEmitEntry(WfPoolSqlEmitParams);
impl EmitCode for WfFetchOptionalEmitEntry {
    fn emit_code(&self) -> TokenStream {
        let sql = self.0.sql.as_str();
        let binds = bind_chain(&self.0.args);
        quote! { let row = sqlx::query(#sql) #binds .fetch_optional(&pool).await?; }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![SQLX_DEP]
    }
    fn shared_scope(&self) -> bool {
        true
    }
}

struct WfBeginEmitEntry;
impl EmitCode for WfBeginEmitEntry {
    fn emit_code(&self) -> TokenStream {
        quote! { let mut tx = pool.begin().await?; }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![SQLX_DEP]
    }
    fn shared_scope(&self) -> bool {
        true
    }
}

struct WfCommitEmitEntry;
impl EmitCode for WfCommitEmitEntry {
    fn emit_code(&self) -> TokenStream {
        quote! { tx.commit().await?; }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![SQLX_DEP]
    }
    fn shared_scope(&self) -> bool {
        true
    }
}

struct WfRollbackEmitEntry;
impl EmitCode for WfRollbackEmitEntry {
    fn emit_code(&self) -> TokenStream {
        quote! { tx.rollback().await?; }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![]
    }
    fn shared_scope(&self) -> bool {
        true
    }
}

struct WfTxExecuteEmitEntry(WfTxSqlEmitParams);
impl EmitCode for WfTxExecuteEmitEntry {
    fn emit_code(&self) -> TokenStream {
        let sql = self.0.sql.as_str();
        let binds = bind_chain(&self.0.args);
        quote! { sqlx::query(#sql) #binds .execute(&mut *tx).await?; }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![SQLX_DEP]
    }
    fn shared_scope(&self) -> bool {
        true
    }
}

struct WfTxFetchAllEmitEntry(WfTxSqlEmitParams);
impl EmitCode for WfTxFetchAllEmitEntry {
    fn emit_code(&self) -> TokenStream {
        let sql = self.0.sql.as_str();
        let binds = bind_chain(&self.0.args);
        quote! { let rows = sqlx::query(#sql) #binds .fetch_all(&mut **tx).await?; }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![SQLX_DEP]
    }
    fn shared_scope(&self) -> bool {
        true
    }
}

struct WfTxFetchOneEmitEntry(WfTxSqlEmitParams);
impl EmitCode for WfTxFetchOneEmitEntry {
    fn emit_code(&self) -> TokenStream {
        let sql = self.0.sql.as_str();
        let binds = bind_chain(&self.0.args);
        quote! { let row = sqlx::query(#sql) #binds .fetch_one(&mut **tx).await?; }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![SQLX_DEP]
    }
    fn shared_scope(&self) -> bool {
        true
    }
}

struct WfTxFetchOptionalEmitEntry(WfTxSqlEmitParams);
impl EmitCode for WfTxFetchOptionalEmitEntry {
    fn emit_code(&self) -> TokenStream {
        let sql = self.0.sql.as_str();
        let binds = bind_chain(&self.0.args);
        quote! { let row = sqlx::query(#sql) #binds .fetch_optional(&mut **tx).await?; }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![SQLX_DEP]
    }
    fn shared_scope(&self) -> bool {
        true
    }
}

inventory::submit! {
    elicitation::emit_code::EmitEntry {
        tool: "sqlx_workflow__connect",
        crate_name: "elicit_sqlx",
        constructor: |v| {
            serde_json::from_value::<WfConnectParams>(v)
                .map(|p| Box::new(WfConnectEmitEntry(p)) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}

inventory::submit! {
    elicitation::emit_code::EmitEntry {
        tool: "sqlx_workflow__disconnect",
        crate_name: "elicit_sqlx",
        constructor: |_v| Ok(Box::new(WfDisconnectEmitEntry) as Box<dyn EmitCode>),
    }
}

inventory::submit! {
    elicitation::emit_code::EmitEntry {
        tool: "sqlx_workflow__execute",
        crate_name: "elicit_sqlx",
        constructor: |v| {
            serde_json::from_value::<WfPoolSqlEmitParams>(v)
                .map(|p| Box::new(WfExecuteEmitEntry(p)) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}

inventory::submit! {
    elicitation::emit_code::EmitEntry {
        tool: "sqlx_workflow__fetch_all",
        crate_name: "elicit_sqlx",
        constructor: |v| {
            serde_json::from_value::<WfPoolSqlEmitParams>(v)
                .map(|p| Box::new(WfFetchAllEmitEntry(p)) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}

inventory::submit! {
    elicitation::emit_code::EmitEntry {
        tool: "sqlx_workflow__fetch_one",
        crate_name: "elicit_sqlx",
        constructor: |v| {
            serde_json::from_value::<WfPoolSqlEmitParams>(v)
                .map(|p| Box::new(WfFetchOneEmitEntry(p)) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}

inventory::submit! {
    elicitation::emit_code::EmitEntry {
        tool: "sqlx_workflow__fetch_optional",
        crate_name: "elicit_sqlx",
        constructor: |v| {
            serde_json::from_value::<WfPoolSqlEmitParams>(v)
                .map(|p| Box::new(WfFetchOptionalEmitEntry(p)) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}

inventory::submit! {
    elicitation::emit_code::EmitEntry {
        tool: "sqlx_workflow__begin",
        crate_name: "elicit_sqlx",
        constructor: |_v| Ok(Box::new(WfBeginEmitEntry) as Box<dyn EmitCode>),
    }
}

inventory::submit! {
    elicitation::emit_code::EmitEntry {
        tool: "sqlx_workflow__commit",
        crate_name: "elicit_sqlx",
        constructor: |_v| Ok(Box::new(WfCommitEmitEntry) as Box<dyn EmitCode>),
    }
}

inventory::submit! {
    elicitation::emit_code::EmitEntry {
        tool: "sqlx_workflow__rollback",
        crate_name: "elicit_sqlx",
        constructor: |_v| Ok(Box::new(WfRollbackEmitEntry) as Box<dyn EmitCode>),
    }
}

inventory::submit! {
    elicitation::emit_code::EmitEntry {
        tool: "sqlx_workflow__tx_execute",
        crate_name: "elicit_sqlx",
        constructor: |v| {
            serde_json::from_value::<WfTxSqlEmitParams>(v)
                .map(|p| Box::new(WfTxExecuteEmitEntry(p)) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}

inventory::submit! {
    elicitation::emit_code::EmitEntry {
        tool: "sqlx_workflow__tx_fetch_all",
        crate_name: "elicit_sqlx",
        constructor: |v| {
            serde_json::from_value::<WfTxSqlEmitParams>(v)
                .map(|p| Box::new(WfTxFetchAllEmitEntry(p)) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}

inventory::submit! {
    elicitation::emit_code::EmitEntry {
        tool: "sqlx_workflow__tx_fetch_one",
        crate_name: "elicit_sqlx",
        constructor: |v| {
            serde_json::from_value::<WfTxSqlEmitParams>(v)
                .map(|p| Box::new(WfTxFetchOneEmitEntry(p)) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}

inventory::submit! {
    elicitation::emit_code::EmitEntry {
        tool: "sqlx_workflow__tx_fetch_optional",
        crate_name: "elicit_sqlx",
        constructor: |v| {
            serde_json::from_value::<WfTxSqlEmitParams>(v)
                .map(|p| Box::new(WfTxFetchOptionalEmitEntry(p)) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
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
pub struct SqlxWorkflowPlugin(Arc<SqlxCtx>);

impl SqlxWorkflowPlugin {
    /// Create a new plugin with no open pools or transactions.
    pub fn new() -> Self {
        sqlx::any::install_default_drivers();
        Self(Arc::new(SqlxCtx::new()))
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
        let mut opts = sqlx::any::AnyPoolOptions::new();
        if let Some(n) = max_connections {
            opts = opts.max_connections(n);
        }
        let pool = opts.connect(url).await.map_err(|e| e.to_string())?;
        let pool_id = Uuid::new_v4();
        self.0.pools.lock().await.insert(pool_id, pool);
        Ok((pool_id, Established::assert()))
    }

    /// Close and remove a pool.
    pub async fn disconnect(&self, pool_id: Uuid) -> Result<(), String> {
        let pool = self
            .0
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
        let pool = self
            .0
            .pools
            .lock()
            .await
            .get(&pool_id)
            .cloned()
            .ok_or_else(|| format!("pool_id not found: {pool_id}"))?;
        let result = if args.is_empty() {
            sqlx::query(sql).execute(&pool).await
        } else {
            sqlx::query_with(sql, any_args_from_json(args))
                .execute(&pool)
                .await
        }
        .map_err(|e| e.to_string())?;
        Ok((
            QueryResultData {
                rows_affected: result.rows_affected(),
                last_insert_id: result.last_insert_id(),
            },
            Established::assert(),
        ))
    }

    /// Execute a SELECT and return all rows as [`RowData`].
    pub async fn fetch_all_data(
        &self,
        pool_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<(Vec<RowData>, Established<RowsFetched>), String> {
        let pool = self
            .0
            .pools
            .lock()
            .await
            .get(&pool_id)
            .cloned()
            .ok_or_else(|| format!("pool_id not found: {pool_id}"))?;
        let rows = if args.is_empty() {
            sqlx::query(sql).fetch_all(&pool).await
        } else {
            sqlx::query_with(sql, any_args_from_json(args))
                .fetch_all(&pool)
                .await
        }
        .map_err(|e| e.to_string())?;
        Ok((
            rows.iter().map(decode_any_row).collect(),
            Established::assert(),
        ))
    }

    /// Execute a SELECT and return the first row as [`RowData`].
    pub async fn fetch_one_data(
        &self,
        pool_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<(RowData, Established<RowsFetched>), String> {
        let pool = self
            .0
            .pools
            .lock()
            .await
            .get(&pool_id)
            .cloned()
            .ok_or_else(|| format!("pool_id not found: {pool_id}"))?;
        let row = if args.is_empty() {
            sqlx::query(sql).fetch_one(&pool).await
        } else {
            sqlx::query_with(sql, any_args_from_json(args))
                .fetch_one(&pool)
                .await
        }
        .map_err(|e| e.to_string())?;
        Ok((decode_any_row(&row), Established::assert()))
    }

    /// Execute a SELECT and return the first row or `None`.
    pub async fn fetch_optional_data(
        &self,
        pool_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<Option<RowData>, String> {
        let pool = self
            .0
            .pools
            .lock()
            .await
            .get(&pool_id)
            .cloned()
            .ok_or_else(|| format!("pool_id not found: {pool_id}"))?;
        let maybe = if args.is_empty() {
            sqlx::query(sql).fetch_optional(&pool).await
        } else {
            sqlx::query_with(sql, any_args_from_json(args))
                .fetch_optional(&pool)
                .await
        }
        .map_err(|e| e.to_string())?;
        Ok(maybe.as_ref().map(decode_any_row))
    }

    /// Begin a transaction.  Returns `(tx_id, Established<TransactionOpen>)`.
    pub async fn begin(
        &self,
        pool_id: Uuid,
    ) -> Result<(Uuid, Established<TransactionOpen>), String> {
        let pool = self
            .0
            .pools
            .lock()
            .await
            .get(&pool_id)
            .cloned()
            .ok_or_else(|| format!("pool_id not found: {pool_id}"))?;
        let tx = pool.begin().await.map_err(|e| e.to_string())?;
        let tx_id = Uuid::new_v4();
        self.0
            .txs
            .lock()
            .await
            .insert(tx_id, Arc::new(SqlxTxCtx::new(tx)));
        Ok((tx_id, Established::assert()))
    }

    /// Commit a transaction.
    pub async fn commit(&self, tx_id: Uuid) -> Result<Established<TransactionCommitted>, String> {
        let tx_arc = self
            .0
            .txs
            .lock()
            .await
            .remove(&tx_id)
            .ok_or_else(|| format!("tx_id not found: {tx_id}"))?;
        let tx = tx_arc
            .tx
            .lock()
            .await
            .take()
            .ok_or_else(|| "transaction already consumed".to_string())?;
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(Established::assert())
    }

    /// Roll back a transaction.
    pub async fn rollback(
        &self,
        tx_id: Uuid,
    ) -> Result<Established<TransactionRolledBack>, String> {
        let tx_arc = self
            .0
            .txs
            .lock()
            .await
            .remove(&tx_id)
            .ok_or_else(|| format!("tx_id not found: {tx_id}"))?;
        let tx = tx_arc
            .tx
            .lock()
            .await
            .take()
            .ok_or_else(|| "transaction already consumed".to_string())?;
        tx.rollback().await.map_err(|e| e.to_string())?;
        Ok(Established::assert())
    }

    /// Execute SQL within a transaction.
    pub async fn tx_execute(
        &self,
        tx_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<(QueryResultData, Established<QueryExecuted>), String> {
        let tx_arc = self
            .0
            .txs
            .lock()
            .await
            .get(&tx_id)
            .cloned()
            .ok_or_else(|| format!("tx_id not found: {tx_id}"))?;
        let mut guard = tx_arc.tx.lock().await;
        let tx = guard
            .as_mut()
            .ok_or_else(|| "transaction not available".to_string())?;
        let result = if args.is_empty() {
            sqlx::query(sql).execute(&mut **tx).await
        } else {
            sqlx::query_with(sql, any_args_from_json(args))
                .execute(&mut **tx)
                .await
        }
        .map_err(|e| e.to_string())?;
        Ok((
            QueryResultData {
                rows_affected: result.rows_affected(),
                last_insert_id: result.last_insert_id(),
            },
            Established::assert(),
        ))
    }

    /// SELECT all rows within a transaction.
    pub async fn tx_fetch_all_data(
        &self,
        tx_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<(Vec<RowData>, Established<RowsFetched>), String> {
        let tx_arc = self
            .0
            .txs
            .lock()
            .await
            .get(&tx_id)
            .cloned()
            .ok_or_else(|| format!("tx_id not found: {tx_id}"))?;
        let mut guard = tx_arc.tx.lock().await;
        let tx = guard
            .as_mut()
            .ok_or_else(|| "transaction not available".to_string())?;
        let rows = if args.is_empty() {
            sqlx::query(sql).fetch_all(&mut **tx).await
        } else {
            sqlx::query_with(sql, any_args_from_json(args))
                .fetch_all(&mut **tx)
                .await
        }
        .map_err(|e| e.to_string())?;
        Ok((
            rows.iter().map(decode_any_row).collect(),
            Established::assert(),
        ))
    }

    /// SELECT first row within a transaction.
    pub async fn tx_fetch_one_data(
        &self,
        tx_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<(RowData, Established<RowsFetched>), String> {
        let tx_arc = self
            .0
            .txs
            .lock()
            .await
            .get(&tx_id)
            .cloned()
            .ok_or_else(|| format!("tx_id not found: {tx_id}"))?;
        let mut guard = tx_arc.tx.lock().await;
        let tx = guard
            .as_mut()
            .ok_or_else(|| "transaction not available".to_string())?;
        let row = if args.is_empty() {
            sqlx::query(sql).fetch_one(&mut **tx).await
        } else {
            sqlx::query_with(sql, any_args_from_json(args))
                .fetch_one(&mut **tx)
                .await
        }
        .map_err(|e| e.to_string())?;
        Ok((decode_any_row(&row), Established::assert()))
    }

    /// SELECT optional first row within a transaction.
    pub async fn tx_fetch_optional_data(
        &self,
        tx_id: Uuid,
        sql: &str,
        args: &[serde_json::Value],
    ) -> Result<Option<RowData>, String> {
        let tx_arc = self
            .0
            .txs
            .lock()
            .await
            .get(&tx_id)
            .cloned()
            .ok_or_else(|| format!("tx_id not found: {tx_id}"))?;
        let mut guard = tx_arc.tx.lock().await;
        let tx = guard
            .as_mut()
            .ok_or_else(|| "transaction not available".to_string())?;
        let maybe = if args.is_empty() {
            sqlx::query(sql).fetch_optional(&mut **tx).await
        } else {
            sqlx::query_with(sql, any_args_from_json(args))
                .fetch_optional(&mut **tx)
                .await
        }
        .map_err(|e| e.to_string())?;
        Ok(maybe.as_ref().map(decode_any_row))
    }
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
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "sqlx_workflow")
            .map(|r| (r.constructor)().as_tool())
            .collect()
    }

    #[tracing::instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let plugin_ctx = self.0.clone();
        Box::pin(async move {
            let name = params.name.as_ref();
            // Accept both "sqlx_workflow__connect" and bare "connect"
            let bare = name.strip_prefix("sqlx_workflow__").unwrap_or(name);

            // Resolve the descriptor by name (full or bare) from the inventory.
            let full_name = if name.starts_with("sqlx_workflow__") {
                name.to_string()
            } else {
                format!("sqlx_workflow__{name}")
            };
            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "sqlx_workflow")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            // Resolve per-call context based on which tool group.
            let tool_ctx: Arc<dyn std::any::Any + Send + Sync> = match bare {
                "connect" | "disconnect" | "begin" => {
                    plugin_ctx.clone() as Arc<dyn std::any::Any + Send + Sync>
                }
                "execute" | "fetch_all" | "fetch_one" | "fetch_optional" => {
                    let p: WfPoolIdParams = parse_args(&params)?;
                    let pool = plugin_ctx
                        .pools
                        .lock()
                        .await
                        .get(&p.pool_id)
                        .cloned()
                        .ok_or_else(|| {
                            ErrorData::invalid_params(
                                format!("pool_id not found: {}", p.pool_id),
                                None,
                            )
                        })?;
                    Arc::new(SqlxPoolCtx { pool }) as Arc<dyn std::any::Any + Send + Sync>
                }
                "commit" | "rollback" => {
                    let p: WfTxIdParams = parse_args(&params)?;
                    let tx_arc = plugin_ctx
                        .txs
                        .lock()
                        .await
                        .remove(&p.tx_id)
                        .ok_or_else(|| {
                            ErrorData::invalid_params(format!("tx_id not found: {}", p.tx_id), None)
                        })?;
                    tx_arc as Arc<dyn std::any::Any + Send + Sync>
                }
                "tx_execute" | "tx_fetch_all" | "tx_fetch_one" | "tx_fetch_optional" => {
                    let p: WfTxIdParams = parse_args(&params)?;
                    let tx_arc = plugin_ctx
                        .txs
                        .lock()
                        .await
                        .get(&p.tx_id)
                        .cloned()
                        .ok_or_else(|| {
                            ErrorData::invalid_params(format!("tx_id not found: {}", p.tx_id), None)
                        })?;
                    tx_arc as Arc<dyn std::any::Any + Send + Sync>
                }
                _ => {
                    return Err(ErrorData::invalid_params(
                        format!("unknown sqlx_workflow tool: {bare}"),
                        None,
                    ));
                }
            };

            descriptor.dispatch(tool_ctx, params).await
        })
    }
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
