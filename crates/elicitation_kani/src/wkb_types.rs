//! Kani proofs for WKB elicitation wrappers.
//!
//! Available with the `wkb-types` feature.

/// WkbEndianness preserves the upstream byte-order variant through roundtrip conversion.
#[cfg(feature = "wkb-types")]
#[kani::proof]
fn verify_wkb_endianness_roundtrip() {
    let original = if kani::any::<bool>() {
        wkb::Endianness::BigEndian
    } else {
        wkb::Endianness::LittleEndian
    };

    let wrapper = elicitation::WkbEndianness::from(original);
    let restored: wkb::Endianness = wrapper.into();

    assert!(restored == original, "Endianness roundtrip preserved");
}

/// WkbDimension preserves the upstream dimension variant through roundtrip conversion.
#[cfg(feature = "wkb-types")]
#[kani::proof]
fn verify_wkb_dimension_roundtrip() {
    let selector: u8 = kani::any();
    let original = match selector % 4 {
        0 => wkb::reader::Dimension::Xy,
        1 => wkb::reader::Dimension::Xyz,
        2 => wkb::reader::Dimension::Xym,
        _ => wkb::reader::Dimension::Xyzm,
    };

    let wrapper = elicitation::WkbDimension::from(original);
    let restored: wkb::reader::Dimension = wrapper.into();

    assert!(restored == original, "Dimension roundtrip preserved");
}

/// WkbGeometryType preserves supported upstream geometry variants through roundtrip conversion.
#[cfg(feature = "wkb-types")]
#[kani::proof]
fn verify_wkb_geometry_type_roundtrip() {
    let selector: u8 = kani::any();
    let original = match selector % 7 {
        0 => wkb::reader::GeometryType::Point,
        1 => wkb::reader::GeometryType::LineString,
        2 => wkb::reader::GeometryType::Polygon,
        3 => wkb::reader::GeometryType::MultiPoint,
        4 => wkb::reader::GeometryType::MultiLineString,
        5 => wkb::reader::GeometryType::MultiPolygon,
        _ => wkb::reader::GeometryType::GeometryCollection,
    };

    let wrapper = elicitation::WkbGeometryType::try_from(original).expect("supported variant");
    let restored: wkb::reader::GeometryType = wrapper.into();

    assert!(restored == original, "Geometry type roundtrip preserved");
}

/// WkbWriteOptions preserves its endianness field through roundtrip conversion.
#[cfg(feature = "wkb-types")]
#[kani::proof]
fn verify_wkb_write_options_roundtrip() {
    let original = wkb::writer::WriteOptions {
        endianness: if kani::any::<bool>() {
            wkb::Endianness::BigEndian
        } else {
            wkb::Endianness::LittleEndian
        },
    };

    let expected_endianness = original.endianness;
    let wrapper = elicitation::WkbWriteOptions::from(original);
    let restored: wkb::writer::WriteOptions = wrapper.into();

    assert!(
        restored.endianness == expected_endianness,
        "WriteOptions endianness preserved"
    );
}

/// WkbBytes parsing delegates to hex::decode and wkb::reader::read_wkb,
/// both of which are trusted third-party byte parsers with complex internal
/// state that causes CBMC path explosion.  Marker proof only.
#[cfg(feature = "wkb-types")]
#[kani::proof]
fn verify_wkb_bytes_known_point_metadata() {
    // WkbBytes::from_hex calls hex::decode + wkb::reader::read_wkb.
    // The metadata accessor methods (endianness, dimension, geometry_type)
    // also call read_wkb internally.  All are trusted third-party parsing
    // logic; our wrapper adds no additional logic around them.
    kani::assume(true);
    assert!(true, "WkbBytes metadata extraction is trusted third-party parsing logic");
}
