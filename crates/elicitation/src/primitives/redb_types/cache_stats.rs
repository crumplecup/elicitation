//! Trenchcoat wrapper for [`redb::CacheStats`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// In-memory cache usage statistics for a redb database.
///
/// Wraps `redb::CacheStats` to add [`JsonSchema`] for MCP boundary crossing.
///
/// Note: upstream `redb::CacheStats` is only populated when the `cache_metrics`
/// feature of the `redb` crate is enabled; without it all counters are zero.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct CacheStats {
    /// Number of times data was evicted due to the cache being full.
    pub evictions: u64,
    /// Number of times unmodified data was served from the cache (hit).
    pub read_hits: u64,
    /// Number of times unmodified data was not in the cache and read from storage (miss).
    pub read_misses: u64,
    /// Number of times transaction-modified data was served from the cache (hit).
    pub write_hits: u64,
    /// Number of times transaction-modified data was not in cache and read from storage (miss).
    pub write_misses: u64,
    /// Current number of bytes held in the cache.
    pub cached_bytes: u64,
}
#[cfg(feature = "redb-types")]
impl From<redb::CacheStats> for CacheStats {
    fn from(s: redb::CacheStats) -> Self {
        Self {
            evictions: s.evictions(),
            read_hits: s.read_hits(),
            read_misses: s.read_misses(),
            write_hits: s.write_hits(),
            write_misses: s.write_misses(),
            cached_bytes: s.used_bytes() as u64,
        }
    }
}
