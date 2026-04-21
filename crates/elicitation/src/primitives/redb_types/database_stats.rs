//! Trenchcoat wrapper for [`redb::DatabaseStats`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Informational storage statistics for a redb database.
///
/// Wraps `redb::DatabaseStats` to add [`JsonSchema`] for MCP boundary crossing.
/// All fields are derived from getter methods since `redb::DatabaseStats` fields
/// are private.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct DatabaseStats {
    /// Maximum traversal distance to reach the deepest key-value pair across all tables.
    pub tree_height: u32,
    /// Total number of pages allocated in the database file.
    pub allocated_pages: u64,
    /// Number of leaf pages storing user data.
    pub leaf_pages: u64,
    /// Number of branch pages in B-trees storing user data.
    pub branch_pages: u64,
    /// Bytes consumed by inserted keys and values (excluding indexing overhead).
    pub stored_bytes: u64,
    /// Bytes consumed by internal branch-page keys and other metadata.
    pub metadata_bytes: u64,
    /// Bytes consumed by fragmentation in data pages and internal tables.
    pub fragmented_bytes: u64,
    /// Bytes per page in this database.
    pub page_size: usize,
}

#[cfg(feature = "redb-types")]
impl From<redb::DatabaseStats> for DatabaseStats {
    fn from(s: redb::DatabaseStats) -> Self {
        Self {
            tree_height: s.tree_height(),
            allocated_pages: s.allocated_pages(),
            leaf_pages: s.leaf_pages(),
            branch_pages: s.branch_pages(),
            stored_bytes: s.stored_bytes(),
            metadata_bytes: s.metadata_bytes(),
            fragmented_bytes: s.fragmented_bytes(),
            page_size: s.page_size(),
        }
    }
}
