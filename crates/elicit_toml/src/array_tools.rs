//! Tools for working with live `Array` and `ArrayOfTables` instances.

use std::sync::Arc;

use elicitation::elicit_tool;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::plugin::{TomlCtx, err_text, ok_text, parse_uuid};

// ── Array: new ───────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ArrayNewParams {}

/// Create a new empty `Array` and return its UUID.
#[elicit_tool(
    plugin = "toml",
    name = "toml__array__new",
    description = "Create a new empty Array. Returns an array_id UUID for toml__array__* tools."
)]
async fn array_new(ctx: Arc<TomlCtx>, _p: ArrayNewParams) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.arrays
        .lock()
        .unwrap()
        .insert(id, toml_edit::Array::new());
    ok_text(id.to_string())
}

// ── Array: push ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ArrayPushParams {
    /// UUID of the Array.
    pub array_id: String,
    /// TOML value fragment to append, e.g. `42` or `"hello"`.
    pub value_toml: String,
}

/// Append a value to an `Array`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__array__push",
    description = "Append a value to an Array. value_toml must be a valid TOML scalar or inline value fragment."
)]
async fn array_push(ctx: Arc<TomlCtx>, p: ArrayPushParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.array_id)?;
    let wrapper = format!("__v = {}", p.value_toml);
    let synthetic: toml_edit::DocumentMut = wrapper
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("value_toml parse error: {}", e), None))?;
    let val = synthetic
        .get("__v")
        .and_then(|item| item.as_value())
        .cloned();
    match val {
        Some(v) => {
            let mut arrays = ctx.arrays.lock().unwrap();
            match arrays.get_mut(&id) {
                Some(arr) => {
                    arr.push_formatted(v);
                    ok_text(format!("pushed to array '{}'", p.array_id))
                }
                None => err_text(format!("array_id '{}' not found", p.array_id)),
            }
        }
        None => err_text("value_toml must be a scalar or inline value, not a table section"),
    }
}

// ── Array: get ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ArrayGetParams {
    /// UUID of the Array.
    pub array_id: String,
    /// Zero-based index of the element to retrieve.
    pub index: usize,
}

/// Get the value at an index in an `Array`. Returns JSON or "null" if out of bounds.
#[elicit_tool(
    plugin = "toml",
    name = "toml__array__get",
    description = "Get the value at an index in an Array. Returns JSON-serialized value, or null if out of bounds."
)]
async fn array_get(ctx: Arc<TomlCtx>, p: ArrayGetParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.array_id)?;
    let arrays = ctx.arrays.lock().unwrap();
    match arrays.get(&id) {
        Some(arr) => match arr.get(p.index) {
            Some(val) => ok_text(val.to_string()),
            None => ok_text("null"),
        },
        None => err_text(format!("array_id '{}' not found", p.array_id)),
    }
}

// ── Array: remove ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ArrayRemoveParams {
    /// UUID of the Array.
    pub array_id: String,
    /// Zero-based index of the element to remove.
    pub index: usize,
}

/// Remove the element at an index from an `Array`. Returns the removed value as JSON.
#[elicit_tool(
    plugin = "toml",
    name = "toml__array__remove",
    description = "Remove the element at an index from an Array. Returns the removed value as JSON."
)]
async fn array_remove(
    ctx: Arc<TomlCtx>,
    p: ArrayRemoveParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.array_id)?;
    let mut arrays = ctx.arrays.lock().unwrap();
    match arrays.get_mut(&id) {
        Some(arr) => {
            if p.index < arr.len() {
                let removed = arr.remove(p.index);
                ok_text(removed.to_string())
            } else {
                err_text(format!(
                    "index {} out of bounds for array of length {}",
                    p.index,
                    arr.len()
                ))
            }
        }
        None => err_text(format!("array_id '{}' not found", p.array_id)),
    }
}

// ── Array: len ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ArrayLenParams {
    /// UUID of the Array.
    pub array_id: String,
}

/// Return the number of elements in an `Array`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__array__len",
    description = "Return the number of elements in an Array."
)]
async fn array_len(ctx: Arc<TomlCtx>, p: ArrayLenParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.array_id)?;
    let arrays = ctx.arrays.lock().unwrap();
    match arrays.get(&id) {
        Some(arr) => ok_text(arr.len().to_string()),
        None => err_text(format!("array_id '{}' not found", p.array_id)),
    }
}

// ── Array: is_empty ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ArrayIsEmptyParams {
    /// UUID of the Array.
    pub array_id: String,
}

/// Return `true` if the `Array` has no elements.
#[elicit_tool(
    plugin = "toml",
    name = "toml__array__is_empty",
    description = "Return true if the Array has no elements."
)]
async fn array_is_empty(
    ctx: Arc<TomlCtx>,
    p: ArrayIsEmptyParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.array_id)?;
    let arrays = ctx.arrays.lock().unwrap();
    match arrays.get(&id) {
        Some(arr) => ok_text(arr.is_empty().to_string()),
        None => err_text(format!("array_id '{}' not found", p.array_id)),
    }
}

// ── Array: clear ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ArrayClearParams {
    /// UUID of the Array.
    pub array_id: String,
}

/// Remove all elements from an `Array`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__array__clear",
    description = "Remove all elements from an Array."
)]
async fn array_clear(ctx: Arc<TomlCtx>, p: ArrayClearParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.array_id)?;
    let mut arrays = ctx.arrays.lock().unwrap();
    match arrays.get_mut(&id) {
        Some(arr) => {
            arr.clear();
            ok_text(format!("array '{}' cleared", p.array_id))
        }
        None => err_text(format!("array_id '{}' not found", p.array_id)),
    }
}

// ── Array: to_string ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ArrayToStringParams {
    /// UUID of the Array.
    pub array_id: String,
}

/// Serialize an `Array` to its TOML inline representation.
#[elicit_tool(
    plugin = "toml",
    name = "toml__array__to_string",
    description = "Serialize an Array to its TOML inline representation, e.g. [1, 2, 3]."
)]
async fn array_to_string(
    ctx: Arc<TomlCtx>,
    p: ArrayToStringParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.array_id)?;
    let arrays = ctx.arrays.lock().unwrap();
    match arrays.get(&id) {
        Some(arr) => {
            let mut doc = toml_edit::DocumentMut::new();
            doc.insert(
                "__v",
                toml_edit::Item::Value(toml_edit::Value::Array(arr.clone())),
            );
            let s = doc.to_string();
            let rendered = s.trim_start_matches("__v = ").trim().to_string();
            ok_text(rendered)
        }
        None => err_text(format!("array_id '{}' not found", p.array_id)),
    }
}

// ── Array: drop ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ArrayDropParams {
    /// UUID of the Array to drop.
    pub array_id: String,
}

/// Drop a live `Array` from the plugin context.
#[elicit_tool(
    plugin = "toml",
    name = "toml__array__drop",
    description = "Drop a live Array from the plugin context, freeing its memory."
)]
async fn array_drop(ctx: Arc<TomlCtx>, p: ArrayDropParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.array_id)?;
    let removed = ctx.arrays.lock().unwrap().remove(&id).is_some();
    if removed {
        ok_text(format!("array '{}' dropped", p.array_id))
    } else {
        err_text(format!("array_id '{}' not found", p.array_id))
    }
}

// ── ArrayOfTables: snippet ───────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ArrayOfTablesSnippetParams {
    /// Name of the array-of-tables key (e.g. "products").
    pub key: String,
    /// Number of example table entries to generate.
    #[serde(default = "default_entries")]
    pub entries: usize,
    /// Example field names for each entry (comma-separated).
    #[serde(default)]
    pub fields: String,
}

fn default_entries() -> usize {
    2
}

/// Generate a TOML `[[key]]` array-of-tables scaffold snippet.
///
/// `toml_edit::ArrayOfTables` is a live editing type; its most common use is
/// reading `[[key]]` sections that were parsed from a document. This tool
/// provides the boilerplate TOML source for authoring such sections.
#[elicit_tool(
    plugin = "toml",
    name = "toml__array_of_tables__snippet",
    description = "Generate a [[key]] array-of-tables TOML snippet. Useful for scaffolding multi-entry configuration."
)]
async fn array_of_tables_snippet(
    p: ArrayOfTablesSnippetParams,
) -> Result<CallToolResult, ErrorData> {
    let fields: Vec<&str> = if p.fields.is_empty() {
        vec!["field1", "field2"]
    } else {
        p.fields.split(',').map(str::trim).collect()
    };

    let mut out = String::new();
    for _ in 0..p.entries.max(1) {
        out.push_str(&format!("\n[[{}]]\n", p.key));
        for field in &fields {
            out.push_str(&format!("{} = \"value\"\n", field));
        }
    }
    ok_text(out.trim_start_matches('\n').to_string())
}
