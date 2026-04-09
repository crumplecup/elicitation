//! AxumHandlerGenPlugin — emit axum handler function fragments.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Types ─────────────────────────────────────────────────────────────────────

/// Extractor binding definition for handler parameters.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExtractorDef {
    /// The binding name (e.g. `payload`).
    pub binding: String,
    /// The extractor type (e.g. `Json<CreateUserRequest>`).
    pub extractor_type: String,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for emit_handler_fn.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitHandlerParams {
    /// Handler function name.
    pub name: String,
    /// Extractor bindings in parameter order.
    pub extractors: Vec<ExtractorDef>,
    /// Return type expression (e.g. `impl IntoResponse`).
    pub return_type: String,
    /// Function body code.
    pub body: String,
}

/// Parameters for emit_handler_with_state.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitHandlerWithStateParams {
    /// Handler function name.
    pub name: String,
    /// Application state type (e.g. `AppState`).
    pub state_type: String,
    /// Additional extractor bindings after the state parameter.
    pub extractors: Vec<ExtractorDef>,
    /// Return type expression.
    pub return_type: String,
    /// Function body code.
    pub body: String,
}

/// Parameters for emit_handler_with_db.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitHandlerWithDbParams {
    /// Handler function name.
    pub name: String,
    /// Database pool type (e.g. `PgPool`).
    pub db_type: String,
    /// Description of the query operation for the comment.
    pub query_description: String,
}

/// Parameters for emit_crud_handler_set.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitCrudHandlerParams {
    /// Resource name in singular lowercase (e.g. `user`).
    pub resource: String,
    /// Application state type holding the database pool.
    pub state_type: String,
    /// Type of the resource ID (e.g. `i64` or `uuid::Uuid`).
    pub id_type: String,
}

/// Parameters for emit_handler_error_type.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitHandlerErrorParams {
    /// Error enum name.
    pub name: String,
    /// Variant names for the error enum.
    pub variants: Vec<String>,
}

/// Parameters for emit_handler_with_auth.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitHandlerWithAuthParams {
    /// Handler function name.
    pub name: String,
    /// TypedHeader type for the authorization header.
    pub auth_header: String,
    /// Return type expression.
    pub return_type: String,
    /// Function body executed after successful auth extraction.
    pub body: String,
}

/// Parameters for emit_handler_websocket.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitWebSocketHandlerParams {
    /// Handler function name.
    pub name: String,
    /// Body code run inside the on_message callback.
    pub on_message_body: String,
}

/// Parameters for emit_handler_sse_stream.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitSseHandlerParams {
    /// Handler function name.
    pub name: String,
    /// The SSE event data type (e.g. `String`).
    pub event_type: String,
    /// Body of the async stream generator.
    pub stream_body: String,
}

/// Parameters for emit_handler_file_upload.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitFileUploadParams {
    /// Handler function name.
    pub name: String,
    /// Directory path for saving uploaded files.
    pub upload_dir: String,
}

/// Parameters for emit_handler_streaming_response.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitStreamingParams {
    /// Handler function name.
    pub name: String,
    /// The chunk data type for the body stream.
    pub chunk_type: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_handler_gen",
    emit = None,
    name = "emit_handler_fn",
    description = "Emit a basic async handler function with the given extractors and body."
)]
#[instrument]
async fn emit_handler_fn(p: EmitHandlerParams) -> Result<CallToolResult, ErrorData> {
    let extractor_list = p
        .extractors
        .iter()
        .map(|e| format!("{}: {}", e.binding, e.extractor_type))
        .collect::<Vec<_>>()
        .join(", ");
    let code = format!(
        "pub async fn {}({}) -> {} {{\n    {}\n}}",
        p.name, extractor_list, p.return_type, p.body
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_handler_gen",
    emit = None,
    name = "emit_handler_with_state",
    description = "Emit a handler function with State<T> as the first extractor parameter."
)]
#[instrument]
async fn emit_handler_with_state(
    p: EmitHandlerWithStateParams,
) -> Result<CallToolResult, ErrorData> {
    let extra = if p.extractors.is_empty() {
        String::new()
    } else {
        let list = p
            .extractors
            .iter()
            .map(|e| format!("\n    {}: {}", e.binding, e.extractor_type))
            .collect::<Vec<_>>()
            .join(",");
        format!(",{}", list)
    };
    let code = format!(
        "pub async fn {}(\n    State(state): State<{}>{},\n) -> {} {{\n    {}\n}}",
        p.name, p.state_type, extra, p.return_type, p.body
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_handler_gen",
    emit = None,
    name = "emit_handler_with_db",
    description = "Emit a handler that receives a database pool via State and executes a query."
)]
#[instrument]
async fn emit_handler_with_db(p: EmitHandlerWithDbParams) -> Result<CallToolResult, ErrorData> {
    let mut code = String::new();
    code.push_str(&format!(
        "pub async fn {}(\n    State(db): State<{}>,\n) -> impl IntoResponse {{\n",
        p.name, p.db_type
    ));
    code.push_str(&format!("    // {}\n", p.query_description));
    code.push_str("    match sqlx::query(\"SELECT 1\").fetch_optional(&db).await {\n");
    code.push_str("        Ok(_) => StatusCode::OK,\n");
    code.push_str("        Err(e) => {\n");
    code.push_str("            tracing::error!(error = ?e, \"Database query failed\");\n");
    code.push_str("            StatusCode::INTERNAL_SERVER_ERROR\n");
    code.push_str("        }\n");
    code.push_str("    }\n}");
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_handler_gen",
    emit = None,
    name = "emit_crud_handler_set",
    description = "Emit a complete set of five CRUD handlers (list, create, get, update, delete) for a resource."
)]
#[instrument]
async fn emit_crud_handler_set(p: EmitCrudHandlerParams) -> Result<CallToolResult, ErrorData> {
    let resource = &p.resource;
    let state = &p.state_type;
    let id_type = &p.id_type;
    let code = format!(
        r#"pub async fn list_{resource}s(
    State(_state): State<{state}>,
) -> impl IntoResponse {{
    // TODO: query all {resource}s
    StatusCode::OK
}}

pub async fn create_{resource}(
    State(_state): State<{state}>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {{
    // TODO: insert {resource} from payload
    let _ = payload;
    StatusCode::CREATED
}}

pub async fn get_{resource}(
    State(_state): State<{state}>,
    Path(id): Path<{id_type}>,
) -> impl IntoResponse {{
    // TODO: fetch {resource} by id
    let _ = id;
    StatusCode::OK
}}

pub async fn update_{resource}(
    State(_state): State<{state}>,
    Path(id): Path<{id_type}>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {{
    // TODO: update {resource} by id
    let _ = (id, payload);
    StatusCode::OK
}}

pub async fn delete_{resource}(
    State(_state): State<{state}>,
    Path(id): Path<{id_type}>,
) -> impl IntoResponse {{
    // TODO: delete {resource} by id
    let _ = id;
    StatusCode::NO_CONTENT
}}"#
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_handler_gen",
    emit = None,
    name = "emit_handler_error_type",
    description = "Emit an error enum with IntoResponse mapping each variant to a 4xx/5xx status."
)]
#[instrument]
async fn emit_handler_error_type(p: EmitHandlerErrorParams) -> Result<CallToolResult, ErrorData> {
    let status_for = |v: &str| -> &'static str {
        let lower = v.to_lowercase();
        if lower.contains("notfound") || lower.contains("not_found") {
            "NOT_FOUND"
        } else if lower.contains("unauthorized") || lower.contains("unauth") {
            "UNAUTHORIZED"
        } else if lower.contains("forbidden") {
            "FORBIDDEN"
        } else if lower.contains("bad") || lower.contains("invalid") {
            "BAD_REQUEST"
        } else if lower.contains("conflict") {
            "CONFLICT"
        } else if lower.contains("timeout") {
            "REQUEST_TIMEOUT"
        } else {
            "INTERNAL_SERVER_ERROR"
        }
    };

    let variants_def = p
        .variants
        .iter()
        .map(|v| format!("    {},", v))
        .collect::<Vec<_>>()
        .join("\n");

    let match_arms = p
        .variants
        .iter()
        .map(|v| {
            let status = status_for(v);
            format!(
                "            {}::{} => (StatusCode::{}, \"{}\").into_response(),",
                p.name, v, status, v
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let code = format!(
        "#[derive(Debug)]\npub enum {} {{\n{}\n}}\n\nimpl IntoResponse for {} {{\n    fn into_response(self) -> Response {{\n        match self {{\n{}\n        }}\n    }}\n}}",
        p.name, variants_def, p.name, match_arms
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_handler_gen",
    emit = None,
    name = "emit_handler_with_auth",
    description = "Emit a handler that validates a TypedHeader authorization before executing."
)]
#[instrument]
async fn emit_handler_with_auth(p: EmitHandlerWithAuthParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        "pub async fn {}(\n    TypedHeader(auth): TypedHeader<{}>,\n) -> {} {{\n    // Authorization header validated by extractor\n    let _ = auth;\n    {}\n}}",
        p.name, p.auth_header, p.return_type, p.body
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_handler_gen",
    emit = None,
    name = "emit_handler_websocket",
    description = "Emit a WebSocket upgrade handler."
)]
#[instrument]
async fn emit_handler_websocket(
    p: EmitWebSocketHandlerParams,
) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"pub async fn {}(ws: WebSocketUpgrade) -> impl IntoResponse {{
    ws.on_upgrade(|mut socket| async move {{
        while let Some(Ok(msg)) = socket.recv().await {{
            {}
        }}
    }})
}}"#,
        p.name, p.on_message_body
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_handler_gen",
    emit = None,
    name = "emit_handler_sse_stream",
    description = "Emit a Server-Sent Events stream handler."
)]
#[instrument]
async fn emit_handler_sse_stream(p: EmitSseHandlerParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"pub async fn {}() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {{
    // Produces events of type: {event_type}
    let stream = async_stream::stream! {{
        {stream_body}
    }};
    Sse::new(stream)
}}"#,
        p.name,
        event_type = p.event_type,
        stream_body = p.stream_body
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_handler_gen",
    emit = None,
    name = "emit_handler_file_upload",
    description = "Emit a multipart file upload handler."
)]
#[instrument]
async fn emit_handler_file_upload(p: EmitFileUploadParams) -> Result<CallToolResult, ErrorData> {
    let upload_dir = &p.upload_dir;
    let code = format!(
        r#"pub async fn {}(mut multipart: Multipart) -> impl IntoResponse {{
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {{
        let name = field.file_name().unwrap_or("upload").to_string();
        let data = field.bytes().await.unwrap_or_default();
        let path = std::path::Path::new("{upload_dir}").join(&name);
        if let Err(e) = tokio::fs::write(&path, &data).await {{
            tracing::error!(error = ?e, "Failed to write upload");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }}
    }}
    StatusCode::CREATED
}}"#,
        p.name
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_handler_gen",
    emit = None,
    name = "emit_handler_streaming_response",
    description = "Emit a streaming body response handler."
)]
#[instrument]
async fn emit_handler_streaming_response(
    p: EmitStreamingParams,
) -> Result<CallToolResult, ErrorData> {
    let chunk_type = &p.chunk_type;
    let code = format!(
        r#"pub async fn {}() -> impl IntoResponse {{
    let stream = async_stream::stream! {{
        // yield {chunk_type} chunks
        let chunk: {chunk_type} = todo!("produce chunk");
        yield Ok::<_, std::convert::Infallible>(
            bytes::Bytes::from(format!("{{:?}}", chunk))
        );
    }};
    axum::body::Body::from_stream(stream)
}}"#,
        p.name
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

/// Plugin exposing axum handler generation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_handler_gen")]
pub struct AxumHandlerGenPlugin;
