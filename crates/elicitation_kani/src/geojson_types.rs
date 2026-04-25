//! Kani proofs for GeoJSON elicitation support.
//!
//! Available with the `geojson-types` feature.

/// GeoJSON value preserves the `Point` variant name.
#[cfg(feature = "geojson-types")]
#[kani::proof]
fn verify_geojson_value_point_type_name() {
    let value = geojson::Value::Point { coordinates: geojson::Position::from(vec![1.0_f64, 2.0_f64]) };
    assert!(value.type_name() == "Point", "Point type name preserved");
}

/// Geometry construction preserves the inner point value.
#[cfg(feature = "geojson-types")]
#[kani::proof]
fn verify_geojson_geometry_new_point() {
    let geometry = geojson::Geometry::new(geojson::Value::Point { coordinates: geojson::Position::from(vec![3.0_f64, 4.0_f64]) });
    assert!(
        matches!(geometry.value, geojson::Value::Point { .. }),
        "Geometry::new preserves point variant"
    );
}

/// Feature property helpers expose inserted properties consistently.
#[cfg(feature = "geojson-types")]
#[kani::proof]
fn verify_geojson_feature_property_access() {
    let mut feature = geojson::Feature::from(geojson::Value::Point { coordinates: geojson::Position::from(vec![5.0_f64, 6.0_f64]) });
    feature.set_property("name", serde_json::json!("sample"));

    assert!(feature.contains_property("name"), "property key is present");
    assert!(
        feature.property("name").is_some(),
        "property lookup succeeds"
    );
    assert!(feature.len_properties() == 1, "property count matches");
}

/// Feature collection preserves the number of collected features.
#[cfg(feature = "geojson-types")]
#[kani::proof]
fn verify_geojson_feature_collection_len() {
    let first = geojson::Feature::from(geojson::Value::Point { coordinates: geojson::Position::from(vec![0.0_f64, 0.0_f64]) });
    let second = geojson::Feature::from(geojson::Value::Point { coordinates: geojson::Position::from(vec![1.0_f64, 1.0_f64]) });
    let collection: geojson::FeatureCollection = vec![first, second].into_iter().collect();

    assert!(collection.features.len() == 2, "feature count preserved");
}

/// String feature identifiers preserve their variant.
#[cfg(feature = "geojson-types")]
#[kani::proof]
fn verify_geojson_id_string_variant() {
    let id = geojson::feature::Id::String("feature-1".to_string());
    assert!(
        matches!(id, geojson::feature::Id::String(_)),
        "string identifier variant preserved"
    );
}
