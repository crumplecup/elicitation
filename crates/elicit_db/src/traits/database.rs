//! [`DbDatabaseManager`] — database-level DDL operations.
//!
//! Source: ISO/IEC 9075-2 §17; PostgreSQL docs §22 — Managing Databases.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{AuditLogged, DatabaseCreated, DbResult};

/// Creates, drops, and lists databases.
///
/// Source: PostgreSQL docs §22 — Managing Databases
pub trait DbDatabaseManager: Send + Sync {
    /// Create a new database.
    ///
    /// Source: ISO/IEC 9075-2 §17; PostgreSQL docs §22.2 — `CREATE DATABASE`
    fn create_database(
        &self,
        name: &str,
    ) -> BoxFuture<'_, DbResult<(Established<DatabaseCreated>, Established<AuditLogged>)>>;

    /// Drop an existing database.
    ///
    /// Source: PostgreSQL docs §22.5 — `DROP DATABASE`
    fn drop_database(&self, name: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>>;

    /// List all databases visible to the current role.
    ///
    /// Source: PostgreSQL docs §54.8 — pg_database
    fn list_databases(&self) -> BoxFuture<'_, DbResult<Vec<String>>>;

    /// Rename a database.
    ///
    /// Source: PostgreSQL docs §22 — `ALTER DATABASE ... RENAME TO`
    fn rename_database(
        &self,
        from: &str,
        to: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>>;

    /// Return the total size of a database in bytes.
    ///
    /// Source: PostgreSQL docs §9.29 — `pg_database_size()`
    fn database_size(&self, name: &str) -> BoxFuture<'_, DbResult<u64>>;
}
