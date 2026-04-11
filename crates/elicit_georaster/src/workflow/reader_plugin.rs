//! `GeoTiffReaderPlugin` — open/configure readers and inspect metadata.

use crate::GeoTiffReader;
use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::json_result;

/// Proposition: a GeoTIFF reader was successfully opened.
#[derive(Prop)]
pub struct GeoTiffReaderOpened;

impl VerifiedWorkflow for GeoTiffReaderOpened {}

/// Proposition: a GeoTIFF reader was successfully reconfigured.
#[derive(Prop)]
pub struct GeoTiffReaderConfigured;

impl VerifiedWorkflow for GeoTiffReaderConfigured {}

/// Parameters for opening a reader from bytes.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderOpenBytesParams {
    /// Raw TIFF/GeoTIFF bytes.
    pub bytes: Vec<u8>,
}

/// Parameters for opening a reader from a filesystem path.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderOpenPathParams {
    /// Path to a TIFF/GeoTIFF file.
    pub path: String,
}

/// Parameters for listing image metadata.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderImagesParams {
    /// Reader to inspect.
    pub reader: GeoTiffReader,
}

/// Parameters for fetching current image info.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderImageInfoParams {
    /// Reader to inspect.
    pub reader: GeoTiffReader,
}

/// Parameters for seeking to an image/IFD.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderSeekToImageParams {
    /// Reader to update.
    pub reader: GeoTiffReader,
    /// Image/IFD index.
    pub index: usize,
}

/// Parameters for selecting a raster band.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderSelectRasterBandParams {
    /// Reader to update.
    pub reader: GeoTiffReader,
    /// 1-based raster band.
    pub band: u8,
}

/// Parameters for origin inspection.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderOriginParams {
    /// Reader to inspect.
    pub reader: GeoTiffReader,
}

/// Parameters for pixel-size inspection.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderPixelSizeParams {
    /// Reader to inspect.
    pub reader: GeoTiffReader,
}

#[elicit_tool(
    plugin = "geotiff_reader",
    name = "reader_open_bytes",
    description = "Open a GeoTIFF reader from bytes. Establishes: GeoTiffReaderOpened."
)]
#[instrument]
async fn reader_open_bytes(
    p: ReaderOpenBytesParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let reader = GeoTiffReader::from_bytes(p.bytes)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoTiffReaderOpened>::assert();
    json_result(&reader)
}

#[elicit_tool(
    plugin = "geotiff_reader",
    name = "reader_open_path",
    description = "Open a GeoTIFF reader from a filesystem path. Establishes: GeoTiffReaderOpened."
)]
#[instrument]
async fn reader_open_path(
    p: ReaderOpenPathParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let reader = GeoTiffReader::from_path(&p.path)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoTiffReaderOpened>::assert();
    json_result(&reader)
}

#[elicit_tool(
    plugin = "geotiff_reader",
    name = "reader_images",
    description = "Return metadata for all images/IFDs in the reader."
)]
#[instrument]
async fn reader_images(p: ReaderImagesParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&p.reader.images())
}

#[elicit_tool(
    plugin = "geotiff_reader",
    name = "reader_image_info",
    description = "Return metadata for the current image/IFD."
)]
#[instrument]
async fn reader_image_info(
    p: ReaderImageInfoParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&p.reader.image_info())
}

#[elicit_tool(
    plugin = "geotiff_reader",
    name = "reader_seek_to_image",
    description = "Seek the reader to a specific image/IFD and return the updated reader. Establishes: GeoTiffReaderConfigured."
)]
#[instrument]
async fn reader_seek_to_image(
    p: ReaderSeekToImageParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let mut reader = p.reader;
    reader
        .seek_to_image(p.index)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoTiffReaderConfigured>::assert();
    json_result(&reader)
}

#[elicit_tool(
    plugin = "geotiff_reader",
    name = "reader_select_raster_band",
    description = "Select a 1-based raster band and return the updated reader. Establishes: GeoTiffReaderConfigured."
)]
#[instrument]
async fn reader_select_raster_band(
    p: ReaderSelectRasterBandParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let mut reader = p.reader;
    reader
        .select_raster_band(p.band)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoTiffReaderConfigured>::assert();
    json_result(&reader)
}

#[elicit_tool(
    plugin = "geotiff_reader",
    name = "reader_origin",
    description = "Return the raster origin if geoinformation is available."
)]
#[instrument]
async fn reader_origin(p: ReaderOriginParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&p.reader.origin())
}

#[elicit_tool(
    plugin = "geotiff_reader",
    name = "reader_pixel_size",
    description = "Return pixel size if geoinformation is available."
)]
#[instrument]
async fn reader_pixel_size(
    p: ReaderPixelSizeParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&p.reader.pixel_size())
}

/// The GeoTIFF reader MCP plugin.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geotiff_reader")]
pub struct GeoTiffReaderPlugin;

impl GeoTiffReaderPlugin {
    /// Create a new GeoTIFF reader plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for GeoTiffReaderPlugin {
    fn default() -> Self {
        Self::new()
    }
}
