//! `HeaderMapPlugin` — MCP tools for every `http::HeaderMap` method.
//!
//! Headers are represented as JSON objects (`{ "Content-Type": "application/json" }`).
//! Multi-value headers use the last value (to keep the wire format flat and JSON-friendly).
//!
//! Registered under the `"header_map"` namespace, producing tools:
//! `header_map__new`, `header_map__get`, `header_map__contains_key`,
//! `header_map__insert`, `header_map__append`, `header_map__remove`,
//! `header_map__len`, `header_map__keys_len`, `header_map__is_empty`,
//! `header_map__keys`, `header_map__values`, `header_map__clear`.

use std::collections::HashMap;

use elicitation::{ElicitPlugin, elicit_tool};
use http::{HeaderName, HeaderValue};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

/// A header map represented as a JSON object.
type Headers = HashMap<String, String>;

/// Parameters for tools that only need the headers map.
#[derive(Debug, Deserialize, JsonSchema)]
struct HeadersParams {
    /// The header map (key → value).
    headers: Headers,
}

/// Parameters for key lookups.
#[derive(Debug, Deserialize, JsonSchema)]
struct GetParams {
    /// The header map.
    headers: Headers,
    /// Header name to look up (case-insensitive).
    key: String,
}

/// Parameters for insert/replace operations.
#[derive(Debug, Deserialize, JsonSchema)]
struct InsertParams {
    /// The header map to modify.
    headers: Headers,
    /// Header name.
    key: String,
    /// Header value.
    value: String,
}

/// Parameters for remove operations.
#[derive(Debug, Deserialize, JsonSchema)]
struct RemoveParams {
    /// The header map to modify.
    headers: Headers,
    /// Header name to remove.
    key: String,
}

/// Construct an `http::HeaderMap` from a `HashMap<String, String>`.
fn to_header_map(headers: &Headers) -> Result<http::HeaderMap, CallToolResult> {
    let mut map = http::HeaderMap::new();
    for (k, v) in headers {
        let name = HeaderName::from_bytes(k.as_bytes())
            .map_err(|e| CallToolResult::error(vec![Content::text(e.to_string())]))?;
        let value = HeaderValue::from_str(v)
            .map_err(|e| CallToolResult::error(vec![Content::text(e.to_string())]))?;
        map.insert(name, value);
    }
    Ok(map)
}

/// Serialize an `http::HeaderMap` back to a `HashMap<String, String>`.
fn from_header_map(map: &http::HeaderMap) -> Headers {
    map.iter()
        .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
        .collect()
}

/// MCP plugin exposing all `http::HeaderMap` methods as tools.
///
/// Register under the `"header_map"` namespace:
/// ```rust,no_run
/// use elicitation::PluginRegistry;
/// use elicit_reqwest::plugins::HeaderMapPlugin;
///
/// let registry = PluginRegistry::new()
///     .register("header_map", HeaderMapPlugin);
/// ```
#[derive(ElicitPlugin)]
#[plugin(name = "header_map")]
pub struct HeaderMapPlugin;

// ── Tool handlers ─────────────────────────────────────────────────────────────

/// Empty params for `new` and `clear` which take no meaningful input.
#[derive(Debug, Deserialize, JsonSchema)]
struct EmptyParams {}

#[elicit_tool(
    plugin = "header_map",
    name = "new",
    description = "Return an empty header map as a JSON object."
)]
#[instrument(skip_all)]
async fn hm_new(_p: EmptyParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text("{}")]))
}

#[elicit_tool(
    plugin = "header_map",
    name = "get",
    description = "Return the value of a header by name, or null if absent."
)]
#[instrument(skip_all, fields(key = %p.key))]
async fn hm_get(p: GetParams) -> Result<CallToolResult, ErrorData> {
    let map = match to_header_map(&p.headers) {
        Ok(m) => m,
        Err(r) => return Ok(r),
    };
    let result = map
        .get(p.key.as_str())
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "null".to_string());
    Ok(CallToolResult::success(vec![Content::text(result)]))
}

#[elicit_tool(
    plugin = "header_map",
    name = "contains_key",
    description = "Return true if the header map contains the given key."
)]
#[instrument(skip_all, fields(key = %p.key))]
async fn hm_contains_key(p: GetParams) -> Result<CallToolResult, ErrorData> {
    let map = match to_header_map(&p.headers) {
        Ok(m) => m,
        Err(r) => return Ok(r),
    };
    Ok(CallToolResult::success(vec![Content::text(
        map.contains_key(p.key.as_str()).to_string(),
    )]))
}

#[elicit_tool(
    plugin = "header_map",
    name = "insert",
    description = "Insert or replace a header; returns the updated header map and the previous value (or null)."
)]
#[instrument(skip_all, fields(key = %p.key))]
async fn hm_insert(p: InsertParams) -> Result<CallToolResult, ErrorData> {
    let mut map = match to_header_map(&p.headers) {
        Ok(m) => m,
        Err(r) => return Ok(r),
    };
    let name = match HeaderName::from_bytes(p.key.as_bytes()) {
        Ok(n) => n,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    };
    let value = match HeaderValue::from_str(&p.value) {
        Ok(v) => v,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    };
    let previous = map
        .insert(name, value)
        .and_then(|v| v.to_str().ok().map(|s| s.to_string()));
    let result = serde_json::json!({
        "headers": from_header_map(&map),
        "previous": previous,
    });
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "header_map",
    name = "append",
    description = "Append a header (allows multiple values per key); returns the updated header map."
)]
#[instrument(skip_all, fields(key = %p.key))]
async fn hm_append(p: InsertParams) -> Result<CallToolResult, ErrorData> {
    let mut map = match to_header_map(&p.headers) {
        Ok(m) => m,
        Err(r) => return Ok(r),
    };
    let name = match HeaderName::from_bytes(p.key.as_bytes()) {
        Ok(n) => n,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    };
    let value = match HeaderValue::from_str(&p.value) {
        Ok(v) => v,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    };
    map.append(name, value);
    let json = serde_json::to_string(&from_header_map(&map)).unwrap_or_else(|_| "{}".to_string());
    Ok(CallToolResult::success(vec![Content::text(json)]))
}

#[elicit_tool(
    plugin = "header_map",
    name = "remove",
    description = "Remove a header by name; returns the updated header map and the removed value (or null)."
)]
#[instrument(skip_all, fields(key = %p.key))]
async fn hm_remove(p: RemoveParams) -> Result<CallToolResult, ErrorData> {
    let mut map = match to_header_map(&p.headers) {
        Ok(m) => m,
        Err(r) => return Ok(r),
    };
    let removed = map
        .remove(p.key.as_str())
        .and_then(|v| v.to_str().ok().map(|s| s.to_string()));
    let result = serde_json::json!({
        "headers": from_header_map(&map),
        "removed": removed,
    });
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "header_map",
    name = "len",
    description = "Return the total number of header entries (counting multi-value headers separately)."
)]
#[instrument(skip_all)]
async fn hm_len(p: HeadersParams) -> Result<CallToolResult, ErrorData> {
    let map = match to_header_map(&p.headers) {
        Ok(m) => m,
        Err(r) => return Ok(r),
    };
    Ok(CallToolResult::success(vec![Content::text(
        map.len().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "header_map",
    name = "keys_len",
    description = "Return the number of distinct header names."
)]
#[instrument(skip_all)]
async fn hm_keys_len(p: HeadersParams) -> Result<CallToolResult, ErrorData> {
    let map = match to_header_map(&p.headers) {
        Ok(m) => m,
        Err(r) => return Ok(r),
    };
    Ok(CallToolResult::success(vec![Content::text(
        map.keys_len().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "header_map",
    name = "is_empty",
    description = "Return true if the header map contains no entries."
)]
#[instrument(skip_all)]
async fn hm_is_empty(p: HeadersParams) -> Result<CallToolResult, ErrorData> {
    let map = match to_header_map(&p.headers) {
        Ok(m) => m,
        Err(r) => return Ok(r),
    };
    Ok(CallToolResult::success(vec![Content::text(
        map.is_empty().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "header_map",
    name = "keys",
    description = "Return a list of all header names (may contain duplicates for multi-value headers)."
)]
#[instrument(skip_all)]
async fn hm_keys(p: HeadersParams) -> Result<CallToolResult, ErrorData> {
    let map = match to_header_map(&p.headers) {
        Ok(m) => m,
        Err(r) => return Ok(r),
    };
    let keys: Vec<&str> = map.keys().map(|k| k.as_str()).collect();
    let json = serde_json::to_string(&keys).unwrap_or_else(|_| "[]".to_string());
    Ok(CallToolResult::success(vec![Content::text(json)]))
}

#[elicit_tool(
    plugin = "header_map",
    name = "values",
    description = "Return a list of all header values."
)]
#[instrument(skip_all)]
async fn hm_values(p: HeadersParams) -> Result<CallToolResult, ErrorData> {
    let map = match to_header_map(&p.headers) {
        Ok(m) => m,
        Err(r) => return Ok(r),
    };
    let values: Vec<&str> = map.values().filter_map(|v| v.to_str().ok()).collect();
    let json = serde_json::to_string(&values).unwrap_or_else(|_| "[]".to_string());
    Ok(CallToolResult::success(vec![Content::text(json)]))
}

#[elicit_tool(
    plugin = "header_map",
    name = "clear",
    description = "Return an empty header map (clears all entries)."
)]
#[instrument(skip_all)]
async fn hm_clear(_p: EmptyParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text("{}")]))
}
