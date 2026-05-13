//! `elicit_geojson` — elicitation-enabled wrappers around GeoJSON types.
//!
//! This crate intentionally mirrors the upstream `geojson` document/value
//! surface:
//! `GeoJson`, `Geometry`, `Value`, `Feature`, `FeatureCollection`, and
//! `feature::Id`.
//!
//! # Workflow plugins
//!
//! | Plugin | Namespace | Description |
//! |---|---|---|
//! | [`GeoJsonDocumentPlugin`] | `geojson_document__*` | Parse and inspect top-level GeoJSON documents |
//! | [`GeoJsonGeometryPlugin`] | `geojson_geometry__*` | Construct and inspect `Geometry` / `Value` |
//! | [`GeoJsonFeaturePlugin`] | `geojson_feature__*` | Construct and inspect `Feature` / `FeatureCollection` / `Id` |
//! | [`GeoJsonConversionPlugin`] | `geojson_conversion__*` | Bridge GeoJSON wrappers to and from `elicit_geo_types` |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod error;
mod feature;
mod feature_collection;
mod geojson;
mod geometry;
mod geometry_value;
mod helpers;
mod id;
mod workflow;

pub use error::{GeoJsonError, GeoJsonResult};
pub use feature::Feature;
pub use feature_collection::FeatureCollection;
pub use geojson::GeoJson;
pub use geometry::Geometry;
pub use geometry_value::Value;
pub use id::Id;
pub use workflow::{
    FeatureCollectionFromFeaturesParams, FeatureCollectionFromGeoGeometryCollectionParams,
    FeatureCollectionFromJsonValueParams, FeatureContainsPropertyParams, FeatureFromGeometryParams,
    FeatureFromJsonValueParams, FeatureFromValueParams, FeatureLenPropertiesParams,
    FeaturePropertyParams, FeatureRemovePropertyParams, FeatureSetPropertyParams,
    GeoGeometryFromFeatureCollectionParams, GeoGeometryFromFeatureParams,
    GeoGeometryFromGeoJsonParams, GeoGeometryFromGeometryParams, GeoGeometryFromValueParams,
    GeoJsonConversionPlugin, GeoJsonConverted, GeoJsonDocumentParsed, GeoJsonDocumentPlugin,
    GeoJsonFeatureCreated, GeoJsonFeaturePlugin, GeoJsonFromJsonValueParams, GeoJsonFromStrParams,
    GeoJsonGeometryCreated, GeoJsonGeometryPlugin, GeoJsonToJsonValueParams,
    GeoJsonToStringPrettyParams, GeoJsonVariantParams, GeometryFromGeoGeometryParams,
    GeometryFromJsonValueParams, GeometryNewParams, IdNumberParams, IdStringParams,
    ValueFromGeoGeometryParams, ValueFromJsonValueParams, ValueGeometryCollectionParams,
    ValueLineStringParams, ValueMultiLineStringParams, ValueMultiPointParams,
    ValueMultiPolygonParams, ValuePointParams, ValuePolygonParams, ValueTypeNameParams,
};
