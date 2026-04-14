//! Axis direction enumeration and coordinate system type enumeration.
//!
//! Source: ISO 19111:2019 §8.3 (axis directions) and §8.4 (CS types).

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Direction of a coordinate system axis.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list.
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
pub enum AxisDirection {
    /// Increasing toward geographic north.
    North,
    /// Increasing toward geographic south.
    South,
    /// Increasing toward geographic east.
    East,
    /// Increasing toward geographic west.
    West,
    /// Increasing upward (height / elevation).
    Up,
    /// Increasing downward (depth).
    Down,
    /// North-northeast direction.
    NorthNorthEast,
    /// Northeast direction.
    NorthEast,
    /// East-northeast direction.
    EastNorthEast,
    /// East-southeast direction.
    EastSouthEast,
    /// Southeast direction.
    SouthEast,
    /// South-southeast direction.
    SouthSouthEast,
    /// South-southwest direction.
    SouthSouthWest,
    /// Southwest direction.
    SouthWest,
    /// West-southwest direction.
    WestSouthWest,
    /// West-northwest direction.
    WestNorthWest,
    /// Northwest direction.
    NorthWest,
    /// North-northwest direction.
    NorthNorthWest,
    /// Geocentric X axis (toward intersection of equator and prime meridian, 0°N 0°E).
    GeocentricX,
    /// Geocentric Y axis (toward 0°N 90°E).
    GeocentricY,
    /// Geocentric Z axis (toward North Pole).
    GeocentricZ,
    /// Increasing column index (grid / raster positive column).
    ColumnPositive,
    /// Decreasing column index.
    ColumnNegative,
    /// Increasing row index (grid / raster positive row).
    RowPositive,
    /// Decreasing row index.
    RowNegative,
    /// Display right (screen X+).
    DisplayRight,
    /// Display left (screen X−).
    DisplayLeft,
    /// Display up (screen Y+).
    DisplayUp,
    /// Display down (screen Y−).
    DisplayDown,
    /// Forward in time.
    Future,
    /// Backward in time.
    Past,
    /// Toward a body (decreasing distance).
    Towards,
    /// Away from a body (increasing distance).
    AwayFrom,
    /// Counter-clockwise angular increase.
    CounterClockwise,
    /// Clockwise angular increase.
    Clockwise,
    /// Direction is not specified.
    Unspecified,
}

/// Type of coordinate system.
///
/// Source: ISO 19111:2019 §8.4 — CS type enumeration.
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
pub enum CsType {
    /// Ellipsoidal CS: latitude + longitude (+ optional height).
    Ellipsoidal,
    /// Cartesian CS: perpendicular axes (projected easting/northing or geocentric XYZ).
    Cartesian,
    /// Vertical CS: single height or depth axis.
    Vertical,
    /// Temporal CS: single time axis.
    Temporal,
    /// Parametric CS: single parametric quantity axis (e.g. pressure).
    Parametric,
    /// Ordinal CS: discrete ordered labels, no unit of measure.
    Ordinal,
    /// Affine CS: general axes with a defined origin, not necessarily orthogonal.
    Affine,
    /// Polar CS: 2D distance + angle.
    Polar,
    /// Cylindrical CS: 3D distance + azimuth angle + height.
    Cylindrical,
    /// Spherical CS: 3D spherical latitude + spherical longitude + radius.
    Spherical,
}
