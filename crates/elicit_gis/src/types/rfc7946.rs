//! RFC 7946 GeoJSON value types.
//!
//! These types are the construction inputs and descriptor receipts used by
//! [`GeoJsonGeometryFactory`] and [`GeoJsonFeatureFactory`].
//!
//! Source: RFC 7946 — The GeoJSON Format.
//!
//! [`GeoJsonGeometryFactory`]: crate::GeoJsonGeometryFactory
//! [`GeoJsonFeatureFactory`]: crate::GeoJsonFeatureFactory

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Input types ───────────────────────────────────────────────────────────────

/// A GeoJSON position: [longitude, latitude] or [longitude, latitude, altitude].
///
/// Coordinates use the WGS 84 CRS (EPSG 4326) with longitude first, as
/// mandated by RFC 7946 §3.1.1.  Altitude, when present, is in metres above
/// the WGS 84 ellipsoid.
///
/// The factory validates `PositionHasAtLeastTwoElements`,
/// `PositionElementsAreJsonNumbers`, `PositionLongitudeIsFinite`,
/// `PositionLatitudeIsFinite`, `PositionLongitudeInRange`, and
/// `PositionLatitudeInRange` before issuing `Established<PositionValid>`.
///
/// Source: RFC 7946 §3.1.1 — Position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GeoJsonPosition {
    /// Longitude in decimal degrees (WGS 84), element \[0\] of the JSON array.
    pub longitude: f64,
    /// Latitude in decimal degrees (WGS 84), element \[1\] of the JSON array.
    pub latitude: f64,
    /// Optional altitude in metres above the WGS 84 ellipsoid, element \[2\].
    pub altitude: Option<f64>,
}

/// Discriminant for the optional GeoJSON feature id.
///
/// RFC 7946 §3.2 states the "id" member, when present, is a JSON string or
/// JSON number.  `GeoJsonFeatureId` captures this union and excludes all other
/// JSON value types (null, boolean, array, object).
///
/// Source: RFC 7946 §3.2 — Feature Object id member.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum GeoJsonFeatureId {
    /// Feature id expressed as a JSON string.
    String(String),
    /// Feature id expressed as a JSON number.
    ///
    /// `f64` is used because RFC 7946 is built on I-JSON (RFC 7159), which
    /// allows fractional numbers.  Integer ids must fit within the IEEE 754
    /// double safe-integer range.
    Number(f64),
}

// ── Descriptor / receipt types ────────────────────────────────────────────────

/// Discriminant for a GeoJSON geometry type.
///
/// Used inside [`GeoJsonGeometryDescriptor`] to record which of the seven
/// geometry kinds was successfully constructed or parsed.
///
/// Source: RFC 7946 §3.1 — Geometry Object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum GeoJsonGeometryKind {
    /// RFC 7946 §3.1.2 — Point.
    Point,
    /// RFC 7946 §3.1.3 — MultiPoint.
    MultiPoint,
    /// RFC 7946 §3.1.4 — LineString.
    LineString,
    /// RFC 7946 §3.1.5 — MultiLineString.
    MultiLineString,
    /// RFC 7946 §3.1.6 — Polygon.
    Polygon,
    /// RFC 7946 §3.1.7 — MultiPolygon.
    MultiPolygon,
    /// RFC 7946 §3.1.8 — GeometryCollection.
    GeometryCollection,
}

/// Construction receipt for a GeoJSON geometry object.
///
/// Returned by all [`GeoJsonGeometryFactory`] build and parse methods.
/// Records the geometry kind, total position count, coordinate dimensionality,
/// ring count, and component count.
///
/// Source: RFC 7946 §3.1 — Geometry Object.
///
/// [`GeoJsonGeometryFactory`]: crate::GeoJsonGeometryFactory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GeoJsonGeometryDescriptor {
    /// The type of geometry that was constructed.
    pub kind: GeoJsonGeometryKind,
    /// Total number of positions across all rings and components.
    pub total_positions: usize,
    /// Whether any position carries an altitude (third element).
    pub has_altitude: bool,
    /// For Polygon and MultiPolygon: total number of rings (exterior + holes).
    pub num_rings: usize,
    /// For multi-geometry types and GeometryCollection: number of components.
    pub num_components: usize,
}

/// Construction receipt for a GeoJSON Feature object.
///
/// Returned by [`GeoJsonFeatureFactory::build_geojson_feature`].
///
/// Source: RFC 7946 §3.2 — Feature Object.
///
/// [`GeoJsonFeatureFactory::build_geojson_feature`]: crate::GeoJsonFeatureFactory::build_geojson_feature
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GeoJsonFeatureDescriptor {
    /// The optional feature identifier.
    pub id: Option<GeoJsonFeatureId>,
    /// Whether the "geometry" member is JSON null (an unlocated feature).
    pub geometry_is_null: bool,
    /// Whether the "properties" member is JSON null.
    pub properties_is_null: bool,
}

/// Construction receipt for a GeoJSON FeatureCollection object.
///
/// Returned by [`GeoJsonFeatureFactory::build_geojson_feature_collection`].
///
/// Source: RFC 7946 §3.3 — FeatureCollection Object.
///
/// [`GeoJsonFeatureFactory::build_geojson_feature_collection`]: crate::GeoJsonFeatureFactory::build_geojson_feature_collection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GeoJsonFeatureCollectionDescriptor {
    /// Number of Feature objects in the collection.
    pub num_features: usize,
    /// Whether a top-level "bbox" member was supplied.
    pub has_bbox: bool,
}

/// Construction receipt for a complete GeoJSON document.
///
/// Returned by the document parse factory
/// ([`GeoJsonFeatureFactory::document_from_geojson_str`]).
///
/// Source: RFC 7946 §2 — GeoJSON Text.
///
/// [`GeoJsonFeatureFactory::document_from_geojson_str`]: crate::GeoJsonFeatureFactory::document_from_geojson_str
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GeoJsonDocumentDescriptor {
    /// The top-level GeoJSON type discriminant.
    ///
    /// One of the nine RFC 7946 type strings: `"Point"`, `"MultiPoint"`,
    /// `"LineString"`, `"MultiLineString"`, `"Polygon"`, `"MultiPolygon"`,
    /// `"GeometryCollection"`, `"Feature"`, or `"FeatureCollection"`.
    pub root_type: String,
    /// Whether a top-level "bbox" member is present.
    pub has_top_level_bbox: bool,
}
