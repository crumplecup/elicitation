//! Integration tests for elicit_geo plugins.

use elicit_geo::{
    GeoBooleanOpsPlugin, GeoCalculationsPlugin, GeoGeodesicPlugin, GeoMeasurementsPlugin,
    GeoPredicatesPlugin, GeoTransformationsPlugin, GeoValidationPlugin, GeoWorkflowPlugin,
    TransformationApplied,
};
use elicitation::{ElicitPlugin, VerifiedWorkflow};

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn plugins_exist() {
    let _ = GeoPredicatesPlugin;
    let _ = GeoMeasurementsPlugin;
    let _ = GeoGeodesicPlugin;
    let _ = GeoCalculationsPlugin;
    let _ = GeoTransformationsPlugin;
    let _ = GeoValidationPlugin;
    let _ = GeoBooleanOpsPlugin;
    let _ = GeoWorkflowPlugin;
}

#[test]
fn transformations_plugin_name() {
    assert_eq!(GeoTransformationsPlugin.name(), "geo_transformations");
}

#[test]
fn transformations_tool_names() {
    let tools: Vec<_> = GeoTransformationsPlugin
        .list_tools()
        .iter()
        .map(|t| t.name.as_ref().to_owned())
        .collect();
    for expected in &[
        "skew_linestring",
        "skew_polygon",
        "smooth_linestring",
        "smooth_polygon",
        "densify_linestring",
        "densify_polygon",
        "map_coords_point",
        "map_coords_linestring",
        "map_coords_polygon",
    ] {
        assert!(
            tools.iter().any(|t| t == expected),
            "missing tool: {expected}"
        );
    }
}

#[test]
fn transformations_propositions_non_empty() {
    assert_verified::<TransformationApplied>("TransformationApplied");
}
