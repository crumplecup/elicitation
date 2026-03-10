//! `FetchAndParsePlugin` — HTTP fetch composed with JSON parsing in a single verified operation.
//!
//! Combines `elicit_reqwest` HTTP tooling with `elicit_serde_json` JSON navigation so
//! agents can fetch a URL and extract or validate its JSON body atomically:
//!
//! - **`fetch_and_extract`** — GET URL, parse body as JSON, extract JSON Pointer
//! - **`fetch_and_validate`** — GET URL, parse body as JSON object, assert required keys present
//!
//! The typestate proof chain:
//! ```text
//! (reqwest fetch) → RequestCompleted → (serde_json parse) → JsonParsed → PointerResolved
//! ```
//!
//! # Emit support
//!
//! With the `emit` feature, `dispatch_fetch_and_parse_emit` maps tool names to
//! [`EmitCode`](elicitation::emit_code::EmitCode) impls.

use elicitation::ElicitPlugin;
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::util::{parse_args, typed_tool};
use elicitation::rmcp::RoleServer;

// ── Param types ────────────────────────────────────────────────────────────────

/// Parameters for `fetch_and_extract`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct FetchAndExtractParams {
    /// URL to fetch.
    pub url: String,
    /// RFC 6901 JSON Pointer path to extract (e.g. `"/user/name"`).
    pub pointer: String,
    /// Request timeout in seconds (default: 30).
    #[serde(default = "default_timeout")]
    pub timeout_secs: f64,
}

fn default_timeout() -> f64 {
    30.0
}

/// Parameters for `fetch_and_validate`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct FetchAndValidateParams {
    /// URL to fetch.
    pub url: String,
    /// Keys that must be present in the top-level JSON object.
    pub required_keys: Vec<String>,
    /// Request timeout in seconds (default: 30).
    #[serde(default = "default_timeout")]
    pub timeout_secs: f64,
}

// ── Plugin ─────────────────────────────────────────────────────────────────────

/// Cross-crate plugin composing HTTP fetch with JSON parsing.
///
/// Agents get a single tool that fetches a URL and inspects its JSON body —
/// eliminating the round-trip between `fetch` and `parse_and_focus` when the
/// URL and extraction path are both known upfront.
pub struct FetchAndParsePlugin;

impl ElicitPlugin for FetchAndParsePlugin {
    fn name(&self) -> &'static str {
        "fetch_and_parse"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            typed_tool::<FetchAndExtractParams>(
                "fetch_and_extract",
                "Fetch a URL and extract a value at an RFC 6901 JSON Pointer path. Combines \
                 elicit_reqwest HTTP tooling with elicit_serde_json JSON navigation. The proof \
                 chain RequestCompleted ∧ JsonParsed ∧ PointerResolved is established \
                 atomically in a single tool call.",
            ),
            typed_tool::<FetchAndValidateParams>(
                "fetch_and_validate",
                "Fetch a URL and assert that the JSON body is an object with all required keys \
                 present. Combines HTTP fetch with elicit_serde_json object validation. \
                 Returns the full validated JSON object.",
            ),
        ]
    }

    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _cx: RequestContext<RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        Box::pin(async move {
            let bare = params
                .name
                .strip_prefix("fetch_and_parse__")
                .unwrap_or(&params.name);
            match bare {
                "fetch_and_extract" => {
                    let p: FetchAndExtractParams = parse_args(&params)?;
                    fetch_and_extract_impl(p).await
                }
                "fetch_and_validate" => {
                    let p: FetchAndValidateParams = parse_args(&params)?;
                    fetch_and_validate_impl(p).await
                }
                name => Err(ErrorData::invalid_params(
                    format!("Unknown tool: {name}"),
                    None,
                )),
            }
        })
    }
}

// ── Implementations ────────────────────────────────────────────────────────────

#[instrument(skip_all, fields(url = %p.url, pointer = %p.pointer))]
async fn fetch_and_extract_impl(p: FetchAndExtractParams) -> Result<CallToolResult, ErrorData> {
    let body = http_get(&p.url, p.timeout_secs).await?;

    let value: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| ErrorData::invalid_params(format!("JSON parse failed: {e}"), None))?;

    let extracted = value.pointer(&p.pointer).ok_or_else(|| {
        ErrorData::invalid_params(
            format!("JSON Pointer '{}' not found in response", p.pointer),
            None,
        )
    })?;

    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        extracted.to_string(),
    )]))
}

#[instrument(skip_all, fields(url = %p.url, keys = ?p.required_keys))]
async fn fetch_and_validate_impl(p: FetchAndValidateParams) -> Result<CallToolResult, ErrorData> {
    let body = http_get(&p.url, p.timeout_secs).await?;

    let value: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| ErrorData::invalid_params(format!("JSON parse failed: {e}"), None))?;

    let obj = value.as_object().ok_or_else(|| {
        ErrorData::invalid_params("Response body is not a JSON object".to_string(), None)
    })?;

    let missing: Vec<&str> = p
        .required_keys
        .iter()
        .filter(|k| !obj.contains_key(k.as_str()))
        .map(String::as_str)
        .collect();

    if !missing.is_empty() {
        return Err(ErrorData::invalid_params(
            format!("Missing required keys: {}", missing.join(", ")),
            None,
        ));
    }

    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        serde_json::to_string_pretty(&value).unwrap_or_default(),
    )]))
}

async fn http_get(url: &str, timeout_secs: f64) -> Result<String, ErrorData> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .timeout(std::time::Duration::from_secs_f64(timeout_secs))
        .send()
        .await
        .map_err(|e| ErrorData::internal_error(format!("HTTP request failed: {e}"), None))?;

    response
        .text()
        .await
        .map_err(|e| ErrorData::internal_error(format!("Failed to read body: {e}"), None))
}

// ── Emit support ───────────────────────────────────────────────────────────────

#[cfg(feature = "emit")]
mod emit {
    use super::{FetchAndExtractParams, FetchAndValidateParams};
    use elicitation::emit_code::{CrateDep, EmitCode};
    use proc_macro2::TokenStream;
    use quote::quote;

    const REQWEST_DEP: CrateDep = CrateDep::new("reqwest", "0.13");
    const SERDE_JSON_DEP: CrateDep = CrateDep::new("serde_json", "1");

    impl EmitCode for FetchAndExtractParams {
        fn emit_code(&self) -> TokenStream {
            let url = &self.url;
            let pointer = &self.pointer;
            let timeout = self.timeout_secs;
            quote! {
                let _client = reqwest::Client::new();
                let _response = _client
                    .get(#url)
                    .timeout(std::time::Duration::from_secs_f64(#timeout))
                    .send()
                    .await
                    .map_err(|e| format!("HTTP request failed: {e}"))?;
                let _body = _response.text().await.map_err(|e| format!("Body error: {e}"))?;
                let _value: serde_json::Value =
                    serde_json::from_str(&_body).map_err(|e| format!("JSON parse failed: {e}"))?;
                let _extracted = _value
                    .pointer(#pointer)
                    .ok_or_else(|| format!("Pointer '{}' not found", #pointer))?;
                println!("Extracted: {}", _extracted);
            }
        }

        fn crate_deps(&self) -> Vec<CrateDep> {
            vec![REQWEST_DEP, SERDE_JSON_DEP]
        }
    }

    impl EmitCode for FetchAndValidateParams {
        fn emit_code(&self) -> TokenStream {
            let url = &self.url;
            let timeout = self.timeout_secs;
            let keys: Vec<_> = self.required_keys.iter().map(String::as_str).collect();
            quote! {
                let _client = reqwest::Client::new();
                let _response = _client
                    .get(#url)
                    .timeout(std::time::Duration::from_secs_f64(#timeout))
                    .send()
                    .await
                    .map_err(|e| format!("HTTP request failed: {e}"))?;
                let _body = _response.text().await.map_err(|e| format!("Body error: {e}"))?;
                let _value: serde_json::Value =
                    serde_json::from_str(&_body).map_err(|e| format!("JSON parse failed: {e}"))?;
                let _obj = _value
                    .as_object()
                    .ok_or("Response is not a JSON object")?;
                let _required: &[&str] = &[#(#keys),*];
                let _missing: Vec<&&str> = _required
                    .iter()
                    .filter(|k| !_obj.contains_key(**k))
                    .collect();
                if !_missing.is_empty() {
                    return Err(format!("Missing keys: {:?}", _missing).into());
                }
                println!("{}", serde_json::to_string_pretty(&_value).unwrap_or_default());
            }
        }

        fn crate_deps(&self) -> Vec<CrateDep> {
            vec![REQWEST_DEP, SERDE_JSON_DEP]
        }
    }
}

/// Dispatch a `fetch_and_parse` tool name + JSON params to an [`EmitCode`] boxed impl.
///
/// [`EmitCode`]: elicitation::emit_code::EmitCode
#[cfg(feature = "emit")]
pub fn dispatch_fetch_and_parse_emit(
    tool: &str,
    params: serde_json::Value,
) -> Result<Box<dyn elicitation::emit_code::EmitCode>, String> {
    use elicitation::emit_code::EmitCode;
    let bare = tool.strip_prefix("fetch_and_parse__").unwrap_or(tool);
    match bare {
        "fetch_and_extract" => {
            let p: FetchAndExtractParams =
                serde_json::from_value(params).map_err(|e| e.to_string())?;
            Ok(Box::new(p) as Box<dyn EmitCode>)
        }
        "fetch_and_validate" => {
            let p: FetchAndValidateParams =
                serde_json::from_value(params).map_err(|e| e.to_string())?;
            Ok(Box::new(p) as Box<dyn EmitCode>)
        }
        other => Err(format!("Unknown fetch_and_parse tool: {other}")),
    }
}

// ── Global emit registry ──────────────────────────────────────────────────────

#[cfg(feature = "emit")]
elicitation::register_emit!("fetch_and_extract", FetchAndExtractParams);
#[cfg(feature = "emit")]
elicitation::register_emit!("fetch_and_validate", FetchAndValidateParams);
