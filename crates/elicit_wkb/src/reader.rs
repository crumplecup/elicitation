//! Reader-facing wrappers for the upstream `wkb::reader` module.

use crate::{Endianness, WkbError, WkbResult};

/// Serializable mirror of [`wkb::reader::Dimension`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub enum Dimension {
    /// 2D: X and Y.
    Xy,
    /// 3D: X, Y, and Z.
    Xyz,
    /// 3D: X, Y, and M.
    Xym,
    /// 4D: X, Y, Z, and M.
    Xyzm,
}

impl From<wkb::reader::Dimension> for Dimension {
    fn from(value: wkb::reader::Dimension) -> Self {
        match value {
            wkb::reader::Dimension::Xy => Self::Xy,
            wkb::reader::Dimension::Xyz => Self::Xyz,
            wkb::reader::Dimension::Xym => Self::Xym,
            wkb::reader::Dimension::Xyzm => Self::Xyzm,
        }
    }
}

/// Serializable mirror of [`wkb::reader::GeometryType`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub enum GeometryType {
    /// A WKB point.
    Point,
    /// A WKB line string.
    LineString,
    /// A WKB polygon.
    Polygon,
    /// A WKB multi-point.
    MultiPoint,
    /// A WKB multi-line string.
    MultiLineString,
    /// A WKB multi-polygon.
    MultiPolygon,
    /// A WKB geometry collection.
    GeometryCollection,
}

impl TryFrom<wkb::reader::GeometryType> for GeometryType {
    type Error = WkbError;

    fn try_from(value: wkb::reader::GeometryType) -> Result<Self, Self::Error> {
        match value {
            wkb::reader::GeometryType::Point => Ok(Self::Point),
            wkb::reader::GeometryType::LineString => Ok(Self::LineString),
            wkb::reader::GeometryType::Polygon => Ok(Self::Polygon),
            wkb::reader::GeometryType::MultiPoint => Ok(Self::MultiPoint),
            wkb::reader::GeometryType::MultiLineString => Ok(Self::MultiLineString),
            wkb::reader::GeometryType::MultiPolygon => Ok(Self::MultiPolygon),
            wkb::reader::GeometryType::GeometryCollection => Ok(Self::GeometryCollection),
            _ => Err(WkbError::General(
                "Unsupported upstream wkb::reader::GeometryType variant".to_string(),
            )),
        }
    }
}

/// Owned shadow wrapper for the upstream opaque `wkb::reader::Wkb<'a>` parser view.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct Wkb {
    bytes: elicitation::WkbBytes,
    endianness: Endianness,
    dimension: Dimension,
    geometry_type: GeometryType,
}

impl Wkb {
    /// Parses WKB bytes into an owned validated wrapper.
    #[tracing::instrument]
    pub fn try_new(buf: &[u8]) -> WkbResult<Self> {
        let bytes = elicitation::WkbBytes::new(buf.to_vec()).map_err(WkbError::General)?;
        let parsed = wkb::reader::read_wkb(&bytes.bytes).map_err(WkbError::from)?;
        let endianness = wkb::Endianness::try_from(bytes.bytes[0])
            .map_err(|error| WkbError::General(error.to_string()))?
            .into();
        let dimension = parsed.dimension().into();
        let geometry_type = parsed.geometry_type().try_into()?;

        Ok(Self {
            bytes,
            endianness,
            dimension,
            geometry_type,
        })
    }

    /// Returns the parsed byte order.
    #[tracing::instrument(skip(self))]
    pub fn endianness(&self) -> Endianness {
        self.endianness
    }

    /// Returns the parsed coordinate dimension.
    #[tracing::instrument(skip(self))]
    pub fn dimension(&self) -> Dimension {
        self.dimension
    }

    /// Returns the parsed geometry type.
    #[tracing::instrument(skip(self))]
    pub fn geometry_type(&self) -> GeometryType {
        self.geometry_type
    }

    /// Returns the validated underlying bytes.
    #[tracing::instrument(skip(self))]
    pub fn bytes(&self) -> &elicitation::WkbBytes {
        &self.bytes
    }
}

/// Parse a WKB byte slice into an owned geometry wrapper.
#[tracing::instrument]
pub fn read_wkb(buf: &[u8]) -> WkbResult<Wkb> {
    Wkb::try_new(buf)
}
