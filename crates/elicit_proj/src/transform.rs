//! Serializable snapshot wrapper for [`proj::Proj`].
//!
//! Because [`proj::Proj`] holds raw FFI pointers it cannot be held across async
//! suspension points or serialised to JSON.  [`ProjTransform`] stores only the
//! *specification* used to create the transform (a PROJ string or a pair of
//! known CRS identifiers) and rebuilds the underlying [`proj::Proj`] on each
//! operation.  This makes the type safe to round-trip through MCP tool calls.

use crate::{ProjResult, ProjTransformError};
use elicitation::{GeoCoord, GeoGeometry, ProjArea};
use geo::MapCoords;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// How a [`ProjTransform`] was specified.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind")]
pub enum ProjSpec {
    /// Defined by a PROJ string (pipeline, datum conversion, etc.).
    ProjString {
        /// The PROJ string definition.
        definition: String,
    },
    /// Defined by two known CRS identifiers (EPSG codes or authority strings).
    KnownCrs {
        /// Source CRS (e.g. `"EPSG:4326"`).
        from: String,
        /// Target CRS (e.g. `"EPSG:3857"`).
        to: String,
        /// Optional area of use to guide the best CRS selection.
        area: Option<ProjArea>,
    },
}

/// Serializable snapshot of a [`proj::Proj`] coordinate transformation.
///
/// Each mutating operation rebuilds the underlying PROJ handle from the stored
/// specification.  The snapshot is cheap to clone and safe to pass through MCP
/// tool calls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ProjTransform {
    /// The specification used to recreate the transformation on demand.
    pub spec: ProjSpec,
}

impl ProjTransform {
    /// Create a [`ProjTransform`] from a PROJ string definition.
    #[instrument(skip(definition))]
    pub fn from_proj_string(definition: impl Into<String>) -> Self {
        Self {
            spec: ProjSpec::ProjString {
                definition: definition.into(),
            },
        }
    }

    /// Create a [`ProjTransform`] from two known CRS identifiers.
    ///
    /// The `from` and `to` arguments accept EPSG codes (`"EPSG:4326"`),
    /// authority strings, or PROJ strings.  Pass an optional [`ProjArea`] to
    /// narrow the operation selection for cases where multiple transforms exist.
    #[instrument(skip(from, to))]
    pub fn from_known_crs(
        from: impl Into<String>,
        to: impl Into<String>,
        area: Option<ProjArea>,
    ) -> Self {
        Self {
            spec: ProjSpec::KnownCrs {
                from: from.into(),
                to: to.into(),
                area,
            },
        }
    }

    /// Rebuild the underlying [`proj::Proj`] from this snapshot.
    #[instrument]
    pub(crate) fn build(&self) -> ProjResult<proj::Proj> {
        match &self.spec {
            ProjSpec::ProjString { definition } => {
                proj::Proj::new(definition).map_err(|e| ProjTransformError::create(e))
            }
            ProjSpec::KnownCrs { from, to, area } => {
                proj::Proj::new_known_crs(from, to, area.map(|a| a.into()))
                    .map_err(|e| ProjTransformError::create(e))
            }
        }
    }

    /// Convert a single coordinate from the source CRS to the target CRS.
    ///
    /// Coordinate order is normalised to Longitude/Latitude (or Easting/Northing)
    /// when the transform was created with [`from_known_crs`][Self::from_known_crs].
    #[instrument]
    pub fn convert_coord(&self, coord: GeoCoord) -> ProjResult<GeoCoord> {
        let p = self.build()?;
        let (nx, ny) = p
            .convert((coord.x, coord.y))
            .map_err(|e| ProjTransformError::operation(e))?;
        Ok(GeoCoord { x: nx, y: ny })
    }

    /// Project a geodetic coordinate to/from the projection plane.
    ///
    /// Input coordinates are in radians for the forward direction and in
    /// projected units for the inverse direction.  Set `inverse = true` for the
    /// reverse (projected → geodetic in radians) operation.
    #[instrument]
    pub fn project_coord(&self, coord: GeoCoord, inverse: bool) -> ProjResult<GeoCoord> {
        let p = self.build()?;
        let (nx, ny) = p
            .project((coord.x, coord.y), inverse)
            .map_err(|e| ProjTransformError::operation(e))?;
        Ok(GeoCoord { x: nx, y: ny })
    }

    /// Convert all coordinates in a geometry from the source CRS to the target CRS.
    #[instrument(skip(geometry))]
    pub fn convert_geometry(&self, geometry: GeoGeometry) -> ProjResult<GeoGeometry> {
        let p = self.build()?;
        let upstream: geo::Geometry<f64> = geometry.into();
        let transformed = upstream
            .try_map_coords(|coord| {
                let (nx, ny) = p
                    .convert((coord.x, coord.y))
                    .map_err(|e| ProjTransformError::operation(e))?;
                Ok(geo::Coord { x: nx, y: ny })
            })
            .map_err(|e: ProjTransformError| e)?;
        Ok(GeoGeometry::from(transformed))
    }

    /// Return the PROJ string definition that describes this transformation.
    #[instrument]
    pub fn definition(&self) -> ProjResult<String> {
        self.build()?
            .def()
            .map_err(|e| ProjTransformError::operation(e))
    }

    /// Transform a bounding box, densifying edges to account for non-linear curvature.
    ///
    /// Returns `[west, south, east, north]` in the target CRS.
    /// A `densify_pts` of `21` is a reasonable default for geographic bounding boxes.
    #[instrument]
    pub fn transform_bounds(
        &self,
        west: f64,
        south: f64,
        east: f64,
        north: f64,
        densify_pts: i32,
    ) -> ProjResult<[f64; 4]> {
        let p = self.build()?;
        p.transform_bounds(west, south, east, north, densify_pts)
            .map_err(|e| ProjTransformError::operation(e))
    }
}

impl elicitation::emit_code::ToCodeLiteral for ProjTransform {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let json = serde_json::to_string(self).expect("ProjTransform is always serializable");
        quote::quote! {
            ::serde_json::from_str::<::elicit_proj::ProjTransform>(#json)
                .expect("valid ProjTransform JSON")
        }
    }
}
