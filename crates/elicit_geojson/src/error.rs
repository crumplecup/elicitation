//! Error aliases for the GeoJSON shadow crate.

/// Boxed GeoJSON error type used by helper methods to keep public `Result` sizes small.
pub type GeoJsonError = Box<geojson::Error>;

/// Boxed GeoJSON result type used by helper methods on wrapper types.
pub type GeoJsonResult<T> = Result<T, GeoJsonError>;
