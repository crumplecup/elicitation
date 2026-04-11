use verus_builtin_macros::verus;

verus! {

pub struct ShadowGeoRasterCoordinate {
    pub x: u32,
    pub y: u32,
}

pub fn make_georaster_coordinate(x: u32, y: u32) -> (result: ShadowGeoRasterCoordinate)
    ensures result.x == x, result.y == y,
{
    ShadowGeoRasterCoordinate { x, y }
}

pub fn verify_georaster_coordinate_new_semantics() -> (result: ShadowGeoRasterCoordinate)
    ensures result.x == 160, result.y == 50,
{
    make_georaster_coordinate(160u32, 50u32)
}

pub struct ShadowGeoRasterPlanarConfiguration {
    pub is_chunky: bool,
}

pub fn make_chunky_planar_configuration() -> (result: ShadowGeoRasterPlanarConfiguration)
    ensures result.is_chunky,
{
    ShadowGeoRasterPlanarConfiguration { is_chunky: true }
}

pub fn verify_georaster_planar_configuration_chunky() -> (result: ShadowGeoRasterPlanarConfiguration)
    ensures result.is_chunky,
{
    make_chunky_planar_configuration()
}

pub struct ShadowGeoRasterColorType {
    pub bits: u32,
    pub is_rgb: bool,
}

pub fn make_rgb_color_type(bits: u32) -> (result: ShadowGeoRasterColorType)
    ensures result.bits == bits, result.is_rgb,
{
    ShadowGeoRasterColorType { bits, is_rgb: true }
}

pub fn verify_georaster_color_type_rgb_bits() -> (result: ShadowGeoRasterColorType)
    ensures result.bits == 8, result.is_rgb,
{
    make_rgb_color_type(8u32)
}

pub struct ShadowGeoRasterValue {
    pub is_rgb8: bool,
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

pub fn make_rgb8_value(r: u32, g: u32, b: u32) -> (result: ShadowGeoRasterValue)
    ensures result.is_rgb8, result.r == r, result.g == g, result.b == b,
{
    ShadowGeoRasterValue { is_rgb8: true, r, g, b }
}

pub fn verify_georaster_raster_value_rgb8_variant() -> (result: ShadowGeoRasterValue)
    ensures result.is_rgb8, result.r == 1, result.g == 2, result.b == 3,
{
    make_rgb8_value(1u32, 2u32, 3u32)
}

pub struct ShadowGeoRasterImageInfo {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
}

pub fn make_image_info(width: u32, height: u32, samples: u32) -> (result: ShadowGeoRasterImageInfo)
    ensures result.width == width, result.height == height, result.samples == samples,
{
    ShadowGeoRasterImageInfo { width, height, samples }
}

pub fn verify_georaster_image_info_fields() -> (result: ShadowGeoRasterImageInfo)
    ensures result.width == 2, result.height == 3, result.samples == 1,
{
    make_image_info(2u32, 3u32, 1u32)
}

}
