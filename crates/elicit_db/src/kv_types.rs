//! Types specific to embedded key-value store operations.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use elicitation::Elicit;

use crate::DbValue;

/// A single key-value entry from a KV table scan or range query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct KvEntry {
    /// The entry key.
    pub key: DbValue,
    /// The entry value.
    pub value: DbValue,
}

/// Metadata for a KV table.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct KvTableInfo {
    /// Table name.
    pub name: String,
    /// Number of key-value entries currently stored in the table.
    pub entry_count: u64,
}

/// Storage-level statistics for an embedded database.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct DbStorageStats {
    /// Total bytes occupied by live data pages.
    pub stored_bytes: u64,
    /// Bytes in pages that are no longer live but not yet reclaimed.
    pub fragmented_bytes: u64,
    /// Bytes used by database metadata (root pages, free-list, etc.).
    pub metadata_bytes: u64,
    /// Number of tables currently open in the database.
    pub table_count: usize,
    /// Estimated fraction of read requests satisfied from the page cache (0.0–1.0).
    pub cache_hit_ratio: f64,
}

/// Opaque handle for a durable snapshot of an embedded database.
///
/// Returned by [`crate::DbSnapshotManager::create_snapshot`]; passed back to
/// restore or drop operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct SnapshotHandle {
    /// Human-readable label supplied at creation time.
    pub name: String,
    /// Backend-assigned numeric identifier.
    pub id: u64,
}
