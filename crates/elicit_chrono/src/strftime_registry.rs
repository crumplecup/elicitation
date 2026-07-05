//! `StrftimeItemsPlugin` — stateful MCP plugin for `StrftimeItems` iteration.
//!
//! `StrftimeItems` is an iterator that cannot advance its cursor across MCP tool
//! calls.  This plugin maintains a UUID-keyed server-side registry of in-progress
//! `StrftimeItems` cursors, exposing `next` and `drop` as discrete MCP tools.
//!
//! # Tool namespace: `chrono_strftime__*`

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

use crate::StrftimeItems;

// ── Shared registry type ──────────────────────────────────────────────────────

type SharedRegistry = Arc<Mutex<HashMap<Uuid, StrftimeItems>>>;

// ── Plugin context ─────────────────────────────────────────────────────────────

struct StrftimeItemsCtx {
    registry: SharedRegistry,
}

impl StrftimeItemsCtx {
    fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl elicitation::PluginContext for StrftimeItemsCtx {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn strftime_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_text(s: impl ToString) -> CallToolResult {
    CallToolResult::success(vec![Content::text(s.to_string())])
}

fn parse_uuid(s: &str) -> Result<Uuid, ErrorData> {
    s.parse::<Uuid>()
        .map_err(|_| strftime_err(format!("invalid UUID: {s}")))
}

#[derive(Serialize)]
struct IdResult {
    id: String,
}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `chrono_strftime__create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StrftimeCreateParams {
    /// The strftime format string to parse (e.g. `"%Y-%m-%d"`).
    pub fmt: String,
}

/// Parameters for `chrono_strftime__next`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StrftimeNextParams {
    /// UUID returned by `chrono_strftime__create`.
    pub id: String,
}

/// Parameters for `chrono_strftime__drop`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StrftimeDropParams {
    /// UUID returned by `chrono_strftime__create`.
    pub id: String,
}

// ── Tool functions ────────────────────────────────────────────────────────────

/// Parse a strftime format string and store the iterator in the registry. Returns a UUID handle.
#[elicitation::elicit_tool(
    plugin = "chrono_strftime",
    name = "chrono_strftime__create",
    description = "Parse a strftime format string into a StrftimeItems iterator and store it in the registry. Returns a UUID handle for use with chrono_strftime__next and chrono_strftime__drop.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_strftime_create(
    ctx: Arc<StrftimeItemsCtx>,
    p: StrftimeCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let items = StrftimeItems::new(p.fmt);
    let id = Uuid::new_v4();
    ctx.registry.lock().await.insert(id, items);
    let body = serde_json::to_string(&IdResult { id: id.to_string() }).map_err(strftime_err)?;
    Ok(ok_text(body))
}

/// Advance the iterator cursor and return the next `Item`, or `null` if exhausted.
#[elicitation::elicit_tool(
    plugin = "chrono_strftime",
    name = "chrono_strftime__next",
    description = "Return the next Item from a StrftimeItems iterator cursor, advancing it by one step. Returns the JSON-encoded Item, or null if the iterator is exhausted.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_strftime_next(
    ctx: Arc<StrftimeItemsCtx>,
    p: StrftimeNextParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let mut guard = ctx.registry.lock().await;
    let items = guard
        .get_mut(&id)
        .ok_or_else(|| strftime_err(format!("id not found: {id}")))?;
    let item = items.next();
    let body = serde_json::to_string(&item).map_err(strftime_err)?;
    Ok(ok_text(body))
}

/// Remove a `StrftimeItems` iterator cursor from the registry.
#[elicitation::elicit_tool(
    plugin = "chrono_strftime",
    name = "chrono_strftime__drop",
    description = "Remove a StrftimeItems iterator cursor from the registry, freeing the handle.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_strftime_drop(
    ctx: Arc<StrftimeItemsCtx>,
    p: StrftimeDropParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let removed = ctx.registry.lock().await.remove(&id).is_some();
    if removed {
        Ok(ok_text("dropped"))
    } else {
        Err(strftime_err(format!("id not found: {id}")))
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for stateful `StrftimeItems` iteration via a UUID-keyed registry.
///
/// Provides `next` semantics over MCP by maintaining server-side iteration state.
pub struct StrftimeItemsPlugin(Arc<StrftimeItemsCtx>);

impl StrftimeItemsPlugin {
    /// Create a new plugin with an empty registry.
    pub fn new() -> Self {
        Self(Arc::new(StrftimeItemsCtx::new()))
    }
}

impl Default for StrftimeItemsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for StrftimeItemsPlugin {
    fn name(&self) -> &'static str {
        "chrono_strftime"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "chrono_strftime")
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
            let full_name = if name.starts_with("chrono_strftime__") {
                name.to_string()
            } else {
                format!("chrono_strftime__{name}")
            };
            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "chrono_strftime")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;
            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
