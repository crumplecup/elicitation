//! MCP plugin exposing reqwest HTTP tools via the `ElicitPlugin` interface.
//!
//! Register with a [`PluginRegistry`][elicitation::PluginRegistry] to expose
//! six HTTP method tools (`get`, `post`, `put`, `delete`, `patch`, `head`)
//! namespaced as `http__get`, `http__post`, etc.

use std::sync::Arc;
use std::time::Duration;

use elicitation::ElicitPlugin;
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
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
pub struct Plugin {
    client: Arc<reqwest::Client>,
}

impl Plugin {
    /// Create a plugin wrapping a new default HTTP client.
    pub fn new() -> Self {
        Self {
            client: Arc::new(reqwest::Client::new()),
        }
    }

    /// Create a plugin wrapping a pre-configured [`reqwest::Client`].
    pub fn with_client(client: reqwest::Client) -> Self {
        Self {
            client: Arc::new(client),
        }
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
    url: String,

    /// Request body string (used by `post`, `put`, `patch`).
    body: Option<String>,

    /// `Content-Type` header value (e.g., `"application/json"`).
    content_type: Option<String>,

    /// Request timeout in seconds.
    timeout_secs: Option<f64>,

    /// Bearer token for `Authorization: Bearer <token>` header.
    bearer_token: Option<String>,

    /// Additional headers as `"Name: Value"` strings (colon-separated).
    headers: Option<Vec<String>>,
}

/// Build a [`Tool`] with a typed input schema.
fn typed_tool<T: JsonSchema + 'static>(name: &'static str, description: &'static str) -> Tool {
    Tool::new(name, description, Arc::new(Default::default())).with_input_schema::<T>()
}

/// Deserialize tool arguments from the call params.
fn parse_args<T: for<'de> Deserialize<'de>>(
    params: &CallToolRequestParams,
) -> Result<T, ErrorData> {
    let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
    serde_json::from_value(value).map_err(|e| ErrorData::invalid_params(e.to_string(), None))
}

/// Apply common [`HttpParams`] options to a request builder.
fn apply_options(mut builder: reqwest::RequestBuilder, p: &HttpParams) -> reqwest::RequestBuilder {
    if let Some(t) = p.timeout_secs {
        builder = builder.timeout(Duration::from_secs_f64(t));
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

impl ElicitPlugin for Plugin {
    fn name(&self) -> &'static str {
        "http"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            typed_tool::<HttpParams>(
                "get",
                "Send an HTTP GET request; returns status, URL, and response body.",
            ),
            typed_tool::<HttpParams>(
                "post",
                "Send an HTTP POST request with optional body; returns status, URL, and response body.",
            ),
            typed_tool::<HttpParams>(
                "put",
                "Send an HTTP PUT request with optional body; returns status, URL, and response body.",
            ),
            typed_tool::<HttpParams>(
                "delete",
                "Send an HTTP DELETE request; returns status, URL, and response body.",
            ),
            typed_tool::<HttpParams>(
                "patch",
                "Send an HTTP PATCH request with optional body; returns status, URL, and response body.",
            ),
            typed_tool::<HttpParams>(
                "head",
                "Send an HTTP HEAD request; returns status and URL only (no body).",
            ),
        ]
    }

    #[instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        Box::pin(async move {
            let p: HttpParams = parse_args(&params)?;
            let client = Arc::clone(&self.client);

            let builder = match params.name.as_ref() {
                "get" => client.get(&p.url),
                "post" => client.post(&p.url),
                "put" => client.put(&p.url),
                "delete" => client.delete(&p.url),
                "patch" => client.patch(&p.url),
                "head" => {
                    let b = apply_options(client.head(&p.url), &p);
                    return execute_head(b).await;
                }
                other => {
                    return Err(ErrorData::invalid_params(
                        format!("unknown tool: {other}"),
                        None,
                    ));
                }
            };

            execute(apply_options(builder, &p)).await
        })
    }
}
