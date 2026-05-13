//! Authority/code identifiers for CRS and related objects.
//!
//! Source: ISO 19111:2019 §6.2 — RS_Identifier.

use derive_more::{Display, From};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

/// An authority-qualified identifier referencing a registered object.
///
/// Corresponds to `RS_Identifier` in ISO 19111:2019 §6.2.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub struct AuthorityCode {
    /// Registry authority name (e.g. "EPSG", "OGC", "ESRI", "IGNF").
    pub authority: String,
    /// Code within the authority's registry.
    pub code: String,
}

impl AuthorityCode {
    /// Construct an EPSG-authority code.
    pub fn epsg(code: u32) -> Self {
        Self {
            authority: "EPSG".into(),
            code: code.to_string(),
        }
    }

    /// Construct an OGC-authority code.
    pub fn ogc(code: impl Into<String>) -> Self {
        Self {
            authority: "OGC".into(),
            code: code.into(),
        }
    }

    /// Returns `true` when both authority and code are non-empty.
    pub fn is_valid(&self) -> bool {
        !self.authority.is_empty() && !self.code.is_empty()
    }
}

impl fmt::Display for AuthorityCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.authority, self.code)
    }
}

/// Lightweight CRS listing entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CrsInfo {
    /// Authority and code identifying this CRS.
    pub authority_code: AuthorityCode,
    /// Human-readable CRS name.
    pub name: String,
    /// CRS type classification.
    pub crs_type: super::crs::CrsType,
    /// Informal description of the area of use, if known.
    pub area_of_use: Option<String>,
}

/// Datum ensemble metadata.
///
/// Source: ISO 19111:2019 §6.5 — CD_DatumEnsemble.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DatumEnsembleInfo {
    /// Ensemble name (e.g. "World Geodetic System 1984 ensemble").
    pub name: String,
    /// Authority code identifying the ensemble (e.g. EPSG:6326).
    pub authority_code: AuthorityCode,
    /// Individual datum realizations that are members of this ensemble.
    ///
    /// Multiplicity: 2..* (at least two members required).
    pub members: Vec<AuthorityCode>,
    /// Positional accuracy (metres) within which all members agree.
    ///
    /// Must be > 0.
    pub ensemble_accuracy: f64,
}

/// Ellipsoid parameters.
///
/// Source: ISO 19111:2019 §7.3 — CD_Ellipsoid.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct EllipsoidParams {
    /// Ellipsoid name.
    pub name: String,
    /// Semi-major axis in metres (must be > 0).
    pub semi_major_axis: f64,
    /// Inverse flattening (≥ 0; 0 = sphere).
    ///
    /// Either this or `semi_minor_axis` must be provided.
    pub inverse_flattening: Option<f64>,
    /// Semi-minor axis in metres (must be < `semi_major_axis` when provided).
    ///
    /// Either this or `inverse_flattening` must be provided.
    pub semi_minor_axis: Option<f64>,
}

impl EllipsoidParams {
    /// Returns `true` when the ellipsoid represents a sphere
    /// (`inverse_flattening == 0` or `semi_major_axis == semi_minor_axis`).
    pub fn is_sphere(&self) -> bool {
        self.inverse_flattening == Some(0.0)
            || self
                .semi_minor_axis
                .map(|b| (b - self.semi_major_axis).abs() < f64::EPSILON)
                .unwrap_or(false)
    }
}

/// Newtype for EPSG integer codes.
///
/// Source: ISO 19111:2019 §16 — EPSG registry codes are positive integers.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Display,
    From,
    Serialize,
    Deserialize,
    JsonSchema,
)]
pub struct EpsgCode(pub u32);

impl From<EpsgCode> for AuthorityCode {
    fn from(code: EpsgCode) -> Self {
        AuthorityCode::epsg(code.0)
    }
}
