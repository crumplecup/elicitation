//! MCP workflow plugins for PROJ coordinate transformations.

mod transform_plugin;

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use serde::Serialize;

pub use transform_plugin::{
    ConvertCoordParams, ConvertGeometryParams, CreateFromKnownCrsParams,
    CreateFromProjStringParams, DefinitionParams, ProjCreated, ProjTransformPlugin,
    ProjectCoordParams, TransformBoundsParams, TransformBoundsResult,
};

pub(crate) fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(value)
            .map_err(|e: serde_json::Error| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}
