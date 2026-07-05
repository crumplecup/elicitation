//! `NaiveDateRegistryPlugin` — stateful MCP tool plugin for mutable `NaiveDate` operations.
//!
//! `NaiveDate` wrapped in `Arc<T>` cannot implement `AddAssign`/`SubAssign` directly, so this
//! plugin maintains a UUID-keyed server-side registry and exposes mutation as discrete MCP tools.
//!
//! # Tool namespace: `chrono_date__*`

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

/// Shared registry of `NaiveDate` values keyed by UUID.
pub type SharedNaiveDateRegistry = Arc<Mutex<HashMap<Uuid, chrono::NaiveDate>>>;

// ── Plugin context ─────────────────────────────────────────────────────────────

struct NaiveDateCtx {
    registry: SharedNaiveDateRegistry,
}

impl NaiveDateCtx {
    fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl elicitation::PluginContext for NaiveDateCtx {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn date_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_text(s: impl ToString) -> CallToolResult {
    CallToolResult::success(vec![Content::text(s.to_string())])
}

fn parse_uuid(s: &str) -> Result<Uuid, ErrorData> {
    s.parse::<Uuid>()
        .map_err(|_| date_err(format!("invalid UUID: {s}")))
}

fn parse_date(s: &str) -> Result<chrono::NaiveDate, ErrorData> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .or_else(|_| chrono::NaiveDate::parse_from_str(s, "%Y/%m/%d"))
        .map_err(|e| date_err(format!("invalid date '{s}': {e}")))
}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `chrono_date__create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DateCreateParams {
    /// ISO 8601 date string (e.g. `"2024-01-15"`).
    pub date: String,
}

/// Parameters for `chrono_date__get`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DateGetParams {
    /// UUID returned by `chrono_date__create`.
    pub date_id: String,
}

/// Parameters for `chrono_date__add_assign`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DateAddAssignParams {
    /// UUID of the stored `NaiveDate`.
    pub date_id: String,
    /// Number of whole seconds to add (negative to subtract).
    pub seconds: i64,
}

/// Parameters for `chrono_date__sub_assign`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DateSubAssignParams {
    /// UUID of the stored `NaiveDate`.
    pub date_id: String,
    /// Number of whole seconds to subtract (negative to add).
    pub seconds: i64,
}

/// Parameters for `chrono_date__drop`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DateDropParams {
    /// UUID of the stored `NaiveDate` to remove.
    pub date_id: String,
}

#[derive(Serialize)]
struct DateIdResult {
    date_id: String,
}

// ── Tool functions ────────────────────────────────────────────────────────────

/// Parse an ISO 8601 date string and store it in the registry, returning a UUID handle.
#[elicitation::elicit_tool(
    plugin = "chrono_date",
    name = "chrono_date__create",
    description = "Parse an ISO 8601 NaiveDate string and store it in the registry. Returns a UUID handle for subsequent mutation tools.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_date_create(
    ctx: Arc<NaiveDateCtx>,
    p: DateCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let d = parse_date(&p.date)?;
    let id = Uuid::new_v4();
    ctx.registry.lock().await.insert(id, d);
    let body = serde_json::to_string(&DateIdResult {
        date_id: id.to_string(),
    })
    .map_err(date_err)?;
    Ok(ok_text(body))
}

/// Retrieve the current value of a stored `NaiveDate` as an ISO 8601 string.
#[elicitation::elicit_tool(
    plugin = "chrono_date",
    name = "chrono_date__get",
    description = "Return the current ISO 8601 value of a stored NaiveDate.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_date_get(
    ctx: Arc<NaiveDateCtx>,
    p: DateGetParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.date_id)?;
    let guard = ctx.registry.lock().await;
    let d = guard
        .get(&id)
        .ok_or_else(|| date_err(format!("date_id not found: {id}")))?;
    Ok(ok_text(d.to_string()))
}

/// Add a number of whole seconds to a stored `NaiveDate` in place (`AddAssign<Duration>`).
#[elicitation::elicit_tool(
    plugin = "chrono_date",
    name = "chrono_date__add_assign",
    description = "Add whole seconds to a stored NaiveDate in place (AddAssign). Returns the updated ISO 8601 value.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_date_add_assign(
    ctx: Arc<NaiveDateCtx>,
    p: DateAddAssignParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.date_id)?;
    let mut guard = ctx.registry.lock().await;
    let d = guard
        .get_mut(&id)
        .ok_or_else(|| date_err(format!("date_id not found: {id}")))?;
    *d += chrono::TimeDelta::seconds(p.seconds);
    Ok(ok_text(d.to_string()))
}

/// Subtract a number of whole seconds from a stored `NaiveDate` in place (`SubAssign<Duration>`).
#[elicitation::elicit_tool(
    plugin = "chrono_date",
    name = "chrono_date__sub_assign",
    description = "Subtract whole seconds from a stored NaiveDate in place (SubAssign). Returns the updated ISO 8601 value.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_date_sub_assign(
    ctx: Arc<NaiveDateCtx>,
    p: DateSubAssignParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.date_id)?;
    let mut guard = ctx.registry.lock().await;
    let d = guard
        .get_mut(&id)
        .ok_or_else(|| date_err(format!("date_id not found: {id}")))?;
    *d -= chrono::TimeDelta::seconds(p.seconds);
    Ok(ok_text(d.to_string()))
}

/// Remove a stored `NaiveDate` from the registry.
#[elicitation::elicit_tool(
    plugin = "chrono_date",
    name = "chrono_date__drop",
    description = "Remove a stored NaiveDate from the registry, freeing the handle.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_date_drop(
    ctx: Arc<NaiveDateCtx>,
    p: DateDropParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.date_id)?;
    let removed = ctx.registry.lock().await.remove(&id).is_some();
    if removed {
        Ok(ok_text("dropped"))
    } else {
        Err(date_err(format!("date_id not found: {id}")))
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for stateful `NaiveDate` mutation via a UUID-keyed registry.
///
/// Exposes `add_assign` and `sub_assign` semantics over MCP by maintaining a
/// server-side registry of `chrono::NaiveDate` values.
pub struct NaiveDateRegistryPlugin(Arc<NaiveDateCtx>);

impl NaiveDateRegistryPlugin {
    /// Create a new plugin with an empty registry.
    pub fn new() -> Self {
        Self(Arc::new(NaiveDateCtx::new()))
    }
}

impl Default for NaiveDateRegistryPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for NaiveDateRegistryPlugin {
    fn name(&self) -> &'static str {
        "chrono_date"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "chrono_date")
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
            let full_name = if name.starts_with("chrono_date__") {
                name.to_string()
            } else {
                format!("chrono_date__{name}")
            };
            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "chrono_date")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;
            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
