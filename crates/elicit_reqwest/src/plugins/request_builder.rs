//! `RequestBuilderPlugin` — stateless pipeline tools for building and sending HTTP requests.
//!
//! Unlike the `HttpPlugin` which fires one-shot requests, this plugin exposes
//! the full builder pipeline as individual composable tools. A request is
//! represented as a plain JSON **request spec**:
//!
//! ```json
//! {
//!   "method": "POST",
//!   "url": "https://api.example.com/v1/items",
//!   "headers": { "Content-Type": "application/json" },
//!   "body": "{\"name\":\"widget\"}",
//!   "timeout_secs": 30.0
//! }
//! ```
//!
//! Each `with_*` tool takes a spec, applies one mutation, and returns the new spec.
//! `send` takes a spec and executes the HTTP call, returning `{ status, url, headers, body }`.
//!
//! Registered under the `"request_builder"` namespace.

use std::collections::HashMap;
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
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::plugins::util::{parse_args, typed_tool};

// ── Request spec ──────────────────────────────────────────────────────────────

/// A serializable representation of a pending HTTP request.
///
/// Passed between `with_*` tools and ultimately consumed by `send`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RequestSpec {
    /// HTTP method (e.g. `"GET"`, `"POST"`).
    pub method: String,
    /// Destination URL.
    pub url: String,
    /// Request headers.
    #[serde(default)]
    pub headers: HashMap<String, String>,
    /// Raw request body.
    pub body: Option<String>,
    /// Request timeout in seconds.
    pub timeout_secs: Option<f64>,
}

// ── Parameter types ──────────────────────────────────────────────────────────

/// Parameters for `new_*` tools that require a URL only.
#[derive(Debug, Deserialize, JsonSchema)]
struct NewParams {
    /// Destination URL.
    url: String,
}

/// Parameters for `new_*` tools with an optional body.
#[derive(Debug, Deserialize, JsonSchema)]
struct NewBodyParams {
    /// Destination URL.
    url: String,
    /// Optional request body.
    body: Option<String>,
}

/// Parameters for `with_header`.
#[derive(Debug, Deserialize, JsonSchema)]
struct WithHeaderParams {
    /// The current request spec.
    spec: RequestSpec,
    /// Header name.
    key: String,
    /// Header value.
    value: String,
}

/// Parameters for `with_bearer_auth`.
#[derive(Debug, Deserialize, JsonSchema)]
struct WithBearerAuthParams {
    /// The current request spec.
    spec: RequestSpec,
    /// Bearer token (without the `Bearer ` prefix).
    token: String,
}

/// Parameters for `with_basic_auth`.
#[derive(Debug, Deserialize, JsonSchema)]
struct WithBasicAuthParams {
    /// The current request spec.
    spec: RequestSpec,
    /// Username.
    username: String,
    /// Password (optional).
    password: Option<String>,
}

/// Parameters for `with_body`.
#[derive(Debug, Deserialize, JsonSchema)]
struct WithBodyParams {
    /// The current request spec.
    spec: RequestSpec,
    /// New body string.
    body: String,
}

/// Parameters for `with_json_body`.
#[derive(Debug, Deserialize, JsonSchema)]
struct WithJsonBodyParams {
    /// The current request spec.
    spec: RequestSpec,
    /// JSON body as a raw string (already serialized).
    json: String,
}

/// Parameters for `with_timeout`.
#[derive(Debug, Deserialize, JsonSchema)]
struct WithTimeoutParams {
    /// The current request spec.
    spec: RequestSpec,
    /// Timeout in seconds.
    timeout_secs: f64,
}

/// Parameters for tools that just take a spec.
#[derive(Debug, Deserialize, JsonSchema)]
struct SpecParams {
    /// The request spec to execute or inspect.
    spec: RequestSpec,
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing the full request builder pipeline as composable tools.
///
/// Each `with_*` tool is a pure function: input spec → output spec.
/// `send` executes the final spec.
///
/// Register under the `"request_builder"` namespace:
/// ```rust,no_run
/// use elicitation::PluginRegistry;
/// use elicit_reqwest::plugins::RequestBuilderPlugin;
///
/// let registry = PluginRegistry::new()
///     .register("request_builder", RequestBuilderPlugin::new());
/// ```
pub struct RequestBuilderPlugin {
    client: Arc<reqwest::Client>,
}

impl RequestBuilderPlugin {
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

impl Default for RequestBuilderPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl ElicitPlugin for RequestBuilderPlugin {
    fn name(&self) -> &'static str {
        "request_builder"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            // Constructors
            typed_tool::<NewParams>("new_get", "Create a GET request spec for the given URL."),
            typed_tool::<NewBodyParams>(
                "new_post",
                "Create a POST request spec with an optional body.",
            ),
            typed_tool::<NewBodyParams>(
                "new_put",
                "Create a PUT request spec with an optional body.",
            ),
            typed_tool::<NewParams>(
                "new_delete",
                "Create a DELETE request spec for the given URL.",
            ),
            typed_tool::<NewBodyParams>(
                "new_patch",
                "Create a PATCH request spec with an optional body.",
            ),
            typed_tool::<NewParams>("new_head", "Create a HEAD request spec for the given URL."),
            // Mutations (return new spec)
            typed_tool::<WithHeaderParams>(
                "with_header",
                "Add or replace a single header in the request spec; returns the updated spec.",
            ),
            typed_tool::<WithBearerAuthParams>(
                "with_bearer_auth",
                "Set `Authorization: Bearer <token>` in the request spec; returns the updated spec.",
            ),
            typed_tool::<WithBasicAuthParams>(
                "with_basic_auth",
                "Set `Authorization: Basic ...` in the request spec; returns the updated spec.",
            ),
            typed_tool::<WithBodyParams>(
                "with_body",
                "Set or replace the raw request body; returns the updated spec.",
            ),
            typed_tool::<WithJsonBodyParams>(
                "with_json_body",
                "Set the body to a JSON string and add `Content-Type: application/json`; returns the updated spec.",
            ),
            typed_tool::<WithTimeoutParams>(
                "with_timeout",
                "Set the request timeout in seconds; returns the updated spec.",
            ),
            // Inspection
            typed_tool::<SpecParams>(
                "inspect",
                "Return a human-readable summary of the request spec as JSON.",
            ),
            // Execution
            typed_tool::<SpecParams>(
                "send",
                "Execute the request spec; returns `{ status, url, headers, body }`.",
            ),
        ]
    }

    #[instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let client = Arc::clone(&self.client);
        Box::pin(async move {
            match params.name.as_ref() {
                // ── Constructors ──────────────────────────────────────────────
                "new_get" => {
                    let p: NewParams = parse_args(&params)?;
                    Ok(spec_result(new_spec("GET", &p.url, None)))
                }
                "new_post" => {
                    let p: NewBodyParams = parse_args(&params)?;
                    Ok(spec_result(new_spec("POST", &p.url, p.body)))
                }
                "new_put" => {
                    let p: NewBodyParams = parse_args(&params)?;
                    Ok(spec_result(new_spec("PUT", &p.url, p.body)))
                }
                "new_delete" => {
                    let p: NewParams = parse_args(&params)?;
                    Ok(spec_result(new_spec("DELETE", &p.url, None)))
                }
                "new_patch" => {
                    let p: NewBodyParams = parse_args(&params)?;
                    Ok(spec_result(new_spec("PATCH", &p.url, p.body)))
                }
                "new_head" => {
                    let p: NewParams = parse_args(&params)?;
                    Ok(spec_result(new_spec("HEAD", &p.url, None)))
                }
                // ── Mutations ─────────────────────────────────────────────────
                "with_header" => {
                    let p: WithHeaderParams = parse_args(&params)?;
                    let mut spec = p.spec;
                    spec.headers.insert(p.key, p.value);
                    Ok(spec_result(spec))
                }
                "with_bearer_auth" => {
                    let p: WithBearerAuthParams = parse_args(&params)?;
                    let mut spec = p.spec;
                    spec.headers
                        .insert("Authorization".to_string(), format!("Bearer {}", p.token));
                    Ok(spec_result(spec))
                }
                "with_basic_auth" => {
                    let p: WithBasicAuthParams = parse_args(&params)?;
                    let mut spec = p.spec;
                    let encoded = base64_basic_auth(&p.username, p.password.as_deref());
                    spec.headers
                        .insert("Authorization".to_string(), format!("Basic {encoded}"));
                    Ok(spec_result(spec))
                }
                "with_body" => {
                    let p: WithBodyParams = parse_args(&params)?;
                    let mut spec = p.spec;
                    spec.body = Some(p.body);
                    Ok(spec_result(spec))
                }
                "with_json_body" => {
                    let p: WithJsonBodyParams = parse_args(&params)?;
                    let mut spec = p.spec;
                    spec.body = Some(p.json);
                    spec.headers
                        .entry("Content-Type".to_string())
                        .or_insert_with(|| "application/json".to_string());
                    Ok(spec_result(spec))
                }
                "with_timeout" => {
                    let p: WithTimeoutParams = parse_args(&params)?;
                    let mut spec = p.spec;
                    spec.timeout_secs = Some(p.timeout_secs);
                    Ok(spec_result(spec))
                }
                // ── Inspection ────────────────────────────────────────────────
                "inspect" => {
                    let p: SpecParams = parse_args(&params)?;
                    let json =
                        serde_json::to_string_pretty(&p.spec).unwrap_or_else(|e| e.to_string());
                    Ok(CallToolResult::success(vec![Content::text(json)]))
                }
                // ── Execution ─────────────────────────────────────────────────
                "send" => {
                    let p: SpecParams = parse_args(&params)?;
                    execute(client, p.spec).await
                }
                other => Err(ErrorData::invalid_params(
                    format!("unknown tool: {other}"),
                    None,
                )),
            }
        })
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn new_spec(method: &str, url: &str, body: Option<String>) -> RequestSpec {
    RequestSpec {
        method: method.to_string(),
        url: url.to_string(),
        headers: HashMap::new(),
        body,
        timeout_secs: None,
    }
}

fn spec_result(spec: RequestSpec) -> CallToolResult {
    let json = serde_json::to_string(&spec).unwrap_or_else(|e| e.to_string());
    CallToolResult::success(vec![Content::text(json)])
}

/// Encode `username:password` in Base64 for Basic auth.
fn base64_basic_auth(username: &str, password: Option<&str>) -> String {
    use std::io::Write;
    let credential = match password {
        Some(pw) => format!("{username}:{pw}"),
        None => format!("{username}:"),
    };
    // Use a simple byte-level Base64 via the standard library through write!
    let bytes = credential.as_bytes();
    // base64 isn't in std; use a manual table
    let table = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = Vec::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = if chunk.len() > 1 {
            chunk[1] as usize
        } else {
            0
        };
        let b2 = if chunk.len() > 2 {
            chunk[2] as usize
        } else {
            0
        };
        out.push(table[(b0 >> 2) & 0x3F]);
        out.push(table[((b0 << 4) | (b1 >> 4)) & 0x3F]);
        out.push(if chunk.len() > 1 {
            table[((b1 << 2) | (b2 >> 6)) & 0x3F]
        } else {
            b'='
        });
        out.push(if chunk.len() > 2 {
            table[b2 & 0x3F]
        } else {
            b'='
        });
        let _ = out.write(&[]); // silence write trait import warning
    }
    String::from_utf8(out).unwrap_or_default()
}

/// Build a `reqwest::RequestBuilder` from a `RequestSpec` and execute it.
async fn execute(
    client: Arc<reqwest::Client>,
    spec: RequestSpec,
) -> Result<CallToolResult, ErrorData> {
    let method = reqwest::Method::from_bytes(spec.method.as_bytes())
        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

    let mut builder = client.request(method, &spec.url);

    for (k, v) in &spec.headers {
        builder = builder.header(k.as_str(), v.as_str());
    }
    if let Some(t) = spec.timeout_secs {
        builder = builder.timeout(Duration::from_secs_f64(t));
    }
    if let Some(body) = spec.body {
        builder = builder.body(body);
    }

    match builder.send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let url = resp.url().to_string();
            let headers: HashMap<String, String> = resp
                .headers()
                .iter()
                .filter_map(|(k, v)| {
                    v.to_str()
                        .ok()
                        .map(|v| (k.as_str().to_string(), v.to_string()))
                })
                .collect();
            let body = resp.text().await.unwrap_or_default();
            let json = serde_json::json!({ "status": status, "url": url, "headers": headers, "body": body });
            Ok(CallToolResult::success(vec![Content::text(
                json.to_string(),
            )]))
        }
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}
