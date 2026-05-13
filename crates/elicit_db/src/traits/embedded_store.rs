//! [`DbEmbeddedStore`] — embedded database lifecycle management.
//!
//! Provides operations that are meaningful for embedded, file-backed stores
//! but have no equivalent in client-server relational databases: compaction,
//! integrity verification, and storage statistics.
//!
//! Source: redb documentation — Database maintenance and statistics.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{DbResult, DbStorageStats, KvIntegrityVerified, KvTableCompacted};

/// Embedded database lifecycle: compaction, integrity checks, and storage stats.
///
/// These operations work directly on the database file and do not require an
/// open transaction.
pub trait DbEmbeddedStore: Send + Sync {
    /// Compact the database file, reclaiming fragmented space from deleted entries.
    ///
    /// Returns a proof that the database was compacted successfully.
    ///
    /// Source: redb — `Database::compact()`
    fn compact(&self) -> BoxFuture<'_, DbResult<Established<KvTableCompacted>>>;

    /// Verify the structural integrity of all database pages.
    ///
    /// Returns a proof that integrity was verified without error.
    ///
    /// Source: redb — `Database::check_integrity()`
    fn check_integrity(&self) -> BoxFuture<'_, DbResult<Established<KvIntegrityVerified>>>;

    /// Return storage-level statistics for the embedded database.
    ///
    /// Includes stored bytes, fragmented bytes, metadata bytes, table count,
    /// and estimated cache hit ratio.
    ///
    /// Source: redb — `Database::stats()`
    fn storage_stats(&self) -> BoxFuture<'_, DbResult<DbStorageStats>>;
}
