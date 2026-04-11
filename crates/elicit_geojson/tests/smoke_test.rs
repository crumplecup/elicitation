use elicit_geo_types::{Geometry as GeoGeometry, GeometryCollection as GeoGeometryCollection};
use elicit_geojson::{Feature, FeatureCollection, GeoJson, Geometry, Id, Value};
use std::str::FromStr;

#[test]
fn document_and_value_round_trip() {
    let document = GeoJson::from_str(
        r#"{"type":"Feature","geometry":{"type":"Point","coordinates":[1.0,2.0]},"properties":{"name":"sample"}}"#,
    )
    .expect("valid GeoJSON");

    assert_eq!(document.to_json_value()["type"], "Feature");

    let geo_geometry = GeoGeometry::from(elicitation::GeoGeometry::from(
        geo_types::Geometry::from(geo_types::Point::new(1.0, 2.0)),
    ));
    let value = Value::from(&geo_geometry);
    let geometry = Geometry::new(value.clone());

    assert_eq!(value.type_name(), "Point");
    assert_eq!(geometry.value.type_name(), "Point");
    assert_eq!(Geometry::from(&geo_geometry).value.type_name(), "Point");
}

#[test]
fn feature_property_helpers_work() {
    let mut feature = Feature::from(Value::point(vec![1.0, 2.0]));
    feature.set_property("name", serde_json::json!("sample"));
    feature.set_property("count", serde_json::json!(3));

    assert!(feature.contains_property("name"));
    assert_eq!(feature.property("name"), Some(&serde_json::json!("sample")));
    assert_eq!(feature.len_properties(), 2);
    assert_eq!(feature.remove_property("count"), Some(serde_json::json!(3)));
    assert_eq!(feature.len_properties(), 1);
}

#[test]
fn geo_types_conversions_work() {
    let geo_geometry = GeoGeometry::from(elicitation::GeoGeometry::from(
        geo_types::Geometry::from(geo_types::Point::new(3.0, 4.0)),
    ));
    let geometry = Geometry::from(&geo_geometry);
    let recovered = GeoGeometry::try_from(geometry).expect("geometry conversion");
    assert_eq!(recovered.geometry_type(), "Point");

    let collection = GeoGeometryCollection::from(elicitation::GeoGeometryCollection(vec![
        elicitation::GeoGeometry::from(geo_types::Geometry::from(geo_types::Point::new(3.0, 4.0))),
    ]));
    let feature_collection = FeatureCollection::from(&collection);
    let recovered_collection =
        GeoGeometry::try_from(feature_collection).expect("collection conversion");
    assert_eq!(recovered_collection.geometry_type(), "GeometryCollection");

    let id = Id::string("feature-1");
    assert_eq!(
        serde_json::to_value(id).expect("id json"),
        serde_json::json!("feature-1")
    );
}
