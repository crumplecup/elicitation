//! `ArchiveKvPlugin` — embedded key-value store tools via `ArchiveKvBackend`.
//!
//! Wraps [`DbKvStore`], [`DbEmbeddedStore`], [`DbTransactor`], and
//! [`DbSnapshotManager`] methods to expose them as MCP tools.  Each tool
//! opens (or creates) the redb database at the given path, performs its
//! operation, and returns the result as a serialised descriptor type.
//!
//! Keys and values are accepted as plain strings and stored as
//! [`DbValue::Text`]; scan results render all [`DbValue`] variants to JSON
//! strings for portability.

use elicit_db::{DbEmbeddedStore, DbKvStore, DbSnapshotManager, DbValue, SnapshotHandle};
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, contracts::Established, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::archive::backend::ArchiveKvBackend;
use crate::archive::{KvEntryDescriptor, KvScanResult, KvSnapshotDescriptor, KvStatsDescriptor};

// ── helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        serde_json::to_string(value).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

fn db_err(e: impl std::fmt::Display) -> ErrorData {
    ErrorData::internal_error(e.to_string(), None)
}

fn open(path: &str) -> Result<ArchiveKvBackend, ErrorData> {
    ArchiveKvBackend::open(path).map_err(db_err)
}

/// Render a [`DbValue`] to a display string for descriptor types.
fn dbvalue_display(v: &DbValue) -> String {
    match v {
        DbValue::Text(s) => s.clone(),
        other => serde_json::to_string(other).unwrap_or_else(|_| format!("{other:?}")),
    }
}

// ── propositions ──────────────────────────────────────────────────────────────

/// Proposition: KV storage statistics were successfully read.
#[derive(Prop)]
pub struct KvStatsRead;
impl VerifiedWorkflow for KvStatsRead {}

/// Proposition: a KV table scan completed successfully.
#[derive(Prop)]
pub struct KvScanCompleted;
impl VerifiedWorkflow for KvScanCompleted {}

/// Proposition: a KV get operation completed (key may or may not be present).
#[derive(Prop)]
pub struct KvGetCompleted;
impl VerifiedWorkflow for KvGetCompleted {}

/// Proposition: a KV range scan completed successfully.
#[derive(Prop)]
pub struct KvRangeCompleted;
impl VerifiedWorkflow for KvRangeCompleted {}

/// Proposition: the KV database was compacted successfully.
#[derive(Prop)]
pub struct KvCompactCompleted;
impl VerifiedWorkflow for KvCompactCompleted {}

/// Proposition: KV database integrity was verified.
#[derive(Prop)]
pub struct KvIntegrityChecked;
impl VerifiedWorkflow for KvIntegrityChecked {}

/// Proposition: a KV snapshot list was successfully read.
#[derive(Prop)]
pub struct KvSnapshotListRead;
impl VerifiedWorkflow for KvSnapshotListRead {}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `archive_kv__storage_stats`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KvStorageStatsParams {
    /// Filesystem path to the redb database file.
    pub path: String,
}

/// Parameters for `archive_kv__compact`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KvCompactParams {
    /// Filesystem path to the redb database file.
    pub path: String,
}

/// Parameters for `archive_kv__check_integrity`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KvCheckIntegrityParams {
    /// Filesystem path to the redb database file.
    pub path: String,
}

/// Parameters for `archive_kv__scan`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KvScanParams {
    /// Filesystem path to the redb database file.
    pub path: String,
    /// Name of the KV table to scan.
    pub table: String,
}

/// Parameters for `archive_kv__range`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KvRangeParams {
    /// Filesystem path to the redb database file.
    pub path: String,
    /// Name of the KV table to scan.
    pub table: String,
    /// Lower bound key (inclusive), treated as a text key.
    pub from: String,
    /// Upper bound key (exclusive), treated as a text key.
    pub to: String,
}

/// Parameters for `archive_kv__get`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KvGetParams {
    /// Filesystem path to the redb database file.
    pub path: String,
    /// Name of the KV table.
    pub table: String,
    /// Key to look up (stored as text).
    pub key: String,
}

/// Parameters for `archive_kv__insert`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KvInsertParams {
    /// Filesystem path to the redb database file.
    pub path: String,
    /// Name of the KV table.
    pub table: String,
    /// Key (stored as text).
    pub key: String,
    /// Value (stored as text).
    pub value: String,
}

/// Parameters for `archive_kv__remove`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KvRemoveParams {
    /// Filesystem path to the redb database file.
    pub path: String,
    /// Name of the KV table.
    pub table: String,
    /// Key to remove (stored as text).
    pub key: String,
}

/// Parameters for `archive_kv__create_snapshot`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KvCreateSnapshotParams {
    /// Filesystem path to the redb database file.
    pub path: String,
    /// Human-readable label for the snapshot.
    pub name: String,
}

/// Parameters for `archive_kv__list_snapshots`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KvListSnapshotsParams {
    /// Filesystem path to the redb database file.
    pub path: String,
}

/// Parameters for `archive_kv__drop_snapshot`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KvDropSnapshotParams {
    /// Filesystem path to the redb database file.
    pub path: String,
    /// Snapshot name (as returned by `list_snapshots`).
    pub name: String,
    /// Backend-assigned snapshot id (as returned by `list_snapshots`).
    pub id: u64,
}

/// Parameters for `archive_kv__restore_snapshot`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KvRestoreSnapshotParams {
    /// Filesystem path to the redb database file.
    pub path: String,
    /// Snapshot name (as returned by `list_snapshots`).
    pub name: String,
    /// Backend-assigned snapshot id (as returned by `list_snapshots`).
    pub id: u64,
}

// ── tools ─────────────────────────────────────────────────────────────────────

/// Return storage statistics for the redb database at `path`.
#[elicit_tool(
    plugin = "archive_kv",
    name = "archive_kv__storage_stats",
    description = "Return storage statistics (bytes, fragmentation, table count, cache hit ratio) for a redb database."
)]
#[instrument]
async fn storage_stats(p: KvStorageStatsParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<KvStatsRead>::assert();
    let backend = open(&p.path)?;
    let stats = backend.storage_stats().await.map_err(db_err)?;
    let desc = KvStatsDescriptor {
        path: p.path,
        stored_bytes: stats.stored_bytes,
        fragmented_bytes: stats.fragmented_bytes,
        metadata_bytes: stats.metadata_bytes,
        table_count: stats.table_count,
        cache_hit_ratio: stats.cache_hit_ratio,
    };
    json_result(&desc)
}

/// Compact the redb database at `path`, reclaiming fragmented space.
#[elicit_tool(
    plugin = "archive_kv",
    name = "archive_kv__compact",
    description = "Compact the redb database file, reclaiming space from deleted entries."
)]
#[instrument]
async fn compact(p: KvCompactParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<KvCompactCompleted>::assert();
    let backend = open(&p.path)?;
    let _audit = backend.compact().await.map_err(db_err)?;
    json_result(&serde_json::json!({ "compacted": true, "path": p.path }))
}

/// Verify structural integrity of all pages in the redb database at `path`.
#[elicit_tool(
    plugin = "archive_kv",
    name = "archive_kv__check_integrity",
    description = "Verify the structural integrity of all database pages in a redb file."
)]
#[instrument]
async fn check_integrity(p: KvCheckIntegrityParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<KvIntegrityChecked>::assert();
    let backend = open(&p.path)?;
    let _audit = backend.check_integrity().await.map_err(db_err)?;
    json_result(&serde_json::json!({ "integrity_ok": true, "path": p.path }))
}

/// Scan all entries in a KV table, returning a [`KvScanResult`].
#[elicit_tool(
    plugin = "archive_kv",
    name = "archive_kv__scan",
    description = "Scan all entries in the named KV table, returning key-value pairs in key order."
)]
#[instrument]
async fn scan(p: KvScanParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<KvScanCompleted>::assert();
    let backend = open(&p.path)?;
    let entries = backend.kv_scan(&p.table).await.map_err(db_err)?;
    let total = entries.len() as u64;
    let descriptors: Vec<KvEntryDescriptor> = entries
        .iter()
        .map(|e| KvEntryDescriptor {
            key: dbvalue_display(&e.key),
            value: dbvalue_display(&e.value),
        })
        .collect();
    let result = KvScanResult {
        table_name: p.table,
        entries: descriptors,
        total_count: total,
        offset: 0,
    };
    json_result(&result)
}

/// Range-scan a KV table between `from` (inclusive) and `to` (exclusive).
#[elicit_tool(
    plugin = "archive_kv",
    name = "archive_kv__range",
    description = "Scan entries in a KV table whose key falls in [from, to)."
)]
#[instrument]
async fn range(p: KvRangeParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<KvRangeCompleted>::assert();
    let backend = open(&p.path)?;
    let from = DbValue::Text(p.from.clone());
    let to = DbValue::Text(p.to.clone());
    let entries = backend
        .kv_range(&p.table, &from, &to)
        .await
        .map_err(db_err)?;
    let total = entries.len() as u64;
    let descriptors: Vec<KvEntryDescriptor> = entries
        .iter()
        .map(|e| KvEntryDescriptor {
            key: dbvalue_display(&e.key),
            value: dbvalue_display(&e.value),
        })
        .collect();
    let result = KvScanResult {
        table_name: p.table,
        entries: descriptors,
        total_count: total,
        offset: 0,
    };
    json_result(&result)
}

/// Look up a single key in a KV table.
#[elicit_tool(
    plugin = "archive_kv",
    name = "archive_kv__get",
    description = "Look up the value for a key in the named KV table. Returns null if the key is absent."
)]
#[instrument]
async fn get(p: KvGetParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<KvGetCompleted>::assert();
    let backend = open(&p.path)?;
    let key = DbValue::Text(p.key.clone());
    match backend.kv_get(&p.table, &key).await.map_err(db_err)? {
        Some(v) => {
            let desc = KvEntryDescriptor {
                key: p.key,
                value: dbvalue_display(&v),
            };
            json_result(&desc)
        }
        None => json_result(&serde_json::Value::Null),
    }
}

/// Insert or replace a key-value entry in a KV table.
#[elicit_tool(
    plugin = "archive_kv",
    name = "archive_kv__insert",
    description = "Insert or replace the value for a key in the named KV table."
)]
#[instrument]
async fn insert(p: KvInsertParams) -> Result<CallToolResult, ErrorData> {
    let backend = open(&p.path)?;
    let key = DbValue::Text(p.key.clone());
    let value = DbValue::Text(p.value.clone());
    let _proof = backend
        .kv_insert(&p.table, key, value)
        .await
        .map_err(db_err)?;
    let desc = KvEntryDescriptor {
        key: p.key,
        value: p.value,
    };
    json_result(&desc)
}

/// Remove a key from a KV table.
#[elicit_tool(
    plugin = "archive_kv",
    name = "archive_kv__remove",
    description = "Remove the entry for a key from the named KV table."
)]
#[instrument]
async fn remove(p: KvRemoveParams) -> Result<CallToolResult, ErrorData> {
    let backend = open(&p.path)?;
    let key = DbValue::Text(p.key.clone());
    let _proof = backend.kv_remove(&p.table, &key).await.map_err(db_err)?;
    json_result(&serde_json::json!({ "removed": p.key, "table": p.table }))
}

/// Create a durable named snapshot of the current database state.
#[elicit_tool(
    plugin = "archive_kv",
    name = "archive_kv__create_snapshot",
    description = "Create a durable named snapshot (persistent savepoint) of the redb database."
)]
#[instrument]
async fn create_snapshot(p: KvCreateSnapshotParams) -> Result<CallToolResult, ErrorData> {
    let backend = open(&p.path)?;
    let (handle, _proof) = backend.create_snapshot(&p.name).await.map_err(db_err)?;
    let desc = KvSnapshotDescriptor {
        name: handle.name,
        id: handle.id,
    };
    json_result(&desc)
}

/// List all durable snapshots held by the redb database.
#[elicit_tool(
    plugin = "archive_kv",
    name = "archive_kv__list_snapshots",
    description = "List all durable snapshots currently held by the redb database."
)]
#[instrument]
async fn list_snapshots(p: KvListSnapshotsParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<KvSnapshotListRead>::assert();
    let backend = open(&p.path)?;
    let handles = backend.list_snapshots().await.map_err(db_err)?;
    let descs: Vec<KvSnapshotDescriptor> = handles
        .into_iter()
        .map(|h| KvSnapshotDescriptor {
            name: h.name,
            id: h.id,
        })
        .collect();
    json_result(&descs)
}

/// Drop a durable snapshot by id, freeing its storage.
#[elicit_tool(
    plugin = "archive_kv",
    name = "archive_kv__drop_snapshot",
    description = "Release a durable snapshot by id, allowing its storage to be reclaimed."
)]
#[instrument]
async fn drop_snapshot(p: KvDropSnapshotParams) -> Result<CallToolResult, ErrorData> {
    let backend = open(&p.path)?;
    let handle = SnapshotHandle {
        name: p.name.clone(),
        id: p.id,
    };
    backend.drop_snapshot(handle).await.map_err(db_err)?;
    json_result(&serde_json::json!({ "dropped": p.name, "id": p.id }))
}

/// Restore the database to the state captured in a named snapshot.
#[elicit_tool(
    plugin = "archive_kv",
    name = "archive_kv__restore_snapshot",
    description = "Restore the redb database to the state captured in the named snapshot."
)]
#[instrument]
async fn restore_snapshot(p: KvRestoreSnapshotParams) -> Result<CallToolResult, ErrorData> {
    let backend = open(&p.path)?;
    let handle = SnapshotHandle {
        name: p.name.clone(),
        id: p.id,
    };
    let _proof = backend.restore_snapshot(&handle).await.map_err(db_err)?;
    json_result(&serde_json::json!({ "restored": p.name, "id": p.id }))
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for embedded key-value store operations via redb.
///
/// Exposes scan, get, insert, remove, range, compaction, integrity check,
/// storage stats, and snapshot management as MCP tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "archive_kv")]
pub struct ArchiveKvPlugin;

impl ArchiveKvPlugin {
    /// Create a new KV plugin instance.
    pub fn new() -> Self {
        Self
    }
}

impl Default for ArchiveKvPlugin {
    fn default() -> Self {
        Self::new()
    }
}
