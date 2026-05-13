//! [`DbBackupManager`] — backup initiation, listing, and verification.
//!
//! Source: PostgreSQL docs §26 — Backup and Restore; §30 — Write-Ahead Logging.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{AuditLogged, BackupConsistent, DbResult, WALReplayable};

/// Manages database backups and WAL status.
///
/// Source: PostgreSQL docs §26 — Backup and Restore
pub trait DbBackupManager: Send + Sync {
    /// Initiate a base backup with the given label.
    ///
    /// Source: PostgreSQL docs §26.3 — `pg_backup_start()` / `pg_backup_stop()`
    fn initiate_backup(
        &self,
        label: &str,
    ) -> BoxFuture<'_, DbResult<(Established<BackupConsistent>, Established<AuditLogged>)>>;

    /// List available backup labels.
    ///
    /// Source: PostgreSQL docs §26 — Backup and Restore
    fn list_backups(&self) -> BoxFuture<'_, DbResult<Vec<String>>>;

    /// Verify the integrity of a named backup.
    ///
    /// Source: PostgreSQL docs §26 — Backup and Restore
    fn verify_backup(&self, label: &str) -> BoxFuture<'_, DbResult<Established<BackupConsistent>>>;

    /// Return current WAL replay status.
    ///
    /// Source: PostgreSQL docs §30 — Write-Ahead Logging
    fn wal_status(&self) -> BoxFuture<'_, DbResult<Established<WALReplayable>>>;
}
