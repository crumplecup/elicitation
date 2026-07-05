//! `NaiveTimeRegistryPlugin` — stateful MCP tool plugin for mutable `NaiveTime` operations.
//!
//! `NaiveTime` wrapped in `Arc<T>` cannot implement `AddAssign`/`SubAssign` directly, so this
//! plugin maintains a UUID-keyed server-side registry and exposes mutation as discrete MCP tools.
//!
//! # Tool namespace: `chrono_time__*`

use std::collections::HashMap;
use std::sync::Arc;

use futures::future::BoxFuture;
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

// ── Shared registry type ──────────────────────────────────────────────────────

/// Shared registry of `NaiveTime` values keyed by UUID.
pub type SharedNaiveTimeRegistry = Arc<Mutex<HashMap<Uuid, chrono::NaiveTime>>>;

// ── Plugin context ─────────────────────────────────────────────────────────────

struct NaiveTimeCtx {
    registry: SharedNaiveTimeRegistry,
}

impl NaiveTimeCtx {
    fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl elicitation::PluginContext for NaiveTimeCtx {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn time_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_text(s: impl ToString) -> CallToolResult {
    CallToolResult::success(vec![Content::text(s.to_string())])
}

fn parse_uuid(s: &str) -> Result<Uuid, ErrorData> {
    s.parse::<Uuid>()
        .map_err(|_| time_err(format!("invalid UUID: {s}")))
}

fn parse_time(s: &str) -> Result<chrono::NaiveTime, ErrorData> {
    chrono::NaiveTime::parse_from_str(s, "%H:%M:%S")
        .or_else(|_| chrono::NaiveTime::parse_from_str(s, "%H:%M:%S%.f"))
        .map_err(|e| time_err(format!("invalid time '{s}': {e}")))
}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `chrono_time__create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TimeCreateParams {
    /// ISO 8601 time string (e.g. `"12:30:00"`).
    pub time: String,
}

/// Parameters for `chrono_time__get`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TimeGetParams {
    /// UUID returned by `chrono_time__create`.
    pub time_id: String,
}

/// Parameters for `chrono_time__add_assign`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TimeAddAssignParams {
    /// UUID of the stored `NaiveTime`.
    pub time_id: String,
    /// Number of whole seconds to add (negative to subtract).
    pub seconds: i64,
}

/// Parameters for `chrono_time__sub_assign`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TimeSubAssignParams {
    /// UUID of the stored `NaiveTime`.
    pub time_id: String,
    /// Number of whole seconds to subtract (negative to add).
    pub seconds: i64,
}

/// Parameters for `chrono_time__drop`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TimeDropParams {
    /// UUID of the stored `NaiveTime` to remove.
    pub time_id: String,
}

#[derive(Serialize)]
struct TimeIdResult {
    time_id: String,
}

// ── Tool functions ────────────────────────────────────────────────────────────

/// Parse an ISO 8601 time string and store it in the registry, returning a UUID handle.
#[elicitation::elicit_tool(
    plugin = "chrono_time",
    name = "chrono_time__create",
    description = "Parse an ISO 8601 NaiveTime string and store it in the registry. Returns a UUID handle for subsequent mutation tools.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_time_create(
    ctx: Arc<NaiveTimeCtx>,
    p: TimeCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let t = parse_time(&p.time)?;
    let id = Uuid::new_v4();
    ctx.registry.lock().await.insert(id, t);
    let body = serde_json::to_string(&TimeIdResult {
        time_id: id.to_string(),
    })
    .map_err(time_err)?;
    Ok(ok_text(body))
}

/// Retrieve the current value of a stored `NaiveTime` as an ISO 8601 string.
#[elicitation::elicit_tool(
    plugin = "chrono_time",
    name = "chrono_time__get",
    description = "Return the current ISO 8601 value of a stored NaiveTime.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_time_get(
    ctx: Arc<NaiveTimeCtx>,
    p: TimeGetParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.time_id)?;
    let guard = ctx.registry.lock().await;
    let t = guard
        .get(&id)
        .ok_or_else(|| time_err(format!("time_id not found: {id}")))?;
    Ok(ok_text(t.to_string()))
}

/// Add a number of whole seconds to a stored `NaiveTime` in place (`AddAssign<Duration>`).
#[elicitation::elicit_tool(
    plugin = "chrono_time",
    name = "chrono_time__add_assign",
    description = "Add whole seconds to a stored NaiveTime in place (AddAssign). Returns the updated ISO 8601 value.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_time_add_assign(
    ctx: Arc<NaiveTimeCtx>,
    p: TimeAddAssignParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.time_id)?;
    let mut guard = ctx.registry.lock().await;
    let t = guard
        .get_mut(&id)
        .ok_or_else(|| time_err(format!("time_id not found: {id}")))?;
    *t += chrono::TimeDelta::seconds(p.seconds);
    Ok(ok_text(t.to_string()))
}

/// Subtract a number of whole seconds from a stored `NaiveTime` in place (`SubAssign<Duration>`).
#[elicitation::elicit_tool(
    plugin = "chrono_time",
    name = "chrono_time__sub_assign",
    description = "Subtract whole seconds from a stored NaiveTime in place (SubAssign). Returns the updated ISO 8601 value.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_time_sub_assign(
    ctx: Arc<NaiveTimeCtx>,
    p: TimeSubAssignParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.time_id)?;
    let mut guard = ctx.registry.lock().await;
    let t = guard
        .get_mut(&id)
        .ok_or_else(|| time_err(format!("time_id not found: {id}")))?;
    *t -= chrono::TimeDelta::seconds(p.seconds);
    Ok(ok_text(t.to_string()))
}

/// Remove a stored `NaiveTime` from the registry.
#[elicitation::elicit_tool(
    plugin = "chrono_time",
    name = "chrono_time__drop",
    description = "Remove a stored NaiveTime from the registry, freeing the handle.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_time_drop(
    ctx: Arc<NaiveTimeCtx>,
    p: TimeDropParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.time_id)?;
    let removed = ctx.registry.lock().await.remove(&id).is_some();
    if removed {
        Ok(ok_text("dropped"))
    } else {
        Err(time_err(format!("time_id not found: {id}")))
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for stateful `NaiveTime` mutation via a UUID-keyed registry.
///
/// Exposes `add_assign` and `sub_assign` semantics over MCP by maintaining a
/// server-side registry of `chrono::NaiveTime` values.
pub struct NaiveTimeRegistryPlugin(Arc<NaiveTimeCtx>);

impl NaiveTimeRegistryPlugin {
    /// Create a new plugin with an empty registry.
    pub fn new() -> Self {
        Self(Arc::new(NaiveTimeCtx::new()))
    }
}

impl Default for NaiveTimeRegistryPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for NaiveTimeRegistryPlugin {
    fn name(&self) -> &'static str {
        "chrono_time"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "chrono_time")
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
            let full_name = if name.starts_with("chrono_time__") {
                name.to_string()
            } else {
                format!("chrono_time__{name}")
            };
            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "chrono_time")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;
            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
