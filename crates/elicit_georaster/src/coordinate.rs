//! `Coordinate` — georaster coordinate wrapper.

/// Serializable shadow of [`georaster::Coordinate`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct Coordinate {
    /// Longitude / X coordinate.
    pub x: f64,
    /// Latitude / Y coordinate.
    pub y: f64,
}

impl Coordinate {
    /// Create a new coordinate from latitude and longitude, matching upstream semantics.
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            x: longitude,
            y: latitude,
        }
    }
}

impl From<georaster::Coordinate> for Coordinate {
    fn from(value: georaster::Coordinate) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Coordinate> for georaster::Coordinate {
    fn from(value: Coordinate) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<(f64, f64)> for Coordinate {
    fn from(value: (f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<[f64; 2]> for Coordinate {
    fn from(value: [f64; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}
