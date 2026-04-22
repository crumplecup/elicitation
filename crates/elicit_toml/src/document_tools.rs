//! Tools for inspecting and modifying live `DocumentMut` instances.

use std::sync::Arc;

use elicitation::elicit_tool;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::plugin::{TomlCtx, err_text, ok_json, ok_text, parse_uuid};

// ── New empty document ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NewDocumentParams {}

/// Create a new empty `DocumentMut` and return its UUID.
#[elicit_tool(
    plugin = "toml",
    name = "toml__document__new",
    description = "Create a new empty DocumentMut. Returns a document_id UUID."
)]
async fn document_new(
    ctx: Arc<TomlCtx>,
    _p: NewDocumentParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.documents
        .lock()
        .unwrap()
        .insert(id, toml_edit::DocumentMut::new());
    ok_text(id.to_string())
}

// ── Get a key ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DocumentGetParams {
    /// UUID of the DocumentMut.
    pub document_id: String,
    /// Key to look up.
    pub key: String,
}

/// Get the `Item` at a key in a `DocumentMut`. Returns the item as JSON, or null if absent.
#[elicit_tool(
    plugin = "toml",
    name = "toml__document__get",
    description = "Get the Item at a top-level key in a DocumentMut. Returns JSON-serialized Item, or null if absent."
)]
async fn document_get(
    ctx: Arc<TomlCtx>,
    p: DocumentGetParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.document_id)?;
    let docs = ctx.documents.lock().unwrap();
    match docs.get(&id) {
        Some(doc) => match doc.get(&p.key) {
            Some(item) => ok_text(item.to_string()),
            None => ok_text("null"),
        },
        None => err_text(format!("document_id '{}' not found", p.document_id)),
    }
}

// ── Insert / set a key ───────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DocumentInsertParams {
    /// UUID of the DocumentMut.
    pub document_id: String,
    /// Key to insert or replace.
    pub key: String,
    /// TOML text for the value, e.g. `"hello"` or `42` or `[1, 2, 3]`.
    pub value_toml: String,
}

/// Insert or replace a key in a `DocumentMut`.
///
/// `value_toml` is a TOML value fragment (e.g. `"hello"`, `42`, `[1,2,3]`,
/// or an inline table `{a=1,b=2}`).
#[elicit_tool(
    plugin = "toml",
    name = "toml__document__insert",
    description = "Insert or replace a top-level key in a DocumentMut. value_toml must be a valid TOML value fragment."
)]
async fn document_insert(
    ctx: Arc<TomlCtx>,
    p: DocumentInsertParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.document_id)?;
    // Parse the value fragment by wrapping it in a synthetic document
    let wrapper = format!("__v = {}", p.value_toml);
    let synthetic: toml_edit::DocumentMut = wrapper
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("value_toml parse error: {}", e), None))?;
    let item = synthetic
        .get("__v")
        .cloned()
        .unwrap_or(toml_edit::Item::None);
    let mut docs = ctx.documents.lock().unwrap();
    match docs.get_mut(&id) {
        Some(doc) => {
            doc.insert(&p.key, item);
            ok_text(format!(
                "inserted '{}' into document '{}'",
                p.key, p.document_id
            ))
        }
        None => err_text(format!("document_id '{}' not found", p.document_id)),
    }
}

// ── Remove a key ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DocumentRemoveParams {
    /// UUID of the DocumentMut.
    pub document_id: String,
    /// Key to remove.
    pub key: String,
}

/// Remove a key from a `DocumentMut`. Returns the removed item as JSON, or null.
#[elicit_tool(
    plugin = "toml",
    name = "toml__document__remove",
    description = "Remove a top-level key from a DocumentMut. Returns the removed Item as JSON, or null if absent."
)]
async fn document_remove(
    ctx: Arc<TomlCtx>,
    p: DocumentRemoveParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.document_id)?;
    let mut docs = ctx.documents.lock().unwrap();
    match docs.get_mut(&id) {
        Some(doc) => match doc.remove(&p.key) {
            Some(item) => ok_text(item.to_string()),
            None => ok_text("null"),
        },
        None => err_text(format!("document_id '{}' not found", p.document_id)),
    }
}

// ── Contains key ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DocumentContainsKeyParams {
    /// UUID of the DocumentMut.
    pub document_id: String,
    /// Key to check.
    pub key: String,
}

/// Check whether a `DocumentMut` contains a given key.
#[elicit_tool(
    plugin = "toml",
    name = "toml__document__contains_key",
    description = "Return true if the DocumentMut contains the given top-level key."
)]
async fn document_contains_key(
    ctx: Arc<TomlCtx>,
    p: DocumentContainsKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.document_id)?;
    let docs = ctx.documents.lock().unwrap();
    match docs.get(&id) {
        Some(doc) => ok_text(doc.contains_key(&p.key).to_string()),
        None => err_text(format!("document_id '{}' not found", p.document_id)),
    }
}

// ── List keys ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DocumentKeysParams {
    /// UUID of the DocumentMut.
    pub document_id: String,
}

/// List all top-level keys in a `DocumentMut` as a JSON array of strings.
#[elicit_tool(
    plugin = "toml",
    name = "toml__document__keys",
    description = "List all top-level keys in a DocumentMut as a JSON array."
)]
async fn document_keys(
    ctx: Arc<TomlCtx>,
    p: DocumentKeysParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.document_id)?;
    let docs = ctx.documents.lock().unwrap();
    match docs.get(&id) {
        Some(doc) => {
            let keys: Vec<&str> = doc.iter().map(|(k, _)| k).collect();
            ok_json(&keys)
        }
        None => err_text(format!("document_id '{}' not found", p.document_id)),
    }
}

// ── Length ───────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DocumentLenParams {
    /// UUID of the DocumentMut.
    pub document_id: String,
}

/// Return the number of top-level keys in a `DocumentMut`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__document__len",
    description = "Return the number of top-level key-value pairs in a DocumentMut."
)]
async fn document_len(
    ctx: Arc<TomlCtx>,
    p: DocumentLenParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.document_id)?;
    let docs = ctx.documents.lock().unwrap();
    match docs.get(&id) {
        Some(doc) => ok_text(doc.len().to_string()),
        None => err_text(format!("document_id '{}' not found", p.document_id)),
    }
}

// ── Is empty ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DocumentIsEmptyParams {
    /// UUID of the DocumentMut.
    pub document_id: String,
}

/// Return `true` if the `DocumentMut` has no top-level keys.
#[elicit_tool(
    plugin = "toml",
    name = "toml__document__is_empty",
    description = "Return true if the DocumentMut has no top-level keys."
)]
async fn document_is_empty(
    ctx: Arc<TomlCtx>,
    p: DocumentIsEmptyParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.document_id)?;
    let docs = ctx.documents.lock().unwrap();
    match docs.get(&id) {
        Some(doc) => ok_text(doc.is_empty().to_string()),
        None => err_text(format!("document_id '{}' not found", p.document_id)),
    }
}

// ── As table UUID ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DocumentAsTableParams {
    /// UUID of the DocumentMut.
    pub document_id: String,
}

/// Extract the root table of a `DocumentMut` as a live `Table` and return its UUID.
///
/// The returned table UUID can be used with `toml__table__*` tools.
#[elicit_tool(
    plugin = "toml",
    name = "toml__document__as_table",
    description = "Clone the root Table of a DocumentMut into the tables map. Returns a table_id UUID for toml__table__* tools."
)]
async fn document_as_table(
    ctx: Arc<TomlCtx>,
    p: DocumentAsTableParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.document_id)?;
    let docs = ctx.documents.lock().unwrap();
    match docs.get(&id) {
        Some(doc) => {
            let table = doc.as_table().clone();
            let tid = Uuid::new_v4();
            drop(docs);
            ctx.tables.lock().unwrap().insert(tid, table);
            ok_text(tid.to_string())
        }
        None => err_text(format!("document_id '{}' not found", p.document_id)),
    }
}

// ── Trailing whitespace ───────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DocumentTrailingParams {
    /// UUID of the DocumentMut.
    pub document_id: String,
}

/// Return the trailing whitespace/comment text at the end of a `DocumentMut`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__document__trailing",
    description = "Return the trailing whitespace or comment at the end of a DocumentMut, if any."
)]
async fn document_trailing(
    ctx: Arc<TomlCtx>,
    p: DocumentTrailingParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.document_id)?;
    let docs = ctx.documents.lock().unwrap();
    match docs.get(&id) {
        Some(doc) => ok_text(doc.trailing().as_str().unwrap_or("").to_string()),
        None => err_text(format!("document_id '{}' not found", p.document_id)),
    }
}
