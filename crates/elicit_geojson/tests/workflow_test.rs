//! Integration tests for `elicit_geojson` workflow plugins.

use elicit_geojson::{
    GeoJsonConversionPlugin, GeoJsonConverted, GeoJsonDocumentParsed, GeoJsonDocumentPlugin,
    GeoJsonFeatureCreated, GeoJsonFeaturePlugin, GeoJsonGeometryCreated, GeoJsonGeometryPlugin,
};
use elicitation::{ElicitPlugin, VerifiedWorkflow};

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn plugins_create_successfully() {
    assert_eq!(GeoJsonDocumentPlugin::new().name(), "geojson_document");
    assert_eq!(GeoJsonGeometryPlugin::new().name(), "geojson_geometry");
    assert_eq!(GeoJsonFeaturePlugin::new().name(), "geojson_feature");
    assert_eq!(GeoJsonConversionPlugin::new().name(), "geojson_conversion");
}

#[test]
fn document_plugin_lists_expected_tools() {
    let names: Vec<String> = GeoJsonDocumentPlugin::new()
        .list_tools()
        .iter()
        .map(|tool| tool.name.to_string())
        .collect();

    for name in &[
        "geojson_from_str",
        "geojson_from_json_value",
        "geojson_to_json_value",
        "geojson_to_string_pretty",
        "geojson_variant_name",
    ] {
        assert!(
            names.iter().any(|candidate| candidate == name),
            "missing tool: {name}"
        );
    }
}

#[test]
fn geometry_plugin_lists_expected_tools() {
    let names: Vec<String> = GeoJsonGeometryPlugin::new()
        .list_tools()
        .iter()
        .map(|tool| tool.name.to_string())
        .collect();

    for name in &[
        "geometry_new",
        "value_point",
        "value_multi_point",
        "value_line_string",
        "value_multi_line_string",
        "value_polygon",
        "value_multi_polygon",
        "value_geometry_collection",
        "value_type_name",
        "geometry_from_json_value",
        "value_from_json_value",
    ] {
        assert!(
            names.iter().any(|candidate| candidate == name),
            "missing tool: {name}"
        );
    }
}

#[test]
fn feature_plugin_lists_expected_tools() {
    let names: Vec<String> = GeoJsonFeaturePlugin::new()
        .list_tools()
        .iter()
        .map(|tool| tool.name.to_string())
        .collect();

    for name in &[
        "feature_from_geometry",
        "feature_from_value",
        "feature_from_json_value",
        "feature_collection_from_json_value",
        "feature_collection_from_features",
        "id_string",
        "id_number",
        "feature_property",
        "feature_contains_property",
        "feature_set_property",
        "feature_remove_property",
        "feature_len_properties",
    ] {
        assert!(
            names.iter().any(|candidate| candidate == name),
            "missing tool: {name}"
        );
    }
}

#[test]
fn conversion_plugin_lists_expected_tools() {
    let names: Vec<String> = GeoJsonConversionPlugin::new()
        .list_tools()
        .iter()
        .map(|tool| tool.name.to_string())
        .collect();

    for name in &[
        "geometry_from_geo_geometry",
        "value_from_geo_geometry",
        "feature_collection_from_geo_geometry_collection",
        "geo_geometry_from_geojson",
        "geo_geometry_from_geometry",
        "geo_geometry_from_value",
        "geo_geometry_from_feature",
        "geo_geometry_from_feature_collection",
    ] {
        assert!(
            names.iter().any(|candidate| candidate == name),
            "missing tool: {name}"
        );
    }
}

#[test]
fn workflow_propositions_non_empty() {
    assert_verified::<GeoJsonDocumentParsed>("GeoJsonDocumentParsed");
    assert_verified::<GeoJsonGeometryCreated>("GeoJsonGeometryCreated");
    assert_verified::<GeoJsonFeatureCreated>("GeoJsonFeatureCreated");
    assert_verified::<GeoJsonConverted>("GeoJsonConverted");
}
