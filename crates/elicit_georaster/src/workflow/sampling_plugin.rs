//! `GeoTiffSamplingPlugin` — sample pixels and convert coordinates.

use crate::{Coordinate, GeoTiffReader};
use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::json_result;

/// Proposition: a GeoTIFF reader was used for sampling.
#[derive(Prop)]
pub struct GeoTiffSampled;

impl VerifiedWorkflow for GeoTiffSampled {}

/// Parameters for `read_pixel`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderReadPixelParams {
    /// Reader to sample.
    pub reader: GeoTiffReader,
    /// Pixel x coordinate.
    pub x: u32,
    /// Pixel y coordinate.
    pub y: u32,
}

/// Parameters for `read_pixel_at_location`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderReadPixelAtLocationParams {
    /// Reader to sample.
    pub reader: GeoTiffReader,
    /// Geographic coordinate to sample.
    pub coord: Coordinate,
}

/// Parameters for `pixels`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderPixelsParams {
    /// Reader to sample.
    pub reader: GeoTiffReader,
    /// Starting x coordinate.
    pub x: u32,
    /// Starting y coordinate.
    pub y: u32,
    /// Window width.
    pub width: u32,
    /// Window height.
    pub height: u32,
}

/// Parameters for `coord_to_pixel`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderCoordToPixelParams {
    /// Reader to inspect.
    pub reader: GeoTiffReader,
    /// Geographic coordinate to convert.
    pub coord: Coordinate,
}

/// Parameters for `pixel_to_coord`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderPixelToCoordParams {
    /// Reader to inspect.
    pub reader: GeoTiffReader,
    /// Pixel x coordinate.
    pub x: u32,
    /// Pixel y coordinate.
    pub y: u32,
}

#[elicit_tool(
    plugin = "geotiff_sampling",
    name = "reader_read_pixel",
    description = "Read a pixel value at integer x/y coordinates. Establishes: GeoTiffSampled."
)]
#[instrument]
async fn reader_read_pixel(
    p: ReaderReadPixelParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let value = p.reader.read_pixel(p.x, p.y);
    let _proof = Established::<GeoTiffSampled>::assert();
    json_result(&value)
}

#[elicit_tool(
    plugin = "geotiff_sampling",
    name = "reader_read_pixel_at_location",
    description = "Read a pixel value at a geographic coordinate. Establishes: GeoTiffSampled."
)]
#[instrument]
async fn reader_read_pixel_at_location(
    p: ReaderReadPixelAtLocationParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let value = p.reader.read_pixel_at_location(p.coord);
    let _proof = Established::<GeoTiffSampled>::assert();
    json_result(&value)
}

#[elicit_tool(
    plugin = "geotiff_sampling",
    name = "reader_pixels",
    description = "Collect a pixel window into an owned Pixels wrapper. Establishes: GeoTiffSampled."
)]
#[instrument]
async fn reader_pixels(p: ReaderPixelsParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let pixels = p.reader.pixels(p.x, p.y, p.width, p.height);
    let _proof = Established::<GeoTiffSampled>::assert();
    json_result(&pixels)
}

#[elicit_tool(
    plugin = "geotiff_sampling",
    name = "reader_coord_to_pixel",
    description = "Convert a geographic coordinate into a pixel coordinate if geoinformation is available."
)]
#[instrument]
async fn reader_coord_to_pixel(
    p: ReaderCoordToPixelParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&p.reader.coord_to_pixel(p.coord))
}

#[elicit_tool(
    plugin = "geotiff_sampling",
    name = "reader_pixel_to_coord",
    description = "Convert a pixel coordinate into a geographic coordinate if geoinformation is available."
)]
#[instrument]
async fn reader_pixel_to_coord(
    p: ReaderPixelToCoordParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&p.reader.pixel_to_coord(p.x, p.y))
}

/// The GeoTIFF sampling MCP plugin.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geotiff_sampling")]
pub struct GeoTiffSamplingPlugin;

impl GeoTiffSamplingPlugin {
    /// Create a new GeoTIFF sampling plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for GeoTiffSamplingPlugin {
    fn default() -> Self {
        Self::new()
    }
}
