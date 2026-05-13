//! `LeptosReactivePlugin` — MCP tools for Leptos reactive primitives.
//!
//! # Tool namespace: `leptos_reactive__*`

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
use serde_json::Value;
use tracing::instrument;
use uuid::Uuid;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn tool_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_json<T: Serialize>(v: &T) -> Result<CallToolResult, ErrorData> {
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
    let schema_obj: std::sync::Arc<rmcp::model::JsonObject> = match schema {
        serde_json::Value::Object(m) => std::sync::Arc::new(m),
        _ => std::sync::Arc::new(Default::default()),
    };
    Tool::new(name, description, schema_obj)
}

fn schema_of<T: schemars::JsonSchema>() -> serde_json::Value {
    serde_json::to_value(schemars::schema_for!(T)).unwrap_or_default()
}

// ── Stored entries ────────────────────────────────────────────────────────────

/// A stored reactive signal.
#[derive(Debug, Clone)]
pub struct SignalEntry {
    /// Human-readable name.
    pub name: String,
    /// Current value.
    pub value: Value,
    /// Number of times value has been set.
    pub change_count: u64,
}

/// A stored memo (derived value).
#[derive(Debug, Clone)]
pub struct MemoEntry {
    /// Source signal UUID.
    pub source_id: Uuid,
    /// Transformation operation name.
    pub op: String,
    /// Human-readable name.
    pub name: String,
}

/// A stored server action.
#[derive(Debug, Clone)]
pub struct ActionEntry {
    /// Action name.
    pub name: String,
    /// Whether there is a pending dispatch.
    pub pending: bool,
    /// Last dispatched input.
    pub last_input: Option<Value>,
    /// Last resolved output.
    pub last_output: Option<Value>,
}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for `leptos_reactive__*` tools.
pub struct LeptosReactiveContext {
    signals: Mutex<HashMap<Uuid, SignalEntry>>,
    memos: Mutex<HashMap<Uuid, MemoEntry>>,
    actions: Mutex<HashMap<Uuid, ActionEntry>>,
    ctx_map: Mutex<HashMap<String, Value>>,
}

impl LeptosReactiveContext {
    fn new() -> Self {
        Self {
            signals: Mutex::new(HashMap::new()),
            memos: Mutex::new(HashMap::new()),
            actions: Mutex::new(HashMap::new()),
            ctx_map: Mutex::new(HashMap::new()),
        }
    }
}

impl elicitation::PluginContext for LeptosReactiveContext {}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `leptos_reactive__signal_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SignalNewParams {
    /// Human-readable name for the signal.
    pub name: String,
    /// Initial value.
    pub value: Value,
}

/// Parameters for tools that take a signal/memo/action id.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct IdParams {
    /// UUID of the signal/memo/action.
    pub id: String,
}

/// Parameters for `leptos_reactive__signal_set`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SignalSetParams {
    /// UUID of the signal.
    pub id: String,
    /// New value to set.
    pub value: Value,
}

/// Parameters for `leptos_reactive__signal_update`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SignalUpdateParams {
    /// UUID of the signal.
    pub id: String,
    /// Operation: "increment", "decrement", "append", "clear", "toggle", "merge".
    pub op: String,
    /// Operand for operations that need it (e.g. append).
    #[serde(default)]
    pub operand: Option<Value>,
}

/// Parameters for `leptos_reactive__memo_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MemoNewParams {
    /// UUID of the source signal.
    pub source_id: String,
    /// Transformation: "upper", "lower", "negate", "to_number", "to_bool", "length", "abs", "not".
    pub op: String,
    /// Optional human-readable name.
    #[serde(default)]
    pub name: Option<String>,
}

/// Parameters for `leptos_reactive__ctx_provide`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CtxProvideParams {
    /// Context key.
    pub key: String,
    /// Value to provide.
    pub value: Value,
}

/// Parameters for `leptos_reactive__ctx_get` and `leptos_reactive__ctx_remove`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CtxKeyParams {
    /// Context key.
    pub key: String,
}

/// Parameters for `leptos_reactive__action_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ActionNewParams {
    /// Action name.
    pub name: String,
}

/// Parameters for `leptos_reactive__action_dispatch`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ActionDispatchParams {
    /// UUID of the action.
    pub id: String,
    /// Input value to dispatch.
    pub input: Value,
}

/// Parameters for `leptos_reactive__action_resolve`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ActionResolveParams {
    /// UUID of the action.
    pub id: String,
    /// Output value to resolve with.
    pub output: Value,
}

// ── Tool implementations ──────────────────────────────────────────────────────

#[instrument(skip(ctx, p))]
async fn signal_new(
    ctx: Arc<LeptosReactiveContext>,
    p: SignalNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    let entry = SignalEntry {
        name: p.name.clone(),
        value: p.value,
        change_count: 0,
    };
    ctx.signals
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .insert(id, entry);
    ok_json(&serde_json::json!({ "id": id.to_string(), "name": p.name }))
}

#[instrument(skip(ctx, p))]
async fn signal_get(
    ctx: Arc<LeptosReactiveContext>,
    p: IdParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let signals = ctx
        .signals
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let entry = signals
        .get(&id)
        .ok_or_else(|| tool_err(format!("signal not found: {id}")))?;
    ok_json(&serde_json::json!({ "id": p.id, "value": entry.value }))
}

#[instrument(skip(ctx, p))]
async fn signal_set(
    ctx: Arc<LeptosReactiveContext>,
    p: SignalSetParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let mut signals = ctx
        .signals
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let entry = signals
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("signal not found: {id}")))?;
    entry.value = p.value;
    entry.change_count += 1;
    let cc = entry.change_count;
    ok_json(&serde_json::json!({ "id": p.id, "change_count": cc }))
}

#[instrument(skip(ctx, p))]
async fn signal_update(
    ctx: Arc<LeptosReactiveContext>,
    p: SignalUpdateParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let mut signals = ctx
        .signals
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let entry = signals
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("signal not found: {id}")))?;
    match p.op.as_str() {
        "increment" => {
            if let Some(n) = entry.value.as_f64() {
                entry.value = Value::from(n + 1.0);
            } else {
                return Err(tool_err("increment requires numeric value"));
            }
        }
        "decrement" => {
            if let Some(n) = entry.value.as_f64() {
                entry.value = Value::from(n - 1.0);
            } else {
                return Err(tool_err("decrement requires numeric value"));
            }
        }
        "append" => {
            let s = entry
                .value
                .as_str()
                .ok_or_else(|| tool_err("append requires string value"))?;
            let operand = p.operand.as_ref().and_then(|v| v.as_str()).unwrap_or("");
            entry.value = Value::String(format!("{s}{operand}"));
        }
        "clear" => {
            entry.value = Value::Null;
        }
        "toggle" => {
            if let Some(b) = entry.value.as_bool() {
                entry.value = Value::Bool(!b);
            } else {
                return Err(tool_err("toggle requires boolean value"));
            }
        }
        "merge" => {
            let operand = p.operand.clone().unwrap_or(Value::Null);
            if let (Value::Object(base), Value::Object(patch)) = (&mut entry.value, operand) {
                for (k, v) in patch {
                    base.insert(k, v);
                }
            } else {
                return Err(tool_err("merge requires object values"));
            }
        }
        other => return Err(tool_err(format!("unknown op: {other}"))),
    }
    entry.change_count += 1;
    let new_val = entry.value.clone();
    let cc = entry.change_count;
    ok_json(&serde_json::json!({ "id": p.id, "value": new_val, "change_count": cc }))
}

#[instrument(skip(ctx, p))]
async fn signal_describe(
    ctx: Arc<LeptosReactiveContext>,
    p: IdParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let signals = ctx
        .signals
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let entry = signals
        .get(&id)
        .ok_or_else(|| tool_err(format!("signal not found: {id}")))?;
    ok_json(&serde_json::json!({
        "id": p.id,
        "name": entry.name,
        "value": entry.value,
        "change_count": entry.change_count
    }))
}

#[instrument(skip(ctx))]
async fn signal_list(ctx: Arc<LeptosReactiveContext>) -> Result<CallToolResult, ErrorData> {
    let signals = ctx
        .signals
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let items: Vec<Value> = signals
        .iter()
        .map(|(id, e)| {
            serde_json::json!({
                "id": id.to_string(),
                "name": e.name,
                "change_count": e.change_count
            })
        })
        .collect();
    ok_json(&items)
}

#[instrument(skip(ctx, p))]
async fn signal_delete(
    ctx: Arc<LeptosReactiveContext>,
    p: IdParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let removed = ctx
        .signals
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .remove(&id)
        .is_some();
    ok_json(&serde_json::json!({ "deleted": removed, "id": p.id }))
}

#[instrument(skip(ctx, p))]
async fn signal_track(
    ctx: Arc<LeptosReactiveContext>,
    p: IdParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let signals = ctx
        .signals
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let entry = signals
        .get(&id)
        .ok_or_else(|| tool_err(format!("signal not found: {id}")))?;
    ok_json(&serde_json::json!({ "id": p.id, "change_count": entry.change_count }))
}

fn compute_memo(
    signals: &HashMap<Uuid, SignalEntry>,
    memo: &MemoEntry,
) -> Result<Value, ErrorData> {
    let src = signals
        .get(&memo.source_id)
        .ok_or_else(|| tool_err(format!("source signal not found: {}", memo.source_id)))?;
    match memo.op.as_str() {
        "upper" => {
            let s = src
                .value
                .as_str()
                .ok_or_else(|| tool_err("upper requires string"))?;
            Ok(Value::String(s.to_uppercase()))
        }
        "lower" => {
            let s = src
                .value
                .as_str()
                .ok_or_else(|| tool_err("lower requires string"))?;
            Ok(Value::String(s.to_lowercase()))
        }
        "negate" => {
            let n = src
                .value
                .as_f64()
                .ok_or_else(|| tool_err("negate requires number"))?;
            Ok(Value::from(-n))
        }
        "to_number" => {
            let s = src
                .value
                .as_str()
                .ok_or_else(|| tool_err("to_number requires string"))?;
            let n: f64 = s
                .parse()
                .map_err(|_| tool_err(format!("cannot parse as number: {s}")))?;
            Ok(Value::from(n))
        }
        "to_bool" => {
            let b = match &src.value {
                Value::Bool(b) => *b,
                Value::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
                Value::String(s) => !s.is_empty(),
                Value::Null => false,
                _ => true,
            };
            Ok(Value::Bool(b))
        }
        "length" => {
            let len = match &src.value {
                Value::String(s) => s.len(),
                Value::Array(a) => a.len(),
                _ => return Err(tool_err("length requires string or array")),
            };
            Ok(Value::from(len as u64))
        }
        "abs" => {
            let n = src
                .value
                .as_f64()
                .ok_or_else(|| tool_err("abs requires number"))?;
            Ok(Value::from(n.abs()))
        }
        "not" => {
            let b = src
                .value
                .as_bool()
                .ok_or_else(|| tool_err("not requires boolean"))?;
            Ok(Value::Bool(!b))
        }
        other => Err(tool_err(format!("unknown memo op: {other}"))),
    }
}

#[instrument(skip(ctx, p))]
async fn memo_new(
    ctx: Arc<LeptosReactiveContext>,
    p: MemoNewParams,
) -> Result<CallToolResult, ErrorData> {
    let source_id: Uuid = p
        .source_id
        .parse()
        .map_err(|_| tool_err(format!("invalid UUID: {}", p.source_id)))?;
    {
        let signals = ctx
            .signals
            .lock()
            .map_err(|e| tool_err(format!("lock: {e}")))?;
        if !signals.contains_key(&source_id) {
            return Err(tool_err(format!("source signal not found: {source_id}")));
        }
    }
    let id = Uuid::new_v4();
    let name = p
        .name
        .unwrap_or_else(|| format!("memo_{}", &id.to_string()[..8]));
    let entry = MemoEntry {
        source_id,
        op: p.op,
        name: name.clone(),
    };
    ctx.memos
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .insert(id, entry);
    ok_json(&serde_json::json!({ "id": id.to_string(), "name": name }))
}

#[instrument(skip(ctx, p))]
async fn memo_get(
    ctx: Arc<LeptosReactiveContext>,
    p: IdParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let memo_clone = {
        let memos = ctx
            .memos
            .lock()
            .map_err(|e| tool_err(format!("lock: {e}")))?;
        memos
            .get(&id)
            .ok_or_else(|| tool_err(format!("memo not found: {id}")))?
            .clone()
    };
    let signals = ctx
        .signals
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let value = compute_memo(&signals, &memo_clone)?;
    ok_json(&serde_json::json!({ "id": p.id, "value": value }))
}

#[instrument(skip(ctx))]
async fn memo_list(ctx: Arc<LeptosReactiveContext>) -> Result<CallToolResult, ErrorData> {
    let memo_clones: Vec<(Uuid, MemoEntry)> = {
        let memos = ctx
            .memos
            .lock()
            .map_err(|e| tool_err(format!("lock: {e}")))?;
        memos.iter().map(|(id, m)| (*id, m.clone())).collect()
    };
    let signals = ctx
        .signals
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let items: Vec<Value> = memo_clones
        .iter()
        .map(|(id, m)| {
            let val = compute_memo(&signals, m).unwrap_or(Value::Null);
            serde_json::json!({
                "id": id.to_string(),
                "name": m.name,
                "op": m.op,
                "source_id": m.source_id.to_string(),
                "value": val
            })
        })
        .collect();
    ok_json(&items)
}

#[instrument(skip(ctx, p))]
async fn memo_delete(
    ctx: Arc<LeptosReactiveContext>,
    p: IdParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let removed = ctx
        .memos
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .remove(&id)
        .is_some();
    ok_json(&serde_json::json!({ "deleted": removed, "id": p.id }))
}

#[instrument(skip(ctx, p))]
async fn ctx_provide(
    ctx: Arc<LeptosReactiveContext>,
    p: CtxProvideParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.ctx_map
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .insert(p.key.clone(), p.value);
    ok_json(&serde_json::json!({ "key": p.key, "provided": true }))
}

#[instrument(skip(ctx, p))]
async fn ctx_get(
    ctx: Arc<LeptosReactiveContext>,
    p: CtxKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let map = ctx
        .ctx_map
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let value = map.get(&p.key).cloned().unwrap_or(Value::Null);
    ok_json(&serde_json::json!({ "key": p.key, "value": value }))
}

#[instrument(skip(ctx))]
async fn ctx_list(ctx: Arc<LeptosReactiveContext>) -> Result<CallToolResult, ErrorData> {
    let map = ctx
        .ctx_map
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let items: Vec<Value> = map
        .iter()
        .map(|(k, v)| serde_json::json!({ "key": k, "value": v }))
        .collect();
    ok_json(&items)
}

#[instrument(skip(ctx, p))]
async fn ctx_remove(
    ctx: Arc<LeptosReactiveContext>,
    p: CtxKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let removed = ctx
        .ctx_map
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .remove(&p.key)
        .is_some();
    ok_json(&serde_json::json!({ "key": p.key, "removed": removed }))
}

#[instrument(skip(ctx, p))]
async fn action_new(
    ctx: Arc<LeptosReactiveContext>,
    p: ActionNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    let entry = ActionEntry {
        name: p.name.clone(),
        pending: false,
        last_input: None,
        last_output: None,
    };
    ctx.actions
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .insert(id, entry);
    ok_json(&serde_json::json!({ "id": id.to_string(), "name": p.name }))
}

#[instrument(skip(ctx, p))]
async fn action_dispatch(
    ctx: Arc<LeptosReactiveContext>,
    p: ActionDispatchParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let mut actions = ctx
        .actions
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let entry = actions
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("action not found: {id}")))?;
    entry.pending = true;
    entry.last_input = Some(p.input.clone());
    ok_json(&serde_json::json!({ "id": p.id, "pending": true, "input": p.input }))
}

#[instrument(skip(ctx, p))]
async fn action_resolve(
    ctx: Arc<LeptosReactiveContext>,
    p: ActionResolveParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let mut actions = ctx
        .actions
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let entry = actions
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("action not found: {id}")))?;
    entry.pending = false;
    entry.last_output = Some(p.output.clone());
    ok_json(&serde_json::json!({ "id": p.id, "pending": false, "output": p.output }))
}

#[instrument(skip(ctx))]
async fn action_list(ctx: Arc<LeptosReactiveContext>) -> Result<CallToolResult, ErrorData> {
    let actions = ctx
        .actions
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let items: Vec<Value> = actions
        .iter()
        .map(|(id, a)| {
            serde_json::json!({
                "id": id.to_string(),
                "name": a.name,
                "pending": a.pending
            })
        })
        .collect();
    ok_json(&items)
}

#[instrument(skip(ctx))]
async fn owner_reset(ctx: Arc<LeptosReactiveContext>) -> Result<CallToolResult, ErrorData> {
    ctx.signals
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .clear();
    ctx.memos
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .clear();
    ctx.actions
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .clear();
    ctx.ctx_map
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .clear();
    ok_json(&serde_json::json!({ "reset": true }))
}

#[instrument(skip(ctx))]
async fn owner_status(ctx: Arc<LeptosReactiveContext>) -> Result<CallToolResult, ErrorData> {
    let signals = ctx
        .signals
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .len();
    let memos = ctx
        .memos
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .len();
    let actions = ctx
        .actions
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .len();
    let ctx_entries = ctx
        .ctx_map
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .len();
    ok_json(&serde_json::json!({
        "signals": signals,
        "memos": memos,
        "actions": actions,
        "context_entries": ctx_entries
    }))
}

// ── Dispatch ──────────────────────────────────────────────────────────────────

async fn dispatch_reactive(
    ctx: Arc<LeptosReactiveContext>,
    name: &str,
    params: &CallToolRequestParams,
) -> Result<CallToolResult, ErrorData> {
    match name {
        "leptos_reactive__signal_new" => signal_new(ctx, parse_params(params)?).await,
        "leptos_reactive__signal_get" => signal_get(ctx, parse_params(params)?).await,
        "leptos_reactive__signal_set" => signal_set(ctx, parse_params(params)?).await,
        "leptos_reactive__signal_update" => signal_update(ctx, parse_params(params)?).await,
        "leptos_reactive__signal_describe" => signal_describe(ctx, parse_params(params)?).await,
        "leptos_reactive__signal_list" => signal_list(ctx).await,
        "leptos_reactive__signal_delete" => signal_delete(ctx, parse_params(params)?).await,
        "leptos_reactive__signal_track" => signal_track(ctx, parse_params(params)?).await,
        "leptos_reactive__memo_new" => memo_new(ctx, parse_params(params)?).await,
        "leptos_reactive__memo_get" => memo_get(ctx, parse_params(params)?).await,
        "leptos_reactive__memo_list" => memo_list(ctx).await,
        "leptos_reactive__memo_delete" => memo_delete(ctx, parse_params(params)?).await,
        "leptos_reactive__ctx_provide" => ctx_provide(ctx, parse_params(params)?).await,
        "leptos_reactive__ctx_get" => ctx_get(ctx, parse_params(params)?).await,
        "leptos_reactive__ctx_list" => ctx_list(ctx).await,
        "leptos_reactive__ctx_remove" => ctx_remove(ctx, parse_params(params)?).await,
        "leptos_reactive__action_new" => action_new(ctx, parse_params(params)?).await,
        "leptos_reactive__action_dispatch" => action_dispatch(ctx, parse_params(params)?).await,
        "leptos_reactive__action_resolve" => action_resolve(ctx, parse_params(params)?).await,
        "leptos_reactive__action_list" => action_list(ctx).await,
        "leptos_reactive__owner_reset" => owner_reset(ctx).await,
        "leptos_reactive__owner_status" => owner_status(ctx).await,
        _ => Err(ErrorData::invalid_params(
            format!("unknown tool: {name}"),
            None,
        )),
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin providing `leptos_reactive__*` tools.
pub struct LeptosReactivePlugin(pub Arc<LeptosReactiveContext>);

impl LeptosReactivePlugin {
    /// Create a new plugin with empty reactive state.
    pub fn new() -> Self {
        Self(Arc::new(LeptosReactiveContext::new()))
    }

    /// Invoke a tool by name with a JSON arguments object.
    ///
    /// Convenience for tests and direct integration.
    pub async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<CallToolResult, ErrorData> {
        let owned: String = name.to_string();
        let params = if let Some(m) = args.as_object().cloned() {
            CallToolRequestParams::new(owned).with_arguments(m)
        } else {
            CallToolRequestParams::new(owned)
        };
        dispatch_reactive(self.0.clone(), name, &params).await
    }
}

impl Default for LeptosReactivePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for LeptosReactivePlugin {
    fn name(&self) -> &'static str {
        "leptos_reactive"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            build_tool(
                "leptos_reactive__signal_new",
                "Create a new reactive RwSignal with an initial value. Returns a UUID handle.",
                schema_of::<SignalNewParams>(),
            ),
            build_tool(
                "leptos_reactive__signal_get",
                "Get the current value of a signal by UUID.",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_reactive__signal_set",
                "Set a new value on a signal by UUID.",
                schema_of::<SignalSetParams>(),
            ),
            build_tool(
                "leptos_reactive__signal_update",
                "Apply an in-place update operation to a signal (increment, decrement, append, clear, toggle, merge).",
                schema_of::<SignalUpdateParams>(),
            ),
            build_tool(
                "leptos_reactive__signal_describe",
                "Describe a signal: name, value, and change count.",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_reactive__signal_list",
                "List all stored signals with name and change count.",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            build_tool(
                "leptos_reactive__signal_delete",
                "Delete a stored signal by UUID.",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_reactive__signal_track",
                "Get the change count of a signal (for dependency tracking).",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_reactive__memo_new",
                "Create a derived memo from a source signal and a transformation op.",
                schema_of::<MemoNewParams>(),
            ),
            build_tool(
                "leptos_reactive__memo_get",
                "Compute and return the current derived value of a memo.",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_reactive__memo_list",
                "List all memos with their computed values.",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            build_tool(
                "leptos_reactive__memo_delete",
                "Delete a memo by UUID.",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_reactive__ctx_provide",
                "Provide a value into the reactive context store.",
                schema_of::<CtxProvideParams>(),
            ),
            build_tool(
                "leptos_reactive__ctx_get",
                "Get a value from the reactive context store by key.",
                schema_of::<CtxKeyParams>(),
            ),
            build_tool(
                "leptos_reactive__ctx_list",
                "List all entries in the reactive context store.",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            build_tool(
                "leptos_reactive__ctx_remove",
                "Remove a value from the reactive context store by key.",
                schema_of::<CtxKeyParams>(),
            ),
            build_tool(
                "leptos_reactive__action_new",
                "Create a new server action entry.",
                schema_of::<ActionNewParams>(),
            ),
            build_tool(
                "leptos_reactive__action_dispatch",
                "Dispatch an input to a server action (marks it as pending).",
                schema_of::<ActionDispatchParams>(),
            ),
            build_tool(
                "leptos_reactive__action_resolve",
                "Resolve a pending server action with an output value.",
                schema_of::<ActionResolveParams>(),
            ),
            build_tool(
                "leptos_reactive__action_list",
                "List all actions with name and pending status.",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            build_tool(
                "leptos_reactive__owner_reset",
                "Clear all signals, memos, actions, and context (reset reactive owner).",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            build_tool(
                "leptos_reactive__owner_status",
                "Get counts of all stored reactive primitives.",
                serde_json::json!({"type":"object","properties":{}}),
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
        Box::pin(async move {
            let name = params.name.as_ref();
            dispatch_reactive(ctx, name, &params).await
        })
    }
}
