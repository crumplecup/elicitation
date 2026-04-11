#![cfg(feature = "geo-types")]

use elicit_db::DbValue;
use elicit_geo_types::Geometry as GeoTypesGeometry;
use elicit_geojson::{GeoJson, Geometry};
use elicitation::{GeoCoord, GeoGeometry, GeoPoint};

fn sample_point() -> GeoGeometry {
    GeoGeometry::Point(GeoPoint {
        coord: GeoCoord { x: 1.0, y: 2.0 },
    })
}

#[test]
fn json_value_roundtrips_geojson_document() {
    let geometry = GeoTypesGeometry::from(sample_point());
    let document = GeoJson::from(Geometry::from(&geometry));

    let value = DbValue::json_from_geojson(&document);
    let restored = value
        .try_to_geojson()
        .expect("GeoJSON JSON payload should roundtrip");

    assert_eq!(
        serde_json::to_value(document).expect("serialize GeoJSON document"),
        serde_json::to_value(restored).expect("serialize restored GeoJSON document")
    );
}

#[test]
fn json_geojson_geometry_roundtrips_geo_geometry() {
    let geom = sample_point();
    let value = DbValue::json_from_geo_as_geojson(&geom);

    let restored = value
        .try_json_to_geo_geometry()
        .expect("GeoJSON JSON payload should recover GeoGeometry");

    assert_eq!(restored, geom);
}

#[test]
fn non_geojson_json_is_rejected() {
    let value = DbValue::Json(serde_json::json!({"hello": "world"}));

    let error = value
        .try_to_geojson()
        .expect_err("non-GeoJSON JSON should be rejected");

    assert!(!error.is_empty());
}
