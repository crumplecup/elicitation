//! Trenchcoat wrapper for [`redb::TableStats`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Informational storage statistics for a single redb table.
///
/// Wraps `redb::TableStats` to add [`JsonSchema`] for MCP boundary crossing.
/// All fields are derived from getter methods since `redb::TableStats` fields
/// are private.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct TableStats {
    /// Maximum traversal distance to reach the deepest key-value pair in the table.
    pub tree_height: u32,
    /// Number of leaf pages storing user data.
    pub leaf_pages: u64,
    /// Number of branch pages in the B-tree storing user data.
    pub branch_pages: u64,
    /// Bytes consumed by inserted keys and values (excluding indexing overhead).
    pub stored_bytes: u64,
    /// Bytes consumed by internal branch-page keys and other metadata.
    pub metadata_bytes: u64,
    /// Bytes consumed by fragmentation in data pages and internal tables.
    pub fragmented_bytes: u64,
}

#[cfg(feature = "redb-types")]
impl From<redb::TableStats> for TableStats {
    fn from(s: redb::TableStats) -> Self {
        Self {
            tree_height: s.tree_height(),
            leaf_pages: s.leaf_pages(),
            branch_pages: s.branch_pages(),
            stored_bytes: s.stored_bytes(),
            metadata_bytes: s.metadata_bytes(),
            fragmented_bytes: s.fragmented_bytes(),
        }
    }
}
