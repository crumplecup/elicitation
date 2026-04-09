//! `AxumCoreIntoResponsePlugin` — IntoResponse factory MCP tools.

use elicitation::{ElicitPlugin, ToCodeLiteral, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::workflow::from_request_plugin::BodyDescriptor;

fn json_err(e: impl std::fmt::Display) -> ErrorData {
    ErrorData::internal_error(e.to_string(), None)
}

// ── Types ─────────────────────────────────────────────────────────────────────

/// Description of an HTTP response produced by `IntoResponse`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct ResponseDescriptor {
    /// HTTP status code.
    pub status: u16,
    /// Response headers as `"Name: Value"` strings.
    pub headers: Vec<String>,
    /// Human-readable description of the response body.
    pub body_description: String,
    /// The `Content-Type` header value, if set.
    pub content_type: Option<String>,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for `into_response_describe`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IntoResponseDescribeParams {
    /// The Rust type that implements `IntoResponse` (e.g. `"Json<MyBody>"`).
    pub type_name: String,
    /// A description of the value being converted.
    pub value_description: String,
}

/// Parameters for `response_status`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResponseStatusParams {
    /// The HTTP status code (e.g. `200`, `404`).
    pub status: u16,
}

/// Parameters for `response_headers_describe`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResponseHeadersParams {
    /// The headers to include, as `"Name: Value"` strings.
    pub headers: Vec<String>,
}

/// Parameters for `response_body_collect`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResponseBodyParams {
    /// A description of the response body content.
    pub body_description: String,
    /// The `Content-Type` of the response body.
    pub content_type: String,
}

/// Parameters for `response_parts_describe`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResponsePartsParams {
    /// The HTTP status code.
    pub status: u16,
    /// The response headers as `"Name: Value"` strings.
    pub headers: Vec<String>,
}

/// Parameters for `response_map_body`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MapBodyParams {
    /// The original response descriptor to transform.
    pub original: ResponseDescriptor,
    /// A description of the new body after mapping.
    pub new_body_description: String,
}

/// Parameters for `append_headers`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AppendHeadersParams {
    /// The existing response descriptor.
    pub existing: ResponseDescriptor,
    /// Additional headers to append, as `"Name: Value"` strings.
    pub headers: Vec<String>,
}

/// Parameters for `extensions_describe`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExtensionsParams {
    /// The types stored in the response extensions map.
    pub extension_types: Vec<String>,
}

/// Parameters for `into_response_tuple`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TupleResponseParams {
    /// The HTTP status code.
    pub status: u16,
    /// The headers as `"Name: Value"` strings.
    pub headers: Vec<String>,
    /// The body content as a string.
    pub body: String,
}

/// Parameters for `into_response_result`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResultResponseParams {
    /// A description of the `Ok` response.
    pub ok_description: String,
    /// A description of the `Err` response.
    pub err_description: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_into_response",
    name = "into_response_describe",
    description = "Describe what HTTP response a given IntoResponse type produces."
)]
#[instrument]
async fn into_response_describe(
    p: IntoResponseDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let (status, content_type) = infer_type_defaults(&p.type_name);
    let descriptor = ResponseDescriptor {
        status,
        headers: content_type
            .as_ref()
            .map(|ct| vec![format!("content-type: {ct}")])
            .unwrap_or_default(),
        body_description: format!(
            "`{}` converts to a response. Value: {}",
            p.type_name, p.value_description
        ),
        content_type,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_into_response",
    name = "response_status",
    description = "Create a response descriptor for a given HTTP status code with an empty body."
)]
#[instrument]
async fn response_status(p: ResponseStatusParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = ResponseDescriptor {
        status: p.status,
        headers: vec![],
        body_description: format!("Empty body with status {}", p.status),
        content_type: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_into_response",
    name = "response_headers_describe",
    description = "Describe a response that sets specific headers. Returns a ResponseDescriptor."
)]
#[instrument]
async fn response_headers_describe(p: ResponseHeadersParams) -> Result<CallToolResult, ErrorData> {
    let content_type = p.headers.iter().find_map(|h| {
        let lower = h.to_lowercase();
        if lower.starts_with("content-type:") {
            h.split_once(':').map(|x| x.1.trim().to_string())
        } else {
            None
        }
    });
    let descriptor = ResponseDescriptor {
        status: 200,
        headers: p.headers,
        body_description: "Body not specified.".to_string(),
        content_type,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_into_response",
    name = "response_body_collect",
    description = "Describe collecting a response body into bytes, with content-type metadata."
)]
#[instrument]
async fn response_body_collect(p: ResponseBodyParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = BodyDescriptor {
        collected_bytes: None,
        content_type: Some(p.content_type.clone()),
        description: format!(
            "Collect response body: {}. Content-Type: `{}`.",
            p.body_description, p.content_type
        ),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_into_response",
    name = "response_parts_describe",
    description = "Describe the parts of a response (status + headers) separately from the body."
)]
#[instrument]
async fn response_parts_describe(p: ResponsePartsParams) -> Result<CallToolResult, ErrorData> {
    let content_type = p.headers.iter().find_map(|h| {
        let lower = h.to_lowercase();
        if lower.starts_with("content-type:") {
            h.split_once(':').map(|x| x.1.trim().to_string())
        } else {
            None
        }
    });
    let descriptor = ResponseDescriptor {
        status: p.status,
        headers: p.headers,
        body_description: "Body not yet attached to parts.".to_string(),
        content_type,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_into_response",
    name = "response_map_body",
    description = "Map the body of an existing response to a new body description, preserving status and headers."
)]
#[instrument]
async fn response_map_body(p: MapBodyParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = ResponseDescriptor {
        status: p.original.status,
        headers: p.original.headers,
        body_description: p.new_body_description,
        content_type: p.original.content_type,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_into_response",
    name = "append_headers",
    description = "Append additional headers to an existing response descriptor."
)]
#[instrument]
async fn append_headers(p: AppendHeadersParams) -> Result<CallToolResult, ErrorData> {
    let mut combined = p.existing.headers;
    combined.extend(p.headers);
    let content_type = combined.iter().find_map(|h| {
        let lower = h.to_lowercase();
        if lower.starts_with("content-type:") {
            h.split_once(':').map(|x| x.1.trim().to_string())
        } else {
            None
        }
    });
    let descriptor = ResponseDescriptor {
        status: p.existing.status,
        headers: combined,
        body_description: p.existing.body_description,
        content_type,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_into_response",
    name = "extensions_describe",
    description = "Describe the type-map extensions on a response. Extensions carry arbitrary typed data through the response."
)]
#[instrument]
async fn extensions_describe(p: ExtensionsParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "extension_types": p.extension_types,
        "description": format!(
            "Response extensions contain {} type(s): [{}]. \
             Extensions are a type-erased map accessible via `response.extensions()`. \
             Each type may be inserted at most once.",
            p.extension_types.len(),
            p.extension_types.join(", ")
        ),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_into_response",
    name = "into_response_tuple",
    description = "Build a ResponseDescriptor from a (status, headers, body) tuple, as axum's tuple IntoResponse impl does."
)]
#[instrument]
async fn into_response_tuple(p: TupleResponseParams) -> Result<CallToolResult, ErrorData> {
    let content_type = p.headers.iter().find_map(|h| {
        let lower = h.to_lowercase();
        if lower.starts_with("content-type:") {
            h.split_once(':').map(|x| x.1.trim().to_string())
        } else {
            None
        }
    });
    let descriptor = ResponseDescriptor {
        status: p.status,
        headers: p.headers,
        body_description: p.body,
        content_type,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_into_response",
    name = "into_response_result",
    description = "Describe how Result<impl IntoResponse, impl IntoResponse> converts: Ok uses the Ok response, Err uses the Err response."
)]
#[instrument]
async fn into_response_result(p: ResultResponseParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "ok_response": p.ok_description,
        "err_response": p.err_description,
        "description": format!(
            "`Result<impl IntoResponse, impl IntoResponse>` implements `IntoResponse`. \
             `Ok({})` produces the ok response. \
             `Err({})` produces the error response. \
             Both branches must independently satisfy `IntoResponse`.",
            p.ok_description, p.err_description
        ),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Infer default status and content-type for well-known axum response types.
fn infer_type_defaults(type_name: &str) -> (u16, Option<String>) {
    let lower = type_name.to_lowercase();
    if lower.contains("json") {
        (200, Some("application/json".to_string()))
    } else if lower.contains("html") {
        (200, Some("text/html; charset=utf-8".to_string()))
    } else if lower.contains("redirect") {
        (302, None)
    } else {
        (200, None)
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Plugin exposing axum-core `IntoResponse` factory tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_into_response")]
pub struct AxumCoreIntoResponsePlugin;
