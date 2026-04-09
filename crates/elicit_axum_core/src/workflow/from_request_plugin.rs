//! `AxumCoreFromRequestPlugin` — FromRequest/FromRequestParts factory MCP tools.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

fn json_err(e: impl std::fmt::Display) -> ErrorData {
    ErrorData::internal_error(e.to_string(), None)
}

// ── Types ─────────────────────────────────────────────────────────────────────

/// Description of a `FromRequest` or `FromRequestParts` extractor.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FromRequestDescriptor {
    /// The extractor type name.
    pub extractor_type: String,
    /// Whether this extractor consumes the request body.
    pub requires_body: bool,
    /// The state type required by this extractor.
    pub state_type: String,
    /// The rejection type produced on failure.
    pub rejection_type: String,
    /// Human-readable description of how the extractor works.
    pub description: String,
}

/// Description of a body collection or streaming operation.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BodyDescriptor {
    /// Number of bytes collected, if known.
    pub collected_bytes: Option<usize>,
    /// The content-type of the body, if known.
    pub content_type: Option<String>,
    /// Human-readable description of the body operation.
    pub description: String,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for `from_request_json_simulate`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FromRequestSimulateParams {
    /// The extractor type to simulate (e.g. `"Json<MyType>"`).
    pub extractor_type: String,
    /// The request body to parse.
    pub request_body: String,
    /// The `Content-Type` header value.
    pub content_type: String,
}

/// Parameters for `from_request_parts_json_simulate`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FromRequestPartsSimulateParams {
    /// The extractor type to simulate (e.g. `"Path<u32>"`).
    pub extractor_type: String,
    /// The request URI.
    pub uri: String,
    /// The HTTP method.
    pub method: String,
    /// The request headers as `"Name: Value"` strings.
    pub headers: Vec<String>,
}

/// Parameters for `from_request_describe`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FromRequestDescribeParams {
    /// The extractor type to describe (e.g. `"Json<T>"`).
    pub extractor_type: String,
}

/// Parameters for `from_request_parts_describe`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FromRequestPartsDescribeParams {
    /// The parts-only extractor type to describe (e.g. `"Path<T>"`).
    pub extractor_type: String,
}

/// Parameters for `rejection_display`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RejectionParams {
    /// The rejection type name (e.g. `"JsonRejection"`).
    pub rejection_type: String,
    /// A human-readable rejection message.
    pub message: String,
}

/// Parameters for `body_collect_bytes`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BodySizeParams {
    /// The `Content-Length` of the body, if provided by the client.
    pub content_length: Option<u64>,
}

/// Parameters for `body_collect_string`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BodyStringParams {
    /// The character encoding used (e.g. `"utf-8"`).
    pub encoding: String,
    /// The `Content-Length` of the body, if provided by the client.
    pub content_length: Option<u64>,
}

/// Parameters for `body_into_data_stream`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BodyStreamParams {
    /// A description of the body stream being created.
    pub description: String,
}

/// Parameters for `body_into_limited`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BodyLimitParams {
    /// The maximum number of bytes the body may contain.
    pub limit_bytes: u64,
}

/// Parameters for `body_frame_describe`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FrameParams {
    /// The frame type (e.g. `"data"`, `"trailers"`).
    pub frame_type: String,
    /// A size hint for the frame in bytes, if known.
    pub size_hint: Option<u64>,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_from_request",
    name = "from_request_json_simulate",
    description = "Simulate parsing a request body using a FromRequest extractor. Returns a descriptor of the extraction attempt."
)]
#[instrument]
async fn from_request_json_simulate(
    p: FromRequestSimulateParams,
) -> Result<CallToolResult, ErrorData> {
    let descriptor = FromRequestDescriptor {
        extractor_type: p.extractor_type.clone(),
        requires_body: true,
        state_type: "()".to_string(),
        rejection_type: format!(
            "{}Rejection",
            p.extractor_type
                .trim_end_matches(">")
                .split('<')
                .next()
                .unwrap_or(&p.extractor_type)
        ),
        description: format!(
            "Simulate extracting `{}` from a request body with Content-Type `{}`. \
             Body payload: `{}`",
            p.extractor_type, p.content_type, p.request_body
        ),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_from_request",
    name = "from_request_parts_json_simulate",
    description = "Simulate extracting data from request parts using a FromRequestParts extractor. Returns a descriptor of the extraction."
)]
#[instrument]
async fn from_request_parts_json_simulate(
    p: FromRequestPartsSimulateParams,
) -> Result<CallToolResult, ErrorData> {
    let descriptor = FromRequestDescriptor {
        extractor_type: p.extractor_type.clone(),
        requires_body: false,
        state_type: "()".to_string(),
        rejection_type: format!(
            "{}Rejection",
            p.extractor_type
                .trim_end_matches(">")
                .split('<')
                .next()
                .unwrap_or(&p.extractor_type)
        ),
        description: format!(
            "Simulate extracting `{}` from request parts: method=`{}`, uri=`{}`, headers=[{}]",
            p.extractor_type,
            p.method,
            p.uri,
            p.headers.join(", ")
        ),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_from_request",
    name = "from_request_describe",
    description = "Describe how a FromRequest extractor works, including body consumption, state requirements, and rejection type."
)]
#[instrument]
async fn from_request_describe(p: FromRequestDescribeParams) -> Result<CallToolResult, ErrorData> {
    let base = p
        .extractor_type
        .trim_end_matches(">")
        .split('<')
        .next()
        .unwrap_or(&p.extractor_type);
    let descriptor = FromRequestDescriptor {
        extractor_type: p.extractor_type.clone(),
        requires_body: true,
        state_type: "S".to_string(),
        rejection_type: format!("{}Rejection", base),
        description: format!(
            "`{}` implements `FromRequest<S>`, consuming the request body. \
             It requires state type `S` and produces `{}Rejection` on failure.",
            p.extractor_type, base
        ),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_from_request",
    name = "from_request_parts_describe",
    description = "Describe how a FromRequestParts extractor works. Parts-only extractors do not consume the body."
)]
#[instrument]
async fn from_request_parts_describe(
    p: FromRequestPartsDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let base = p
        .extractor_type
        .trim_end_matches(">")
        .split('<')
        .next()
        .unwrap_or(&p.extractor_type);
    let descriptor = FromRequestDescriptor {
        extractor_type: p.extractor_type.clone(),
        requires_body: false,
        state_type: "S".to_string(),
        rejection_type: format!("{}Rejection", base),
        description: format!(
            "`{}` implements `FromRequestParts<S>`, extracting from URI/headers/method \
             without consuming the body. Produces `{}Rejection` on failure.",
            p.extractor_type, base
        ),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_from_request",
    name = "rejection_display",
    description = "Format and describe a FromRequest rejection, showing the rejection type and message."
)]
#[instrument]
async fn rejection_display(p: RejectionParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "rejection_type": p.rejection_type,
        "message": p.message,
        "display": format!("{}: {}", p.rejection_type, p.message),
        "description": format!(
            "Rejection of type `{}` with message: `{}`",
            p.rejection_type, p.message
        ),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_from_request",
    name = "body_collect_bytes",
    description = "Describe collecting the full request body as raw bytes. Returns a BodyDescriptor."
)]
#[instrument]
async fn body_collect_bytes(p: BodySizeParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = BodyDescriptor {
        collected_bytes: p.content_length.map(|n| n as usize),
        content_type: None,
        description: match p.content_length {
            Some(n) => format!(
                "Collect request body into `Bytes`. Content-Length: {} bytes. \
                 Reads all frames until the body stream is exhausted.",
                n
            ),
            None => "Collect request body into `Bytes`. \
                     No Content-Length; reads all frames until the body stream is exhausted."
                .to_string(),
        },
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_from_request",
    name = "body_collect_string",
    description = "Describe collecting the full request body as a UTF-8 string. Returns a BodyDescriptor."
)]
#[instrument]
async fn body_collect_string(p: BodyStringParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = BodyDescriptor {
        collected_bytes: p.content_length.map(|n| n as usize),
        content_type: Some(format!("text/plain; charset={}", p.encoding)),
        description: match p.content_length {
            Some(n) => format!(
                "Collect request body into `String` using encoding `{}`. \
                 Content-Length: {} bytes. Fails with `InvalidUtf8` if body is not valid {}.",
                p.encoding, n, p.encoding
            ),
            None => format!(
                "Collect request body into `String` using encoding `{}`. \
                 Fails with `InvalidUtf8` if body is not valid {}.",
                p.encoding, p.encoding
            ),
        },
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_from_request",
    name = "body_into_data_stream",
    description = "Describe converting a body into an axum-core BodyDataStream for streaming access."
)]
#[instrument]
async fn body_into_data_stream(p: BodyStreamParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = BodyDescriptor {
        collected_bytes: None,
        content_type: None,
        description: format!(
            "Convert body into `BodyDataStream` (a `Stream<Item = Result<Bytes, _>>`). \
             Yields data frames one at a time without buffering the full body. \
             Context: {}",
            p.description
        ),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_from_request",
    name = "body_into_limited",
    description = "Describe wrapping a body with a byte limit using http_body_util::Limited."
)]
#[instrument]
async fn body_into_limited(p: BodyLimitParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = BodyDescriptor {
        collected_bytes: Some(p.limit_bytes as usize),
        content_type: None,
        description: format!(
            "Wrap body with `Limited` to enforce a {} byte maximum. \
             Reading beyond the limit produces a `LengthLimitError`. \
             Use `RequestBodyLimitLayer` for automatic limit enforcement in axum.",
            p.limit_bytes
        ),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_from_request",
    name = "body_frame_describe",
    description = "Describe an HTTP body frame, including its type (data or trailers) and size hint."
)]
#[instrument]
async fn body_frame_describe(p: FrameParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "frame_type": p.frame_type,
        "size_hint": p.size_hint,
        "description": format!(
            "HTTP body `Frame` of type `{}`. {}",
            p.frame_type,
            match p.size_hint {
                Some(n) => format!("Size hint: {} bytes. ", n),
                None => "No size hint available. ".to_string(),
            }
        ),
        "variants": ["data", "trailers"],
        "note": "Use `Frame::data(Bytes)` for data frames and `Frame::trailers(HeaderMap)` for trailer frames.",
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Plugin exposing axum-core `FromRequest`/`FromRequestParts` and body utility tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_from_request")]
pub struct AxumCoreFromRequestPlugin;
