//! [`DbSchemaManager`] — schema-level DDL operations.
//!
//! Source: ISO/IEC 9075-2 §11.1; PostgreSQL docs §5.9 — Schemas.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{AuditLogged, DbResult, DbSchema, SchemaCreated};

/// Creates, drops, and introspects schemas.
///
/// Source: PostgreSQL docs §5.9 — Schemas
pub trait DbSchemaManager: Send + Sync {
    /// Create a new schema.
    ///
    /// Source: ISO/IEC 9075-2 §11.1 — `<schema definition>`
    fn create_schema(
        &self,
        name: &str,
    ) -> BoxFuture<'_, DbResult<(Established<SchemaCreated>, Established<AuditLogged>)>>;

    /// Drop a schema, optionally cascading to contained objects.
    ///
    /// Source: PostgreSQL docs §5.9 — `DROP SCHEMA`
    fn drop_schema(
        &self,
        name: &str,
        cascade: bool,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>>;

    /// List all schema names visible to the current role.
    ///
    /// Source: ISO/IEC 9075-11 §SCHEMATA view
    fn list_schemas(&self) -> BoxFuture<'_, DbResult<Vec<String>>>;

    /// Retrieve full metadata for a schema including its tables.
    ///
    /// Source: ISO/IEC 9075-11 §SCHEMATA and §TABLES views
    fn schema_info(&self, name: &str) -> BoxFuture<'_, DbResult<DbSchema>>;
}
