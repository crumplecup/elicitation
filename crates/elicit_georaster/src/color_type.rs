//! `ColorType` — TIFF color type wrapper.

/// Serializable shadow of [`tiff::ColorType`].
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
pub enum ColorType {
    /// Grayscale pixels with a bit depth.
    Gray(u8),
    /// RGB pixels with a bit depth.
    RGB(u8),
    /// Palette-indexed pixels with a bit depth.
    Palette(u8),
    /// Grayscale plus alpha with a bit depth.
    GrayA(u8),
    /// RGBA pixels with a bit depth.
    RGBA(u8),
    /// CMYK pixels with a bit depth.
    CMYK(u8),
    /// YCbCr pixels with a bit depth.
    YCbCr(u8),
}

impl From<tiff::ColorType> for ColorType {
    fn from(value: tiff::ColorType) -> Self {
        match value {
            tiff::ColorType::Gray(bits) => Self::Gray(bits),
            tiff::ColorType::RGB(bits) => Self::RGB(bits),
            tiff::ColorType::Palette(bits) => Self::Palette(bits),
            tiff::ColorType::GrayA(bits) => Self::GrayA(bits),
            tiff::ColorType::RGBA(bits) => Self::RGBA(bits),
            tiff::ColorType::CMYK(bits) => Self::CMYK(bits),
            tiff::ColorType::YCbCr(bits) => Self::YCbCr(bits),
        }
    }
}

impl From<ColorType> for tiff::ColorType {
    fn from(value: ColorType) -> Self {
        match value {
            ColorType::Gray(bits) => Self::Gray(bits),
            ColorType::RGB(bits) => Self::RGB(bits),
            ColorType::Palette(bits) => Self::Palette(bits),
            ColorType::GrayA(bits) => Self::GrayA(bits),
            ColorType::RGBA(bits) => Self::RGBA(bits),
            ColorType::CMYK(bits) => Self::CMYK(bits),
            ColorType::YCbCr(bits) => Self::YCbCr(bits),
        }
    }
}
