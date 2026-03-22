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
//! With the `emit` feature, `#[elicit_tool]` auto-generates
//! [`EmitCode`](elicitation::emit_code::EmitCode) impls for each handler and
//! registers them in the global inventory so agent sessions can be recovered as
//! standalone Rust binaries.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::{ErrorData, model::CallToolResult, model::Content};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

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
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "fetch_and_parse")]
pub struct FetchAndParsePlugin;

// ── Tool handlers ──────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "fetch_and_parse",
    name = "fetch_and_extract",
    description = "Fetch a URL and extract a value at an RFC 6901 JSON Pointer path. Combines \
                   elicit_reqwest HTTP tooling with elicit_serde_json JSON navigation. The proof \
                   chain RequestCompleted ∧ JsonParsed ∧ PointerResolved is established \
                   atomically in a single tool call."
)]
#[instrument(skip_all, fields(url = %p.url, pointer = %p.pointer))]
async fn fetch_and_extract(p: FetchAndExtractParams) -> Result<CallToolResult, ErrorData> {
    let body = http_get(&p.url, p.timeout_secs).await?;

    let value: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| ErrorData::invalid_params(format!("JSON parse failed: {e}"), None))?;

    let extracted = value.pointer(&p.pointer).ok_or_else(|| {
        ErrorData::invalid_params(
            format!("JSON Pointer '{}' not found in response", p.pointer),
            None,
        )
    })?;

    Ok(CallToolResult::success(vec![Content::text(
        extracted.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "fetch_and_parse",
    name = "fetch_and_validate",
    description = "Fetch a URL and assert that the JSON body is an object with all required keys \
                   present. Combines HTTP fetch with elicit_serde_json object validation. \
                   Returns the full validated JSON object."
)]
#[instrument(skip_all, fields(url = %p.url, keys = ?p.required_keys))]
async fn fetch_and_validate(p: FetchAndValidateParams) -> Result<CallToolResult, ErrorData> {
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

    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string_pretty(&value).unwrap_or_default(),
    )]))
}

// ── Helpers ────────────────────────────────────────────────────────────────────

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
