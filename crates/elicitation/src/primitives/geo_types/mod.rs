//! Elicitation implementations for [`geo_types`] spatial primitives.
//!
//! Provides [`Elicitation`](crate::Elicitation) for geo-types 0.7 types
//! that can be interactively constructed from an agent — composite types
//! via Survey (field-by-field elicitation), collection types via Vec delegation,
//! and the `Geometry` enum via Select.
//!
//! # Enabled by the `geo-types` feature
//!
//! ```toml
//! elicitation = { version = "*", features = ["geo-types"] }
//! ```

// ── Composite struct modules ─────────────────────────────────────────
mod coord;
mod geometry;
mod geometry_collection;
mod line;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;
mod rect;
mod triangle;

// ── Composite struct wrapper re-exports ──────────────────────────────
pub use coord::{GeoCoord, GeoCoordStyle};
pub use geometry::{GeoGeometry, GeoGeometryStyle};
pub use geometry_collection::{GeoGeometryCollection, GeoGeometryCollectionStyle};
pub use line::{GeoLine, GeoLineStyle};
pub use line_string::{GeoLineString, GeoLineStringStyle};
pub use multi_line_string::{GeoMultiLineString, GeoMultiLineStringStyle};
pub use multi_point::{GeoMultiPoint, GeoMultiPointStyle};
pub use multi_polygon::{GeoMultiPolygon, GeoMultiPolygonStyle};
pub use point::{GeoPoint, GeoPointStyle};
pub use polygon::{GeoPolygon, GeoPolygonStyle};
pub use rect::{GeoRect, GeoRectStyle};
pub use triangle::{GeoTriangle, GeoTriangleStyle};
