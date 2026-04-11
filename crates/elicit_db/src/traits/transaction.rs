//! [`DbTransactor`] — explicit transaction lifecycle management.
//!
//! Source: ISO/IEC 9075-2 §17 — Transaction management.

use futures::future::BoxFuture;

use crate::{
    DbCommitResult, DbResult, IsolationLevel, Open, RolledBack, TransactionHandle, TxMarker,
};

/// Manages explicit transactions: begin, commit, rollback, and savepoints.
///
/// Source: ISO/IEC 9075-2 §17 — Transaction management
pub trait DbTransactor: Send + Sync {
    /// Begin a new transaction at the specified isolation level.
    ///
    /// Source: ISO/IEC 9075-2 §17.1 — `<start transaction statement>`
    fn begin(
        &self,
        isolation: IsolationLevel,
    ) -> BoxFuture<'_, DbResult<(TransactionHandle, TxMarker<Open>)>>;

    /// Commit an open transaction durably.
    ///
    /// Source: ISO/IEC 9075-2 §17.3 — `<commit statement>`
    fn commit(&self, handle: TransactionHandle) -> BoxFuture<'_, DbCommitResult>;

    /// Roll back an open transaction, discarding all changes.
    ///
    /// Source: ISO/IEC 9075-2 §17.4 — `<rollback statement>`
    fn rollback(&self, handle: TransactionHandle) -> BoxFuture<'_, DbResult<TxMarker<RolledBack>>>;

    /// Create a savepoint within an open transaction.
    ///
    /// Source: ISO/IEC 9075-2 §17.6 — `<savepoint statement>`
    fn savepoint(&self, handle: &TransactionHandle, name: &str) -> BoxFuture<'_, DbResult<()>>;

    /// Roll back to a previously created savepoint.
    ///
    /// Source: ISO/IEC 9075-2 §17.7 — `<rollback to savepoint>`
    fn rollback_to_savepoint(
        &self,
        handle: &TransactionHandle,
        name: &str,
    ) -> BoxFuture<'_, DbResult<()>>;
}
