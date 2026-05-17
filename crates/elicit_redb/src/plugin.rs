//! [`RedbPlugin`] — stateful MCP plugin for the redb embedded key-value store.
//!
//! Lock ordering (when holding multiple guards simultaneously):
//! `savepoints` → `write_txns` → `read_txns` → `databases`

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

/// Shared context holding all live redb objects keyed by UUID.
pub struct RedbCtx {
    pub(crate) databases: Mutex<HashMap<Uuid, redb::Database>>,
    pub(crate) write_txns: Mutex<HashMap<Uuid, redb::WriteTransaction>>,
    pub(crate) read_txns: Mutex<HashMap<Uuid, redb::ReadTransaction>>,
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

    /// Lock the databases map.
    pub fn lock_databases(
        &self,
    ) -> Result<MutexGuard<'_, HashMap<Uuid, redb::Database>>, ErrorData> {
        self.databases
            .lock()
            .map_err(|_| ErrorData::internal_error("redb databases lock poisoned", None))
    }

    /// Lock the write transactions map.
    pub fn lock_write_txns(
        &self,
    ) -> Result<MutexGuard<'_, HashMap<Uuid, redb::WriteTransaction>>, ErrorData> {
        self.write_txns
            .lock()
            .map_err(|_| ErrorData::internal_error("redb write_txns lock poisoned", None))
    }

    /// Lock the read transactions map.
    pub fn lock_read_txns(
        &self,
    ) -> Result<MutexGuard<'_, HashMap<Uuid, redb::ReadTransaction>>, ErrorData> {
        self.read_txns
            .lock()
            .map_err(|_| ErrorData::internal_error("redb read_txns lock poisoned", None))
    }

    /// Lock the savepoints map.
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
        f.debug_struct("RedbCtx")
            .field(
                "databases",
                &self.databases.lock().map(|m| m.len()).unwrap_or(0),
            )
            .field(
                "write_txns",
                &self.write_txns.lock().map(|m| m.len()).unwrap_or(0),
            )
            .field(
                "read_txns",
                &self.read_txns.lock().map(|m| m.len()).unwrap_or(0),
            )
            .field(
                "savepoints",
                &self.savepoints.lock().map(|m| m.len()).unwrap_or(0),
            )
            .finish()
    }
}

impl PluginContext for RedbCtx {}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Stateful MCP plugin for the redb embedded key-value store.
///
/// Holds all live redb objects in a shared [`RedbCtx`] keyed by UUID.
/// Register a single instance with your MCP server; all `redb__*` tools share
/// the same context.
#[derive(ElicitPlugin)]
#[plugin(name = "redb")]
pub struct RedbPlugin(pub Arc<RedbCtx>);

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

impl Default for RedbPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for RedbPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RedbPlugin").field(&self.0).finish()
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
