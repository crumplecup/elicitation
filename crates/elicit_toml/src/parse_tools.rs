//! Tools for parsing TOML text and serialising documents back to text.
//!
//! Covers the top-level `toml` free functions and `toml_edit::DocumentMut::parse`.

use std::sync::Arc;

use elicitation::elicit_tool;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::plugin::{TomlCtx, err_text, ok_json, ok_text, parse_uuid};

// ── Deserialize any TOML text ─────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ParseFromStrParams {
    /// Raw TOML text to parse.
    pub toml_text: String,
}

/// Parse a TOML string into a `toml::Value` (serde round-trip, no edit support).
///
/// Returns JSON representation of the deserialized value.
#[elicit_tool(
    plugin = "toml",
    name = "toml__parse__value_from_str",
    description = "Parse a TOML string into a JSON-serialized toml::Value (read-only serde path)."
)]
async fn value_from_str(p: ParseFromStrParams) -> Result<CallToolResult, ErrorData> {
    match toml::from_str::<toml::Value>(&p.toml_text) {
        Ok(val) => ok_json(&val),
        Err(e) => err_text(format!("TOML parse error: {}", e)),
    }
}

// ── Serialize a toml::Value JSON back to TOML text ────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SerializeValueParams {
    /// JSON representation of a `toml::Value` to serialize to TOML text.
    pub value_json: String,
    /// Use pretty (multi-line table) output.
    #[serde(default)]
    pub pretty: bool,
}

/// Serialize a JSON-encoded `toml::Value` back to a TOML string.
#[elicit_tool(
    plugin = "toml",
    name = "toml__parse__value_to_string",
    description = "Serialize a JSON-encoded toml::Value to a TOML string. Set pretty=true for multi-line table format."
)]
async fn value_to_string(p: SerializeValueParams) -> Result<CallToolResult, ErrorData> {
    let val: toml::Value = serde_json::from_str(&p.value_json)
        .map_err(|e| ErrorData::invalid_params(format!("JSON parse error: {}", e), None))?;
    let result = if p.pretty {
        toml::to_string_pretty(&val)
    } else {
        toml::to_string(&val)
    };
    match result {
        Ok(s) => ok_text(s),
        Err(e) => err_text(format!("TOML serialize error: {}", e)),
    }
}

// ── DocumentMut: parse ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ParseDocumentParams {
    /// Raw TOML text to parse into a mutable document.
    pub toml_text: String,
}

/// Parse a TOML string into a live `DocumentMut` and return its UUID.
///
/// The document can then be inspected and modified with `toml__document__*` tools.
#[elicit_tool(
    plugin = "toml",
    name = "toml__parse__document_from_str",
    description = "Parse TOML text into a mutable DocumentMut. Returns a document_id UUID for use with toml__document__* tools."
)]
async fn parse_document_from_str(
    ctx: Arc<TomlCtx>,
    p: ParseDocumentParams,
) -> Result<CallToolResult, ErrorData> {
    let doc = p
        .toml_text
        .parse::<toml_edit::DocumentMut>()
        .map_err(|e| ErrorData::invalid_params(format!("TOML parse error: {}", e), None))?;
    let id = Uuid::new_v4();
    ctx.documents.lock().unwrap().insert(id, doc);
    ok_text(id.to_string())
}

// ── DocumentMut: serialize ────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SerializeDocumentParams {
    /// UUID of the DocumentMut returned by `toml__parse__document_from_str`.
    pub document_id: String,
}

/// Serialize a live `DocumentMut` back to TOML text (preserving formatting).
#[elicit_tool(
    plugin = "toml",
    name = "toml__document__to_string",
    description = "Serialize a live DocumentMut back to TOML text, preserving whitespace and comments."
)]
async fn document_to_string(
    ctx: Arc<TomlCtx>,
    p: SerializeDocumentParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.document_id)?;
    let docs = ctx.documents.lock().unwrap();
    match docs.get(&id) {
        Some(doc) => ok_text(doc.to_string()),
        None => err_text(format!("document_id '{}' not found", p.document_id)),
    }
}

// ── DocumentMut: drop ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DropDocumentParams {
    /// UUID of the DocumentMut to drop.
    pub document_id: String,
}

/// Drop a live `DocumentMut` from memory, freeing its resources.
#[elicit_tool(
    plugin = "toml",
    name = "toml__document__drop",
    description = "Drop a live DocumentMut from the plugin context, freeing its memory."
)]
async fn drop_document(
    ctx: Arc<TomlCtx>,
    p: DropDocumentParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.document_id)?;
    let removed = ctx.documents.lock().unwrap().remove(&id).is_some();
    if removed {
        ok_text(format!("document '{}' dropped", p.document_id))
    } else {
        err_text(format!("document_id '{}' not found", p.document_id))
    }
}
