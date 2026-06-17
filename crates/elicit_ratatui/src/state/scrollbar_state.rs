//! UUID tools for [`ratatui::widgets::ScrollbarState`].

use std::sync::Arc;

use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::plugin::{RatatuiCtx, ok_json, ok_text};

/// Parameters for `scrollbar_state__new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScrollbarStateNewParams {
    /// Total content length (number of scrollable units).
    pub content_length: usize,
}

/// Parameters for tools that operate on an existing `ScrollbarState`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScrollbarStateIdParams {
    /// UUID returned by `scrollbar_state__new`.
    pub id: Uuid,
}

/// Parameters for `scrollbar_state__set_position`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScrollbarStateSetPositionParams {
    /// UUID returned by `scrollbar_state__new`.
    pub id: Uuid,
    /// New scroll position.
    pub position: usize,
}

/// Create a new `ScrollbarState` and return its UUID.
#[instrument(skip(ctx))]
pub async fn scrollbar_state_new(
    ctx: Arc<RatatuiCtx>,
    p: ScrollbarStateNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.lock_scrollbar_states()?
        .insert(id, ratatui::widgets::ScrollbarState::new(p.content_length));
    ok_json(&id)
}

/// Return the current scroll position.
#[instrument(skip(ctx))]
pub async fn scrollbar_state_position(
    ctx: Arc<RatatuiCtx>,
    p: ScrollbarStateIdParams,
) -> Result<CallToolResult, ErrorData> {
    let states = ctx.lock_scrollbar_states()?;
    let state = states.get(&p.id).ok_or_else(|| {
        ErrorData::invalid_params(format!("no scrollbar_state for id {}", p.id), None)
    })?;
    ok_json(&state.get_position())
}

/// Set the scroll position.
#[instrument(skip(ctx))]
pub async fn scrollbar_state_set_position(
    ctx: Arc<RatatuiCtx>,
    p: ScrollbarStateSetPositionParams,
) -> Result<CallToolResult, ErrorData> {
    let mut states = ctx.lock_scrollbar_states()?;
    let state = states.get_mut(&p.id).ok_or_else(|| {
        ErrorData::invalid_params(format!("no scrollbar_state for id {}", p.id), None)
    })?;
    *state = state.clone().position(p.position);
    ok_text("ok")
}

/// Destroy a `ScrollbarState` by UUID.
#[instrument(skip(ctx))]
pub async fn scrollbar_state_destroy(
    ctx: Arc<RatatuiCtx>,
    p: ScrollbarStateIdParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.lock_scrollbar_states()?.remove(&p.id);
    ok_text("ok")
}
