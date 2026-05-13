//! Smoke tests for `elicit_wkb`.

use elicit_wkb::{Dimension, Endianness, GeometryType, WriteOptions, read_wkb, write_point};
use elicitation::{GeoCoord, GeoPoint};

#[test]
fn write_and_read_point_round_trip() {
    let point = GeoPoint {
        coord: GeoCoord { x: 1.0, y: 2.0 },
    };
    let options = WriteOptions {
        endianness: Endianness::LittleEndian,
    };

    let bytes = write_point(&point, &options).expect("write point");
    let parsed = read_wkb(&bytes.bytes).expect("parse point");

    assert_eq!(parsed.endianness(), Endianness::LittleEndian);
    assert_eq!(parsed.dimension(), Dimension::Xy);
    assert_eq!(parsed.geometry_type(), GeometryType::Point);
    assert_eq!(
        bytes.hex_string(),
        "0101000000000000000000f03f0000000000000040"
    );
}
