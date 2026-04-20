//! Trenchcoat wrapper for [`surrealdb_types::Number`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A SurrealDB numeric value.
///
/// Wraps an upstream `surrealdb_types::Number` to add [`JsonSchema`] for MCP
/// boundary crossing. Numbers can be 64-bit integers, 64-bit floats, or
/// arbitrary-precision decimals (represented as strings).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Number {
    /// A 64-bit signed integer.
    Int(i64),
    /// A 64-bit floating-point number.
    Float(f64),
    /// An arbitrary-precision decimal, represented as a string.
    ///
    /// Use standard decimal notation, e.g. `"3.14159265358979323846"`.
    Decimal(String),
}
