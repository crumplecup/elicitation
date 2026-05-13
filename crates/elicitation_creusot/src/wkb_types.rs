//! Creusot proofs for WKB elicitation wrappers.
//!
//! Trust the source. Verify the wrapper.

#![cfg(feature = "wkb-types")]

use creusot_std::prelude::*;

const POINT_HEX: &str = "0101000000000000000000f03f0000000000000040";

/// Trusted axiom: WkbEndianness roundtrip preserves the selected variant.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wkb_endianness_roundtrip() -> bool {
    let wrapper = elicitation::WkbEndianness::from(wkb::Endianness::LittleEndian);
    let restored: wkb::Endianness = wrapper.into();
    restored == wkb::Endianness::LittleEndian
}

/// Trusted axiom: WkbDimension roundtrip preserves the selected variant.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wkb_dimension_roundtrip() -> bool {
    let wrapper = elicitation::WkbDimension::from(wkb::reader::Dimension::Xyzm);
    let restored: wkb::reader::Dimension = wrapper.into();
    restored == wkb::reader::Dimension::Xyzm
}

/// Trusted axiom: WkbGeometryType roundtrip preserves supported variants.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wkb_geometry_type_roundtrip() -> bool {
    let wrapper = elicitation::WkbGeometryType::try_from(wkb::reader::GeometryType::MultiPolygon)
        .expect("supported geometry type");
    let restored: wkb::reader::GeometryType = wrapper.into();
    restored == wkb::reader::GeometryType::MultiPolygon
}

/// Trusted axiom: WkbWriteOptions preserves its endianness field through conversion.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wkb_write_options_roundtrip() -> bool {
    let options = wkb::writer::WriteOptions {
        endianness: wkb::Endianness::BigEndian,
    };
    let wrapper = elicitation::WkbWriteOptions::from(options);
    let restored: wkb::writer::WriteOptions = wrapper.into();
    restored.endianness == wkb::Endianness::BigEndian
}

/// Trusted axiom: known point bytes expose consistent metadata.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wkb_bytes_known_point_metadata() -> bool {
    let bytes = elicitation::WkbBytes::from_hex(POINT_HEX).expect("known point WKB");
    bytes.hex_string() == POINT_HEX
        && bytes.endianness() == elicitation::WkbEndianness::LittleEndian
        && bytes.dimension() == elicitation::WkbDimension::Xy
        && bytes.geometry_type().expect("known point geometry")
            == elicitation::WkbGeometryType::Point
}
