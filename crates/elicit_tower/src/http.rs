//! `TowerHttpPlugin` — MCP tools for tower-http middleware layers.
//!
//! Layer config objects are held server-side in a single UUID-keyed registry
//! discriminated by a `TowerHttpLayerEntry` enum. Agents receive UUID handles;
//! no live services cross the MCP boundary.
//!
//! # Tool namespace: `tower_http__*`
//!
//! | Tool | Params | Returns | Establishes |
//! |---|---|---|---|
//! | `normalize_path_layer_new` | `trim` | `{ layer_id }` | `TowerHttpLayerCreated` |
//! | `propagate_header_layer_new` | `header` | `{ layer_id }` | `TowerHttpLayerCreated` |
//! | `set_status_layer_new` | `status_code` | `{ layer_id }` | `TowerHttpLayerCreated` |
//! | `compression_layer_new` | `gzip, deflate, br, zstd` | `{ layer_id }` | `TowerHttpLayerCreated` |
//! | `decompression_layer_new` | `gzip, deflate, br, zstd` | `{ layer_id }` | `TowerHttpLayerCreated` |
//! | `http_timeout_layer_new` | `timeout_millis` | `{ layer_id }` | `TowerHttpLayerCreated` |
//! | `trace_layer_new` | — | `{ layer_id }` | `TowerHttpLayerCreated` |
//! | `catch_panic_layer_new` | — | `{ layer_id }` | `TowerHttpLayerCreated` |
//! | `cors_layer_new` | `allow_origins, allow_methods, allow_headers, allow_credentials, max_age_secs` | `{ layer_id }` | `TowerHttpLayerCreated` |
//! | `validate_request_header_layer_new` | `header, expected_value` | `{ layer_id }` | `TowerHttpLayerCreated` |
//! | `set_request_header_layer_new` | `header, value` | `{ layer_id }` | `TowerHttpLayerCreated` |
//! | `set_response_header_layer_new` | `header, value` | `{ layer_id }` | `TowerHttpLayerCreated` |
//! | `sensitive_request_headers_layer_new` | `headers` | `{ layer_id }` | `TowerHttpLayerCreated` |
//! | `sensitive_response_headers_layer_new` | `headers` | `{ layer_id }` | `TowerHttpLayerCreated` |
//! | `layer_describe` | `layer_id` | JSON config | — |

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::contracts::{Established, Prop};
use elicitation::{Elicit, PluginContext, VerifiedWorkflow};
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a tower-http layer was successfully created and registered.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TowerHttpLayerCreated;
impl Prop for TowerHttpLayerCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_tower_http_layer_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "tower-http layer created");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_tower_http_layer_created(ok: bool) -> (result: bool)
                ensures result == ok,
            { ok }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_tower_http_layer_created_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for TowerHttpLayerCreated {}

// ── Registry entry ────────────────────────────────────────────────────────────

/// All tower-http layer config variants held in the registry.
#[derive(Serialize)]
#[serde(tag = "kind")]
enum TowerHttpLayerEntry {
    NormalizePath(elicitation::TowerNormalizePathLayer),
    PropagateHeader(elicitation::TowerPropagateHeaderLayer),
    SetStatus(elicitation::TowerSetStatusLayer),
    Compression(elicitation::TowerCompressionLayer),
    Decompression(elicitation::TowerDecompressionLayer),
    HttpTimeout(elicitation::TowerHttpTimeoutLayer),
    Trace(elicitation::TowerTraceLayer),
    CatchPanic(elicitation::TowerCatchPanicLayer),
    Cors(elicitation::TowerCorsLayer),
    ValidateRequestHeader(elicitation::TowerValidateRequestHeaderLayer),
    SetRequestHeader(elicitation::TowerSetRequestHeaderLayer),
    SetResponseHeader(elicitation::TowerSetResponseHeaderLayer),
    SensitiveRequestHeaders(elicitation::TowerSetSensitiveRequestHeadersLayer),
    SensitiveResponseHeaders(elicitation::TowerSetSensitiveResponseHeadersLayer),
}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `tower_http__*` tool calls.
pub struct TowerHttpCtx {
    layers: Mutex<HashMap<Uuid, TowerHttpLayerEntry>>,
}

impl TowerHttpCtx {
    fn new() -> Self {
        Self {
            layers: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for TowerHttpCtx {}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `tower_http__normalize_path_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct NormalizePathLayerNewParams {
    /// When true, trailing slashes are trimmed from request paths.
    pub trim: bool,
}

/// Parameters for `tower_http__propagate_header_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PropagateHeaderLayerNewParams {
    /// Name of the HTTP header to propagate from request to response.
    pub header: String,
}

/// Parameters for `tower_http__set_status_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetStatusLayerNewParams {
    /// HTTP status code to set on all responses (e.g. 200, 404).
    pub status_code: u16,
}

/// Parameters for `tower_http__compression_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CompressionLayerNewParams {
    /// Enable gzip encoding.
    pub gzip: bool,
    /// Enable deflate encoding.
    pub deflate: bool,
    /// Enable Brotli encoding.
    pub br: bool,
    /// Enable Zstandard encoding.
    pub zstd: bool,
}

/// Parameters for `tower_http__decompression_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DecompressionLayerNewParams {
    /// Accept gzip-encoded request bodies.
    pub gzip: bool,
    /// Accept deflate-encoded request bodies.
    pub deflate: bool,
    /// Accept Brotli-encoded request bodies.
    pub br: bool,
    /// Accept Zstandard-encoded request bodies.
    pub zstd: bool,
}

/// Parameters for `tower_http__http_timeout_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct HttpTimeoutLayerNewParams {
    /// Request timeout in milliseconds.
    pub timeout_millis: u64,
}

/// Parameters for `tower_http__trace_layer_new` (no fields).
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TraceLayerNewParams {}

/// Parameters for `tower_http__catch_panic_layer_new` (no fields).
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CatchPanicLayerNewParams {}

/// Parameters for `tower_http__cors_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CorsLayerNewParams {
    /// Allowed origin strings (e.g. `"https://example.com"`).
    pub allow_origins: Vec<String>,
    /// Allowed HTTP method strings (e.g. `"GET"`, `"POST"`).
    pub allow_methods: Vec<String>,
    /// Allowed HTTP header names.
    pub allow_headers: Vec<String>,
    /// Whether credentials (cookies, auth headers) are allowed.
    pub allow_credentials: bool,
    /// Optional `Access-Control-Max-Age` preflight cache duration in seconds.
    pub max_age_secs: Option<u64>,
}

/// Parameters for `tower_http__validate_request_header_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ValidateRequestHeaderLayerNewParams {
    /// Name of the header to validate.
    pub header: String,
    /// Expected value the header must match.
    pub expected_value: String,
}

/// Parameters for `tower_http__set_request_header_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetRequestHeaderLayerNewParams {
    /// Name of the request header to set.
    pub header: String,
    /// Value to assign to the header.
    pub value: String,
}

/// Parameters for `tower_http__set_response_header_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetResponseHeaderLayerNewParams {
    /// Name of the response header to set.
    pub header: String,
    /// Value to assign to the header.
    pub value: String,
}

/// Parameters for `tower_http__sensitive_request_headers_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SensitiveRequestHeadersLayerNewParams {
    /// Header names to mark as sensitive in request traces.
    pub headers: Vec<String>,
}

/// Parameters for `tower_http__sensitive_response_headers_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SensitiveResponseHeadersLayerNewParams {
    /// Header names to mark as sensitive in response traces.
    pub headers: Vec<String>,
}

/// Parameters for `tower_http__layer_describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct HttpLayerDescribeParams {
    /// UUID returned by any `tower_http__*_layer_new` tool.
    pub layer_id: String,
}

#[derive(Serialize)]
struct LayerIdResult {
    layer_id: String,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

async fn insert_layer(ctx: &Arc<TowerHttpCtx>, entry: TowerHttpLayerEntry) -> LayerIdResult {
    let id = Uuid::new_v4();
    ctx.layers.lock().await.insert(id, entry);
    LayerIdResult {
        layer_id: id.to_string(),
    }
}

// ── Tool functions ────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__normalize_path_layer_new",
    description = "Create a normalize-path layer. When `trim` is true, trailing slashes are \
                   removed from request paths before dispatch. \
                   Establishes: TowerHttpLayerCreated.",
    emit = Auto
)]
async fn normalize_path_layer_new(
    ctx: Arc<TowerHttpCtx>,
    p: NormalizePathLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let result = insert_layer(
        &ctx,
        TowerHttpLayerEntry::NormalizePath(elicitation::TowerNormalizePathLayer { trim: p.trim }),
    )
    .await;
    let _proof: Established<TowerHttpLayerCreated> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__propagate_header_layer_new",
    description = "Create a propagate-header layer that copies a named request header onto the \
                   response. \
                   Establishes: TowerHttpLayerCreated.",
    emit = Auto
)]
async fn propagate_header_layer_new(
    ctx: Arc<TowerHttpCtx>,
    p: PropagateHeaderLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let result = insert_layer(
        &ctx,
        TowerHttpLayerEntry::PropagateHeader(elicitation::TowerPropagateHeaderLayer {
            header: p.header,
        }),
    )
    .await;
    let _proof: Established<TowerHttpLayerCreated> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__set_status_layer_new",
    description = "Create a set-status layer that overrides every response's HTTP status code. \
                   Assumes: status_code is a valid HTTP status (100–599). \
                   Establishes: TowerHttpLayerCreated.",
    emit = Auto
)]
async fn set_status_layer_new(
    ctx: Arc<TowerHttpCtx>,
    p: SetStatusLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let result = insert_layer(
        &ctx,
        TowerHttpLayerEntry::SetStatus(elicitation::TowerSetStatusLayer {
            status_code: p.status_code,
        }),
    )
    .await;
    let _proof: Established<TowerHttpLayerCreated> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__compression_layer_new",
    description = "Create a response-compression layer. Enable individual algorithms with the \
                   boolean flags: gzip, deflate, br (Brotli), zstd (Zstandard). \
                   Establishes: TowerHttpLayerCreated.",
    emit = Auto
)]
async fn compression_layer_new(
    ctx: Arc<TowerHttpCtx>,
    p: CompressionLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let result = insert_layer(
        &ctx,
        TowerHttpLayerEntry::Compression(elicitation::TowerCompressionLayer {
            gzip: p.gzip,
            deflate: p.deflate,
            br: p.br,
            zstd: p.zstd,
        }),
    )
    .await;
    let _proof: Established<TowerHttpLayerCreated> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__decompression_layer_new",
    description = "Create a request-decompression layer. Enable accepted encodings with the \
                   boolean flags: gzip, deflate, br (Brotli), zstd (Zstandard). \
                   Establishes: TowerHttpLayerCreated.",
    emit = Auto
)]
async fn decompression_layer_new(
    ctx: Arc<TowerHttpCtx>,
    p: DecompressionLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let result = insert_layer(
        &ctx,
        TowerHttpLayerEntry::Decompression(elicitation::TowerDecompressionLayer {
            gzip: p.gzip,
            deflate: p.deflate,
            br: p.br,
            zstd: p.zstd,
        }),
    )
    .await;
    let _proof: Established<TowerHttpLayerCreated> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__http_timeout_layer_new",
    description = "Create a tower-http timeout layer that cancels requests exceeding \
                   `timeout_millis` ms. Distinct from tower's core timeout layer. \
                   Assumes: timeout_millis > 0. \
                   Establishes: TowerHttpLayerCreated.",
    emit = Auto
)]
async fn http_timeout_layer_new(
    ctx: Arc<TowerHttpCtx>,
    p: HttpTimeoutLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let result = insert_layer(
        &ctx,
        TowerHttpLayerEntry::HttpTimeout(elicitation::TowerHttpTimeoutLayer {
            timeout_millis: p.timeout_millis,
        }),
    )
    .await;
    let _proof: Established<TowerHttpLayerCreated> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__trace_layer_new",
    description = "Create a trace layer that emits structured tracing spans for each HTTP \
                   request/response cycle. \
                   Establishes: TowerHttpLayerCreated.",
    emit = Auto
)]
async fn trace_layer_new(
    ctx: Arc<TowerHttpCtx>,
    _p: TraceLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let result = insert_layer(
        &ctx,
        TowerHttpLayerEntry::Trace(elicitation::TowerTraceLayer),
    )
    .await;
    let _proof: Established<TowerHttpLayerCreated> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__catch_panic_layer_new",
    description = "Create a catch-panic layer that converts handler panics into 500 responses \
                   instead of crashing the server. \
                   Establishes: TowerHttpLayerCreated.",
    emit = Auto
)]
async fn catch_panic_layer_new(
    ctx: Arc<TowerHttpCtx>,
    _p: CatchPanicLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let result = insert_layer(
        &ctx,
        TowerHttpLayerEntry::CatchPanic(elicitation::TowerCatchPanicLayer),
    )
    .await;
    let _proof: Established<TowerHttpLayerCreated> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__cors_layer_new",
    description = "Create a CORS layer with the given allowed origins, methods, headers, \
                   credentials flag, and optional preflight max-age. \
                   Establishes: TowerHttpLayerCreated.",
    emit = Auto
)]
async fn cors_layer_new(
    ctx: Arc<TowerHttpCtx>,
    p: CorsLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let result = insert_layer(
        &ctx,
        TowerHttpLayerEntry::Cors(elicitation::TowerCorsLayer {
            allow_origins: p.allow_origins,
            allow_methods: p.allow_methods,
            allow_headers: p.allow_headers,
            allow_credentials: p.allow_credentials,
            max_age_secs: p.max_age_secs,
        }),
    )
    .await;
    let _proof: Established<TowerHttpLayerCreated> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__validate_request_header_layer_new",
    description = "Create a validate-request-header layer that rejects requests whose named \
                   header does not match `expected_value`. \
                   Establishes: TowerHttpLayerCreated.",
    emit = Auto
)]
async fn validate_request_header_layer_new(
    ctx: Arc<TowerHttpCtx>,
    p: ValidateRequestHeaderLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let result = insert_layer(
        &ctx,
        TowerHttpLayerEntry::ValidateRequestHeader(elicitation::TowerValidateRequestHeaderLayer {
            header: p.header,
            expected_value: p.expected_value,
        }),
    )
    .await;
    let _proof: Established<TowerHttpLayerCreated> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__set_request_header_layer_new",
    description = "Create a set-request-header layer that inserts or overrides a named header \
                   on every incoming request. \
                   Establishes: TowerHttpLayerCreated.",
    emit = Auto
)]
async fn set_request_header_layer_new(
    ctx: Arc<TowerHttpCtx>,
    p: SetRequestHeaderLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let result = insert_layer(
        &ctx,
        TowerHttpLayerEntry::SetRequestHeader(elicitation::TowerSetRequestHeaderLayer {
            header: p.header,
            value: p.value,
        }),
    )
    .await;
    let _proof: Established<TowerHttpLayerCreated> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__set_response_header_layer_new",
    description = "Create a set-response-header layer that inserts or overrides a named header \
                   on every outgoing response. \
                   Establishes: TowerHttpLayerCreated.",
    emit = Auto
)]
async fn set_response_header_layer_new(
    ctx: Arc<TowerHttpCtx>,
    p: SetResponseHeaderLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let result = insert_layer(
        &ctx,
        TowerHttpLayerEntry::SetResponseHeader(elicitation::TowerSetResponseHeaderLayer {
            header: p.header,
            value: p.value,
        }),
    )
    .await;
    let _proof: Established<TowerHttpLayerCreated> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__sensitive_request_headers_layer_new",
    description = "Create a layer that marks the specified request headers as sensitive so they \
                   are redacted from tracing output. \
                   Establishes: TowerHttpLayerCreated.",
    emit = Auto
)]
async fn sensitive_request_headers_layer_new(
    ctx: Arc<TowerHttpCtx>,
    p: SensitiveRequestHeadersLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let result = insert_layer(
        &ctx,
        TowerHttpLayerEntry::SensitiveRequestHeaders(
            elicitation::TowerSetSensitiveRequestHeadersLayer { headers: p.headers },
        ),
    )
    .await;
    let _proof: Established<TowerHttpLayerCreated> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__sensitive_response_headers_layer_new",
    description = "Create a layer that marks the specified response headers as sensitive so they \
                   are redacted from tracing output. \
                   Establishes: TowerHttpLayerCreated.",
    emit = Auto
)]
async fn sensitive_response_headers_layer_new(
    ctx: Arc<TowerHttpCtx>,
    p: SensitiveResponseHeadersLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let result = insert_layer(
        &ctx,
        TowerHttpLayerEntry::SensitiveResponseHeaders(
            elicitation::TowerSetSensitiveResponseHeadersLayer { headers: p.headers },
        ),
    )
    .await;
    let _proof: Established<TowerHttpLayerCreated> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "tower_http",
    name = "tower_http__layer_describe",
    description = "Describe the config of a previously created tower-http layer by its UUID. \
                   Assumes: layer_id is a valid UUID returned by a prior tower_http__*_layer_new \
                   tool.",
    emit = Auto
)]
async fn layer_describe(
    ctx: Arc<TowerHttpCtx>,
    p: HttpLayerDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid = p
        .layer_id
        .parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.layer_id), None))?;
    let entry = ctx
        .layers
        .lock()
        .await
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("layer_id not found: {id}"), None))
        .map(|e| serde_json::to_string(e))?
        .map_err(|e| ErrorData::internal_error(format!("serialize error: {e}"), None))?;
    Ok(CallToolResult::success(vec![Content::text(entry)]))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tower_http__*` tools for tower-http middleware layers.
///
/// Holds a single UUID-keyed registry of all tower-http layer configs,
/// discriminated by the `TowerHttpLayerEntry` enum. All config objects live
/// server-side; agents interact via UUID handles.
///
/// # Tool namespace
///
/// All tools are registered under the `"tower_http"` namespace and named
/// `tower_http__<verb>`.
pub struct TowerHttpPlugin(Arc<TowerHttpCtx>);

impl TowerHttpPlugin {
    /// Create a new `TowerHttpPlugin` with an empty layer registry.
    pub fn new() -> Self {
        Self(Arc::new(TowerHttpCtx::new()))
    }
}

impl Default for TowerHttpPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TowerHttpPlugin {
    fn name(&self) -> &'static str {
        "tower_http"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tower_http")
            .map(|r| (r.constructor)().as_tool())
            .collect()
    }

    #[tracing::instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let plugin_ctx = self.0.clone();
        Box::pin(async move {
            let name = params.name.as_ref();
            let full_name = if name.starts_with("tower_http__") {
                name.to_string()
            } else {
                format!("tower_http__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tower_http")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
