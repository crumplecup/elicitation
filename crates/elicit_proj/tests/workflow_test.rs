//! Integration tests for `elicit_proj` workflow plugins.

use elicit_proj::{ProjCreated, ProjTransformPlugin};
use elicitation::{ElicitPlugin, VerifiedWorkflow};

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn plugin_creates_successfully() {
    assert_eq!(ProjTransformPlugin::new().name(), "proj");
}

#[test]
fn plugin_lists_expected_tools() {
    let names: Vec<String> = ProjTransformPlugin::new()
        .list_tools()
        .iter()
        .map(|tool| tool.name.to_string())
        .collect();

    for name in &[
        "create_from_proj_string",
        "create_from_known_crs",
        "convert_coord",
        "project_coord",
        "convert_geometry",
        "transform_bounds",
        "definition",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn workflow_propositions_non_empty() {
    assert_verified::<ProjCreated>("ProjCreated");
}
