//! Integration tests for `elicit_wkt` workflow plugins.

use elicit_wkt::{WktParsePlugin, WktParsed, WktTypeCreated, WktTypesPlugin};
use elicitation::{ElicitPlugin, VerifiedWorkflow};

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn types_plugin_creates_successfully() {
    let plugin = WktTypesPlugin::new();
    assert_eq!(plugin.name(), "wkt_types");
}

#[test]
fn parse_plugin_creates_successfully() {
    let plugin = WktParsePlugin::new();
    assert_eq!(plugin.name(), "wkt_parse");
}

#[test]
fn types_plugin_list_tools_expected_names() {
    let tools = WktTypesPlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|tool| tool.name.as_ref()).collect();

    for name in &[
        "coord_new",
        "coord_new_3d",
        "coord_new_with_m",
        "point_new",
        "point_empty",
        "linestring_new",
        "polygon_new",
        "multipoint_new",
        "multilinestring_new",
        "multipolygon_new",
        "geometry_collection_new",
    ] {
        assert!(names.contains(name), "missing tool: {name}");
    }
}

#[test]
fn parse_plugin_list_tools_expected_names() {
    let tools = WktParsePlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|tool| tool.name.as_ref()).collect();

    for name in &[
        "wkt_item_from_str",
        "wkt_item_wkt_string",
        "wkt_item_geometry_type",
    ] {
        assert!(names.contains(name), "missing tool: {name}");
    }
}

#[test]
fn workflow_propositions_non_empty() {
    assert_verified::<WktTypeCreated>("WktTypeCreated");
    assert_verified::<WktParsed>("WktParsed");
}
