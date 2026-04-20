//! Trenchcoat wrapper for [`surrealdb_types::Kind`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A SurrealDB geometry type variant for use in `DEFINE FIELD TYPE geometry(…)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum GeometryKind {
    /// Point geometry.
    Point,
    /// Line geometry.
    Line,
    /// Polygon geometry.
    Polygon,
    /// MultiPoint geometry.
    MultiPoint,
    /// MultiLine geometry.
    MultiLine,
    /// MultiPolygon geometry.
    MultiPolygon,
    /// Collection of geometries.
    Collection,
    /// Any geometry type.
    Feature,
}

/// A SurrealDB type kind for schema field type declarations.
///
/// Used in `DEFINE FIELD … TYPE kind` authoring. Mirrors
/// `surrealdb_types::Kind` without the recursive heap allocations that make
/// the upstream type difficult to use as a JSON parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "params")]
pub enum Kind {
    /// Any value — the most permissive type.
    Any,
    /// No value (option-like absence).
    None,
    /// Null value.
    Null,
    /// Boolean.
    Bool,
    /// Raw bytes.
    Bytes,
    /// Datetime.
    Datetime,
    /// Arbitrary-precision decimal.
    Decimal,
    /// Duration.
    Duration,
    /// 64-bit float.
    Float,
    /// 64-bit signed integer.
    Int,
    /// Any numeric type (int, float, or decimal).
    Number,
    /// JSON object.
    Object,
    /// String.
    String,
    /// UUID.
    Uuid,
    /// Regular expression.
    Regex,
    /// Range.
    Range,
    /// A specific table type (optionally restricted to named tables).
    Table(Vec<String>),
    /// A record reference (optionally restricted to named tables).
    Record(Vec<String>),
    /// A geometry type (optionally restricted to named geometry kinds).
    Geometry(Vec<GeometryKind>),
    /// One of several types.
    Either(Vec<Kind>),
    /// A typed set with an optional maximum size.
    Set(Box<Kind>, Option<u64>),
    /// A typed array with an optional maximum length.
    Array(Box<Kind>, Option<u64>),
    /// File reference (optionally restricted to named buckets).
    File(Vec<String>),
    /// A literal string constant.
    LiteralString(String),
    /// A literal integer constant.
    LiteralInt(i64),
    /// A literal float constant.
    LiteralFloat(f64),
    /// A literal boolean constant.
    LiteralBool(bool),
}
