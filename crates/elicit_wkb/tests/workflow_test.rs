//! Integration tests for `elicit_wkb` workflow plugins.

use elicit_wkb::{WkbParsed, WkbReaderPlugin, WkbWriterPlugin, WkbWritten};
use elicitation::{ElicitPlugin, VerifiedWorkflow};

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn reader_plugin_creates_successfully() {
    let plugin = WkbReaderPlugin::new();
    assert_eq!(plugin.name(), "wkb_reader");
}

#[test]
fn writer_plugin_creates_successfully() {
    let plugin = WkbWriterPlugin::new();
    assert_eq!(plugin.name(), "wkb_writer");
}

#[test]
fn reader_plugin_list_tools_expected_names() {
    let tools = WkbReaderPlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|tool| tool.name.as_ref()).collect();

    for name in &[
        "read_wkb",
        "wkb_try_new",
        "wkb_endianness",
        "wkb_dimension",
        "wkb_geometry_type",
    ] {
        assert!(names.contains(name), "missing tool: {name}");
    }
}

#[test]
fn writer_plugin_list_tools_expected_names() {
    let tools = WkbWriterPlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|tool| tool.name.as_ref()).collect();

    for name in &[
        "write_geometry",
        "write_point",
        "write_line_string",
        "write_polygon",
        "write_multi_point",
        "write_multi_line_string",
        "write_multi_polygon",
        "write_geometry_collection",
        "write_rect",
        "write_triangle",
        "write_line",
        "geometry_wkb_size",
        "point_wkb_size",
        "line_string_wkb_size",
        "polygon_wkb_size",
        "multi_point_wkb_size",
        "multi_line_string_wkb_size",
        "multi_polygon_wkb_size",
        "geometry_collection_wkb_size",
        "rect_wkb_size",
        "triangle_wkb_size",
        "line_wkb_size",
    ] {
        assert!(names.contains(name), "missing tool: {name}");
    }
}

#[test]
fn workflow_propositions_non_empty() {
    assert_verified::<WkbParsed>("WkbParsed");
    assert_verified::<WkbWritten>("WkbWritten");
}
