//! Trenchcoat wrapper for [`surrealdb_types::Table`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A SurrealDB table name.
///
/// Wraps an upstream `surrealdb_types::Table` to add [`JsonSchema`] for MCP
/// boundary crossing. Table names must be alphanumeric or underscore.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct Table {
    /// The table name string.
    pub name: String,
}

impl Table {
    /// Create a new table name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[cfg(feature = "surreal-types")]
impl From<surrealdb_types::Table> for Table {
    fn from(t: surrealdb_types::Table) -> Self {
        Self {
            name: t.into_inner(),
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<Table> for surrealdb_types::Table {
    fn from(t: Table) -> Self {
        surrealdb_types::Table::new(t.name)
    }
}
