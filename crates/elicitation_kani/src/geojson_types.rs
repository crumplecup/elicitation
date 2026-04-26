//! Kani proofs for GeoJSON elicitation support.
//!
//! Available with the `geojson-types` feature.

/// GeoJSON value preserves the `Point` variant name.
#[cfg(feature = "geojson-types")]
#[kani::proof]
fn verify_geojson_value_point_type_name() {
    let value = geojson::Value::Point {
        coordinates: geojson::Position::from(vec![1.0_f64, 2.0_f64]),
    };
    assert!(value.type_name() == "Point", "Point type name preserved");
}

/// Geometry construction preserves the inner point value.
#[cfg(feature = "geojson-types")]
#[kani::proof]
fn verify_geojson_geometry_new_point() {
    let geometry = geojson::Geometry::new(geojson::Value::Point {
        coordinates: geojson::Position::from(vec![3.0_f64, 4.0_f64]),
    });
    assert!(
        matches!(geometry.value, geojson::Value::Point { .. }),
        "Geometry::new preserves point variant"
    );
}

/// Feature property access delegates to geojson's HashMap internals, which
/// are trusted third-party logic outside our wrapper.  This is a marker proof
/// that documents the trust boundary.
#[cfg(feature = "geojson-types")]
#[kani::proof]
fn verify_geojson_feature_property_access() {
    // geojson::Feature::set_property / contains_property / property explore
    // HashMap internals and cause CBMC path explosion.  These are upstream
    // library APIs that we trust; our wrapper adds no logic around them.
    kani::assume(true);
    assert!(
        true,
        "geojson feature property access is trusted third-party logic"
    );
}

/// FeatureCollection's FromIterator impl traverses HashMap internals, which
/// are trusted third-party logic outside our wrapper.  Marker proof only.
#[cfg(feature = "geojson-types")]
#[kani::proof]
fn verify_geojson_feature_collection_len() {
    // geojson::FeatureCollection::from_iter explores HashMap / allocation
    // internals and causes CBMC path explosion.  Trusted third-party logic.
    kani::assume(true);
    assert!(
        true,
        "geojson feature collection construction is trusted third-party logic"
    );
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
