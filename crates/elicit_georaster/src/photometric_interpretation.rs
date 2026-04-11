//! `PhotometricInterpretation` — TIFF photometric interpretation wrapper.

/// Serializable shadow of [`tiff::tags::PhotometricInterpretation`].
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
pub enum PhotometricInterpretation {
    /// Zero means white for grayscale data.
    WhiteIsZero,
    /// Zero means black for grayscale data.
    BlackIsZero,
    /// RGB color channels.
    RGB,
    /// Palette-indexed color.
    RGBPalette,
    /// Transparency mask.
    TransparencyMask,
    /// CMYK color channels.
    CMYK,
    /// YCbCr color channels.
    YCbCr,
    /// CIELab color channels.
    CIELab,
}

impl From<tiff::tags::PhotometricInterpretation> for PhotometricInterpretation {
    fn from(value: tiff::tags::PhotometricInterpretation) -> Self {
        match value {
            tiff::tags::PhotometricInterpretation::WhiteIsZero => Self::WhiteIsZero,
            tiff::tags::PhotometricInterpretation::BlackIsZero => Self::BlackIsZero,
            tiff::tags::PhotometricInterpretation::RGB => Self::RGB,
            tiff::tags::PhotometricInterpretation::RGBPalette => Self::RGBPalette,
            tiff::tags::PhotometricInterpretation::TransparencyMask => Self::TransparencyMask,
            tiff::tags::PhotometricInterpretation::CMYK => Self::CMYK,
            tiff::tags::PhotometricInterpretation::YCbCr => Self::YCbCr,
            tiff::tags::PhotometricInterpretation::CIELab => Self::CIELab,
            _ => panic!("unsupported future PhotometricInterpretation variant"),
        }
    }
}

impl From<PhotometricInterpretation> for tiff::tags::PhotometricInterpretation {
    fn from(value: PhotometricInterpretation) -> Self {
        match value {
            PhotometricInterpretation::WhiteIsZero => Self::WhiteIsZero,
            PhotometricInterpretation::BlackIsZero => Self::BlackIsZero,
            PhotometricInterpretation::RGB => Self::RGB,
            PhotometricInterpretation::RGBPalette => Self::RGBPalette,
            PhotometricInterpretation::TransparencyMask => Self::TransparencyMask,
            PhotometricInterpretation::CMYK => Self::CMYK,
            PhotometricInterpretation::YCbCr => Self::YCbCr,
            PhotometricInterpretation::CIELab => Self::CIELab,
        }
    }
}
