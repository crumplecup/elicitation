//! Elicitation implementations for [`geojson`] document and value types.
//!
//! Provides [`Elicitation`](crate::Elicitation) for the core GeoJSON RFC 7946
//! document/value vocabulary:
//! - [`geojson::GeoJson`]
//! - [`geojson::Geometry`]
//! - [`geojson::GeometryValue`]
//! - [`geojson::Feature`]
//! - [`geojson::FeatureCollection`]
//! - [`geojson::feature::Id`]

mod feature;
mod feature_collection;
mod geojson;
mod geometry;
mod geometry_value;
mod helpers;
mod id;

pub use feature::GeoJsonFeatureStyle;
pub use feature_collection::GeoJsonFeatureCollectionStyle;
pub use geojson::GeoJsonStyle;
pub use geometry::GeoJsonGeometryStyle;
pub use geometry_value::GeoJsonGeometryValueStyle;
pub use id::GeoJsonIdStyle;
