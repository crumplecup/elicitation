//! `SurrealTransactionPlugin` — stateful transaction block builder.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use elicitation::{
    PluginContext, PluginToolRegistration, StatefulPlugin, ToolDescriptor, elicit_tool,
};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content, Tool},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

fn ok_text(text: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(text.into())]))
}

fn not_found(id: &str) -> ErrorData {
    ErrorData::invalid_params(format!("no transaction descriptor for id {id}"), None)
}

// ── descriptor ────────────────────────────────────────────────────────────────

/// State of an in-progress SurrealDB transaction.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TxnDescriptor {
    /// Unique ID for this transaction.
    pub id: String,
    /// Accumulated SurrealQL statements.
    pub statements: Vec<String>,
}

impl TxnDescriptor {
    fn new(id: String) -> Self {
        Self {
            id,
            statements: Vec::new(),
        }
    }

    fn build_commit(&self) -> String {
        let mut s = String::from("BEGIN TRANSACTION;\n");
        for stmt in &self.statements {
            s.push_str(&format!("  {stmt}\n"));
        }
        s.push_str("COMMIT TRANSACTION;");
        s
    }

    fn build_cancel(&self) -> String {
        let mut s = String::from("BEGIN TRANSACTION;\n");
        for stmt in &self.statements {
            s.push_str(&format!("  {stmt}\n"));
        }
        s.push_str("CANCEL TRANSACTION;");
        s
    }

    fn build_rust(&self) -> String {
        let mut s = String::from("let transaction = db.begin().await?;\n");
        for stmt in &self.statements {
            s.push_str(&format!("transaction.query(\"{stmt}\").await?;\n"));
        }
        s.push_str("transaction.commit().await?;\n");
        s
    }
}

// ── context ───────────────────────────────────────────────────────────────────

/// Shared state for `SurrealTransactionPlugin`.
pub struct SurrealTxnCtx {
    items: Mutex<HashMap<Uuid, TxnDescriptor>>,
}

impl PluginContext for SurrealTxnCtx {}

impl SurrealTxnCtx {
    fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
        }
    }
}

// ── plugin struct ─────────────────────────────────────────────────────────────

/// Stateful MCP plugin for building SurrealDB transaction blocks.
pub struct SurrealTransactionPlugin(Arc<SurrealTxnCtx>);

impl SurrealTransactionPlugin {
    /// Creates a new transaction plugin.
    pub fn new() -> Self {
        Self(Arc::new(SurrealTxnCtx::new()))
    }
}

impl Default for SurrealTransactionPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulPlugin for SurrealTransactionPlugin {
    type Context = SurrealTxnCtx;

    fn name(&self) -> &'static str {
        "surreal_txn"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|r| r.plugin == "surreal_txn")
            .map(|r| (r.constructor)().as_tool())
            .collect()
    }

    fn tool_descriptors(&self) -> Vec<ToolDescriptor> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|r| r.plugin == "surreal_txn")
            .map(|r| (r.constructor)())
            .collect()
    }

    fn context(&self) -> Arc<Self::Context> {
        self.0.clone()
    }
}

// ── parameter structs ─────────────────────────────────────────────────────────

/// Parameters to start a new transaction.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TxnStartParams {
    /// Optional label (for documentation only).
    #[serde(default)]
    pub label: Option<String>,
}

/// Parameters to add a statement.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TxnAddStatementParams {
    /// Transaction UUID.
    pub id: String,
    /// SurrealQL statement to append.
    pub statement: String,
}

/// Parameters that only require a transaction ID (for inspect).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TxnInspectParams {
    /// Transaction UUID.
    pub id: String,
}

/// Parameters that only require a transaction ID (for emit_commit).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TxnEmitCommitParams {
    /// Transaction UUID.
    pub id: String,
}

/// Parameters that only require a transaction ID (for emit_cancel).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TxnEmitCancelParams {
    /// Transaction UUID.
    pub id: String,
}

/// Parameters that only require a transaction ID (for emit_rust).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TxnEmitRustParams {
    /// Transaction UUID.
    pub id: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "surreal_txn",
    name = "start",
    description = "Begin a new transaction descriptor. Returns a UUID to reference in subsequent calls."
)]
#[instrument(skip(ctx))]
async fn txn_start(
    ctx: Arc<SurrealTxnCtx>,
    p: TxnStartParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    let descriptor = TxnDescriptor::new(id.to_string());
    ctx.items.lock().unwrap().insert(id, descriptor);
    let out = match &p.label {
        Some(lbl) => format!("{id}  # {lbl}"),
        None => id.to_string(),
    };
    ok_text(out)
}

#[elicit_tool(
    plugin = "surreal_txn",
    name = "add_statement",
    description = "Append a SurrealQL statement to an existing transaction descriptor."
)]
#[instrument(skip(ctx))]
async fn txn_add_statement(
    ctx: Arc<SurrealTxnCtx>,
    p: TxnAddStatementParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let mut items = ctx.items.lock().unwrap();
    let entry = items.get_mut(&id).ok_or_else(|| not_found(&p.id))?;
    entry.statements.push(p.statement);
    ok_text(format!("{} statements", entry.statements.len()))
}

#[elicit_tool(
    plugin = "surreal_txn",
    name = "inspect",
    description = "Return the current transaction descriptor as a JSON summary."
)]
#[instrument(skip(ctx))]
async fn txn_inspect(
    ctx: Arc<SurrealTxnCtx>,
    p: TxnInspectParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let items = ctx.items.lock().unwrap();
    let entry = items.get(&id).ok_or_else(|| not_found(&p.id))?;
    let json = serde_json::to_string_pretty(entry)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    ok_text(json)
}

#[elicit_tool(
    plugin = "surreal_txn",
    name = "emit_commit",
    description = "Emit a BEGIN … COMMIT TRANSACTION SurrealQL block for the descriptor."
)]
#[instrument(skip(ctx))]
async fn txn_emit_commit(
    ctx: Arc<SurrealTxnCtx>,
    p: TxnEmitCommitParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let items = ctx.items.lock().unwrap();
    let entry = items.get(&id).ok_or_else(|| not_found(&p.id))?;
    ok_text(entry.build_commit())
}

#[elicit_tool(
    plugin = "surreal_txn",
    name = "emit_cancel",
    description = "Emit a BEGIN … CANCEL TRANSACTION SurrealQL block for the descriptor."
)]
#[instrument(skip(ctx))]
async fn txn_emit_cancel(
    ctx: Arc<SurrealTxnCtx>,
    p: TxnEmitCancelParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let items = ctx.items.lock().unwrap();
    let entry = items.get(&id).ok_or_else(|| not_found(&p.id))?;
    ok_text(entry.build_cancel())
}

#[elicit_tool(
    plugin = "surreal_txn",
    name = "emit_rust",
    description = "Emit Rust SDK transaction code using db.begin().await? and db.commit().await?."
)]
#[instrument(skip(ctx))]
async fn txn_emit_rust(
    ctx: Arc<SurrealTxnCtx>,
    p: TxnEmitRustParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let items = ctx.items.lock().unwrap();
    let entry = items.get(&id).ok_or_else(|| not_found(&p.id))?;
    ok_text(entry.build_rust())
}
