//! [`RatatuiPlugin`] — stateful MCP plugin for ratatui.
//!
//! Lock ordering (when holding multiple guards simultaneously):
//! `list_states` → `table_states` → `scrollbar_states` → `terminals`

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use elicitation::{ElicitPlugin, plugin::PluginContext};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use uuid::Uuid;

// ── Context ───────────────────────────────────────────────────────────────────

#[cfg(feature = "runtime")]
type TerminalMap =
    HashMap<Uuid, ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>>;

/// Shared context holding all live ratatui objects keyed by UUID.
pub struct RatatuiCtx {
    pub(crate) list_states: Mutex<HashMap<Uuid, ratatui::widgets::ListState>>,
    pub(crate) table_states: Mutex<HashMap<Uuid, ratatui::widgets::TableState>>,
    pub(crate) scrollbar_states: Mutex<HashMap<Uuid, ratatui::widgets::ScrollbarState>>,
    #[cfg(feature = "runtime")]
    pub(crate) terminals: Mutex<
        HashMap<Uuid, ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>>,
    >,
}

impl RatatuiCtx {
    fn new() -> Self {
        Self {
            list_states: Mutex::new(HashMap::new()),
            table_states: Mutex::new(HashMap::new()),
            scrollbar_states: Mutex::new(HashMap::new()),
            #[cfg(feature = "runtime")]
            terminals: Mutex::new(HashMap::new()),
        }
    }

    /// Lock the list states map.
    pub fn lock_list_states(
        &self,
    ) -> Result<MutexGuard<'_, HashMap<Uuid, ratatui::widgets::ListState>>, ErrorData> {
        self.list_states
            .lock()
            .map_err(|_| ErrorData::internal_error("ratatui list_states lock poisoned", None))
    }

    /// Lock the table states map.
    pub fn lock_table_states(
        &self,
    ) -> Result<MutexGuard<'_, HashMap<Uuid, ratatui::widgets::TableState>>, ErrorData> {
        self.table_states
            .lock()
            .map_err(|_| ErrorData::internal_error("ratatui table_states lock poisoned", None))
    }

    /// Lock the scrollbar states map.
    pub fn lock_scrollbar_states(
        &self,
    ) -> Result<MutexGuard<'_, HashMap<Uuid, ratatui::widgets::ScrollbarState>>, ErrorData> {
        self.scrollbar_states
            .lock()
            .map_err(|_| ErrorData::internal_error("ratatui scrollbar_states lock poisoned", None))
    }

    #[cfg(feature = "runtime")]
    /// Lock the terminals map.
    pub fn lock_terminals(
        &self,
    ) -> Result<MutexGuard<'_, TerminalMap>, ErrorData> {
        self.terminals
            .lock()
            .map_err(|_| ErrorData::internal_error("ratatui terminals lock poisoned", None))
    }
}

impl std::fmt::Debug for RatatuiCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RatatuiCtx")
            .field(
                "list_states",
                &self.list_states.lock().map(|m| m.len()).unwrap_or(0),
            )
            .field(
                "table_states",
                &self.table_states.lock().map(|m| m.len()).unwrap_or(0),
            )
            .field(
                "scrollbar_states",
                &self.scrollbar_states.lock().map(|m| m.len()).unwrap_or(0),
            )
            .finish()
    }
}

impl PluginContext for RatatuiCtx {}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Stateful MCP plugin for ratatui.
///
/// Holds all live ratatui objects in a shared [`RatatuiCtx`] keyed by UUID.
/// Register a single instance with your MCP server; all `ratatui__*` tools
/// share the same context.
#[derive(ElicitPlugin)]
#[plugin(name = "ratatui")]
pub struct RatatuiPlugin(pub Arc<RatatuiCtx>);

impl RatatuiPlugin {
    /// Creates a new plugin with an empty context.
    pub fn new() -> Self {
        Self(Arc::new(RatatuiCtx::new()))
    }

    /// Returns a shared reference to the underlying context.
    pub fn ctx(&self) -> Arc<RatatuiCtx> {
        Arc::clone(&self.0)
    }
}

impl Default for RatatuiPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for RatatuiPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RatatuiPlugin").field(&self.0).finish()
    }
}

// ── Shared helpers ────────────────────────────────────────────────────────────

/// Wrap a text message in a successful [`CallToolResult`].
pub(crate) fn ok_text(msg: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(msg.into())]))
}

/// Serialize a value as JSON in a successful [`CallToolResult`].
pub(crate) fn ok_json<T: serde::Serialize>(v: &T) -> Result<CallToolResult, ErrorData> {
    match serde_json::to_string(v) {
        Ok(s) => Ok(CallToolResult::success(vec![Content::text(s)])),
        Err(e) => Err(ErrorData::internal_error(
            format!("serialization error: {e}"),
            None,
        )),
    }
}
