//! Trenchcoat wrapper for [`surrealdb_types::Duration`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A SurrealDB duration value.
///
/// Wraps an upstream `surrealdb_types::Duration` to add [`JsonSchema`] for
/// MCP boundary crossing.  Duration strings use SurrealDB notation, e.g.
/// `"1y2w3d4h5m6s"` or `"500ms"`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct Duration {
    /// SurrealDB duration string, e.g. `"1y2w3d4h5m6s"`.
    ///
    /// Supported units: `y` (years), `w` (weeks), `d` (days), `h` (hours),
    /// `m` (minutes), `s` (seconds), `ms` (milliseconds), `us`/`µs`
    /// (microseconds), `ns` (nanoseconds).
    pub value: String,
}

impl Duration {
    /// Create a new duration from a SurrealDB duration string.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}
