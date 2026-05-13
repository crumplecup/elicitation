//! [`TomlPlugin`] — stateful MCP plugin holding live toml_edit objects.
//!
//! Four UUID-keyed maps:
//! - `documents`     — live [`toml_edit::DocumentMut`] instances
//! - `tables`        — live [`toml_edit::Table`] instances
//! - `arrays`        — live [`toml_edit::Array`] instances
//! - `inline_tables` — live [`toml_edit::InlineTable`] instances

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use elicitation::{ElicitPlugin, plugin::PluginContext};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use uuid::Uuid;

pub struct TomlCtx {
    pub documents: Mutex<HashMap<Uuid, toml_edit::DocumentMut>>,
    pub tables: Mutex<HashMap<Uuid, toml_edit::Table>>,
    pub arrays: Mutex<HashMap<Uuid, toml_edit::Array>>,
    pub inline_tables: Mutex<HashMap<Uuid, toml_edit::InlineTable>>,
}

impl std::fmt::Debug for TomlCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TomlCtx")
            .field(
                "documents",
                &self.documents.lock().map(|g| g.len()).unwrap_or(0),
            )
            .field("tables", &self.tables.lock().map(|g| g.len()).unwrap_or(0))
            .field("arrays", &self.arrays.lock().map(|g| g.len()).unwrap_or(0))
            .field(
                "inline_tables",
                &self.inline_tables.lock().map(|g| g.len()).unwrap_or(0),
            )
            .finish()
    }
}

impl Default for TomlCtx {
    fn default() -> Self {
        TomlCtx {
            documents: Mutex::new(HashMap::new()),
            tables: Mutex::new(HashMap::new()),
            arrays: Mutex::new(HashMap::new()),
            inline_tables: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for TomlCtx {}

/// MCP plugin for TOML document editing operations.
///
/// Holds live [`toml_edit::DocumentMut`], [`toml_edit::Table`],
/// [`toml_edit::Array`], and [`toml_edit::InlineTable`] instances keyed by UUID.
#[derive(ElicitPlugin)]
#[plugin(name = "toml")]
pub struct TomlPlugin(pub Arc<TomlCtx>);

impl TomlPlugin {
    pub fn new() -> Self {
        TomlPlugin(Arc::new(TomlCtx::default()))
    }
}

impl Default for TomlPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for TomlPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TomlPlugin").field(&self.0).finish()
    }
}

// ── Shared helpers ────────────────────────────────────────────────────────────

/// Parse a UUID string, returning an [`ErrorData`] on failure.
pub(crate) fn parse_uuid(s: &str) -> Result<Uuid, ErrorData> {
    s.parse::<Uuid>()
        .map_err(|e| ErrorData::invalid_params(format!("invalid UUID '{}': {}", s, e), None))
}

/// Wrap a plain text message in a successful [`CallToolResult`].
pub(crate) fn ok_text(msg: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(msg.into())]))
}

/// Serialize a value as JSON and wrap in a successful [`CallToolResult`].
pub(crate) fn ok_json<T: serde::Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    match serde_json::to_string(value) {
        Ok(s) => Ok(CallToolResult::success(vec![Content::text(s)])),
        Err(e) => ok_text(format!("serialization error: {}", e)),
    }
}

/// Return an error result with an invalid_params [`ErrorData`].
pub(crate) fn err_text(msg: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Err(ErrorData::invalid_params(msg.into(), None))
}
