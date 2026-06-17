//! UUID tools for [`ratatui::widgets::ListState`].

use std::sync::Arc;

use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::plugin::{RatatuiCtx, ok_json, ok_text};

// ── Param types ───────────────────────────────────────────────────────────────

/// Parameters for `list_state__new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListStateNewParams {}

/// Parameters for tools that operate on an existing `ListState`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListStateIdParams {
    /// UUID returned by `list_state__new`.
    pub id: Uuid,
}

/// Parameters for `list_state__select`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListStateSelectParams {
    /// UUID returned by `list_state__new`.
    pub id: Uuid,
    /// Index to select, or `null` to deselect.
    pub index: Option<usize>,
}

// ── Tool functions ────────────────────────────────────────────────────────────

/// Create a new `ListState` and return its UUID.
#[instrument(skip(ctx))]
pub async fn list_state_new(
    ctx: Arc<RatatuiCtx>,
    _p: ListStateNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.lock_list_states()?
        .insert(id, ratatui::widgets::ListState::default());
    ok_json(&id)
}

/// Select an item by index (or deselect with `null`).
#[instrument(skip(ctx))]
pub async fn list_state_select(
    ctx: Arc<RatatuiCtx>,
    p: ListStateSelectParams,
) -> Result<CallToolResult, ErrorData> {
    let mut states = ctx.lock_list_states()?;
    let state = states
        .get_mut(&p.id)
        .ok_or_else(|| ErrorData::invalid_params(format!("no list_state for id {}", p.id), None))?;
    state.select(p.index);
    ok_text("ok")
}

/// Return the currently selected index.
#[instrument(skip(ctx))]
pub async fn list_state_selected(
    ctx: Arc<RatatuiCtx>,
    p: ListStateIdParams,
) -> Result<CallToolResult, ErrorData> {
    let states = ctx.lock_list_states()?;
    let state = states
        .get(&p.id)
        .ok_or_else(|| ErrorData::invalid_params(format!("no list_state for id {}", p.id), None))?;
    ok_json(&state.selected())
}

/// Return the current scroll offset.
#[instrument(skip(ctx))]
pub async fn list_state_offset(
    ctx: Arc<RatatuiCtx>,
    p: ListStateIdParams,
) -> Result<CallToolResult, ErrorData> {
    let states = ctx.lock_list_states()?;
    let state = states
        .get(&p.id)
        .ok_or_else(|| ErrorData::invalid_params(format!("no list_state for id {}", p.id), None))?;
    ok_json(&state.offset())
}

/// Destroy a `ListState` by UUID.
#[instrument(skip(ctx))]
pub async fn list_state_destroy(
    ctx: Arc<RatatuiCtx>,
    p: ListStateIdParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.lock_list_states()?.remove(&p.id);
    ok_text("ok")
}
