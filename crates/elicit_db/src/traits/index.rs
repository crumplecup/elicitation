//! [`DbIndexManager`] — index DDL operations.
//!
//! Source: PostgreSQL docs §11 — Indexes.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{AuditLogged, DbIndexInfo, DbResult, IndexExists};

/// Creates, drops, and lists indexes.
///
/// Source: PostgreSQL docs §11 — Indexes
pub trait DbIndexManager: Send + Sync {
    /// Create an index on the specified table columns.
    ///
    /// Source: PostgreSQL docs §11.1 — `CREATE INDEX`
    fn create_index(
        &self,
        table: &str,
        columns: &[String],
        unique: bool,
    ) -> BoxFuture<'_, DbResult<(Established<IndexExists>, Established<AuditLogged>)>>;

    /// Drop a named index.
    ///
    /// Source: PostgreSQL docs §11 — `DROP INDEX`
    fn drop_index(&self, name: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>>;

    /// List all indexes on a table.
    ///
    /// Source: PostgreSQL docs §54.26 — pg_indexes
    fn list_indexes(&self, table: &str) -> BoxFuture<'_, DbResult<Vec<DbIndexInfo>>>;

    /// Rebuild all indexes on a table via `REINDEX TABLE`.
    ///
    /// Source: PostgreSQL docs §11.12 — `REINDEX`
    fn reindex(&self, table: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>>;
}
