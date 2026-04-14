//! CRS classification types, coordinate epoch, and Helmert convention.
//!
//! Source: ISO 19111:2019 §6–§17.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use super::authority::AuthorityCode;

/// Classification of a coordinate reference system by subtype.
///
/// Source: ISO 19111:2019 §6 — SC_CRS class hierarchy.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    JsonSchema,
    EnumIter,
)]
#[serde(rename_all = "camelCase")]
pub enum CrsType {
    /// Geographic 2D (latitude + longitude).
    Geographic2d,
    /// Geographic 3D (latitude + longitude + ellipsoidal height).
    Geographic3d,
    /// Geocentric (Cartesian XYZ).
    Geocentric,
    /// Projected (easting + northing, derived from a geographic CRS).
    Projected,
    /// Vertical (single height or depth axis).
    Vertical,
    /// Engineering (local, non-georeferenced).
    Engineering,
    /// Compound (two or more component CRSes, e.g. horizontal + vertical).
    Compound,
    /// Derived (base CRS + coordinate conversion).
    Derived,
    /// Temporal (single time axis).
    Temporal,
    /// Parametric (single parametric quantity, e.g. pressure).
    Parametric,
}

/// Decimal-year coordinate epoch.
///
/// A positive finite real number representing a year with fractional part
/// (e.g. 2023.5 = 1 July 2023 12:00 UTC).
///
/// Required when coordinates are referenced to a dynamic CRS such as ITRF2020.
///
/// Source: ISO 19111:2019 §17.2 — coordinateEpoch.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, JsonSchema)]
pub struct DecimalYear(pub f64);

impl DecimalYear {
    /// Returns `true` when the epoch value is a positive finite real number.
    pub fn is_valid(&self) -> bool {
        self.0.is_finite() && self.0 > 0.0
    }
}

/// Rotation convention for the 7-parameter Helmert transformation.
///
/// The two conventions use opposite signs for rotation parameters.
/// Mixing them without explicit identification produces systematic errors.
///
/// Source: ISO 19111:2019 §11.4 / EPSG Guidance Note 7-2 §2.4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, EnumIter)]
#[serde(rename_all = "camelCase")]
pub enum HelmertConvention {
    /// Position Vector (ISO method 9606): rotations apply to the coordinate frame;
    /// standard ISO convention.
    PositionVector,
    /// Coordinate Frame (EPSG method 9607 / Bursa-Wolf): rotations apply to the
    /// coordinate system; opposite sign from Position Vector.
    CoordinateFrame,
}

/// Coordinate metadata: a CRS reference plus an optional coordinate epoch.
///
/// Source: ISO 19111:2019 §7.4 — SC_CoordinateMetadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CoordinateMetadata {
    /// The CRS to which these coordinates are referenced.
    pub crs: AuthorityCode,
    /// Coordinate epoch (decimal year) — required for dynamic CRS, absent for static.
    pub coordinate_epoch: Option<DecimalYear>,
}

impl CoordinateMetadata {
    /// Construct coordinate metadata for a static CRS (no epoch).
    pub fn static_crs(crs: impl Into<AuthorityCode>) -> Self {
        Self {
            crs: crs.into(),
            coordinate_epoch: None,
        }
    }

    /// Construct coordinate metadata for a dynamic CRS with a required epoch.
    pub fn dynamic_crs(crs: impl Into<AuthorityCode>, epoch: DecimalYear) -> Self {
        Self {
            crs: crs.into(),
            coordinate_epoch: Some(epoch),
        }
    }
}
