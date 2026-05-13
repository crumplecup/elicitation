//! [`RedbPlugin`] — stateful plugin context for redb live objects.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use elicitation::{PluginContext, PluginToolRegistration, StatefulPlugin, ToolDescriptor};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Tool},
};
use uuid::Uuid;

// ── Context ──────────────────────────────────────────────────────────────────

/// Shared context holding all live redb objects keyed by UUID.
pub struct RedbCtx {
    /// Open `redb::Database` instances.
    pub(crate) databases: Mutex<HashMap<Uuid, redb::Database>>,
    /// Active write transactions.
    pub(crate) write_txns: Mutex<HashMap<Uuid, redb::WriteTransaction>>,
    /// Active read transactions.
    pub(crate) read_txns: Mutex<HashMap<Uuid, redb::ReadTransaction>>,
    /// Savepoints created during write transactions.
    pub(crate) savepoints: Mutex<HashMap<Uuid, redb::Savepoint>>,
}

impl RedbCtx {
    fn new() -> Self {
        Self {
            databases: Mutex::new(HashMap::new()),
            write_txns: Mutex::new(HashMap::new()),
            read_txns: Mutex::new(HashMap::new()),
            savepoints: Mutex::new(HashMap::new()),
        }
    }

    /// Acquires the databases map, returning an error if the mutex is poisoned.
    pub fn lock_databases(
        &self,
    ) -> Result<MutexGuard<'_, HashMap<Uuid, redb::Database>>, ErrorData> {
        self.databases
            .lock()
            .map_err(|_| ErrorData::internal_error("redb databases lock poisoned", None))
    }

    /// Acquires the write transactions map, returning an error if the mutex is poisoned.
    pub fn lock_write_txns(
        &self,
    ) -> Result<MutexGuard<'_, HashMap<Uuid, redb::WriteTransaction>>, ErrorData> {
        self.write_txns
            .lock()
            .map_err(|_| ErrorData::internal_error("redb write_txns lock poisoned", None))
    }

    /// Acquires the read transactions map, returning an error if the mutex is poisoned.
    pub fn lock_read_txns(
        &self,
    ) -> Result<MutexGuard<'_, HashMap<Uuid, redb::ReadTransaction>>, ErrorData> {
        self.read_txns
            .lock()
            .map_err(|_| ErrorData::internal_error("redb read_txns lock poisoned", None))
    }

    /// Acquires the savepoints map, returning an error if the mutex is poisoned.
    pub fn lock_savepoints(
        &self,
    ) -> Result<MutexGuard<'_, HashMap<Uuid, redb::Savepoint>>, ErrorData> {
        self.savepoints
            .lock()
            .map_err(|_| ErrorData::internal_error("redb savepoints lock poisoned", None))
    }
}

impl std::fmt::Debug for RedbCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n_db = self.databases.lock().map(|m| m.len()).unwrap_or(0);
        let n_wt = self.write_txns.lock().map(|m| m.len()).unwrap_or(0);
        let n_rt = self.read_txns.lock().map(|m| m.len()).unwrap_or(0);
        let n_sp = self.savepoints.lock().map(|m| m.len()).unwrap_or(0);
        f.debug_struct("RedbCtx")
            .field("databases", &n_db)
            .field("write_txns", &n_wt)
            .field("read_txns", &n_rt)
            .field("savepoints", &n_sp)
            .finish()
    }
}

impl PluginContext for RedbCtx {}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Stateful MCP plugin for the redb embedded key-value store.
///
/// Holds all live redb objects in a shared [`RedbCtx`] and exposes them via
/// UUID handles.  Create a single instance and register it with your MCP
/// server; all `redb__*` tools will share the same context.
pub struct RedbPlugin(Arc<RedbCtx>);

impl RedbPlugin {
    /// Creates a new plugin with an empty context.
    pub fn new() -> Self {
        Self(Arc::new(RedbCtx::new()))
    }

    /// Returns a shared reference to the underlying context.
    pub fn ctx(&self) -> Arc<RedbCtx> {
        Arc::clone(&self.0)
    }
}

impl std::fmt::Debug for RedbPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RedbPlugin").field(&self.0).finish()
    }
}

impl Default for RedbPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulPlugin for RedbPlugin {
    type Context = RedbCtx;

    fn name(&self) -> &'static str {
        "redb"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|r| r.plugin == "redb")
            .map(|r| (r.constructor)().as_tool())
            .collect()
    }

    fn tool_descriptors(&self) -> Vec<ToolDescriptor> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|r| r.plugin == "redb")
            .map(|r| (r.constructor)())
            .collect()
    }

    fn context(&self) -> Arc<RedbCtx> {
        self.0.clone()
    }
}

// ── Shared helpers ────────────────────────────────────────────────────────────

pub(crate) fn parse_uuid(s: &str) -> Result<Uuid, ErrorData> {
    s.parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {s}"), None))
}

pub(crate) fn ok_text(s: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        s.into(),
    )]))
}

pub(crate) fn ok_json<T: serde::Serialize>(v: &T) -> Result<CallToolResult, ErrorData> {
    let text = serde_json::to_string(v)
        .map_err(|e| ErrorData::internal_error(format!("serialization error: {e}"), None))?;
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        text,
    )]))
}
