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
//! With the `emit` feature, `dispatch_secure_fetch_emit` maps tool names to
//! [`EmitCode`](elicitation::emit_code::EmitCode) impls so agent sessions can be
//! recovered as standalone Rust binaries.

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
    /// HTTPS URL to fetch.
    pub url: String,
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
    /// HTTPS URL of the API endpoint.
    pub url: String,
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
                   UrlParsed ∧ HttpsRequired is established before any network I/O."
)]
#[instrument(skip_all, fields(url = %p.url, timeout = p.timeout_secs))]
async fn secure_fetch(
    ctx: Arc<PluginContext>,
    p: SecureFetchParams,
) -> Result<CallToolResult, ErrorData> {
    // Phase 1: URL validation (elicit_url typestate)
    let (parsed, url_proof) = elicit_url::UnvalidatedUrl::new(p.url.clone())
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("URL parse failed: {e}"), None))?;

    let (_secure, _https_proof) = parsed
        .assert_https(url_proof)
        .map_err(|e| ErrorData::invalid_params(format!("HTTPS required: {e}"), None))?;

    // Phase 2: HTTP request — reuse the shared client from context
    let response = ctx
        .http
        .get(p.url.as_str())
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
                   a single verified operation."
)]
#[instrument(skip(ctx, p), fields(url = %p.url, method = %p.method, timeout = p.timeout_secs))]
async fn validated_api_call(
    ctx: Arc<PluginContext>,
    p: ValidatedApiCallParams,
) -> Result<CallToolResult, ErrorData> {
    // Phase 1: URL validation
    let (parsed, url_proof) = elicit_url::UnvalidatedUrl::new(p.url.clone())
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("URL parse failed: {e}"), None))?;

    let (_secure, _https_proof) = parsed
        .assert_https(url_proof)
        .map_err(|e| ErrorData::invalid_params(format!("HTTPS required: {e}"), None))?;

    // Phase 2: Authenticated HTTP request — reuse the shared client from context
    let builder = match p.method.to_uppercase().as_str() {
        "POST" => {
            let mut b = ctx
                .http
                .post(p.url.as_str())
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
            .get(p.url.as_str())
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

// ── Emit support ───────────────────────────────────────────────────────────────

#[cfg(feature = "emit")]
mod emit {
    use super::{SecureFetchParams, ValidatedApiCallParams};
    use elicitation::emit_code::{CrateDep, EmitCode};
    use proc_macro2::TokenStream;
    use quote::quote;

    const ELICIT_URL_DEP: CrateDep = CrateDep::new("elicit_url", "0.8");
    const REQWEST_DEP: CrateDep = CrateDep::new("reqwest", "0.13");

    impl EmitCode for SecureFetchParams {
        fn emit_code(&self) -> TokenStream {
            let url = &self.url;
            let timeout = self.timeout_secs;
            quote! {
                let (_parsed, _url_proof) = elicit_url::UnvalidatedUrl::new(#url.to_string())
                    .parse()
                    .map_err(|e| format!("URL parse failed: {e}"))?;
                let (_secure, _https_proof) = _parsed
                    .assert_https(_url_proof)
                    .map_err(|e| format!("HTTPS required: {e}"))?;
                let _client = reqwest::Client::new();
                let _response = _client
                    .get(_secure.as_str())
                    .timeout(std::time::Duration::from_secs_f64(#timeout))
                    .send()
                    .await
                    .map_err(|e| format!("HTTP request failed: {e}"))?;
                let _status = _response.status();
                let _body = _response.text().await.map_err(|e| format!("Body error: {e}"))?;
                println!("status={_status}");
                println!("body_len={}", _body.len());
            }
        }

        fn crate_deps(&self) -> Vec<CrateDep> {
            vec![ELICIT_URL_DEP, REQWEST_DEP]
        }
    }

    impl EmitCode for ValidatedApiCallParams {
        fn emit_code(&self) -> TokenStream {
            let url = &self.url;
            let token = &self.token;
            let timeout = self.timeout_secs;
            let method = &self.method;
            let body_expr = match &self.body {
                Some(b) => quote! { Some(#b.to_string()) },
                None => quote! { None::<String> },
            };
            quote! {
                let (_parsed, _url_proof) = elicit_url::UnvalidatedUrl::new(#url.to_string())
                    .parse()
                    .map_err(|e| format!("URL parse failed: {e}"))?;
                let (_secure, _https_proof) = _parsed
                    .assert_https(_url_proof)
                    .map_err(|e| format!("HTTPS required: {e}"))?;
                let _client = reqwest::Client::new();
                let _body_opt: Option<String> = #body_expr;
                let _builder = match #method.to_uppercase().as_str() {
                    "POST" => {
                        let mut b = _client
                            .post(_secure.as_str())
                            .bearer_auth(#token)
                            .timeout(std::time::Duration::from_secs_f64(#timeout));
                        if let Some(body) = &_body_opt {
                            b = b.header("Content-Type", "application/json").body(body.clone());
                        }
                        b
                    }
                    _ => _client
                        .get(_secure.as_str())
                        .bearer_auth(#token)
                        .timeout(std::time::Duration::from_secs_f64(#timeout)),
                };
                let _response = _builder.send().await.map_err(|e| format!("Request failed: {e}"))?;
                let _status = _response.status();
                let _body = _response.text().await.map_err(|e| format!("Body error: {e}"))?;
                println!("status={_status}");
                println!("body_len={}", _body.len());
            }
        }

        fn crate_deps(&self) -> Vec<CrateDep> {
            vec![ELICIT_URL_DEP, REQWEST_DEP]
        }
    }
}

/// Dispatch a `secure_fetch` tool name + JSON params to an [`EmitCode`] boxed impl.
///
/// [`EmitCode`]: elicitation::emit_code::EmitCode
#[cfg(feature = "emit")]
pub fn dispatch_secure_fetch_emit(
    tool: &str,
    params: serde_json::Value,
) -> Result<Box<dyn elicitation::emit_code::EmitCode>, String> {
    use elicitation::emit_code::EmitCode;
    let bare = tool.strip_prefix("secure_fetch__").unwrap_or(tool);
    match bare {
        "secure_fetch" => {
            let p: SecureFetchParams = serde_json::from_value(params).map_err(|e| e.to_string())?;
            Ok(Box::new(p) as Box<dyn EmitCode>)
        }
        "validated_api_call" => {
            let p: ValidatedApiCallParams =
                serde_json::from_value(params).map_err(|e| e.to_string())?;
            Ok(Box::new(p) as Box<dyn EmitCode>)
        }
        other => Err(format!("Unknown secure_fetch tool: {other}")),
    }
}
