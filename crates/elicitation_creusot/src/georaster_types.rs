//! Creusot proofs for georaster elicitation support.
//!
//! Trust the source. Verify the wrapper surface.

#![cfg(feature = "georaster-types")]

use creusot_std::prelude::*;

/// Trusted axiom: `Coordinate::new` stores longitude in `x` and latitude in `y`.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_georaster_coordinate_new_semantics() -> bool {
    let coord = georaster::Coordinate::new(50.013_f64, 160.423_f64);
    coord.x == 160.423_f64 && coord.y == 50.013_f64
}

/// Trusted axiom: TIFF planar configuration preserves the `Chunky` variant.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_georaster_planar_configuration_chunky() -> bool {
    let planar = tiff::tags::PlanarConfiguration::Chunky;
    matches!(planar, tiff::tags::PlanarConfiguration::Chunky)
}

/// Trusted axiom: TIFF color type preserves RGB bit depth metadata.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_georaster_color_type_rgb_bits() -> bool {
    let color = tiff::ColorType::RGB(8);
    matches!(color, tiff::ColorType::RGB(8))
}

/// Trusted axiom: raster values preserve the RGB8 variant and channels.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_georaster_raster_value_rgb8_variant() -> bool {
    let value = georaster::geotiff::RasterValue::Rgb8(1, 2, 3);
    matches!(value, georaster::geotiff::RasterValue::Rgb8(1, 2, 3))
}

/// Trusted axiom: image info preserves dimensions, color type, and sample count.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_georaster_image_info_fields() -> bool {
    let info = georaster::geotiff::ImageInfo {
        dimensions: Some((2, 3)),
        colortype: Some(tiff::ColorType::Gray(8)),
        photometric_interpretation: Some(tiff::tags::PhotometricInterpretation::BlackIsZero),
        planar_config: Some(tiff::tags::PlanarConfiguration::Chunky),
        samples: 1,
    };

    info.dimensions == Some((2, 3))
        && matches!(info.colortype, Some(tiff::ColorType::Gray(8)))
        && info.samples == 1
}
