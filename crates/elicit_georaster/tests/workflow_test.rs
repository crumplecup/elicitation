//! Integration tests for `elicit_georaster` workflow plugins.

use elicit_georaster::{
    GeoTiffReaderConfigured, GeoTiffReaderOpened, GeoTiffReaderPlugin, GeoTiffSampled,
    GeoTiffSamplingPlugin,
};
use elicitation::{ElicitPlugin, VerifiedWorkflow};

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn plugins_create_successfully() {
    assert_eq!(GeoTiffReaderPlugin::new().name(), "geotiff_reader");
    assert_eq!(GeoTiffSamplingPlugin::new().name(), "geotiff_sampling");
}

#[test]
fn reader_plugin_lists_expected_tools() {
    let names: Vec<String> = GeoTiffReaderPlugin::new()
        .list_tools()
        .iter()
        .map(|tool| tool.name.to_string())
        .collect();

    for name in &[
        "reader_open_bytes",
        "reader_open_path",
        "reader_images",
        "reader_image_info",
        "reader_seek_to_image",
        "reader_select_raster_band",
        "reader_origin",
        "reader_pixel_size",
    ] {
        assert!(
            names.iter().any(|candidate| candidate == name),
            "missing tool: {name}"
        );
    }
}

#[test]
fn sampling_plugin_lists_expected_tools() {
    let names: Vec<String> = GeoTiffSamplingPlugin::new()
        .list_tools()
        .iter()
        .map(|tool| tool.name.to_string())
        .collect();

    for name in &[
        "reader_read_pixel",
        "reader_read_pixel_at_location",
        "reader_pixels",
        "reader_coord_to_pixel",
        "reader_pixel_to_coord",
    ] {
        assert!(
            names.iter().any(|candidate| candidate == name),
            "missing tool: {name}"
        );
    }
}

#[test]
fn workflow_propositions_non_empty() {
    assert_verified::<GeoTiffReaderOpened>("GeoTiffReaderOpened");
    assert_verified::<GeoTiffReaderConfigured>("GeoTiffReaderConfigured");
    assert_verified::<GeoTiffSampled>("GeoTiffSampled");
}
