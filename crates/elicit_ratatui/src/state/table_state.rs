//! UUID tools for [`ratatui::widgets::TableState`].

use std::sync::Arc;

use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::plugin::{RatatuiCtx, ok_json, ok_text};

/// Parameters for `table_state__new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableStateNewParams {}

/// Parameters for tools that operate on an existing `TableState`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableStateIdParams {
    /// UUID returned by `table_state__new`.
    pub id: Uuid,
}

/// Parameters for `table_state__select_row` / `table_state__select_column`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableStateSelectParams {
    /// UUID returned by `table_state__new`.
    pub id: Uuid,
    /// Index to select, or `null` to deselect.
    pub index: Option<usize>,
}

/// Create a new `TableState` and return its UUID.
#[instrument(skip(ctx))]
pub async fn table_state_new(
    ctx: Arc<RatatuiCtx>,
    _p: TableStateNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.lock_table_states()?
        .insert(id, ratatui::widgets::TableState::default());
    ok_json(&id)
}

/// Select a row by index.
#[instrument(skip(ctx))]
pub async fn table_state_select_row(
    ctx: Arc<RatatuiCtx>,
    p: TableStateSelectParams,
) -> Result<CallToolResult, ErrorData> {
    let mut states = ctx.lock_table_states()?;
    let state = states.get_mut(&p.id).ok_or_else(|| {
        ErrorData::invalid_params(format!("no table_state for id {}", p.id), None)
    })?;
    state.select(p.index);
    ok_text("ok")
}

/// Select a column by index.
#[instrument(skip(ctx))]
pub async fn table_state_select_column(
    ctx: Arc<RatatuiCtx>,
    p: TableStateSelectParams,
) -> Result<CallToolResult, ErrorData> {
    let mut states = ctx.lock_table_states()?;
    let state = states.get_mut(&p.id).ok_or_else(|| {
        ErrorData::invalid_params(format!("no table_state for id {}", p.id), None)
    })?;
    state.select_column(p.index);
    ok_text("ok")
}

/// Return the currently selected row index.
#[instrument(skip(ctx))]
pub async fn table_state_selected_row(
    ctx: Arc<RatatuiCtx>,
    p: TableStateIdParams,
) -> Result<CallToolResult, ErrorData> {
    let states = ctx.lock_table_states()?;
    let state = states.get(&p.id).ok_or_else(|| {
        ErrorData::invalid_params(format!("no table_state for id {}", p.id), None)
    })?;
    ok_json(&state.selected())
}

/// Destroy a `TableState` by UUID.
#[instrument(skip(ctx))]
pub async fn table_state_destroy(
    ctx: Arc<RatatuiCtx>,
    p: TableStateIdParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.lock_table_states()?.remove(&p.id);
    ok_text("ok")
}
