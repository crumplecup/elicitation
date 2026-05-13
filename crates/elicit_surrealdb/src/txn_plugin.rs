//! SurrealTransactionPlugin — stateful MCP tools for composing SurrealDB transactions.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn tool_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_text(s: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(s.into())]))
}

fn ok_json<T: serde::Serialize>(v: &T) -> Result<CallToolResult, ErrorData> {
    serde_json::to_string(v)
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

// ── Descriptor ────────────────────────────────────────────────────────────────

struct TxnDescriptor {
    statements: Vec<String>,
}

// ── Context ───────────────────────────────────────────────────────────────────

struct SurrealTxnContext {
    descriptors: Mutex<HashMap<Uuid, TxnDescriptor>>,
}

impl SurrealTxnContext {
    fn new() -> Self {
        Self {
            descriptors: Mutex::new(HashMap::new()),
        }
    }
}

impl elicitation::PluginContext for SurrealTxnContext {}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for tools that address a stored transaction by UUID.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TxnIdParams {
    /// UUID of the transaction descriptor.
    pub id: String,
}

/// Parameters for `surreal_txn__add_statement`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddStatementParams {
    /// UUID of the transaction descriptor.
    pub id: String,
    /// SurrealQL statement to append.
    pub statement: String,
}

// ── Tool implementations ──────────────────────────────────────────────────────

#[instrument(skip(ctx))]
async fn txn_start(ctx: Arc<SurrealTxnContext>) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .insert(
            id,
            TxnDescriptor {
                statements: Vec::new(),
            },
        );
    ok_json(&serde_json::json!({ "id": id.to_string() }))
}

#[instrument(skip(ctx, p))]
async fn txn_add_statement(
    ctx: Arc<SurrealTxnContext>,
    p: AddStatementParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let mut descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("transaction not found: {id}")))?;
    desc.statements.push(p.statement);
    let count = desc.statements.len();
    ok_json(&serde_json::json!({ "id": p.id, "count": count }))
}

#[instrument(skip(ctx, p))]
async fn txn_inspect(
    ctx: Arc<SurrealTxnContext>,
    p: TxnIdParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get(&id)
        .ok_or_else(|| tool_err(format!("transaction not found: {id}")))?;
    ok_json(&serde_json::json!({
        "id": p.id,
        "statements": desc.statements,
    }))
}

#[instrument(skip(ctx, p))]
async fn txn_emit_commit(
    ctx: Arc<SurrealTxnContext>,
    p: TxnIdParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get(&id)
        .ok_or_else(|| tool_err(format!("transaction not found: {id}")))?;
    let mut lines = vec!["BEGIN TRANSACTION;".to_string()];
    for stmt in &desc.statements {
        lines.push(format!("{stmt};"));
    }
    lines.push("COMMIT TRANSACTION;".to_string());
    ok_text(lines.join("\n"))
}

#[instrument(skip(ctx, p))]
async fn txn_emit_cancel(
    ctx: Arc<SurrealTxnContext>,
    p: TxnIdParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get(&id)
        .ok_or_else(|| tool_err(format!("transaction not found: {id}")))?;
    let mut lines = vec!["BEGIN TRANSACTION;".to_string()];
    for stmt in &desc.statements {
        lines.push(format!("{stmt};"));
    }
    lines.push("CANCEL TRANSACTION;".to_string());
    ok_text(lines.join("\n"))
}

#[instrument(skip(ctx, p))]
async fn txn_emit_rust(
    ctx: Arc<SurrealTxnContext>,
    p: TxnIdParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get(&id)
        .ok_or_else(|| tool_err(format!("transaction not found: {id}")))?;
    let mut lines = vec!["let result = db".to_string()];
    lines.push("    .query(\"BEGIN TRANSACTION\")".to_string());
    for stmt in &desc.statements {
        let escaped = stmt.replace('"', "\\\"");
        lines.push(format!("    .query(\"{escaped}\")"));
    }
    lines.push("    .query(\"COMMIT TRANSACTION\")".to_string());
    lines.push("    .await?;".to_string());
    ok_text(lines.join("\n"))
}

// ── Dispatch ──────────────────────────────────────────────────────────────────

async fn dispatch_txn(
    ctx: Arc<SurrealTxnContext>,
    name: &str,
    params: &CallToolRequestParams,
) -> Result<CallToolResult, ErrorData> {
    match name {
        "surreal_txn__start" => txn_start(ctx).await,
        "surreal_txn__add_statement" => txn_add_statement(ctx, parse_params(params)?).await,
        "surreal_txn__inspect" => txn_inspect(ctx, parse_params(params)?).await,
        "surreal_txn__emit_commit" => txn_emit_commit(ctx, parse_params(params)?).await,
        "surreal_txn__emit_cancel" => txn_emit_cancel(ctx, parse_params(params)?).await,
        "surreal_txn__emit_rust" => txn_emit_rust(ctx, parse_params(params)?).await,
        _ => Err(ErrorData::invalid_params(
            format!("unknown tool: {name}"),
            None,
        )),
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for composing SurrealDB transactions step-by-step.
pub struct SurrealTransactionPlugin(Arc<SurrealTxnContext>);

impl SurrealTransactionPlugin {
    /// Create a new plugin with empty state.
    pub fn new() -> Self {
        Self(Arc::new(SurrealTxnContext::new()))
    }

    /// Invoke a tool by name with a JSON arguments object.
    ///
    /// Convenience method for tests and direct integration.
    pub async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<CallToolResult, ErrorData> {
        let owned = name.to_string();
        let params = if let Some(m) = args.as_object().cloned() {
            CallToolRequestParams::new(owned).with_arguments(m)
        } else {
            CallToolRequestParams::new(owned)
        };
        dispatch_txn(self.0.clone(), name, &params).await
    }
}

impl Default for SurrealTransactionPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for SurrealTransactionPlugin {
    fn name(&self) -> &'static str {
        "surreal_txn"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            build_tool(
                "surreal_txn__start",
                "Create a new empty transaction descriptor. Returns a UUID handle.",
                serde_json::json!({"type": "object", "properties": {}}),
            ),
            build_tool(
                "surreal_txn__add_statement",
                "Append a SurrealQL statement to a transaction. Returns the current statement count.",
                schema_of::<AddStatementParams>(),
            ),
            build_tool(
                "surreal_txn__inspect",
                "Inspect the statements stored in a transaction descriptor.",
                schema_of::<TxnIdParams>(),
            ),
            build_tool(
                "surreal_txn__emit_commit",
                "Emit the transaction as a SurrealQL block ending with COMMIT TRANSACTION.",
                schema_of::<TxnIdParams>(),
            ),
            build_tool(
                "surreal_txn__emit_cancel",
                "Emit the transaction as a SurrealQL block ending with CANCEL TRANSACTION.",
                schema_of::<TxnIdParams>(),
            ),
            build_tool(
                "surreal_txn__emit_rust",
                "Emit the transaction as a chained Rust SurrealDB SDK `.query()` snippet.",
                schema_of::<TxnIdParams>(),
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
        Box::pin(async move { dispatch_txn(ctx, params.name.as_ref(), &params).await })
    }
}
