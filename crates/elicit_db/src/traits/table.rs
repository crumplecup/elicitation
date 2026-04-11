//! [`DbTableManager`] — table DDL operations.
//!
//! Source: ISO/IEC 9075-2 §11; PostgreSQL docs §5 — DDL.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{
    AuditLogged, ColumnExists, DbColumn, DbResult, DbTableInfo, TableCreated, TableExists,
};

/// Creates, modifies, and introspects tables.
///
/// Source: ISO/IEC 9075-2 §11 — Schema definition statements
pub trait DbTableManager: Send + Sync {
    /// Create a new table with the given column definitions.
    ///
    /// Source: ISO/IEC 9075-2 §11.3 — `<table definition>`
    fn create_table(
        &self,
        schema: &str,
        name: &str,
        columns: Vec<DbColumn>,
    ) -> BoxFuture<'_, DbResult<(Established<TableCreated>, Established<AuditLogged>)>>;

    /// Drop a table, optionally cascading to dependent objects.
    ///
    /// Source: ISO/IEC 9075-2 §11.21 — `<drop table statement>`
    fn drop_table(
        &self,
        schema: &str,
        name: &str,
        cascade: bool,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>>;

    /// List all tables in a schema.
    ///
    /// Source: ISO/IEC 9075-11 §TABLES view
    fn list_tables(&self, schema: &str) -> BoxFuture<'_, DbResult<Vec<DbTableInfo>>>;

    /// Retrieve full metadata for a table.
    ///
    /// Source: ISO/IEC 9075-11 §TABLES and §COLUMNS views
    fn inspect_table(
        &self,
        schema: &str,
        name: &str,
    ) -> BoxFuture<'_, DbResult<(DbTableInfo, Established<TableExists>)>>;

    /// Add a column to an existing table.
    ///
    /// Source: ISO/IEC 9075-2 §11.10 — `<add column definition>`
    fn add_column(
        &self,
        schema: &str,
        table: &str,
        column: DbColumn,
    ) -> BoxFuture<'_, DbResult<(Established<ColumnExists>, Established<AuditLogged>)>>;

    /// Remove a column from an existing table.
    ///
    /// Source: ISO/IEC 9075-2 §11.11 — `<drop column definition>`
    fn drop_column(
        &self,
        schema: &str,
        table: &str,
        column: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>>;

    /// Rename a table.
    ///
    /// Source: PostgreSQL docs — `ALTER TABLE ... RENAME TO`
    fn rename_table(
        &self,
        schema: &str,
        from: &str,
        to: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>>;

    /// Remove all rows from a table efficiently.
    ///
    /// Source: ISO/IEC 9075-2 §14.6 — `<truncate table statement>`
    fn truncate_table(
        &self,
        schema: &str,
        name: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>>;
}
