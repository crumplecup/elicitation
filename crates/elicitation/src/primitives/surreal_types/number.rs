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

#[cfg(feature = "surreal-types")]
impl From<surrealdb_types::Number> for Number {
    fn from(n: surrealdb_types::Number) -> Self {
        match n {
            surrealdb_types::Number::Int(i) => Number::Int(i),
            surrealdb_types::Number::Float(f) => Number::Float(f),
            surrealdb_types::Number::Decimal(d) => Number::Decimal(d.to_string()),
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<Number> for surrealdb_types::Number {
    fn from(n: Number) -> Self {
        match n {
            Number::Int(i) => surrealdb_types::Number::Int(i),
            Number::Float(f) => surrealdb_types::Number::Float(f),
            Number::Decimal(s) => surrealdb_types::Number::Decimal(
                s.parse::<rust_decimal::Decimal>().unwrap_or_default(),
            ),
        }
    }
}
