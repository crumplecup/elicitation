//! JSON Patch operation shadow type.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A JSON Patch operation for `UPDATE … PATCH [PatchOp, …]` in SurrealDB.
///
/// Mirrors the JSON Patch RFC 6902 operations supported by SurrealDB's
/// `PATCH` DML statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op", rename_all = "lowercase")]
pub enum PatchOp {
    /// Add a value at the given JSON pointer path.
    Add {
        /// JSON pointer path (e.g. `/name`).
        path: String,
        /// The value to add.
        value: serde_json::Value,
    },
    /// Remove the value at the given JSON pointer path.
    Remove {
        /// JSON pointer path.
        path: String,
    },
    /// Replace the value at the given JSON pointer path.
    Replace {
        /// JSON pointer path.
        path: String,
        /// The replacement value.
        value: serde_json::Value,
    },
    /// Change a string at the given path using a diff patch.
    Change {
        /// JSON pointer path.
        path: String,
        /// The diff string.
        value: String,
    },
    /// Copy a value from one path to another.
    Copy {
        /// The source JSON pointer path.
        from: String,
        /// The target JSON pointer path.
        path: String,
    },
    /// Move a value from one path to another.
    Move {
        /// The source JSON pointer path.
        from: String,
        /// The target JSON pointer path.
        path: String,
    },
    /// Test that a value at the given path equals a given value.
    Test {
        /// JSON pointer path.
        path: String,
        /// The expected value.
        value: serde_json::Value,
    },
    /// Increment the numeric value at the given path.
    Increment {
        /// JSON pointer path.
        path: String,
        /// The amount to increment by.
        value: serde_json::Value,
    },
    /// Decrement the numeric value at the given path.
    Decrement {
        /// JSON pointer path.
        path: String,
        /// The amount to decrement by.
        value: serde_json::Value,
    },
}
