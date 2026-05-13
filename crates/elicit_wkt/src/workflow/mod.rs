//! MCP workflow plugins for WKT construction and parsing.

mod parse_plugin;
mod types_plugin;

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use serde::Serialize;

pub use parse_plugin::{
    ParseWktParams, WktItemGeometryTypeParams, WktItemStringParams, WktParsePlugin, WktParsed,
};
pub use types_plugin::{
    CoordNew3dParams, CoordNewParams, CoordNewWithMParams, EmptyPointParams,
    GeometryCollectionNewParams, LineStringNewParams, MultiLineStringNewParams,
    MultiPointNewParams, MultiPolygonNewParams, PointNewParams, PolygonNewParams, WktTypeCreated,
    WktTypesPlugin,
};

fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(value)
            .map_err(|error| ErrorData::internal_error(error.to_string(), None))?,
    )]))
}

fn text_result(text: impl Into<String>) -> CallToolResult {
    CallToolResult::success(vec![Content::text(text.into())])
}
