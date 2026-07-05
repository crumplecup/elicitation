//! `NaiveDateIterPlugin` — stateful MCP tool plugin for `NaiveDate` iterator operations.
//!
//! `iter_days` and `iter_weeks` on `chrono::NaiveDate` return lazy iterators that cannot
//! cross the MCP tool boundary. This plugin maintains a UUID-keyed server-side registry of
//! in-progress `NaiveDate` iteration cursors, exposing `next` and `drop` as discrete MCP tools.
//!
//! # Tool namespace: `chrono_date_iter__*`

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

// ── Iterator state ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
enum IterStep {
    Days,
    Weeks,
}

#[derive(Debug, Clone)]
struct IterCursor {
    current: chrono::NaiveDate,
    step: IterStep,
}

impl IterCursor {
    fn advance(&mut self) {
        self.current = match self.step {
            IterStep::Days => self.current.succ_opt().unwrap_or(self.current),
            IterStep::Weeks => self
                .current
                .checked_add_days(chrono::Days::new(7))
                .unwrap_or(self.current),
        };
    }
}

// ── Shared registry type ──────────────────────────────────────────────────────

type SharedIterRegistry = Arc<Mutex<HashMap<Uuid, IterCursor>>>;

// ── Plugin context ─────────────────────────────────────────────────────────────

struct NaiveDateIterCtx {
    registry: SharedIterRegistry,
}

impl NaiveDateIterCtx {
    fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl elicitation::PluginContext for NaiveDateIterCtx {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn iter_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_text(s: impl ToString) -> CallToolResult {
    CallToolResult::success(vec![Content::text(s.to_string())])
}

fn parse_uuid(s: &str) -> Result<Uuid, ErrorData> {
    s.parse::<Uuid>()
        .map_err(|_| iter_err(format!("invalid UUID: {s}")))
}

fn parse_date(s: &str) -> Result<chrono::NaiveDate, ErrorData> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .or_else(|_| chrono::NaiveDate::parse_from_str(s, "%Y/%m/%d"))
        .map_err(|e| iter_err(format!("invalid date '{s}': {e}")))
}

#[derive(Serialize)]
struct IterIdResult {
    iter_id: String,
}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `chrono_date_iter__iter_days`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct IterDaysParams {
    /// ISO 8601 start date (e.g. `"2024-01-15"`).
    pub start: String,
}

/// Parameters for `chrono_date_iter__iter_weeks`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct IterWeeksParams {
    /// ISO 8601 start date (e.g. `"2024-01-15"`).
    pub start: String,
}

/// Parameters for `chrono_date_iter__next`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct IterNextParams {
    /// UUID returned by `chrono_date_iter__iter_days` or `chrono_date_iter__iter_weeks`.
    pub iter_id: String,
}

/// Parameters for `chrono_date_iter__drop`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct IterDropParams {
    /// UUID returned by `chrono_date_iter__iter_days` or `chrono_date_iter__iter_weeks`.
    pub iter_id: String,
}

// ── Tool functions ────────────────────────────────────────────────────────────

/// Create a day-by-day iterator cursor starting at `start`. Returns a UUID handle.
#[elicitation::elicit_tool(
    plugin = "chrono_date_iter",
    name = "chrono_date_iter__iter_days",
    description = "Create a stateful day-by-day NaiveDate iterator starting at the given date. Returns a UUID handle. Use chrono_date_iter__next to advance and chrono_date_iter__drop to free.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_date_iter_iter_days(
    ctx: Arc<NaiveDateIterCtx>,
    p: IterDaysParams,
) -> Result<CallToolResult, ErrorData> {
    let start = parse_date(&p.start)?;
    let id = Uuid::new_v4();
    ctx.registry.lock().await.insert(
        id,
        IterCursor {
            current: start,
            step: IterStep::Days,
        },
    );
    let body = serde_json::to_string(&IterIdResult {
        iter_id: id.to_string(),
    })
    .map_err(iter_err)?;
    Ok(ok_text(body))
}

/// Create a week-by-week iterator cursor starting at `start`. Returns a UUID handle.
#[elicitation::elicit_tool(
    plugin = "chrono_date_iter",
    name = "chrono_date_iter__iter_weeks",
    description = "Create a stateful week-by-week NaiveDate iterator starting at the given date. Returns a UUID handle. Use chrono_date_iter__next to advance and chrono_date_iter__drop to free.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_date_iter_iter_weeks(
    ctx: Arc<NaiveDateIterCtx>,
    p: IterWeeksParams,
) -> Result<CallToolResult, ErrorData> {
    let start = parse_date(&p.start)?;
    let id = Uuid::new_v4();
    ctx.registry.lock().await.insert(
        id,
        IterCursor {
            current: start,
            step: IterStep::Weeks,
        },
    );
    let body = serde_json::to_string(&IterIdResult {
        iter_id: id.to_string(),
    })
    .map_err(iter_err)?;
    Ok(ok_text(body))
}

/// Return the current date of the iterator cursor, then advance it by one step.
#[elicitation::elicit_tool(
    plugin = "chrono_date_iter",
    name = "chrono_date_iter__next",
    description = "Return the current date of a NaiveDate iterator cursor and advance it by one step (one day or one week). Returns the ISO 8601 date before the advance.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_date_iter_next(
    ctx: Arc<NaiveDateIterCtx>,
    p: IterNextParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.iter_id)?;
    let mut guard = ctx.registry.lock().await;
    let cursor = guard
        .get_mut(&id)
        .ok_or_else(|| iter_err(format!("iter_id not found: {id}")))?;
    let current = cursor.current.to_string();
    cursor.advance();
    Ok(ok_text(current))
}

/// Remove an iterator cursor from the registry, freeing the handle.
#[elicitation::elicit_tool(
    plugin = "chrono_date_iter",
    name = "chrono_date_iter__drop",
    description = "Remove a NaiveDate iterator cursor from the registry, freeing the handle.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn chrono_date_iter_drop(
    ctx: Arc<NaiveDateIterCtx>,
    p: IterDropParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.iter_id)?;
    let removed = ctx.registry.lock().await.remove(&id).is_some();
    if removed {
        Ok(ok_text("dropped"))
    } else {
        Err(iter_err(format!("iter_id not found: {id}")))
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for stateful `NaiveDate` iterator cursors via a UUID-keyed registry.
///
/// Provides `iter_days` and `iter_weeks` semantics over MCP by maintaining
/// server-side iteration state keyed by UUID.
pub struct NaiveDateIterPlugin(Arc<NaiveDateIterCtx>);

impl NaiveDateIterPlugin {
    /// Create a new plugin with an empty iterator registry.
    pub fn new() -> Self {
        Self(Arc::new(NaiveDateIterCtx::new()))
    }
}

impl Default for NaiveDateIterPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for NaiveDateIterPlugin {
    fn name(&self) -> &'static str {
        "chrono_date_iter"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "chrono_date_iter")
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
            let full_name = if name.starts_with("chrono_date_iter__") {
                name.to_string()
            } else {
                format!("chrono_date_iter__{name}")
            };
            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "chrono_date_iter")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;
            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
