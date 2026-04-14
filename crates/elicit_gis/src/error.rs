//! Error types for `elicit_gis`.

use derive_more::{Display, Error};

/// Specific error conditions for geospatial operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum GisErrorKind {
    /// CRS identifier could not be resolved.
    #[display("CRS not found: {}", _0)]
    CrsNotFound(String),
    /// CRS definition is structurally invalid.
    #[display("Invalid CRS: {}", _0)]
    InvalidCrs(String),
    /// Coordinate transformation failed.
    #[display("Transform failed: {}", _0)]
    TransformFailed(String),
    /// Coordinate values are out of the valid range for the CRS.
    #[display("Coordinate out of range: {}", _0)]
    CoordinateOutOfRange(String),
    /// Coordinate tuple dimension does not match the CRS axis count.
    #[display("Dimension mismatch: {}", _0)]
    DimensionMismatch(String),
    /// Ellipsoid parameters are invalid.
    #[display("Invalid ellipsoid: {}", _0)]
    InvalidEllipsoid(String),
    /// A dynamic CRS was used without a required coordinate epoch.
    #[display("Missing coordinate epoch for dynamic CRS: {}", _0)]
    MissingCoordinateEpoch(String),
    /// Datum ensemble resolution failed.
    #[display("Datum ensemble error: {}", _0)]
    DatumEnsembleError(String),
    /// Axis order could not be determined or normalized.
    #[display("Axis order error: {}", _0)]
    AxisOrderError(String),
    /// Geometry violates §6.1.3 validity rules.
    #[display("Invalid geometry: {}", _0)]
    InvalidGeometry(String),
    /// WKT or WKB serialization failed.
    #[display("Encode error: {}", _0)]
    EncodeError(String),
    /// WKT or WKB parsing failed.
    #[display("Decode error: {}", _0)]
    DecodeError(String),
    /// Spatial operation failed (e.g., intersection, buffer).
    #[display("Spatial operation failed: {}", _0)]
    SpatialOperationFailed(String),
    /// DE-9IM relate pattern is malformed.
    #[display("Invalid relate pattern: {}", _0)]
    InvalidRelatePattern(String),
    /// Operation is not supported by this backend.
    #[display("Unsupported operation: {}", _0)]
    Unsupported(String),
}

/// Geospatial operation error with source location.
#[derive(Debug, Clone, Display, Error)]
#[display("{} at {}:{}", kind, file, line)]
pub struct GisError {
    /// Specific error kind.
    pub kind: GisErrorKind,
    /// Line number where the error was created.
    pub line: u32,
    /// File where the error was created.
    pub file: &'static str,
}

impl GisError {
    /// Create a new [`GisError`] capturing the call site location.
    #[track_caller]
    pub fn new(kind: GisErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind,
            line: loc.line(),
            file: loc.file(),
        }
    }
}

/// Result type for geospatial operations.
pub type GisResult<T> = Result<T, GisError>;
