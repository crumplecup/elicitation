//! Trenchcoats for `redb::DatabaseStats`, `redb::TableStats`, and `redb::CacheStats`.
//!
//! These types shadow their `redb` counterparts with identical field names, full
//! `Serialize` + `Deserialize` + `JsonSchema` derives, and `From<redb::*Stats>`
//! conversions.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── DatabaseStats ─────────────────────────────────────────────────────────────

/// Shadow of `redb::DatabaseStats`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DatabaseStats {
    /// Maximum traversal distance to the deepest key-value pair across all tables.
    pub tree_height: u32,
    /// Number of pages allocated.
    pub allocated_pages: u64,
    /// Number of leaf pages storing user data.
    pub leaf_pages: u64,
    /// Number of branch pages in btrees storing user data.
    pub branch_pages: u64,
    /// Bytes consumed by inserted keys and values (no indexing overhead).
    pub stored_bytes: u64,
    /// Bytes consumed by keys in internal branch pages and other metadata.
    pub metadata_bytes: u64,
    /// Bytes consumed by fragmentation in data and metadata pages.
    pub fragmented_bytes: u64,
    /// Bytes per page.
    pub page_size: usize,
}

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

// ── TableStats ───────────────────────────────────────────────────────────────

/// Shadow of `redb::TableStats`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TableStats {
    /// Maximum traversal distance to the deepest key-value pair in the table.
    pub tree_height: u32,
    /// Number of leaf pages storing user data.
    pub leaf_pages: u64,
    /// Number of branch pages in the btree.
    pub branch_pages: u64,
    /// Bytes consumed by inserted keys and values (no indexing overhead).
    pub stored_bytes: u64,
    /// Bytes consumed by keys in internal branch pages and other metadata.
    pub metadata_bytes: u64,
    /// Bytes consumed by fragmentation.
    pub fragmented_bytes: u64,
}

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

// ── CacheStats ────────────────────────────────────────────────────────────────

/// Shadow of `redb::CacheStats`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CacheStats {
    /// Times data was evicted due to the cache being full.
    pub evictions: u64,
    /// Times unmodified data was read from the cache.
    pub read_hits: u64,
    /// Times unmodified data was not in the cache and read from storage.
    pub read_misses: u64,
    /// Times modified transaction data was read from the cache.
    pub write_hits: u64,
    /// Times modified transaction data was not in cache and read from storage.
    pub write_misses: u64,
    /// Current bytes in the cache.
    pub used_bytes: usize,
}

impl From<redb::CacheStats> for CacheStats {
    fn from(s: redb::CacheStats) -> Self {
        Self {
            evictions: s.evictions(),
            read_hits: s.read_hits(),
            read_misses: s.read_misses(),
            write_hits: s.write_hits(),
            write_misses: s.write_misses(),
            used_bytes: s.used_bytes(),
        }
    }
}
