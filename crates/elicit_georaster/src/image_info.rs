//! `ImageInfo` — georaster image metadata wrapper.

use crate::{ColorType, PhotometricInterpretation, PlanarConfiguration};

/// Serializable shadow of [`georaster::geotiff::ImageInfo`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct ImageInfo {
    /// Optional image dimensions.
    pub dimensions: Option<(u32, u32)>,
    /// Optional TIFF color type.
    pub colortype: Option<ColorType>,
    /// Optional TIFF photometric interpretation.
    pub photometric_interpretation: Option<PhotometricInterpretation>,
    /// Optional TIFF planar configuration.
    pub planar_config: Option<PlanarConfiguration>,
    /// Samples per pixel.
    pub samples: u8,
}

impl From<&georaster::geotiff::ImageInfo> for ImageInfo {
    fn from(value: &georaster::geotiff::ImageInfo) -> Self {
        Self {
            dimensions: value.dimensions,
            colortype: value.colortype.map(ColorType::from),
            photometric_interpretation: value
                .photometric_interpretation
                .map(PhotometricInterpretation::from),
            planar_config: value.planar_config.map(PlanarConfiguration::from),
            samples: value.samples,
        }
    }
}
