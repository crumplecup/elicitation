//! Trenchcoat wrapper for [`surrealdb_types::Datetime`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A SurrealDB datetime value.
///
/// Wraps an upstream `surrealdb_types::Datetime` to add [`JsonSchema`] for
/// MCP boundary crossing. Stored as an ISO 8601 string, e.g.
/// `"2024-01-15T10:30:00Z"`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct Datetime {
    /// ISO 8601 datetime string in UTC, e.g. `"2024-01-15T10:30:00Z"`.
    pub value: String,
}

impl Datetime {
    /// Create a new datetime from an ISO 8601 string.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}
