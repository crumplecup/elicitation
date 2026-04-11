//! MCP workflow plugins for opening, configuring, and sampling GeoTIFF readers.

mod reader_plugin;
mod sampling_plugin;

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use serde::Serialize;

pub use reader_plugin::{
    GeoTiffReaderConfigured, GeoTiffReaderOpened, GeoTiffReaderPlugin, ReaderImageInfoParams,
    ReaderImagesParams, ReaderOpenBytesParams, ReaderOpenPathParams, ReaderOriginParams,
    ReaderPixelSizeParams, ReaderSeekToImageParams, ReaderSelectRasterBandParams,
};
pub use sampling_plugin::{
    GeoTiffSampled, GeoTiffSamplingPlugin, ReaderCoordToPixelParams, ReaderPixelToCoordParams,
    ReaderPixelsParams, ReaderReadPixelAtLocationParams, ReaderReadPixelParams,
};

fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(value)
            .map_err(|error| ErrorData::internal_error(error.to_string(), None))?,
    )]))
}
