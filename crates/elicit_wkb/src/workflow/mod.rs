//! MCP workflow plugins for WKB reader and writer surfaces.

mod reader_plugin;
mod writer_plugin;

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use serde::Serialize;

pub use reader_plugin::{
    ReadWkbParams, WkbDimensionParams, WkbEndiannessParams, WkbGeometryTypeParams, WkbParsed,
    WkbReaderPlugin, WkbTryNewParams,
};
pub use writer_plugin::{
    GeometryCollectionSizeParams, GeometrySizeParams, LineSizeParams, LineStringSizeParams,
    MultiLineStringSizeParams, MultiPointSizeParams, MultiPolygonSizeParams, PointSizeParams,
    PolygonSizeParams, RectSizeParams, TriangleSizeParams, WkbWriteGeometryCollectionParams,
    WkbWriteGeometryParams, WkbWriteLineParams, WkbWriteLineStringParams,
    WkbWriteMultiLineStringParams, WkbWriteMultiPointParams, WkbWriteMultiPolygonParams,
    WkbWritePointParams, WkbWritePolygonParams, WkbWriteRectParams, WkbWriteTriangleParams,
    WkbWriterPlugin, WkbWritten,
};

fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(value)
            .map_err(|error| ErrorData::internal_error(error.to_string(), None))?,
    )]))
}
