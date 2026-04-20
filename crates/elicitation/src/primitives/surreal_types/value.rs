//! Trenchcoat wrapper for [`surrealdb_types::Value`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{Datetime, Duration, Geometry, Number, RecordId, Table};

/// A SurrealDB value that can cross the MCP JSON boundary.
///
/// Shadow of `surrealdb_types::Value`. All variants that carry inner values
/// use this crate's own bridge types so the whole tree is [`JsonSchema`]-able.
///
/// Variants `File`, `Range`, `Regex`, `Set` and `Array`/`Object` from the
/// upstream enum are represented by `Array` and `Object` here using
/// `serde_json::Value` payloads.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Value {
    /// Absence of a value (`NONE` in SurrealQL).
    None,
    /// A null value (`NULL` in SurrealQL).
    Null,
    /// A boolean value.
    Bool(bool),
    /// A numeric value (int, float, or decimal).
    Number(Number),
    /// A UTF-8 string.
    String(String),
    /// Raw bytes, base-64 encoded.
    Bytes(String),
    /// A duration span.
    Duration(Duration),
    /// A datetime point in time.
    Datetime(Datetime),
    /// A UUID value, as a string.
    Uuid(String),
    /// A geometric shape.
    Geometry(Geometry),
    /// A table name (used as a value, e.g. in graph traversal).
    Table(Table),
    /// A record identifier `table:key`.
    RecordId(RecordId),
    /// A JSON array of values.
    Array(Vec<Value>),
    /// A JSON object of key-value pairs.
    Object(serde_json::Map<String, serde_json::Value>),
}
