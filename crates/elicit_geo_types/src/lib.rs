//! `elicit_geo_types` — elicitation-enabled geo-types shadow crate.
//!
//! Provides elicitation-enabled wrappers around the geo-types spatial primitives,
//! with MCP tools exposed through four plugins:
//!
//! | Plugin | Namespace | Tools |
//! |--------|-----------|-------|
//! | [`GeoTypesPrimitivesPlugin`] | `geo_types_primitives` | 8 |
//! | [`GeoTypesShapesPlugin`] | `geo_types_shapes` | 8 |
//! | [`GeoTypesCollectionsPlugin`] | `geo_types_collections` | 10 |
//! | [`GeoTypesGeometryPlugin`] | `geo_types_geometry` | 6 |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod collections;
mod geometry;
mod primitives;
mod shapes;
pub mod workflow;

pub use collections::{GeometryCollection, LineString, MultiLineString, MultiPoint, MultiPolygon};
pub use geometry::Geometry;
pub use primitives::{Coord, Line, Point, Triangle};
pub use shapes::{Polygon, Rect};
pub use workflow::{
    GeoTypesCollectionsPlugin, GeoTypesGeometryPlugin, GeoTypesPrimitivesPlugin,
    GeoTypesShapesPlugin,
};
