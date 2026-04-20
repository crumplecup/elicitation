//! Trenchcoat wrapper for [`surrealdb_types::RecordId`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// A SurrealDB record identifier.
///
/// Wraps an upstream `surrealdb_types::RecordId` to add [`JsonSchema`] for
/// MCP boundary crossing. A record id is a `(table, key)` pair where the key
/// can be any JSON-serializable value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RecordId {
    /// The table the record belongs to.
    pub table: String,
    /// The key uniquely identifying the record within the table.
    ///
    /// Can be a string, integer, UUID, or a JSON object.
    pub key: JsonValue,
}

impl RecordId {
    /// Create a new record id.
    pub fn new(table: impl Into<String>, key: impl Into<JsonValue>) -> Self {
        Self {
            table: table.into(),
            key: key.into(),
        }
    }
}
