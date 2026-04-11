//! Elicitation implementations for [`wkt`] spatial format types.
//!
//! Provides [`Elicitation`](crate::Elicitation) for wkt 0.11 types
//! that can be interactively constructed from an agent.
//!
//! # Enabled by the `wkt-types` feature
//!
//! ```toml
//! elicitation = { version = "*", features = ["wkt-types"] }
//! ```

mod coord;
mod geometry_collection;
mod linestring;
mod multilinestring;
mod multipoint;
mod multipolygon;
mod point;
mod polygon;
mod wkt_geom;
mod wkt_string;

pub use coord::{WktCoord, WktCoordStyle};
pub use geometry_collection::{WktGeometryCollection, WktGeometryCollectionStyle};
pub use linestring::{WktLineString, WktLineStringStyle};
pub use multilinestring::{WktMultiLineString, WktMultiLineStringStyle};
pub use multipoint::{WktMultiPoint, WktMultiPointStyle};
pub use multipolygon::{WktMultiPolygon, WktMultiPolygonStyle};
pub use point::{WktPoint, WktPointStyle};
pub use polygon::{WktPolygon, WktPolygonStyle};
pub use wkt_geom::{WktGeom, WktGeomStyle};
pub use wkt_string::{WktString, WktStringStyle};
