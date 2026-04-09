//! `AxumExtractMiscPlugin` — MCP tools for axum miscellaneous extractors.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for extract_state_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StateDescribeParams {
    /// The Rust type stored as application state.
    pub state_type: String,
}

/// Parameters for extract_extension_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExtensionDescribeParams {
    /// The Rust type stored as a request extension.
    pub extension_type: String,
}

/// Parameters for extract_connect_info_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ConnectInfoDescribeParams {
    /// The Rust type used for the connect info address, e.g. `SocketAddr`.
    pub addr_type: String,
}

/// Empty parameters for extract_matched_path_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NoParamsExtMisc {}

/// Empty parameters for extract_original_uri_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NoParamsOrigUri {}

/// Empty parameters for extract_host_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NoParamsHost {}

/// Empty parameters for extract_header_map_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NoParamsHeaderMap {}

/// Parameters for extract_typed_header_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TypedHeaderDescribeParams {
    /// The typed header type name, e.g. `Authorization`.
    pub header_type: String,
}

/// Parameters for extension_missing_error.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExtensionMissingParams {
    /// The extension type that was not found.
    pub extension_type: String,
}

/// Parameters for state_missing_error.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StateMissingParams {
    /// The state type that was not found.
    pub state_type: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_extract_misc",
    name = "extract_state_describe",
    description = "Describe the State<T> extractor for shared application state."
)]
#[instrument]
async fn extract_state_describe(p: StateDescribeParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "State<{}> extracts shared application state added via Router::with_state(). \
        Requires the state to implement Clone. \
        Panics if state was not provided.",
        p.state_type
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_misc",
    name = "extract_extension_describe",
    description = "Describe the Extension<T> extractor for request extensions set by middleware."
)]
#[instrument]
async fn extract_extension_describe(
    p: ExtensionDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "Extension<{}> extracts request extensions set by middleware layers. \
        Returns 500 if the extension is missing.",
        p.extension_type
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_misc",
    name = "extract_connect_info_describe",
    description = "Describe the ConnectInfo<T> extractor for remote socket address information."
)]
#[instrument]
async fn extract_connect_info_describe(
    p: ConnectInfoDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "ConnectInfo<{}> extracts the remote socket address. \
        Requires IntoMakeServiceWithConnectInfo on the router.",
        p.addr_type
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_misc",
    name = "extract_matched_path_describe",
    description = "Describe the MatchedPath extractor that returns the matched route pattern."
)]
#[instrument]
async fn extract_matched_path_describe(_p: NoParamsExtMisc) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        "MatchedPath extracts the matched route path pattern (e.g., '/users/:id'). \
        Only available inside a Router."
            .to_string(),
    )]))
}

#[elicit_tool(
    plugin = "axum_extract_misc",
    name = "extract_original_uri_describe",
    description = "Describe the OriginalUri extractor that returns the unmodified request URI."
)]
#[instrument]
async fn extract_original_uri_describe(_p: NoParamsOrigUri) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        "OriginalUri extracts the original, unmodified request URI \
        before any nested Router path modifications."
            .to_string(),
    )]))
}

#[elicit_tool(
    plugin = "axum_extract_misc",
    name = "extract_host_describe",
    description = "Describe the Host extractor that reads the hostname from request headers."
)]
#[instrument]
async fn extract_host_describe(_p: NoParamsHost) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        "Host extracts the hostname from the Host header or X-Forwarded-Host header.".to_string(),
    )]))
}

#[elicit_tool(
    plugin = "axum_extract_misc",
    name = "extract_header_map_describe",
    description = "Describe the HeaderMap extractor that gives access to all request headers."
)]
#[instrument]
async fn extract_header_map_describe(_p: NoParamsHeaderMap) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        "HeaderMap extracts the complete map of all request headers as a reference.".to_string(),
    )]))
}

#[elicit_tool(
    plugin = "axum_extract_misc",
    name = "extract_typed_header_describe",
    description = "Describe the TypedHeader<T> extractor for parsing a specific typed header."
)]
#[instrument]
async fn extract_typed_header_describe(
    p: TypedHeaderDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "TypedHeader<{}> extracts and parses a specific typed header. \
        Returns rejection if header is missing or malformed.",
        p.header_type
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_misc",
    name = "extension_missing_error",
    description = "Format the 500 error message produced when a required request extension is absent."
)]
#[instrument]
async fn extension_missing_error(p: ExtensionMissingParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "500 Internal Server Error: Extension of type `{}` was not found. \
        Did you add the extension with `.layer(Extension(...))`?",
        p.extension_type
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_misc",
    name = "state_missing_error",
    description = "Format the 500 error message produced when application state is not found."
)]
#[instrument]
async fn state_missing_error(p: StateMissingParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "500 Internal Server Error: State of type `{}` was not found. \
        Did you call `.with_state(...)` on the router?",
        p.state_type
    ))]))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Plugin exposing axum miscellaneous extractor tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_extract_misc")]
pub struct AxumExtractMiscPlugin;
