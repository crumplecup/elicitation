//! ISO 19111-specific domain types used in factory-trait signatures.
//!
//! These types represent raw input parameters and validated output descriptors
//! for CRS components built by [`GisCrsBuilder`] and the orthogonal concern
//! traits [`Iso19111Identified`] and [`Iso19111Scoped`].
//!
//! Source: ISO 19111:2019 — Spatial referencing by coordinates.
//!
//! [`GisCrsBuilder`]: crate::GisCrsBuilder
//! [`Iso19111Identified`]: crate::Iso19111Identified
//! [`Iso19111Scoped`]: crate::Iso19111Scoped

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::authority::AuthorityCode;
use super::axis::{AxisDirection, CsType};

// ── Prime meridian ────────────────────────────────────────────────────────────

/// Raw parameters for a prime meridian definition.
///
/// Source: ISO 19111:2019 §7.4 — CD_PrimeMeridian.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PrimeMeridianParams {
    /// Prime meridian name (e.g. "Greenwich", "Paris").
    pub name: String,
    /// Longitude of the prime meridian from Greenwich, in the unit given by `uom`.
    ///
    /// For Greenwich itself this is exactly 0.
    pub greenwich_longitude: f64,
    /// Unit of measure for `greenwich_longitude` (must be an angular unit).
    pub uom: AuthorityCode,
}

// ── Coordinate system ─────────────────────────────────────────────────────────

/// Descriptor for a single coordinate system axis.
///
/// Source: ISO 19111:2019 §8.3 — CS_CoordinateSystemAxis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CoordinateAxisInfo {
    /// Full axis name (e.g. "Latitude", "Easting").
    pub name: String,
    /// Short abbreviation used in formulas (e.g. "φ", "E").
    ///
    /// Must be unique within its coordinate system.
    pub abbreviation: String,
    /// Positive direction of increasing coordinate values.
    pub direction: AxisDirection,
    /// Unit of measure for this axis (absent for dimensionless / ordinal axes).
    pub uom: Option<AuthorityCode>,
}

/// Raw parameters for a coordinate system definition.
///
/// Source: ISO 19111:2019 §8 — CS_CoordinateSystem.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CoordinateSystemParams {
    /// Coordinate system type (ellipsoidal, Cartesian, vertical, …).
    pub cs_type: CsType,
    /// Ordered list of axes.
    ///
    /// Multiplicity: 1..4 (at least one axis, at most four).
    pub axes: Vec<CoordinateAxisInfo>,
}

// ── Geodetic reference frame ──────────────────────────────────────────────────

/// Raw parameters for a geodetic reference frame.
///
/// The associated ellipsoid and prime meridian are supplied separately as
/// `Established<EllipsoidValid>` and `Established<PrimeMeridianValid>` tokens
/// to [`GisCrsBuilder::build_geodetic_frame`].
///
/// Source: ISO 19111:2019 §7.2 — CD_GeodeticReferenceFrame.
///
/// [`GisCrsBuilder::build_geodetic_frame`]: crate::GisCrsBuilder::build_geodetic_frame
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GeodeticFrameParams {
    /// Reference frame name (e.g. "World Geodetic System 1984").
    pub name: String,
    /// Optional free-text description of the relationship to the earth's surface.
    pub anchor_definition: Option<String>,
    /// Realization epoch in ISO 8601 format (e.g. "1984-01-01"), if applicable.
    pub realization_epoch: Option<String>,
    /// Authority code identifying this reference frame in a registry.
    pub authority_code: Option<AuthorityCode>,
}

// ── Domain of validity ────────────────────────────────────────────────────────

/// Geographic bounding box describing an area of use.
///
/// Source: ISO 19115-1:2014 §B.3.1.2 — EX_GeographicBoundingBox.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GeographicBoundingBox {
    /// Western-most longitude (degrees, WGS 84), inclusive.
    pub west_longitude: f64,
    /// Eastern-most longitude (degrees, WGS 84), inclusive.
    pub east_longitude: f64,
    /// Southern-most latitude (degrees, WGS 84), inclusive.
    pub south_latitude: f64,
    /// Northern-most latitude (degrees, WGS 84), inclusive.
    pub north_latitude: f64,
}

/// Extent defining the domain of validity of a CRS or coordinate operation.
///
/// Corresponds to `EX_Extent` in ISO 19115-1:2014 §B.3.1 as referenced by
/// ISO 19111:2019 §6.
///
/// A `None` geographic bounding box implies global applicability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DomainExtent {
    /// Human-readable description of the domain of validity.
    pub description: Option<String>,
    /// Geographic bounding box, if geographically restricted.
    pub geographic_bbox: Option<GeographicBoundingBox>,
}
