//! `RasterValue` — georaster pixel value wrapper.

/// Serializable shadow of [`georaster::geotiff::RasterValue`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub enum RasterValue {
    /// No data at the requested location.
    NoData,
    /// Unsigned 8-bit pixel.
    U8(u8),
    /// Unsigned 16-bit pixel.
    U16(u16),
    /// Unsigned 32-bit pixel.
    U32(u32),
    /// Unsigned 64-bit pixel.
    U64(u64),
    /// 32-bit floating pixel.
    F32(f32),
    /// 64-bit floating pixel.
    F64(f64),
    /// Signed 8-bit pixel.
    I8(i8),
    /// Signed 16-bit pixel.
    I16(i16),
    /// Signed 32-bit pixel.
    I32(i32),
    /// Signed 64-bit pixel.
    I64(i64),
    /// RGB 8-bit pixel.
    Rgb8(u8, u8, u8),
    /// RGBA 8-bit pixel.
    Rgba8(u8, u8, u8, u8),
    /// RGB 16-bit pixel.
    Rgb16(u16, u16, u16),
    /// RGBA 16-bit pixel.
    Rgba16(u16, u16, u16, u16),
}

impl From<georaster::geotiff::RasterValue> for RasterValue {
    fn from(value: georaster::geotiff::RasterValue) -> Self {
        match value {
            georaster::geotiff::RasterValue::NoData => Self::NoData,
            georaster::geotiff::RasterValue::U8(value) => Self::U8(value),
            georaster::geotiff::RasterValue::U16(value) => Self::U16(value),
            georaster::geotiff::RasterValue::U32(value) => Self::U32(value),
            georaster::geotiff::RasterValue::U64(value) => Self::U64(value),
            georaster::geotiff::RasterValue::F32(value) => Self::F32(value),
            georaster::geotiff::RasterValue::F64(value) => Self::F64(value),
            georaster::geotiff::RasterValue::I8(value) => Self::I8(value),
            georaster::geotiff::RasterValue::I16(value) => Self::I16(value),
            georaster::geotiff::RasterValue::I32(value) => Self::I32(value),
            georaster::geotiff::RasterValue::I64(value) => Self::I64(value),
            georaster::geotiff::RasterValue::Rgb8(r, g, b) => Self::Rgb8(r, g, b),
            georaster::geotiff::RasterValue::Rgba8(r, g, b, a) => Self::Rgba8(r, g, b, a),
            georaster::geotiff::RasterValue::Rgb16(r, g, b) => Self::Rgb16(r, g, b),
            georaster::geotiff::RasterValue::Rgba16(r, g, b, a) => Self::Rgba16(r, g, b, a),
            _ => panic!("unsupported future RasterValue variant"),
        }
    }
}

impl From<RasterValue> for georaster::geotiff::RasterValue {
    fn from(value: RasterValue) -> Self {
        match value {
            RasterValue::NoData => Self::NoData,
            RasterValue::U8(value) => Self::U8(value),
            RasterValue::U16(value) => Self::U16(value),
            RasterValue::U32(value) => Self::U32(value),
            RasterValue::U64(value) => Self::U64(value),
            RasterValue::F32(value) => Self::F32(value),
            RasterValue::F64(value) => Self::F64(value),
            RasterValue::I8(value) => Self::I8(value),
            RasterValue::I16(value) => Self::I16(value),
            RasterValue::I32(value) => Self::I32(value),
            RasterValue::I64(value) => Self::I64(value),
            RasterValue::Rgb8(r, g, b) => Self::Rgb8(r, g, b),
            RasterValue::Rgba8(r, g, b, a) => Self::Rgba8(r, g, b, a),
            RasterValue::Rgb16(r, g, b) => Self::Rgb16(r, g, b),
            RasterValue::Rgba16(r, g, b, a) => Self::Rgba16(r, g, b, a),
        }
    }
}
