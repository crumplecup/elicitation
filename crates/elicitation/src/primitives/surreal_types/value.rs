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

#[cfg(feature = "surreal-types")]
impl From<surrealdb_types::Value> for Value {
    fn from(v: surrealdb_types::Value) -> Self {
        match v {
            surrealdb_types::Value::None => Value::None,
            surrealdb_types::Value::Null => Value::Null,
            surrealdb_types::Value::Bool(b) => Value::Bool(b),
            surrealdb_types::Value::Number(n) => Value::Number(Number::from(n)),
            surrealdb_types::Value::String(s) => Value::String(s),
            surrealdb_types::Value::Bytes(b) => Value::Bytes(b.to_string()),
            surrealdb_types::Value::Duration(d) => Value::Duration(Duration::from(d)),
            surrealdb_types::Value::Datetime(dt) => Value::Datetime(Datetime::from(dt)),
            surrealdb_types::Value::Uuid(u) => Value::Uuid(u.to_string()),
            surrealdb_types::Value::Geometry(g) => Value::Geometry(Geometry::from(g)),
            surrealdb_types::Value::Table(t) => Value::Table(Table::from(t)),
            surrealdb_types::Value::RecordId(rid) => Value::RecordId(RecordId::from(rid)),
            surrealdb_types::Value::Array(a) => {
                Value::Array(a.into_inner().into_iter().map(Value::from).collect())
            }
            surrealdb_types::Value::Object(o) => {
                let json = serde_json::to_value(o).unwrap_or(serde_json::Value::Null);
                if let serde_json::Value::Object(map) = json {
                    Value::Object(map)
                } else {
                    Value::None
                }
            }
            // Set serialises like an array; fold into Array.
            surrealdb_types::Value::Set(s) => {
                Value::Array(s.into_inner().into_iter().map(Value::from).collect())
            }
            // Fold opaque types to their string representations.
            surrealdb_types::Value::Regex(r) => Value::String(r.to_string()),
            surrealdb_types::Value::File(f) => Value::String(format!("{}/{}", f.bucket(), f.key())),
            surrealdb_types::Value::Range(r) => {
                use surrealdb_types::ToSql;
                Value::String(r.to_sql())
            }
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<Value> for surrealdb_types::Value {
    fn from(v: Value) -> Self {
        match v {
            Value::None => surrealdb_types::Value::None,
            Value::Null => surrealdb_types::Value::Null,
            Value::Bool(b) => surrealdb_types::Value::Bool(b),
            Value::Number(n) => surrealdb_types::Value::Number(surrealdb_types::Number::from(n)),
            Value::String(s) => surrealdb_types::Value::String(s),
            // Bytes round-trip through the string representation; kept as a string on the way
            // back since we cannot reconstruct the raw byte sequence without knowing the
            // encoding that was used during serialisation.
            Value::Bytes(s) => surrealdb_types::Value::String(s),
            Value::Duration(d) => {
                surrealdb_types::Value::Duration(surrealdb_types::Duration::from(d))
            }
            Value::Datetime(dt) => {
                surrealdb_types::Value::Datetime(surrealdb_types::Datetime::from(dt))
            }
            Value::Uuid(s) => {
                let inner = s
                    .parse::<uuid::Uuid>()
                    .unwrap_or_else(|_| uuid::Uuid::nil());
                surrealdb_types::Value::Uuid(surrealdb_types::Uuid::from(inner))
            }
            Value::Geometry(g) => {
                surrealdb_types::Value::Geometry(surrealdb_types::Geometry::from(g))
            }
            Value::Table(t) => surrealdb_types::Value::Table(surrealdb_types::Table::from(t)),
            Value::RecordId(rid) => {
                surrealdb_types::Value::RecordId(surrealdb_types::RecordId::from(rid))
            }
            Value::Array(arr) => {
                let values: Vec<surrealdb_types::Value> =
                    arr.into_iter().map(surrealdb_types::Value::from).collect();
                let mut surreal_arr = surrealdb_types::Array::with_capacity(values.len());
                surreal_arr.extend(values);
                surrealdb_types::Value::Array(surreal_arr)
            }
            Value::Object(map) => {
                let json = serde_json::Value::Object(map);
                serde_json::from_value::<surrealdb_types::Object>(json)
                    .map(surrealdb_types::Value::Object)
                    .unwrap_or(surrealdb_types::Value::None)
            }
        }
    }
}
