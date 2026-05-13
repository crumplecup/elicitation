//! OGC SFS geometry descriptor types.
//!
//! These types are the construction receipts returned by [`SfsGeometryFactory`]
//! build methods.  Each carries the key parameters used in construction
//! alongside the [`Established<P>`] proof token emitted by the factory.
//!
//! Raw coordinate inputs ([`SfsCoordinate`], [`SfsCoordinate3D`]) flow in;
//! descriptors flow out together with validity tokens.
//!
//! Source: OGC 06-103r4 §6.1 — Geometry class hierarchy.
//!
//! [`SfsGeometryFactory`]: crate::SfsGeometryFactory
//! [`Established<P>`]: elicitation::Established

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Raw coordinate input types ────────────────────────────────────────────────

/// A raw 2D coordinate pair used as input to leaf geometry factory methods.
///
/// Coordinates are in SRS units.  The factory validates finiteness
/// (CoordXIsFinite, CoordYIsFinite) before emitting a proof token.
///
/// Source: OGC 06-103r4 §6.1.2 — coordinate dimensionality.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SfsCoordinate {
    /// X ordinate (easting or longitude in SRS units).
    pub x: f64,
    /// Y ordinate (northing or latitude in SRS units).
    pub y: f64,
}

/// A raw 3D coordinate triple used as input to 3D leaf factory methods.
///
/// Source: OGC 06-103r4 §6.1.2 — XYZ coordinate dimensionality.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SfsCoordinate3D {
    /// X ordinate.
    pub x: f64,
    /// Y ordinate.
    pub y: f64,
    /// Z ordinate (elevation or ellipsoidal height).
    pub z: f64,
}

// ── Geometry descriptor types ─────────────────────────────────────────────────

/// Construction receipt for a Point geometry.
///
/// Source: OGC 06-103r4 §6.1.4 — Point.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PointDescriptor {
    /// X coordinate, or `None` for an empty Point.
    pub x: Option<f64>,
    /// Y coordinate, or `None` for an empty Point.
    pub y: Option<f64>,
    /// Optional Z (elevation) coordinate.
    pub z: Option<f64>,
    /// Optional M (measure) coordinate.
    pub m: Option<f64>,
    /// Spatial reference system identifier.
    pub srid: Option<i32>,
}

/// Construction receipt for a LineString geometry.
///
/// Source: OGC 06-103r4 §6.1.6 — LineString.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LineStringDescriptor {
    /// Number of coordinate positions.
    pub num_points: usize,
    /// Whether coordinates carry a Z ordinate.
    pub has_z: bool,
    /// Whether coordinates carry an M ordinate.
    pub has_m: bool,
    /// Spatial reference system identifier.
    pub srid: Option<i32>,
}

/// Construction receipt for a LinearRing geometry.
///
/// A LinearRing is a closed, simple LineString with at least four positions
/// (first position equals last position).
///
/// Source: OGC 06-103r4 §6.1.7 — LinearRing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LinearRingDescriptor {
    /// Number of coordinate positions (first equals last).
    pub num_points: usize,
    /// Whether coordinates carry a Z ordinate.
    pub has_z: bool,
    /// Whether coordinates carry an M ordinate.
    pub has_m: bool,
    /// Spatial reference system identifier.
    pub srid: Option<i32>,
}

/// Construction receipt for a Polygon geometry.
///
/// Source: OGC 06-103r4 §6.1.11 — Polygon.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PolygonDescriptor {
    /// Number of coordinate positions in the exterior ring.
    pub exterior_num_points: usize,
    /// Number of interior (hole) rings.
    pub num_holes: usize,
    /// Whether coordinates carry a Z ordinate.
    pub has_z: bool,
    /// Spatial reference system identifier.
    pub srid: Option<i32>,
}

/// Construction receipt for a MultiPoint, MultiLineString, or MultiPolygon
/// geometry.
///
/// Shared across the three homogeneous multi-geometry types because their
/// construction receipts carry the same information.
///
/// Source: OGC 06-103r4 §6.1.8–§6.1.13 — Multi-geometry types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct MultiGeometryDescriptor {
    /// Number of component geometries.
    pub num_geometries: usize,
    /// Spatial reference system identifier.
    pub srid: Option<i32>,
}

/// Construction receipt for a GeometryCollection.
///
/// A GeometryCollection may contain any mix of geometry types, including
/// other collections.
///
/// Source: OGC 06-103r4 §6.1.14 — GeometryCollection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GeometryCollectionDescriptor {
    /// Number of component geometries (any types).
    pub num_geometries: usize,
    /// Spatial reference system identifier.
    pub srid: Option<i32>,
}
