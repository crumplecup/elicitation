//! `PolarsSqlPlugin` — MCP tools for polars `SQLContext` runtime interface.
//!
//! SQL contexts are stored server-side in a UUID-keyed registry. DataFrames
//! are shared via `SharedDfRegistry` from `PolarsDataFramePlugin`. The
//! `sql__register` tool looks up a DataFrame by UUID and registers it as a
//! LazyFrame table in a SQLContext.
//!
//! # Tool namespace: `polars_sql__*`

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::contracts::{Established, Prop};
use elicitation::{Elicit, PluginContext, VerifiedWorkflow};
use futures::future::BoxFuture;
use polars::prelude::IntoLazy;
use polars::sql::SQLContext;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::instrument;
use uuid::Uuid;

use crate::dataframe::SharedDfRegistry;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a polars `SQLContext` was successfully created and registered.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct PolarsSqlCreated;

impl Prop for PolarsSqlCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_polars_sql_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "polars sql context created");
            }
        }
    }

    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_polars_sql_created(ok: bool) -> (result: bool)
                ensures result == ok,
            { ok }
            }
        }
    }

    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_polars_sql_created_contract() -> bool { true }
        }
    }
}

impl VerifiedWorkflow for PolarsSqlCreated {}

// ── Plugin context ─────────────────────────────────────────────────────────────

/// Shared state for all `polars_sql__*` tool calls.
pub struct PolarsSqlCtx {
    contexts: Mutex<HashMap<Uuid, SQLContext>>,
    dfs: SharedDfRegistry,
}

impl PolarsSqlCtx {
    fn new(dfs: SharedDfRegistry) -> Self {
        Self {
            contexts: Mutex::new(HashMap::new()),
            dfs,
        }
    }
}

impl PluginContext for PolarsSqlCtx {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

#[derive(Serialize)]
struct SqlCtxIdResult {
    ctx_id: String,
}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `polars_sql__new_context`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SqlNewContextParams {}

/// Parameters for `polars_sql__register`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SqlRegisterParams {
    /// UUID of the SQL context.
    pub ctx_id: String,
    /// Table name to register the DataFrame as.
    pub table_name: String,
    /// UUID of the DataFrame to register.
    pub df_id: String,
}

/// Parameters for `polars_sql__execute`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SqlExecuteParams {
    /// UUID of the SQL context.
    pub ctx_id: String,
    /// SQL query string to execute.
    pub query: String,
}

/// Parameters for `polars_sql__describe` and `polars_sql__list`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SqlCtxLookupParams {
    /// UUID of the SQL context.
    pub ctx_id: String,
}

/// Parameters for `polars_sql__list`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SqlListParams {}

// ── Tool functions ─────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "polars_sql",
    name = "polars_sql__new_context",
    description = "Create a new empty polars SQLContext. \
                   Establishes: PolarsSqlCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn sql_new_context(
    ctx: Arc<PolarsSqlCtx>,
    _p: SqlNewContextParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    let sql_ctx = SQLContext::new();
    ctx.contexts.lock().await.insert(id, sql_ctx);
    let _proof: Established<PolarsSqlCreated> = Established::assert();
    Ok(json_result(&SqlCtxIdResult {
        ctx_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_sql",
    name = "polars_sql__register",
    description = "Register a DataFrame (by UUID) as a named table in a SQLContext.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn sql_register(
    ctx: Arc<PolarsSqlCtx>,
    p: SqlRegisterParams,
) -> Result<CallToolResult, ErrorData> {
    let ctx_id: Uuid = p
        .ctx_id
        .parse()
        .map_err(|_| json_err(format!("invalid ctx UUID: {}", p.ctx_id)))?;
    let df_id: Uuid = p
        .df_id
        .parse()
        .map_err(|_| json_err(format!("invalid df UUID: {}", p.df_id)))?;

    let df = {
        let dfs = ctx.dfs.lock().await;
        dfs.get(&df_id)
            .cloned()
            .ok_or_else(|| json_err(format!("df_id not found: {df_id}")))?
    };

    let mut contexts = ctx.contexts.lock().await;
    let sql_ctx = contexts
        .get_mut(&ctx_id)
        .ok_or_else(|| json_err(format!("ctx_id not found: {ctx_id}")))?;

    sql_ctx.register(&p.table_name, df.lazy());

    Ok(json_result(&serde_json::json!({
        "ctx_id": p.ctx_id,
        "table_name": p.table_name,
        "registered": true
    })))
}

#[elicitation::elicit_tool(
    plugin = "polars_sql",
    name = "polars_sql__execute",
    description = "Execute a SQL query in a SQLContext. \
                   Returns the UUID of the resulting DataFrame.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn sql_execute(
    ctx: Arc<PolarsSqlCtx>,
    p: SqlExecuteParams,
) -> Result<CallToolResult, ErrorData> {
    let ctx_id: Uuid = p
        .ctx_id
        .parse()
        .map_err(|_| json_err(format!("invalid ctx UUID: {}", p.ctx_id)))?;

    let df = {
        let mut contexts = ctx.contexts.lock().await;
        let sql_ctx = contexts
            .get_mut(&ctx_id)
            .ok_or_else(|| json_err(format!("ctx_id not found: {ctx_id}")))?;
        let lf = sql_ctx
            .execute(&p.query)
            .map_err(|e| json_err(e.to_string()))?;
        // polars LazyFrame::collect() may use internal blocking IO; run in a
        // blocking thread so it doesn't conflict with the async executor.
        tokio::task::block_in_place(|| lf.collect()).map_err(|e| json_err(e.to_string()))?
    };

    let result_id = Uuid::new_v4();
    ctx.dfs.lock().await.insert(result_id, df);

    Ok(json_result(&serde_json::json!({
        "ctx_id": p.ctx_id,
        "df_id": result_id.to_string()
    })))
}

#[elicitation::elicit_tool(
    plugin = "polars_sql",
    name = "polars_sql__describe",
    description = "List the registered table names in a SQLContext.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn sql_describe(
    ctx: Arc<PolarsSqlCtx>,
    p: SqlCtxLookupParams,
) -> Result<CallToolResult, ErrorData> {
    let ctx_id: Uuid = p
        .ctx_id
        .parse()
        .map_err(|_| json_err(format!("invalid UUID: {}", p.ctx_id)))?;
    let contexts = ctx.contexts.lock().await;
    let sql_ctx = contexts
        .get(&ctx_id)
        .ok_or_else(|| json_err(format!("ctx_id not found: {ctx_id}")))?;
    let tables = sql_ctx.get_tables();
    Ok(json_result(&serde_json::json!({
        "ctx_id": p.ctx_id,
        "tables": tables
    })))
}

#[elicitation::elicit_tool(
    plugin = "polars_sql",
    name = "polars_sql__list",
    description = "List all stored SQL context UUIDs.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn sql_list(ctx: Arc<PolarsSqlCtx>, _p: SqlListParams) -> Result<CallToolResult, ErrorData> {
    let contexts = ctx.contexts.lock().await;
    let ids: Vec<String> = contexts.keys().map(|id| id.to_string()).collect();
    Ok(json_result(&ids))
}

// ── Plugin struct ─────────────────────────────────────────────────────────────

/// MCP plugin providing `polars_sql__*` tools for polars SQL interface.
///
/// Stores `SQLContext` instances in a UUID-keyed registry. Accepts a shared
/// `SharedDfRegistry` from `PolarsDataFramePlugin` so that DataFrames can be
/// registered as SQL tables by UUID.
pub struct PolarsSqlPlugin(Arc<PolarsSqlCtx>);

impl PolarsSqlPlugin {
    /// Create a new `PolarsSqlPlugin` with the given DataFrame registry.
    pub fn new(dfs: SharedDfRegistry) -> Self {
        Self(Arc::new(PolarsSqlCtx::new(dfs)))
    }

    /// Return the plugin context as a type-erased Arc for tool dispatch in tests.
    pub fn dispatch_ctx(&self) -> Arc<dyn std::any::Any + Send + Sync> {
        self.0.clone() as Arc<dyn std::any::Any + Send + Sync>
    }
}

impl elicitation::ElicitPlugin for PolarsSqlPlugin {
    fn name(&self) -> &'static str {
        "polars_sql"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "polars_sql")
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
            let full_name = if name.starts_with("polars_sql__") {
                name.to_string()
            } else {
                format!("polars_sql__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "polars_sql")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
