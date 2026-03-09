//! MCP plugin exposing reqwest HTTP tools via the `ElicitPlugin` interface.
//!
//! Register with a [`PluginRegistry`][elicitation::PluginRegistry] to expose
//! six HTTP method tools (`get`, `post`, `put`, `delete`, `patch`, `head`)
//! namespaced as `http__get`, `http__post`, etc.

use std::sync::Arc;
use std::time::Duration;

use elicitation::{ElicitPlugin, PluginContext, elicit_tool};
use elicitation::{F64Positive, UrlValid};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

/// MCP plugin for reqwest HTTP operations.
///
/// Registers itself under the `"http"` namespace, exposing six HTTP method
/// tools: `get`, `post`, `put`, `delete`, `patch`, and `head`.
///
/// Each tool performs a complete HTTP round-trip and returns a JSON object
/// with `status`, `url`, and (where applicable) `body` fields.
///
/// # Example
///
/// ```rust,no_run
/// use elicitation::PluginRegistry;
/// use elicit_reqwest::Plugin;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let registry = PluginRegistry::new()
///     .register("http", Plugin::new());
/// // registry.serve(rmcp::transport::stdio()).await?;
/// # Ok(())
/// # }
/// ```
#[derive(ElicitPlugin)]
#[plugin(name = "http")]
pub struct Plugin(pub Arc<PluginContext>);

impl Plugin {
    /// Create a plugin wrapping a new default HTTP client.
    pub fn new() -> Self {
        Self(PluginContext::new())
    }

    /// Create a plugin wrapping a pre-configured [`reqwest::Client`].
    pub fn with_client(client: reqwest::Client) -> Self {
        Self(Arc::new(PluginContext { http: client }))
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}

/// Parameters shared across all HTTP method tools.
#[derive(Debug, Deserialize, JsonSchema)]
struct HttpParams {
    /// Destination URL.
    url: UrlValid,

    /// Request body string (used by `post`, `put`, `patch`).
    body: Option<String>,

    /// `Content-Type` header value (e.g., `"application/json"`).
    content_type: Option<String>,

    /// Request timeout in seconds (must be > 0).
    timeout_secs: Option<F64Positive>,

    /// Bearer token for `Authorization: Bearer <token>` header.
    bearer_token: Option<String>,

    /// Additional headers as `"Name: Value"` strings (colon-separated).
    headers: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct HttpPostParams {
    url: UrlValid,
    body: Option<String>,
    content_type: Option<String>,
    timeout_secs: Option<F64Positive>,
    bearer_token: Option<String>,
    headers: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct HttpPutParams {
    url: UrlValid,
    body: Option<String>,
    content_type: Option<String>,
    timeout_secs: Option<F64Positive>,
    bearer_token: Option<String>,
    headers: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct HttpDeleteParams {
    url: UrlValid,
    body: Option<String>,
    content_type: Option<String>,
    timeout_secs: Option<F64Positive>,
    bearer_token: Option<String>,
    headers: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct HttpPatchParams {
    url: UrlValid,
    body: Option<String>,
    content_type: Option<String>,
    timeout_secs: Option<F64Positive>,
    bearer_token: Option<String>,
    headers: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct HttpHeadParams {
    url: UrlValid,
    body: Option<String>,
    content_type: Option<String>,
    timeout_secs: Option<F64Positive>,
    bearer_token: Option<String>,
    headers: Option<Vec<String>>,
}

/// Apply common HTTP options to a request builder.
///
/// Works on any params type that has `timeout_secs`, `bearer_token`,
/// `content_type`, `headers`, and `body` fields.
macro_rules! apply_http_opts {
    ($builder:expr, $p:expr) => {{
        let mut builder = $builder;
        if let Some(t) = $p.timeout_secs {
            builder = builder.timeout(::std::time::Duration::from_secs_f64(t.get()));
        }
        if let Some(token) = &$p.bearer_token {
            builder = builder.bearer_auth(token);
        }
        if let Some(ct) = &$p.content_type {
            builder = builder.header(reqwest::header::CONTENT_TYPE, ct.as_str());
        }
        for h in $p.headers.iter().flatten() {
            if let Some((k, v)) = h.split_once(':') {
                builder = builder.header(k.trim(), v.trim());
            }
        }
        if let Some(body) = &$p.body {
            builder = builder.body(body.clone());
        }
        builder
    }};
}

/// Apply common [`HttpParams`] options to a request builder.
fn apply_options(mut builder: reqwest::RequestBuilder, p: &HttpParams) -> reqwest::RequestBuilder {
    if let Some(t) = p.timeout_secs {
        builder = builder.timeout(Duration::from_secs_f64(t.get()));
    }
    if let Some(token) = &p.bearer_token {
        builder = builder.bearer_auth(token);
    }
    if let Some(ct) = &p.content_type {
        builder = builder.header(reqwest::header::CONTENT_TYPE, ct.as_str());
    }
    for h in p.headers.iter().flatten() {
        if let Some((k, v)) = h.split_once(':') {
            builder = builder.header(k.trim(), v.trim());
        }
    }
    if let Some(body) = &p.body {
        builder = builder.body(body.clone());
    }
    builder
}

/// Send a request and return `{ status, url, body }` as a text result.
async fn execute(builder: reqwest::RequestBuilder) -> Result<CallToolResult, ErrorData> {
    match builder.send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let url = resp.url().to_string();
            let body = resp.text().await.unwrap_or_default();
            let json = serde_json::json!({ "status": status, "url": url, "body": body });
            Ok(CallToolResult::success(vec![Content::text(
                json.to_string(),
            )]))
        }
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

/// Send a HEAD request and return `{ status, url }` (no body).
async fn execute_head(builder: reqwest::RequestBuilder) -> Result<CallToolResult, ErrorData> {
    match builder.send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let url = resp.url().to_string();
            let json = serde_json::json!({ "status": status, "url": url });
            Ok(CallToolResult::success(vec![Content::text(
                json.to_string(),
            )]))
        }
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

// ── Tool handlers ─────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "http",
    name = "get",
    description = "Send an HTTP GET request; returns status, URL, and response body."
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn http_get(ctx: Arc<PluginContext>, p: HttpParams) -> Result<CallToolResult, ErrorData> {
    let builder = ctx.http.get(p.url.get().as_str());
    execute(apply_options(builder, &p)).await
}

#[elicit_tool(
    plugin = "http",
    name = "post",
    description = "Send an HTTP POST request with optional body; returns status, URL, and response body."
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn http_post(
    ctx: Arc<PluginContext>,
    p: HttpPostParams,
) -> Result<CallToolResult, ErrorData> {
    let builder = ctx.http.post(p.url.get().as_str());
    execute(apply_http_opts!(builder, p)).await
}

#[elicit_tool(
    plugin = "http",
    name = "put",
    description = "Send an HTTP PUT request with optional body; returns status, URL, and response body."
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn http_put(ctx: Arc<PluginContext>, p: HttpPutParams) -> Result<CallToolResult, ErrorData> {
    let builder = ctx.http.put(p.url.get().as_str());
    execute(apply_http_opts!(builder, p)).await
}

#[elicit_tool(
    plugin = "http",
    name = "delete",
    description = "Send an HTTP DELETE request; returns status, URL, and response body."
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn http_delete(
    ctx: Arc<PluginContext>,
    p: HttpDeleteParams,
) -> Result<CallToolResult, ErrorData> {
    let builder = ctx.http.delete(p.url.get().as_str());
    execute(apply_http_opts!(builder, p)).await
}

#[elicit_tool(
    plugin = "http",
    name = "patch",
    description = "Send an HTTP PATCH request with optional body; returns status, URL, and response body."
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn http_patch(
    ctx: Arc<PluginContext>,
    p: HttpPatchParams,
) -> Result<CallToolResult, ErrorData> {
    let builder = ctx.http.patch(p.url.get().as_str());
    execute(apply_http_opts!(builder, p)).await
}

#[elicit_tool(
    plugin = "http",
    name = "head",
    description = "Send an HTTP HEAD request; returns status and URL only (no body)."
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn http_head(
    ctx: Arc<PluginContext>,
    p: HttpHeadParams,
) -> Result<CallToolResult, ErrorData> {
    let builder = ctx.http.head(p.url.get().as_str());
    execute_head(apply_http_opts!(builder, p)).await
}
