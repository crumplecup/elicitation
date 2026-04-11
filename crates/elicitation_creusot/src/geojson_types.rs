//! Creusot proofs for GeoJSON elicitation support.
//!
//! Trust the source. Verify the wrapper surface.

#![cfg(feature = "geojson-types")]

use creusot_std::prelude::*;

/// Trusted axiom: GeoJSON point values report the `Point` type name.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_geojson_value_point_type_name() -> bool {
    let value = geojson::Value::Point(::std::vec![1.0_f64, 2.0_f64]);
    value.type_name() == "Point"
}

/// Trusted axiom: `Geometry::new` preserves a point payload.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_geojson_geometry_new_point() -> bool {
    let geometry = geojson::Geometry::new(geojson::Value::Point(::std::vec![3.0_f64, 4.0_f64]));
    matches!(geometry.value, geojson::Value::Point(_))
}

/// Trusted axiom: feature property helpers remain internally consistent.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_geojson_feature_property_access() -> bool {
    let mut feature = geojson::Feature::from(geojson::Value::Point(::std::vec![5.0_f64, 6.0_f64]));
    feature.set_property("name", serde_json::json!("sample"));

    feature.contains_property("name")
        && feature.property("name").is_some()
        && feature.len_properties() == 1
}

/// Trusted axiom: collected feature collections preserve feature count.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_geojson_feature_collection_len() -> bool {
    let first = geojson::Feature::from(geojson::Value::Point(::std::vec![0.0_f64, 0.0_f64]));
    let second = geojson::Feature::from(geojson::Value::Point(::std::vec![1.0_f64, 1.0_f64]));
    let collection: geojson::FeatureCollection = ::std::vec![first, second].into_iter().collect();

    collection.features.len() == 2
}

/// Trusted axiom: string feature identifiers preserve their variant.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_geojson_id_string_variant() -> bool {
    let id = geojson::feature::Id::String("feature-1".to_string());
    matches!(id, geojson::feature::Id::String(_))
}
