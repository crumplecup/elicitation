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

/// Extract a flat coordinate list from a geo LineString.
#[cfg(feature = "surreal-types")]
fn linestring_to_coords(ls: &geo_types::LineString) -> Vec<[f64; 2]> {
    ls.coords().map(|c| [c.x, c.y]).collect()
}

/// Build a geo LineString from a flat coordinate list.
#[cfg(feature = "surreal-types")]
fn coords_to_linestring(coords: Vec<[f64; 2]>) -> geo_types::LineString {
    geo_types::LineString(
        coords
            .into_iter()
            .map(|[x, y]| geo_types::Coord { x, y })
            .collect(),
    )
}

/// Build a geo Polygon from a rings list (first ring is exterior, rest are holes).
#[cfg(feature = "surreal-types")]
fn rings_to_polygon(rings: Vec<Vec<[f64; 2]>>) -> geo_types::Polygon {
    let mut iter = rings.into_iter();
    let exterior = iter
        .next()
        .map(coords_to_linestring)
        .unwrap_or_else(|| geo_types::LineString::new(vec![]));
    let interiors: Vec<_> = iter.map(coords_to_linestring).collect();
    geo_types::Polygon::new(exterior, interiors)
}

#[cfg(feature = "surreal-types")]
impl From<surrealdb_types::Geometry> for Geometry {
    fn from(g: surrealdb_types::Geometry) -> Self {
        match g {
            surrealdb_types::Geometry::Point(p) => Geometry::Point([p.x(), p.y()]),
            surrealdb_types::Geometry::Line(ls) => Geometry::Line(linestring_to_coords(&ls)),
            surrealdb_types::Geometry::Polygon(poly) => {
                let mut rings = vec![linestring_to_coords(poly.exterior())];
                rings.extend(poly.interiors().iter().map(linestring_to_coords));
                Geometry::Polygon(rings)
            }
            surrealdb_types::Geometry::MultiPoint(mp) => {
                Geometry::MultiPoint(mp.0.iter().map(|p| [p.x(), p.y()]).collect())
            }
            surrealdb_types::Geometry::MultiLine(mls) => {
                Geometry::MultiLine(mls.0.iter().map(linestring_to_coords).collect())
            }
            surrealdb_types::Geometry::MultiPolygon(mpoly) => Geometry::MultiPolygon(
                mpoly
                    .0
                    .iter()
                    .map(|poly| {
                        let mut rings = vec![linestring_to_coords(poly.exterior())];
                        rings.extend(poly.interiors().iter().map(linestring_to_coords));
                        rings
                    })
                    .collect(),
            ),
            surrealdb_types::Geometry::Collection(coll) => {
                Geometry::Collection(coll.into_iter().map(Geometry::from).collect())
            }
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<Geometry> for surrealdb_types::Geometry {
    fn from(g: Geometry) -> Self {
        use geo_types::{Coord, MultiLineString, MultiPoint, MultiPolygon, Point};
        match g {
            Geometry::Point([x, y]) => surrealdb_types::Geometry::Point(Point(Coord { x, y })),
            Geometry::Line(coords) => surrealdb_types::Geometry::Line(coords_to_linestring(coords)),
            Geometry::Polygon(rings) => surrealdb_types::Geometry::Polygon(rings_to_polygon(rings)),
            Geometry::MultiPoint(pts) => surrealdb_types::Geometry::MultiPoint(MultiPoint(
                pts.into_iter()
                    .map(|[x, y]| Point(Coord { x, y }))
                    .collect(),
            )),
            Geometry::MultiLine(lines) => surrealdb_types::Geometry::MultiLine(MultiLineString(
                lines.into_iter().map(coords_to_linestring).collect(),
            )),
            Geometry::MultiPolygon(polys) => surrealdb_types::Geometry::MultiPolygon(MultiPolygon(
                polys.into_iter().map(rings_to_polygon).collect(),
            )),
            Geometry::Collection(coll) => surrealdb_types::Geometry::Collection(
                coll.into_iter()
                    .map(surrealdb_types::Geometry::from)
                    .collect(),
            ),
        }
    }
}
