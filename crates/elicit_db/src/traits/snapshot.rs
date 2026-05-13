//! [`DbSnapshotManager`] — durable snapshot management for embedded stores.
//!
//! redb supports *persistent savepoints*: named snapshots of the entire
//! database state that survive process restart.  This trait exposes that
//! surface in a backend-agnostic way.
//!
//! Unlike SQL savepoints (which are scoped to a single transaction), these
//! snapshots are database-level and durable.
//!
//! Source: redb documentation — Persistent savepoints.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{DbResult, KvSnapshotCreated, KvSnapshotRestored, SnapshotHandle};

/// Durable database-level snapshot management for embedded stores.
///
/// Snapshots capture the complete database state at a point in time and are
/// retained until explicitly dropped.  They survive process restarts.
pub trait DbSnapshotManager: Send + Sync {
    /// Create a durable named snapshot of the current database state.
    ///
    /// Returns the [`SnapshotHandle`] and a proof that the snapshot was created.
    ///
    /// Source: redb — `WriteTransaction::persistent_savepoint()`
    fn create_snapshot(
        &self,
        name: &str,
    ) -> BoxFuture<'_, DbResult<(SnapshotHandle, Established<KvSnapshotCreated>)>>;

    /// Restore the database to the state captured in `handle`.
    ///
    /// Returns a proof that the restore completed successfully.
    ///
    /// Source: redb — `Database::restore_savepoint()`
    fn restore_snapshot(
        &self,
        handle: &SnapshotHandle,
    ) -> BoxFuture<'_, DbResult<Established<KvSnapshotRestored>>>;

    /// Release `handle`, allowing the snapshot's storage to be reclaimed.
    ///
    /// Source: redb — `Database::delete_persistent_savepoint()`
    fn drop_snapshot(&self, handle: SnapshotHandle) -> BoxFuture<'_, DbResult<()>>;

    /// List all currently held snapshots.
    ///
    /// Source: redb — `Database::list_persistent_savepoints()`
    fn list_snapshots(&self) -> BoxFuture<'_, DbResult<Vec<SnapshotHandle>>>;
}
