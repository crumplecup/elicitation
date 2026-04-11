//! [`DbMonitor`] — live database monitoring and diagnostics.
//!
//! Source: PostgreSQL docs §28 — Monitoring Database Activity.

use futures::future::BoxFuture;

use crate::{DbResult, DbSessionInfo, DbStatActivity};

/// Monitors sessions, slow queries, bloat, index usage, locks, and cache.
///
/// Source: PostgreSQL docs §28 — Monitoring Database Activity
pub trait DbMonitor: Send + Sync {
    /// Return all active sessions from `pg_stat_activity`.
    ///
    /// Source: PostgreSQL docs §28.2 — pg_stat_activity
    fn active_sessions(&self) -> BoxFuture<'_, DbResult<DbStatActivity>>;

    /// Return sessions whose current query exceeds `threshold_ms` milliseconds.
    ///
    /// Source: PostgreSQL docs §28.2 — pg_stat_activity (`query_start`)
    fn slow_queries(&self, threshold_ms: u64) -> BoxFuture<'_, DbResult<Vec<DbSessionInfo>>>;

    /// Return `(table_name, bloat_ratio)` pairs for tables in the schema.
    ///
    /// Source: PostgreSQL docs §28.2 — pg_stat_user_tables
    fn table_bloat(&self, schema: &str) -> BoxFuture<'_, DbResult<Vec<(String, f64)>>>;

    /// Return `(index_name, scan_count)` pairs for indexes in the schema.
    ///
    /// Source: PostgreSQL docs §28.2 — pg_stat_user_indexes
    fn index_usage(&self, schema: &str) -> BoxFuture<'_, DbResult<Vec<(String, u64)>>>;

    /// Return `(blocking_pid, blocked_pid)` pairs for current lock waits.
    ///
    /// Source: PostgreSQL docs §54.30 — pg_locks
    fn lock_waits(&self) -> BoxFuture<'_, DbResult<Vec<(i32, i32)>>>;

    /// Return the buffer cache hit ratio (0.0–1.0).
    ///
    /// Source: PostgreSQL docs §28.2 — pg_statio_user_tables
    fn cache_hit_ratio(&self) -> BoxFuture<'_, DbResult<f64>>;
}
