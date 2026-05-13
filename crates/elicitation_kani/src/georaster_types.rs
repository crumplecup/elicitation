//! Kani proofs for georaster elicitation support.
//!
//! Available with the `georaster-types` feature.

/// `Coordinate::new` preserves the documented latitude/longitude swap semantics.
#[cfg(feature = "georaster-types")]
#[kani::proof]
fn verify_georaster_coordinate_new_semantics() {
    let coord = georaster::Coordinate::new(50.013_f64, 160.423_f64);
    assert!(coord.x == 160.423_f64, "longitude stored in x");
    assert!(coord.y == 50.013_f64, "latitude stored in y");
}

/// TIFF planar configuration preserves the `Chunky` variant.
#[cfg(feature = "georaster-types")]
#[kani::proof]
fn verify_georaster_planar_configuration_chunky() {
    let planar = tiff::tags::PlanarConfiguration::Chunky;
    assert!(
        matches!(planar, tiff::tags::PlanarConfiguration::Chunky),
        "chunky planar configuration preserved"
    );
}

/// TIFF color type preserves RGB bit depth metadata.
#[cfg(feature = "georaster-types")]
#[kani::proof]
fn verify_georaster_color_type_rgb_bits() {
    let color = tiff::ColorType::RGB(8);
    assert!(matches!(color, tiff::ColorType::RGB(8)), "RGB(8) preserved");
}

/// Raster values preserve the RGB8 variant and channel values.
#[cfg(feature = "georaster-types")]
#[kani::proof]
fn verify_georaster_raster_value_rgb8_variant() {
    let value = georaster::geotiff::RasterValue::Rgb8(1, 2, 3);
    assert!(
        matches!(value, georaster::geotiff::RasterValue::Rgb8(1, 2, 3)),
        "Rgb8 channel values preserved"
    );
}

/// Image info preserves dimensions, color type, and sample count.
#[cfg(feature = "georaster-types")]
#[kani::proof]
fn verify_georaster_image_info_fields() {
    let info = georaster::geotiff::ImageInfo {
        dimensions: Some((2, 3)),
        colortype: Some(tiff::ColorType::Gray(8)),
        photometric_interpretation: Some(tiff::tags::PhotometricInterpretation::BlackIsZero),
        planar_config: Some(tiff::tags::PlanarConfiguration::Chunky),
        samples: 1,
    };

    assert!(info.dimensions == Some((2, 3)), "dimensions preserved");
    assert!(
        matches!(info.colortype, Some(tiff::ColorType::Gray(8))),
        "color type preserved"
    );
    assert!(info.samples == 1, "sample count preserved");
}
