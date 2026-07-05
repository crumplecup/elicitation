//! `NaiveDateTimeRegistryPlugin` — stateful MCP tool plugin for mutable `NaiveDateTime` operations.
//!
//! `NaiveDateTime` wrapped in `Arc<T>` cannot implement `AddAssign`/`SubAssign` directly, so this
//! plugin maintains a UUID-keyed server-side registry and exposes mutation as discrete MCP tools.
//!
//! # Tool namespace: `chrono_dt__*`

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

/// Shared registry of `NaiveDateTime` values keyed by UUID.
pub type SharedNaiveDateTimeRegistry = Arc<Mutex<HashMap<Uuid, chrono::NaiveDateTime>>>;

// ── Plugin context ─────────────────────────────────────────────────────────────

struct NaiveDateTimeCtx {
    registry: SharedNaiveDateTimeRegistry,
}

impl NaiveDateTimeCtx {
    fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl elicitation::PluginContext for NaiveDateTimeCtx {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn dt_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_text(s: impl ToString) -> CallToolResult {
    CallToolResult::success(vec![Content::text(s.to_string())])
}

fn parse_uuid(s: &str) -> Result<Uuid, ErrorData> {
    s.parse::<Uuid>()
        .map_err(|_| dt_err(format!("invalid UUID: {s}")))
}

fn parse_dt(s: &str) -> Result<chrono::NaiveDateTime, ErrorData> {
    chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f"))
        .or_else(|_| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
        .or_else(|_| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f"))
        .map_err(|e| dt_err(format!("invalid datetime '{s}': {e}")))
}

fn parse_duration_secs(secs: i64) -> chrono::TimeDelta {
    chrono::TimeDelta::seconds(secs)
}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `chrono_dt__create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateParams {
    /// ISO 8601 datetime string (e.g. `"2024-01-15T12:30:00"`).
    pub datetime: String,
}

/// Parameters for `chrono_dt__get`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetParams {
    /// UUID returned by `chrono_dt__create`.
    pub dt_id: String,
}

/// Parameters for `chrono_dt__add_assign`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddAssignParams {
    /// UUID of the stored `NaiveDateTime`.
    pub dt_id: String,
    /// Number of whole seconds to add (negative to subtract).
    pub seconds: i64,
}

/// Parameters for `chrono_dt__sub_assign`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SubAssignParams {
    /// UUID of the stored `NaiveDateTime`.
    pub dt_id: String,
    /// Number of whole seconds to subtract (negative to add).
    pub seconds: i64,
}

/// Parameters for `chrono_dt__drop`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DropParams {
    /// UUID of the stored `NaiveDateTime` to remove.
    pub dt_id: String,
}

#[derive(Serialize)]
struct DtIdResult {
    dt_id: String,
}

// ── Tool functions ────────────────────────────────────────────────────────────

/// Parse an ISO 8601 datetime string and store it in the registry, returning a UUID handle.
#[elicitation::elicit_tool(
    plugin = "chrono_dt",
    name = "chrono_dt__create",
    description = "Parse an ISO 8601 NaiveDateTime string and store it in the registry. Returns a UUID handle for subsequent mutation tools.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_dt_create(
    ctx: Arc<NaiveDateTimeCtx>,
    p: CreateParams,
) -> Result<CallToolResult, ErrorData> {
    let dt = parse_dt(&p.datetime)?;
    let id = Uuid::new_v4();
    ctx.registry.lock().await.insert(id, dt);
    let body = serde_json::to_string(&DtIdResult {
        dt_id: id.to_string(),
    })
    .map_err(dt_err)?;
    Ok(ok_text(body))
}

/// Retrieve the current value of a stored `NaiveDateTime` as an ISO 8601 string.
#[elicitation::elicit_tool(
    plugin = "chrono_dt",
    name = "chrono_dt__get",
    description = "Return the current ISO 8601 value of a stored NaiveDateTime.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_dt_get(
    ctx: Arc<NaiveDateTimeCtx>,
    p: GetParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.dt_id)?;
    let guard = ctx.registry.lock().await;
    let dt = guard
        .get(&id)
        .ok_or_else(|| dt_err(format!("dt_id not found: {id}")))?;
    Ok(ok_text(dt.to_string()))
}

/// Add a number of whole seconds to a stored `NaiveDateTime` in place (`AddAssign<Duration>`).
#[elicitation::elicit_tool(
    plugin = "chrono_dt",
    name = "chrono_dt__add_assign",
    description = "Add whole seconds to a stored NaiveDateTime in place (AddAssign). Returns the updated ISO 8601 value.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_dt_add_assign(
    ctx: Arc<NaiveDateTimeCtx>,
    p: AddAssignParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.dt_id)?;
    let mut guard = ctx.registry.lock().await;
    let dt = guard
        .get_mut(&id)
        .ok_or_else(|| dt_err(format!("dt_id not found: {id}")))?;
    *dt += parse_duration_secs(p.seconds);
    Ok(ok_text(dt.to_string()))
}

/// Subtract a number of whole seconds from a stored `NaiveDateTime` in place (`SubAssign<Duration>`).
#[elicitation::elicit_tool(
    plugin = "chrono_dt",
    name = "chrono_dt__sub_assign",
    description = "Subtract whole seconds from a stored NaiveDateTime in place (SubAssign). Returns the updated ISO 8601 value.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_dt_sub_assign(
    ctx: Arc<NaiveDateTimeCtx>,
    p: SubAssignParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.dt_id)?;
    let mut guard = ctx.registry.lock().await;
    let dt = guard
        .get_mut(&id)
        .ok_or_else(|| dt_err(format!("dt_id not found: {id}")))?;
    *dt -= parse_duration_secs(p.seconds);
    Ok(ok_text(dt.to_string()))
}

/// Remove a stored `NaiveDateTime` from the registry.
#[elicitation::elicit_tool(
    plugin = "chrono_dt",
    name = "chrono_dt__drop",
    description = "Remove a stored NaiveDateTime from the registry, freeing the handle.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_dt_drop(
    ctx: Arc<NaiveDateTimeCtx>,
    p: DropParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.dt_id)?;
    let removed = ctx.registry.lock().await.remove(&id).is_some();
    if removed {
        Ok(ok_text("dropped"))
    } else {
        Err(dt_err(format!("dt_id not found: {id}")))
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for stateful `NaiveDateTime` mutation via a UUID-keyed registry.
///
/// Exposes `add_assign` and `sub_assign` semantics over MCP by maintaining a
/// server-side registry of `chrono::NaiveDateTime` values.
pub struct NaiveDateTimeRegistryPlugin(Arc<NaiveDateTimeCtx>);

impl NaiveDateTimeRegistryPlugin {
    /// Create a new plugin with an empty registry.
    pub fn new() -> Self {
        Self(Arc::new(NaiveDateTimeCtx::new()))
    }
}

impl Default for NaiveDateTimeRegistryPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for NaiveDateTimeRegistryPlugin {
    fn name(&self) -> &'static str {
        "chrono_dt"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "chrono_dt")
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
            let full_name = if name.starts_with("chrono_dt__") {
                name.to_string()
            } else {
                format!("chrono_dt__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "chrono_dt")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
