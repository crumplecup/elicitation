//! `DateTimeRegistryPlugin` — stateful MCP tool plugin for mutable `DateTime` (UTC) operations.
//!
//! `DateTime` wrapped in `Arc<T>` cannot implement `AddAssign`/`SubAssign` directly, so this
//! plugin maintains a UUID-keyed server-side registry and exposes mutation as discrete MCP tools.
//!
//! # Tool namespace: `chrono_utc__*`

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

/// Shared registry of `DateTime<Utc>` values keyed by UUID.
pub type SharedDateTimeRegistry = Arc<Mutex<HashMap<Uuid, chrono::DateTime<chrono::Utc>>>>;

// ── Plugin context ─────────────────────────────────────────────────────────────

struct DateTimeCtx {
    registry: SharedDateTimeRegistry,
}

impl DateTimeCtx {
    fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl elicitation::PluginContext for DateTimeCtx {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn utc_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_text(s: impl ToString) -> CallToolResult {
    CallToolResult::success(vec![Content::text(s.to_string())])
}

fn parse_uuid(s: &str) -> Result<Uuid, ErrorData> {
    s.parse::<Uuid>()
        .map_err(|_| utc_err(format!("invalid UUID: {s}")))
}

fn parse_utc(s: &str) -> Result<chrono::DateTime<chrono::Utc>, ErrorData> {
    s.parse::<chrono::DateTime<chrono::Utc>>()
        .or_else(|_| {
            chrono::DateTime::parse_from_rfc2822(s).map(|dt| dt.with_timezone(&chrono::Utc))
        })
        .map_err(|e| utc_err(format!("invalid datetime '{s}': {e}")))
}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `chrono_utc__create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UtcCreateParams {
    /// RFC 3339 datetime string (e.g. `"2024-01-15T12:30:00Z"`).
    pub datetime: String,
}

/// Parameters for `chrono_utc__get`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UtcGetParams {
    /// UUID returned by `chrono_utc__create`.
    pub utc_id: String,
}

/// Parameters for `chrono_utc__add_assign`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UtcAddAssignParams {
    /// UUID of the stored `DateTime`.
    pub utc_id: String,
    /// Number of whole seconds to add (negative to subtract).
    pub seconds: i64,
}

/// Parameters for `chrono_utc__sub_assign`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UtcSubAssignParams {
    /// UUID of the stored `DateTime`.
    pub utc_id: String,
    /// Number of whole seconds to subtract (negative to add).
    pub seconds: i64,
}

/// Parameters for `chrono_utc__drop`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UtcDropParams {
    /// UUID of the stored `DateTime` to remove.
    pub utc_id: String,
}

#[derive(Serialize)]
struct UtcIdResult {
    utc_id: String,
}

// ── Tool functions ────────────────────────────────────────────────────────────

/// Parse an RFC 3339 datetime string and store it in the registry, returning a UUID handle.
#[elicitation::elicit_tool(
    plugin = "chrono_utc",
    name = "chrono_utc__create",
    description = "Parse an RFC 3339 DateTime<Utc> string and store it in the registry. Returns a UUID handle for subsequent mutation tools.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_utc_create(
    ctx: Arc<DateTimeCtx>,
    p: UtcCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let dt = parse_utc(&p.datetime)?;
    let id = Uuid::new_v4();
    ctx.registry.lock().await.insert(id, dt);
    let body = serde_json::to_string(&UtcIdResult {
        utc_id: id.to_string(),
    })
    .map_err(|e| utc_err(e))?;
    Ok(ok_text(body))
}

/// Retrieve the current value of a stored `DateTime` as an RFC 3339 string.
#[elicitation::elicit_tool(
    plugin = "chrono_utc",
    name = "chrono_utc__get",
    description = "Return the current RFC 3339 value of a stored DateTime<Utc>.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_utc_get(
    ctx: Arc<DateTimeCtx>,
    p: UtcGetParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.utc_id)?;
    let guard = ctx.registry.lock().await;
    let dt = guard
        .get(&id)
        .ok_or_else(|| utc_err(format!("utc_id not found: {id}")))?;
    Ok(ok_text(dt.to_rfc3339()))
}

/// Add a number of whole seconds to a stored `DateTime` in place (`AddAssign<Duration>`).
#[elicitation::elicit_tool(
    plugin = "chrono_utc",
    name = "chrono_utc__add_assign",
    description = "Add whole seconds to a stored DateTime<Utc> in place (AddAssign). Returns the updated RFC 3339 value.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_utc_add_assign(
    ctx: Arc<DateTimeCtx>,
    p: UtcAddAssignParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.utc_id)?;
    let mut guard = ctx.registry.lock().await;
    let dt = guard
        .get_mut(&id)
        .ok_or_else(|| utc_err(format!("utc_id not found: {id}")))?;
    *dt += chrono::TimeDelta::seconds(p.seconds);
    Ok(ok_text(dt.to_rfc3339()))
}

/// Subtract a number of whole seconds from a stored `DateTime` in place (`SubAssign<Duration>`).
#[elicitation::elicit_tool(
    plugin = "chrono_utc",
    name = "chrono_utc__sub_assign",
    description = "Subtract whole seconds from a stored DateTime<Utc> in place (SubAssign). Returns the updated RFC 3339 value.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_utc_sub_assign(
    ctx: Arc<DateTimeCtx>,
    p: UtcSubAssignParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.utc_id)?;
    let mut guard = ctx.registry.lock().await;
    let dt = guard
        .get_mut(&id)
        .ok_or_else(|| utc_err(format!("utc_id not found: {id}")))?;
    *dt -= chrono::TimeDelta::seconds(p.seconds);
    Ok(ok_text(dt.to_rfc3339()))
}

/// Remove a stored `DateTime` from the registry.
#[elicitation::elicit_tool(
    plugin = "chrono_utc",
    name = "chrono_utc__drop",
    description = "Remove a stored DateTime<Utc> from the registry, freeing the handle.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_utc_drop(
    ctx: Arc<DateTimeCtx>,
    p: UtcDropParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.utc_id)?;
    let removed = ctx.registry.lock().await.remove(&id).is_some();
    if removed {
        Ok(ok_text("dropped"))
    } else {
        Err(utc_err(format!("utc_id not found: {id}")))
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for stateful `DateTime<Utc>` mutation via a UUID-keyed registry.
///
/// Exposes `add_assign` and `sub_assign` semantics over MCP by maintaining a
/// server-side registry of `chrono::DateTime<chrono::Utc>` values.
pub struct DateTimeRegistryPlugin(Arc<DateTimeCtx>);

impl DateTimeRegistryPlugin {
    /// Create a new plugin with an empty registry.
    pub fn new() -> Self {
        Self(Arc::new(DateTimeCtx::new()))
    }
}

impl Default for DateTimeRegistryPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for DateTimeRegistryPlugin {
    fn name(&self) -> &'static str {
        "chrono_utc"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "chrono_utc")
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
            let full_name = if name.starts_with("chrono_utc__") {
                name.to_string()
            } else {
                format!("chrono_utc__{name}")
            };
            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "chrono_utc")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;
            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
