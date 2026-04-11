#![cfg(feature = "geo-types")]

use elicit_db::{DbSpatialValue, DbValue};
use elicitation::{GeoCoord, GeoGeometry, GeoPoint, WkbBytes};

fn sample_point() -> GeoGeometry {
    GeoGeometry::Point(GeoPoint {
        coord: GeoCoord { x: 1.0, y: 2.0 },
    })
}

#[test]
fn spatial_wkt_roundtrips_geo_geometry() {
    let geom = sample_point();
    let spatial = DbSpatialValue::from_geo_as_wkt(&geom);

    let restored = spatial
        .try_to_geo_geometry()
        .expect("WKT spatial value should roundtrip");

    assert_eq!(restored, geom);
}

#[test]
fn geometry_value_wkt_roundtrips_geo_geometry() {
    let geom = sample_point();
    let value = DbValue::geometry_from_geo_as_wkt(&geom);

    let restored = value
        .try_to_geo_geometry()
        .expect("WKT geometry value should roundtrip");

    assert_eq!(restored, geom);
}

#[test]
fn geometry_value_wkb_encodes_known_point() {
    let geom = sample_point();
    let value = DbValue::geometry_from_geo_as_wkb(&geom).expect("WKB encoding should succeed");

    match value {
        DbValue::Geometry(DbSpatialValue::Wkb(bytes)) => {
            let wrapped = WkbBytes::new(bytes).expect("WKB bytes should validate");
            assert_eq!(
                wrapped.hex_string(),
                "0101000000000000000000f03f0000000000000040"
            );
        }
        other => panic!("expected geometry WKB payload, got {other:?}"),
    }
}

#[test]
fn spatial_wkb_to_geo_geometry_is_explicitly_unsupported() {
    let geom = sample_point();
    let spatial = DbSpatialValue::from_geo_as_wkb(&geom).expect("WKB encoding should succeed");

    let error = spatial
        .try_to_geo_geometry()
        .expect_err("WKB conversion should remain transport-only");

    assert!(error.contains("transport-only"));
}
