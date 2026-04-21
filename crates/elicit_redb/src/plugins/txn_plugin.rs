//! `RedbTransactionPlugin` — stateful write-transaction descriptor builder.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

// ── helpers ───────────────────────────────────────────────────────────────────

fn tool_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_text(s: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(s.into())]))
}

fn ok_json<T: serde::Serialize>(v: &T) -> Result<CallToolResult, ErrorData> {
    serde_json::to_string_pretty(v)
        .map(|s| CallToolResult::success(vec![Content::text(s)]))
        .map_err(|e| tool_err(format!("serialise: {e}")))
}

fn parse_params<T: serde::de::DeserializeOwned>(
    params: &CallToolRequestParams,
) -> Result<T, ErrorData> {
    let raw = params
        .arguments
        .as_ref()
        .map(|a| serde_json::Value::Object(a.clone()))
        .unwrap_or(serde_json::Value::Object(Default::default()));
    serde_json::from_value(raw)
        .map_err(|e| ErrorData::invalid_params(format!("param parse: {e}"), None))
}

fn build_tool(
    name: impl Into<std::borrow::Cow<'static, str>>,
    description: impl Into<std::borrow::Cow<'static, str>>,
    schema: serde_json::Value,
) -> Tool {
    let schema_obj: Arc<rmcp::model::JsonObject> = match schema {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => Arc::new(Default::default()),
    };
    Tool::new(name, description, schema_obj)
}

fn schema_of<T: schemars::JsonSchema>() -> serde_json::Value {
    serde_json::to_value(schemars::schema_for!(T)).unwrap_or_default()
}

fn parse_uuid(s: &str) -> Result<Uuid, ErrorData> {
    s.parse::<Uuid>()
        .map_err(|_| tool_err(format!("invalid UUID: {s}")))
}

// ── descriptor ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize)]
struct TxnDescriptor {
    db_var: String,
    txn_var: String,
    durability: Option<String>,
    two_phase: bool,
    ops: Vec<String>,
}

// ── context ───────────────────────────────────────────────────────────────────

struct RedbTxnContext {
    descriptors: Mutex<HashMap<Uuid, TxnDescriptor>>,
}

impl RedbTxnContext {
    fn new() -> Self {
        Self {
            descriptors: Mutex::new(HashMap::new()),
        }
    }
}

impl elicitation::PluginContext for RedbTxnContext {}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `redb_txn__start`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TxnStartParams {
    /// Database variable name.
    #[serde(default = "default_db_var")]
    pub db_var: String,
    /// Variable name for the `WriteTransaction`.
    #[serde(default = "default_write_txn")]
    pub txn_var: String,
}
fn default_db_var() -> String {
    "db".into()
}
fn default_write_txn() -> String {
    "write_txn".into()
}

/// Parameters for `redb_txn__add_op`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TxnAddOpParams {
    /// Descriptor UUID returned by `redb_txn__start`.
    pub txn_id: String,
    /// A Rust statement to include in the transaction body.
    pub statement: String,
}

/// Parameters for `redb_txn__set_durability`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TxnSetDurabilityParams {
    /// Descriptor UUID.
    pub txn_id: String,
    /// Durability variant: `"None"` or `"Immediate"`.
    pub durability: String,
}

/// Parameters for `redb_txn__set_two_phase`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TxnSetTwoPhaseParams {
    /// Descriptor UUID.
    pub txn_id: String,
    /// Enable two-phase commit.
    #[serde(default = "default_true")]
    pub enabled: bool,
}
fn default_true() -> bool {
    true
}

/// Parameters for tools that address a stored descriptor by UUID.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TxnIdParams {
    /// Descriptor UUID.
    pub txn_id: String,
}

/// Parameters for `redb_txn__emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TxnEmitParams {
    /// Descriptor UUID.
    pub txn_id: String,
    /// Emit `abort()` at the end instead of `commit()`.
    #[serde(default)]
    pub abort: bool,
}

// ── tool implementations ──────────────────────────────────────────────────────

#[instrument(skip(ctx))]
async fn txn_start(
    ctx: Arc<RedbTxnContext>,
    p: TxnStartParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .insert(
            id,
            TxnDescriptor {
                db_var: p.db_var,
                txn_var: p.txn_var,
                ..Default::default()
            },
        );
    ok_json(&serde_json::json!({ "txn_id": id.to_string() }))
}

#[instrument(skip(ctx, p))]
async fn txn_add_op(
    ctx: Arc<RedbTxnContext>,
    p: TxnAddOpParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.txn_id)?;
    let mut descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("transaction not found: {id}")))?;
    desc.ops.push(p.statement);
    let count = desc.ops.len();
    ok_json(&serde_json::json!({ "txn_id": p.txn_id, "op_count": count }))
}

#[instrument(skip(ctx, p))]
async fn txn_set_durability(
    ctx: Arc<RedbTxnContext>,
    p: TxnSetDurabilityParams,
) -> Result<CallToolResult, ErrorData> {
    let valid = ["None", "Immediate"];
    if !valid.contains(&p.durability.as_str()) {
        return Err(tool_err(format!(
            "durability must be one of: {}",
            valid.join(", ")
        )));
    }
    let id = parse_uuid(&p.txn_id)?;
    let mut descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("transaction not found: {id}")))?;
    desc.durability = Some(p.durability);
    ok_text("Durability set.")
}

#[instrument(skip(ctx, p))]
async fn txn_set_two_phase(
    ctx: Arc<RedbTxnContext>,
    p: TxnSetTwoPhaseParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.txn_id)?;
    let mut descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("transaction not found: {id}")))?;
    desc.two_phase = p.enabled;
    ok_text("Two-phase commit setting updated.")
}

#[instrument(skip(ctx, p))]
async fn txn_inspect(
    ctx: Arc<RedbTxnContext>,
    p: TxnIdParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.txn_id)?;
    let descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get(&id)
        .ok_or_else(|| tool_err(format!("transaction not found: {id}")))?;
    ok_json(desc)
}

#[instrument(skip(ctx, p))]
async fn txn_emit(ctx: Arc<RedbTxnContext>, p: TxnEmitParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.txn_id)?;
    let descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get(&id)
        .ok_or_else(|| tool_err(format!("transaction not found: {id}")))?;

    let mut code = format!(
        "let {txn} = {db}.begin_write()?;\n",
        txn = desc.txn_var,
        db = desc.db_var
    );
    if let Some(ref dur) = desc.durability {
        code.push_str(&format!(
            "{}.set_durability(redb::Durability::{});\n",
            desc.txn_var, dur
        ));
    }
    if desc.two_phase {
        code.push_str(&format!("{}.set_two_phase_commit(true);\n", desc.txn_var));
    }
    code.push_str("{\n");
    for op in &desc.ops {
        code.push_str(&format!("    {op}\n"));
    }
    let finalizer = if p.abort { "abort" } else { "commit" };
    code.push_str(&format!("    {txn}.{finalizer}();\n", txn = desc.txn_var));
    code.push('}');
    ok_text(code)
}

// ── dispatch ──────────────────────────────────────────────────────────────────

async fn dispatch(
    ctx: Arc<RedbTxnContext>,
    name: &str,
    params: &CallToolRequestParams,
) -> Result<CallToolResult, ErrorData> {
    match name {
        "redb_txn__start" => txn_start(ctx, parse_params(params)?).await,
        "redb_txn__add_op" => txn_add_op(ctx, parse_params(params)?).await,
        "redb_txn__set_durability" => txn_set_durability(ctx, parse_params(params)?).await,
        "redb_txn__set_two_phase" => txn_set_two_phase(ctx, parse_params(params)?).await,
        "redb_txn__inspect" => txn_inspect(ctx, parse_params(params)?).await,
        "redb_txn__emit" => txn_emit(ctx, parse_params(params)?).await,
        _ => Err(ErrorData::invalid_params(
            format!("unknown tool: {name}"),
            None,
        )),
    }
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// Stateful MCP plugin for building redb write-transaction code blocks incrementally.
pub struct RedbTransactionPlugin(Arc<RedbTxnContext>);

impl RedbTransactionPlugin {
    /// Create a new plugin with empty state.
    pub fn new() -> Self {
        Self(Arc::new(RedbTxnContext::new()))
    }
}

impl Default for RedbTransactionPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for RedbTransactionPlugin {
    fn name(&self) -> &'static str {
        "redb_txn"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            build_tool(
                "redb_txn__start",
                "Start a new write-transaction descriptor. Returns a UUID handle. Optionally specify db_var and txn_var.",
                schema_of::<TxnStartParams>(),
            ),
            build_tool(
                "redb_txn__add_op",
                "Append a Rust statement to the transaction body (e.g. `table.insert(&1u64, &\"hello\")?;`).",
                schema_of::<TxnAddOpParams>(),
            ),
            build_tool(
                "redb_txn__set_durability",
                "Set the durability level for the write transaction. Valid values: `None`, `Immediate`.",
                schema_of::<TxnSetDurabilityParams>(),
            ),
            build_tool(
                "redb_txn__set_two_phase",
                "Enable or disable two-phase commit for the write transaction.",
                schema_of::<TxnSetTwoPhaseParams>(),
            ),
            build_tool(
                "redb_txn__inspect",
                "Show the current descriptor state as JSON (db_var, txn_var, durability, two_phase, ops).",
                schema_of::<TxnIdParams>(),
            ),
            build_tool(
                "redb_txn__emit",
                "Emit the complete redb write-transaction Rust code block with all accumulated operations.",
                schema_of::<TxnEmitParams>(),
            ),
        ]
    }

    #[tracing::instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let ctx = self.0.clone();
        Box::pin(async move { dispatch(ctx, params.name.as_ref(), &params).await })
    }
}
