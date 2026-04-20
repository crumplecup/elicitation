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

#[cfg(feature = "surreal-types")]
impl From<surrealdb_types::GeometryKind> for GeometryKind {
    fn from(gk: surrealdb_types::GeometryKind) -> Self {
        match gk {
            surrealdb_types::GeometryKind::Point => GeometryKind::Point,
            surrealdb_types::GeometryKind::Line => GeometryKind::Line,
            surrealdb_types::GeometryKind::Polygon => GeometryKind::Polygon,
            surrealdb_types::GeometryKind::MultiPoint => GeometryKind::MultiPoint,
            surrealdb_types::GeometryKind::MultiLine => GeometryKind::MultiLine,
            surrealdb_types::GeometryKind::MultiPolygon => GeometryKind::MultiPolygon,
            surrealdb_types::GeometryKind::Collection => GeometryKind::Collection,
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<GeometryKind> for surrealdb_types::GeometryKind {
    fn from(gk: GeometryKind) -> Self {
        match gk {
            GeometryKind::Point => surrealdb_types::GeometryKind::Point,
            GeometryKind::Line => surrealdb_types::GeometryKind::Line,
            GeometryKind::Polygon => surrealdb_types::GeometryKind::Polygon,
            GeometryKind::MultiPoint => surrealdb_types::GeometryKind::MultiPoint,
            GeometryKind::MultiLine => surrealdb_types::GeometryKind::MultiLine,
            GeometryKind::MultiPolygon => surrealdb_types::GeometryKind::MultiPolygon,
            GeometryKind::Collection => surrealdb_types::GeometryKind::Collection,
            // Feature variant has no direct upstream equivalent; use Collection as a fallback.
            GeometryKind::Feature => surrealdb_types::GeometryKind::Collection,
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<surrealdb_types::Kind> for Kind {
    fn from(k: surrealdb_types::Kind) -> Self {
        match k {
            surrealdb_types::Kind::Any => Kind::Any,
            surrealdb_types::Kind::None => Kind::None,
            surrealdb_types::Kind::Null => Kind::Null,
            surrealdb_types::Kind::Bool => Kind::Bool,
            surrealdb_types::Kind::Bytes => Kind::Bytes,
            surrealdb_types::Kind::Datetime => Kind::Datetime,
            surrealdb_types::Kind::Decimal => Kind::Decimal,
            surrealdb_types::Kind::Duration => Kind::Duration,
            surrealdb_types::Kind::Float => Kind::Float,
            surrealdb_types::Kind::Int => Kind::Int,
            surrealdb_types::Kind::Number => Kind::Number,
            surrealdb_types::Kind::Object => Kind::Object,
            surrealdb_types::Kind::String => Kind::String,
            surrealdb_types::Kind::Uuid => Kind::Uuid,
            surrealdb_types::Kind::Regex => Kind::Regex,
            surrealdb_types::Kind::Range => Kind::Range,
            surrealdb_types::Kind::Table(tables) => {
                Kind::Table(tables.into_iter().map(|t| t.into_inner()).collect())
            }
            surrealdb_types::Kind::Record(tables) => {
                Kind::Record(tables.into_iter().map(|t| t.into_inner()).collect())
            }
            surrealdb_types::Kind::Geometry(gks) => {
                Kind::Geometry(gks.into_iter().map(GeometryKind::from).collect())
            }
            surrealdb_types::Kind::Either(kinds) => {
                Kind::Either(kinds.into_iter().map(Kind::from).collect())
            }
            surrealdb_types::Kind::Set(inner, max) => Kind::Set(Box::new(Kind::from(*inner)), max),
            surrealdb_types::Kind::Array(inner, max) => {
                Kind::Array(Box::new(Kind::from(*inner)), max)
            }
            // Function types have no direct equivalent; treat as Any.
            surrealdb_types::Kind::Function(_, _) => Kind::Any,
            surrealdb_types::Kind::File(buckets) => Kind::File(buckets),
            surrealdb_types::Kind::Literal(lit) => match lit {
                surrealdb_types::KindLiteral::String(s) => Kind::LiteralString(s),
                surrealdb_types::KindLiteral::Integer(i) => Kind::LiteralInt(i),
                surrealdb_types::KindLiteral::Float(f) => Kind::LiteralFloat(f),
                surrealdb_types::KindLiteral::Decimal(d) => Kind::LiteralString(d.to_string()),
                surrealdb_types::KindLiteral::Duration(d) => Kind::LiteralString(d.to_string()),
                surrealdb_types::KindLiteral::Bool(b) => Kind::LiteralBool(b),
                // Array/Object literals collapse to their base kinds.
                surrealdb_types::KindLiteral::Array(_) => Kind::Array(Box::new(Kind::Any), None),
                surrealdb_types::KindLiteral::Object(_) => Kind::Object,
            },
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<Kind> for surrealdb_types::Kind {
    fn from(k: Kind) -> Self {
        match k {
            Kind::Any => surrealdb_types::Kind::Any,
            Kind::None => surrealdb_types::Kind::None,
            Kind::Null => surrealdb_types::Kind::Null,
            Kind::Bool => surrealdb_types::Kind::Bool,
            Kind::Bytes => surrealdb_types::Kind::Bytes,
            Kind::Datetime => surrealdb_types::Kind::Datetime,
            Kind::Decimal => surrealdb_types::Kind::Decimal,
            Kind::Duration => surrealdb_types::Kind::Duration,
            Kind::Float => surrealdb_types::Kind::Float,
            Kind::Int => surrealdb_types::Kind::Int,
            Kind::Number => surrealdb_types::Kind::Number,
            Kind::Object => surrealdb_types::Kind::Object,
            Kind::String => surrealdb_types::Kind::String,
            Kind::Uuid => surrealdb_types::Kind::Uuid,
            Kind::Regex => surrealdb_types::Kind::Regex,
            Kind::Range => surrealdb_types::Kind::Range,
            Kind::Table(names) => surrealdb_types::Kind::Table(
                names.into_iter().map(surrealdb_types::Table::new).collect(),
            ),
            Kind::Record(names) => surrealdb_types::Kind::Record(
                names.into_iter().map(surrealdb_types::Table::new).collect(),
            ),
            Kind::Geometry(gks) => surrealdb_types::Kind::Geometry(
                gks.into_iter()
                    .map(surrealdb_types::GeometryKind::from)
                    .collect(),
            ),
            Kind::Either(kinds) => surrealdb_types::Kind::Either(
                kinds.into_iter().map(surrealdb_types::Kind::from).collect(),
            ),
            Kind::Set(inner, max) => {
                surrealdb_types::Kind::Set(Box::new(surrealdb_types::Kind::from(*inner)), max)
            }
            Kind::Array(inner, max) => {
                surrealdb_types::Kind::Array(Box::new(surrealdb_types::Kind::from(*inner)), max)
            }
            Kind::File(buckets) => surrealdb_types::Kind::File(buckets),
            Kind::LiteralString(s) => {
                surrealdb_types::Kind::Literal(surrealdb_types::KindLiteral::String(s))
            }
            Kind::LiteralInt(i) => {
                surrealdb_types::Kind::Literal(surrealdb_types::KindLiteral::Integer(i))
            }
            Kind::LiteralFloat(f) => {
                surrealdb_types::Kind::Literal(surrealdb_types::KindLiteral::Float(f))
            }
            Kind::LiteralBool(b) => {
                surrealdb_types::Kind::Literal(surrealdb_types::KindLiteral::Bool(b))
            }
        }
    }
}
