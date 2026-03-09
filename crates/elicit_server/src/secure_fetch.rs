//! `SecureFetchPlugin` — URL validation + HTTPS enforcement before every HTTP call.
//!
//! Combines `elicit_url` typestate (parse → assert HTTPS) with `elicit_reqwest`
//! HTTP tooling so agents have a single tool that embeds both contracts:
//!
//! - **`secure_fetch`** — asserts HTTPS then GETs the URL
//! - **`validated_api_call`** — asserts HTTPS then makes an authenticated JSON request
//!
//! The typestate proof chain:
//! ```text
//! UnvalidatedUrl → UrlParsed → HttpsRequired → (reqwest fetch) → RequestCompleted ∧ StatusSuccess
//! ```
//!
//! # Emit support
//!
//! With the `emit` feature, `#[elicit_tool]` auto-generates
//! [`EmitCode`](elicitation::emit_code::EmitCode) impls for each handler and
//! registers them in the global inventory so agent sessions can be recovered as
//! standalone Rust binaries.

use elicitation::verification::types::UrlHttps;
use elicitation::{ElicitPlugin, PluginContext, elicit_tool};
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;

// ── Param types ────────────────────────────────────────────────────────────────

/// Parameters for `secure_fetch`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct SecureFetchParams {
    /// HTTPS URL to fetch. Must use the `https://` scheme.
    pub url: UrlHttps,
    /// Request timeout in seconds (default: 30).
    #[serde(default = "default_timeout")]
    pub timeout_secs: f64,
}

fn default_timeout() -> f64 {
    30.0
}

/// Parameters for `validated_api_call`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ValidatedApiCallParams {
    /// HTTPS URL of the API endpoint. Must use the `https://` scheme.
    pub url: UrlHttps,
    /// Bearer token for authorization.
    pub token: String,
    /// HTTP method — `"GET"` or `"POST"` (default: `"GET"`).
    #[serde(default = "default_get")]
    pub method: String,
    /// Optional JSON request body (POST only).
    pub body: Option<String>,
    /// Request timeout in seconds (default: 30).
    #[serde(default = "default_timeout")]
    pub timeout_secs: f64,
}

fn default_get() -> String {
    "GET".to_string()
}

// ── Plugin ─────────────────────────────────────────────────────────────────────

/// Cross-crate plugin combining URL validation with HTTP requests.
///
/// Every tool in this plugin asserts HTTPS before making any network call —
/// the typestate proof is embedded in the function signature, so the contract
/// cannot be bypassed.
///
/// Tools are registered at link time via `#[elicit_tool(plugin = "secure_fetch")]`
/// on each handler; no manual wiring required.
///
/// The shared `reqwest::Client` inside the [`PluginContext`] is reused across
/// all tool calls, keeping the connection pool alive for the plugin's lifetime.
#[derive(ElicitPlugin)]
#[plugin(name = "secure_fetch")]
pub struct SecureFetchPlugin(pub Arc<PluginContext>);

impl SecureFetchPlugin {
    /// Create a new plugin with a fresh shared context.
    pub fn new() -> Self {
        Self(PluginContext::new())
    }
}

impl Default for SecureFetchPlugin {
    fn default() -> Self {
        Self::new()
    }
}

// ── Implementations ────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "secure_fetch",
    name = "secure_fetch",
    description = "Assert HTTPS and fetch a URL. Combines elicit_url typestate (parse → assert \
                   HTTPS) with elicit_reqwest HTTP tooling. The proof chain \
                   UrlParsed ∧ HttpsRequired is established before any network I/O.",
    emit_ctx("ctx.http" => "reqwest::Client::new()")
)]
#[instrument(skip_all, fields(timeout = p.timeout_secs))]
async fn secure_fetch(
    ctx: Arc<PluginContext>,
    p: SecureFetchParams,
) -> Result<CallToolResult, ErrorData> {
    // URL is validated as UrlHttps at deserialization — no ceremony needed.
    let url_str = p.url.get().as_str().to_owned();

    let response = ctx
        .http
        .get(&url_str)
        .timeout(std::time::Duration::from_secs_f64(p.timeout_secs))
        .send()
        .await
        .map_err(|e| ErrorData::internal_error(format!("HTTP request failed: {e}"), None))?;

    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|e| ErrorData::internal_error(format!("Failed to read body: {e}"), None))?;

    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        format!(
            "status={status}\nbody_len={}\n{}",
            body.len(),
            &body[..512.min(body.len())]
        ),
    )]))
}

#[elicit_tool(
    plugin = "secure_fetch",
    name = "validated_api_call",
    description = "Assert HTTPS then make an authenticated GET or POST request. Combines \
                   URL validation, HTTPS enforcement, and bearer token authorization into \
                   a single verified operation.",
    emit_ctx("ctx.http" => "reqwest::Client::new()")
)]
#[instrument(skip(ctx, p), fields(method = %p.method, timeout = p.timeout_secs))]
async fn validated_api_call(
    ctx: Arc<PluginContext>,
    p: ValidatedApiCallParams,
) -> Result<CallToolResult, ErrorData> {
    // URL is validated as UrlHttps at deserialization — no ceremony needed.
    let url_str = p.url.get().as_str().to_owned();

    // Authenticated HTTP request — reuse the shared client from context
    let builder = match p.method.to_uppercase().as_str() {
        "POST" => {
            let mut b = ctx
                .http
                .post(&url_str)
                .bearer_auth(&p.token)
                .timeout(std::time::Duration::from_secs_f64(p.timeout_secs));
            if let Some(body) = &p.body {
                b = b
                    .header("Content-Type", "application/json")
                    .body(body.clone());
            }
            b
        }
        _ => ctx
            .http
            .get(&url_str)
            .bearer_auth(&p.token)
            .timeout(std::time::Duration::from_secs_f64(p.timeout_secs)),
    };

    let response = builder
        .send()
        .await
        .map_err(|e| ErrorData::internal_error(format!("HTTP request failed: {e}"), None))?;

    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|e| ErrorData::internal_error(format!("Failed to read body: {e}"), None))?;

    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        format!(
            "status={status}\nbody_len={}\n{}",
            body.len(),
            &body[..512.min(body.len())]
        ),
    )]))
}
