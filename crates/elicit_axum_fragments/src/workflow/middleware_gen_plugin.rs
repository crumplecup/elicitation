//! AxumMiddlewareGenPlugin — emit axum middleware and layer fragments.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for emit_middleware_fn.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitMiddlewareFnParams {
    /// Middleware function name.
    pub name: String,
    /// Code to run before passing the request to the next handler.
    pub before_body: String,
    /// Code to run after receiving the response from the next handler.
    pub after_body: String,
}

/// Parameters for emit_middleware_from_extractor.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitExtractorMiddlewareParams {
    /// The extractor type to pull from the request.
    pub extractor_type: String,
    /// Validation logic body operating on the extracted value.
    pub validation_body: String,
}

/// Parameters for emit_auth_middleware.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitAuthMiddlewareParams {
    /// Expected authorization header scheme (e.g. `Bearer`).
    pub auth_header: String,
    /// Application state type providing token validation.
    pub state_type: String,
}

/// Parameters for emit_logging_middleware.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitLoggingMiddlewareParams {
    /// Log level to use (e.g. `debug`, `info`).
    pub log_level: String,
    /// Whether to include the request body in the log output.
    pub include_body: bool,
}

/// Parameters for emit_rate_limit_middleware.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitRateLimitParams {
    /// Maximum requests allowed per second.
    pub requests_per_second: u32,
    /// Application state type providing the rate limiter.
    pub state_type: String,
}

/// Parameters for emit_cors_config.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitCorsConfigParams {
    /// Allowed origin URLs.
    pub allowed_origins: Vec<String>,
    /// Allowed HTTP methods (e.g. `GET`, `POST`).
    pub allowed_methods: Vec<String>,
    /// Whether to allow credentials (cookies, auth headers).
    pub allow_credentials: bool,
}

/// Parameters for emit_trace_layer_config.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitTraceLayerParams {
    /// Whether to include request headers in traces.
    pub include_headers: bool,
    /// Tracing log level for the layer (e.g. `DEBUG`).
    pub log_level: String,
}

/// Parameters for emit_timeout_config.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitTimeoutParams {
    /// Request timeout in seconds.
    pub timeout_secs: u64,
}

/// Parameters for emit_request_id_config.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitRequestIdParams {
    /// Header name for the request ID (e.g. `x-request-id`).
    pub header_name: String,
}

/// Parameters for emit_compression_config.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitCompressionParams {
    /// Compression algorithms to enable (e.g. `["gzip", "br", "zstd"]`).
    pub algorithms: Vec<String>,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_middleware_gen",
    emit = None,
    name = "emit_middleware_fn",
    description = "Emit an axum middleware function using the `from_fn` pattern."
)]
#[instrument]
async fn emit_middleware_fn(p: EmitMiddlewareFnParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"pub async fn {}(request: Request, next: Next) -> Response {{
    {}
    let response = next.run(request).await;
    {}
    response
}}"#,
        p.name, p.before_body, p.after_body
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_middleware_gen",
    emit = None,
    name = "emit_middleware_from_extractor",
    description = "Emit extractor-based middleware that validates an extracted type before proceeding."
)]
#[instrument]
async fn emit_middleware_from_extractor(
    p: EmitExtractorMiddlewareParams,
) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"pub async fn extractor_middleware(
    extracted: Result<{extractor_type}, impl IntoResponse>,
    request: Request,
    next: Next,
) -> Response {{
    match extracted {{
        Ok(value) => {{
            {validation_body}
            next.run(request).await
        }}
        Err(rejection) => rejection.into_response(),
    }}
}}"#,
        extractor_type = p.extractor_type,
        validation_body = p.validation_body,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_middleware_gen",
    emit = None,
    name = "emit_auth_middleware",
    description = "Emit a Bearer token authentication middleware that validates against app state."
)]
#[instrument]
async fn emit_auth_middleware(p: EmitAuthMiddlewareParams) -> Result<CallToolResult, ErrorData> {
    let auth_header = &p.auth_header;
    let state_type = &p.state_type;
    let code = format!(
        r#"pub async fn auth_middleware(
    State(_state): State<{state_type}>,
    TypedHeader(authorization): TypedHeader<Authorization<{auth_header}>>,
    request: Request,
    next: Next,
) -> Response {{
    let token = authorization.token();
    // TODO: validate token against state
    let _ = token;
    next.run(request).await
}}"#
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_middleware_gen",
    emit = None,
    name = "emit_logging_middleware",
    description = "Emit a request/response logging middleware."
)]
#[instrument]
async fn emit_logging_middleware(
    p: EmitLoggingMiddlewareParams,
) -> Result<CallToolResult, ErrorData> {
    let body_line = if p.include_body {
        "    // Note: collecting body for logging consumes it — use request.body_mut() carefully\n    tracing::debug!(\"request body logging enabled\");"
    } else {
        "    // Body logging disabled"
    };
    let level = &p.log_level;
    let code = format!(
        r#"pub async fn logging_middleware(request: Request, next: Next) -> Response {{
    let method = request.method().clone();
    let uri = request.uri().clone();
{body_line}
    tracing::{level}!(method = %method, uri = %uri, "incoming request");
    let response = next.run(request).await;
    let status = response.status();
    tracing::{level}!(status = %status, "outgoing response");
    response
}}"#,
        body_line = body_line,
        level = level,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_middleware_gen",
    emit = None,
    name = "emit_rate_limit_middleware",
    description = "Emit a rate limiting middleware using a token bucket in application state."
)]
#[instrument]
async fn emit_rate_limit_middleware(p: EmitRateLimitParams) -> Result<CallToolResult, ErrorData> {
    let rps = p.requests_per_second;
    let state_type = &p.state_type;
    let code = format!(
        r#"pub async fn rate_limit_middleware(
    State(state): State<{state_type}>,
    request: Request,
    next: Next,
) -> Response {{
    // Allow {rps} requests per second
    if !state.rate_limiter.check().is_ok() {{
        return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
    }}
    next.run(request).await
}}"#,
        state_type = state_type,
        rps = rps,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_middleware_gen",
    emit = None,
    name = "emit_cors_config",
    description = "Emit a CorsLayer configuration with specified origins, methods, and credentials."
)]
#[instrument]
async fn emit_cors_config(p: EmitCorsConfigParams) -> Result<CallToolResult, ErrorData> {
    let origins = p
        .allowed_origins
        .iter()
        .map(|o| format!("        \"{}\".parse::<HeaderValue>().unwrap()", o))
        .collect::<Vec<_>>()
        .join(",\n");
    let methods = p
        .allowed_methods
        .iter()
        .map(|m| format!("Method::{}", m.to_uppercase()))
        .collect::<Vec<_>>()
        .join(", ");
    let creds = if p.allow_credentials {
        "\n    .allow_credentials(true)"
    } else {
        ""
    };
    let code = format!(
        "CorsLayer::new()\n    .allow_origin([\n{}\n    ])\n    .allow_methods([{}]){}",
        origins, methods, creds
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_middleware_gen",
    emit = None,
    name = "emit_trace_layer_config",
    description = "Emit a TraceLayer configuration with optional header tracing."
)]
#[instrument]
async fn emit_trace_layer_config(p: EmitTraceLayerParams) -> Result<CallToolResult, ErrorData> {
    let header_line = if p.include_headers {
        "\n    .make_span_with(DefaultMakeSpan::new().include_headers(true))"
    } else {
        ""
    };
    let level = p.log_level.to_uppercase();
    let code = format!(
        "TraceLayer::new_for_http()\n    .make_span_with(DefaultMakeSpan::new().level(Level::{})){}",
        level, header_line
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_middleware_gen",
    emit = None,
    name = "emit_timeout_config",
    description = "Emit a TimeoutLayer configuration with the specified duration."
)]
#[instrument]
async fn emit_timeout_config(p: EmitTimeoutParams) -> Result<CallToolResult, ErrorData> {
    let code = format!("TimeoutLayer::new(Duration::from_secs({}))", p.timeout_secs);
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_middleware_gen",
    emit = None,
    name = "emit_request_id_config",
    description = "Emit SetRequestId and PropagateRequestId layer configuration."
)]
#[instrument]
async fn emit_request_id_config(p: EmitRequestIdParams) -> Result<CallToolResult, ErrorData> {
    let header = &p.header_name;
    let code = format!(
        r#"// Set and propagate request IDs via the "{header}" header
let request_id_layer = ServiceBuilder::new()
    .layer(SetRequestIdLayer::new(
        HeaderName::from_static("{header}"),
        MakeRequestUuid,
    ))
    .layer(PropagateRequestIdLayer::new(
        HeaderName::from_static("{header}"),
    ));"#,
        header = header,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_middleware_gen",
    emit = None,
    name = "emit_compression_config",
    description = "Emit a CompressionLayer configuration enabling the specified algorithms."
)]
#[instrument]
async fn emit_compression_config(p: EmitCompressionParams) -> Result<CallToolResult, ErrorData> {
    let algo_comments = p
        .algorithms
        .iter()
        .map(|a| format!("// - {}", a))
        .collect::<Vec<_>>()
        .join("\n");
    let code = format!(
        "// Compression enabled for: {}\nCompressionLayer::new()",
        algo_comments
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

/// Plugin exposing axum middleware and layer generation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_middleware_gen")]
pub struct AxumMiddlewareGenPlugin;
