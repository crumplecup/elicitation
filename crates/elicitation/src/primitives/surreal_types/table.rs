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
