//! Trenchcoat wrapper for [`surrealdb_types::Geometry`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/// A SurrealDB geometry value.
///
/// Mirrors `surrealdb_types::Geometry`. GeoJSON-compatible representation
/// for spatial data crossing the MCP boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "coordinates")]
pub enum Geometry {
    /// A single 2D point `[longitude, latitude]`.
    Point([f64; 2]),
    /// A line string — ordered sequence of coordinate pairs.
    Line(Vec<[f64; 2]>),
    /// A polygon — outer ring plus optional holes, each a list of coordinate pairs.
    Polygon(Vec<Vec<[f64; 2]>>),
    /// Multiple points.
    MultiPoint(Vec<[f64; 2]>),
    /// Multiple line strings.
    MultiLine(Vec<Vec<[f64; 2]>>),
    /// Multiple polygons.
    MultiPolygon(Vec<Vec<Vec<[f64; 2]>>>),
    /// A collection of geometry values.
    Collection(Vec<Geometry>),
}
