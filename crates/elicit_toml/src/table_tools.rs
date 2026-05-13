//! Tools for inspecting and modifying live `Table` and `InlineTable` instances.

use std::sync::Arc;

use elicitation::elicit_tool;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::plugin::{TomlCtx, err_text, ok_json, ok_text, parse_uuid};

// ── Table: new ───────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableNewParams {}

/// Create a new empty `Table` and return its UUID.
#[elicit_tool(
    plugin = "toml",
    name = "toml__table__new",
    description = "Create a new empty Table. Returns a table_id UUID for toml__table__* tools."
)]
async fn table_new(ctx: Arc<TomlCtx>, _p: TableNewParams) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.tables
        .lock()
        .unwrap()
        .insert(id, toml_edit::Table::new());
    ok_text(id.to_string())
}

// ── Table: get ───────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableGetParams {
    /// UUID of the Table.
    pub table_id: String,
    /// Key to look up.
    pub key: String,
}

/// Get the `Item` at a key in a `Table`. Returns JSON or "null".
#[elicit_tool(
    plugin = "toml",
    name = "toml__table__get",
    description = "Get the Item at a key in a Table. Returns JSON-serialized Item, or null."
)]
async fn table_get(ctx: Arc<TomlCtx>, p: TableGetParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.table_id)?;
    let tables = ctx.tables.lock().unwrap();
    match tables.get(&id) {
        Some(t) => match t.get(&p.key) {
            Some(item) => ok_text(item.to_string()),
            None => ok_text("null"),
        },
        None => err_text(format!("table_id '{}' not found", p.table_id)),
    }
}

// ── Table: insert ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableInsertParams {
    /// UUID of the Table.
    pub table_id: String,
    /// Key to insert or replace.
    pub key: String,
    /// TOML value fragment, e.g. `"hello"`, `42`, `[1,2,3]`.
    pub value_toml: String,
}

/// Insert or replace a key in a `Table`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__table__insert",
    description = "Insert or replace a key in a Table. value_toml must be a valid TOML value fragment."
)]
async fn table_insert(
    ctx: Arc<TomlCtx>,
    p: TableInsertParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.table_id)?;
    let wrapper = format!("__v = {}", p.value_toml);
    let synthetic: toml_edit::DocumentMut = wrapper
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("value_toml parse error: {}", e), None))?;
    let item = synthetic
        .get("__v")
        .cloned()
        .unwrap_or(toml_edit::Item::None);
    let mut tables = ctx.tables.lock().unwrap();
    match tables.get_mut(&id) {
        Some(t) => {
            t.insert(&p.key, item);
            ok_text(format!("inserted '{}' into table '{}'", p.key, p.table_id))
        }
        None => err_text(format!("table_id '{}' not found", p.table_id)),
    }
}

// ── Table: remove ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableRemoveParams {
    /// UUID of the Table.
    pub table_id: String,
    /// Key to remove.
    pub key: String,
}

/// Remove a key from a `Table`. Returns the removed item as JSON, or "null".
#[elicit_tool(
    plugin = "toml",
    name = "toml__table__remove",
    description = "Remove a key from a Table. Returns the removed Item as JSON, or null."
)]
async fn table_remove(
    ctx: Arc<TomlCtx>,
    p: TableRemoveParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.table_id)?;
    let mut tables = ctx.tables.lock().unwrap();
    match tables.get_mut(&id) {
        Some(t) => match t.remove(&p.key) {
            Some(item) => ok_text(item.to_string()),
            None => ok_text("null"),
        },
        None => err_text(format!("table_id '{}' not found", p.table_id)),
    }
}

// ── Table: contains_key ──────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableContainsKeyParams {
    /// UUID of the Table.
    pub table_id: String,
    /// Key to check.
    pub key: String,
}

/// Check whether a `Table` contains a given key.
#[elicit_tool(
    plugin = "toml",
    name = "toml__table__contains_key",
    description = "Return true if the Table contains the given key."
)]
async fn table_contains_key(
    ctx: Arc<TomlCtx>,
    p: TableContainsKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.table_id)?;
    let tables = ctx.tables.lock().unwrap();
    match tables.get(&id) {
        Some(t) => ok_text(t.contains_key(&p.key).to_string()),
        None => err_text(format!("table_id '{}' not found", p.table_id)),
    }
}

// ── Table: keys ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableKeysParams {
    /// UUID of the Table.
    pub table_id: String,
}

/// List all keys in a `Table` as a JSON array of strings.
#[elicit_tool(
    plugin = "toml",
    name = "toml__table__keys",
    description = "List all keys in a Table as a JSON array of strings."
)]
async fn table_keys(ctx: Arc<TomlCtx>, p: TableKeysParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.table_id)?;
    let tables = ctx.tables.lock().unwrap();
    match tables.get(&id) {
        Some(t) => {
            let keys: Vec<&str> = t.iter().map(|(k, _)| k).collect();
            ok_json(&keys)
        }
        None => err_text(format!("table_id '{}' not found", p.table_id)),
    }
}

// ── Table: len ───────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableLenParams {
    /// UUID of the Table.
    pub table_id: String,
}

/// Return the number of entries in a `Table`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__table__len",
    description = "Return the number of key-value pairs in a Table."
)]
async fn table_len(ctx: Arc<TomlCtx>, p: TableLenParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.table_id)?;
    let tables = ctx.tables.lock().unwrap();
    match tables.get(&id) {
        Some(t) => ok_text(t.len().to_string()),
        None => err_text(format!("table_id '{}' not found", p.table_id)),
    }
}

// ── Table: is_empty ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableIsEmptyParams {
    /// UUID of the Table.
    pub table_id: String,
}

/// Return `true` if the `Table` has no entries.
#[elicit_tool(
    plugin = "toml",
    name = "toml__table__is_empty",
    description = "Return true if the Table has no entries."
)]
async fn table_is_empty(
    ctx: Arc<TomlCtx>,
    p: TableIsEmptyParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.table_id)?;
    let tables = ctx.tables.lock().unwrap();
    match tables.get(&id) {
        Some(t) => ok_text(t.is_empty().to_string()),
        None => err_text(format!("table_id '{}' not found", p.table_id)),
    }
}

// ── Table: clear ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableClearParams {
    /// UUID of the Table.
    pub table_id: String,
}

/// Remove all entries from a `Table`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__table__clear",
    description = "Remove all entries from a Table."
)]
async fn table_clear(ctx: Arc<TomlCtx>, p: TableClearParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.table_id)?;
    let mut tables = ctx.tables.lock().unwrap();
    match tables.get_mut(&id) {
        Some(t) => {
            t.clear();
            ok_text(format!("table '{}' cleared", p.table_id))
        }
        None => err_text(format!("table_id '{}' not found", p.table_id)),
    }
}

// ── Table: sort_values ───────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableSortValuesParams {
    /// UUID of the Table.
    pub table_id: String,
}

/// Sort a `Table`'s entries by key (alphabetical).
#[elicit_tool(
    plugin = "toml",
    name = "toml__table__sort_values",
    description = "Sort all entries in a Table alphabetically by key."
)]
async fn table_sort_values(
    ctx: Arc<TomlCtx>,
    p: TableSortValuesParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.table_id)?;
    let mut tables = ctx.tables.lock().unwrap();
    match tables.get_mut(&id) {
        Some(t) => {
            t.sort_values();
            ok_text(format!("table '{}' sorted", p.table_id))
        }
        None => err_text(format!("table_id '{}' not found", p.table_id)),
    }
}

// ── Table: set_dotted ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableSetDottedParams {
    /// UUID of the Table.
    pub table_id: String,
    /// If true, render as a dotted-key table; otherwise as a standard `[section]`.
    pub dotted: bool,
}

/// Set whether a `Table` should be rendered as a dotted-key table.
#[elicit_tool(
    plugin = "toml",
    name = "toml__table__set_dotted",
    description = "Set whether a Table renders as dotted keys (a.b.c = ...) or a standard [section]."
)]
async fn table_set_dotted(
    ctx: Arc<TomlCtx>,
    p: TableSetDottedParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.table_id)?;
    let mut tables = ctx.tables.lock().unwrap();
    match tables.get_mut(&id) {
        Some(t) => {
            t.set_dotted(p.dotted);
            ok_text(format!("table '{}' dotted={}", p.table_id, p.dotted))
        }
        None => err_text(format!("table_id '{}' not found", p.table_id)),
    }
}

// ── Table: is_dotted ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableIsDottedParams {
    /// UUID of the Table.
    pub table_id: String,
}

/// Return whether a `Table` is rendered as dotted keys.
#[elicit_tool(
    plugin = "toml",
    name = "toml__table__is_dotted",
    description = "Return true if the Table is configured to render as dotted keys."
)]
async fn table_is_dotted(
    ctx: Arc<TomlCtx>,
    p: TableIsDottedParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.table_id)?;
    let tables = ctx.tables.lock().unwrap();
    match tables.get(&id) {
        Some(t) => ok_text(t.is_dotted().to_string()),
        None => err_text(format!("table_id '{}' not found", p.table_id)),
    }
}

// ── Table: drop ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableDropParams {
    /// UUID of the Table to drop.
    pub table_id: String,
}

/// Drop a live `Table` from the plugin context.
#[elicit_tool(
    plugin = "toml",
    name = "toml__table__drop",
    description = "Drop a live Table from the plugin context, freeing its memory."
)]
async fn table_drop(ctx: Arc<TomlCtx>, p: TableDropParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.table_id)?;
    let removed = ctx.tables.lock().unwrap().remove(&id).is_some();
    if removed {
        ok_text(format!("table '{}' dropped", p.table_id))
    } else {
        err_text(format!("table_id '{}' not found", p.table_id))
    }
}

// ── InlineTable: new ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InlineTableNewParams {}

/// Create a new empty `InlineTable` and return its UUID.
#[elicit_tool(
    plugin = "toml",
    name = "toml__inline_table__new",
    description = "Create a new empty InlineTable. Returns an inline_table_id UUID."
)]
async fn inline_table_new(
    ctx: Arc<TomlCtx>,
    _p: InlineTableNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.inline_tables
        .lock()
        .unwrap()
        .insert(id, toml_edit::InlineTable::new());
    ok_text(id.to_string())
}

// ── InlineTable: insert ──────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InlineTableInsertParams {
    /// UUID of the InlineTable.
    pub inline_table_id: String,
    /// Key to insert or replace.
    pub key: String,
    /// TOML value fragment.
    pub value_toml: String,
}

/// Insert or replace a key in an `InlineTable`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__inline_table__insert",
    description = "Insert or replace a key in an InlineTable. value_toml must be a valid TOML value fragment."
)]
async fn inline_table_insert(
    ctx: Arc<TomlCtx>,
    p: InlineTableInsertParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.inline_table_id)?;
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
            let mut inline_tables = ctx.inline_tables.lock().unwrap();
            match inline_tables.get_mut(&id) {
                Some(it) => {
                    it.insert(&p.key, v);
                    ok_text(format!(
                        "inserted '{}' into inline_table '{}'",
                        p.key, p.inline_table_id
                    ))
                }
                None => err_text(format!("inline_table_id '{}' not found", p.inline_table_id)),
            }
        }
        None => err_text("value_toml must be a scalar or array, not a table section"),
    }
}

// ── InlineTable: get ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InlineTableGetParams {
    /// UUID of the InlineTable.
    pub inline_table_id: String,
    /// Key to look up.
    pub key: String,
}

/// Get the value at a key in an `InlineTable`. Returns JSON or "null".
#[elicit_tool(
    plugin = "toml",
    name = "toml__inline_table__get",
    description = "Get the Value at a key in an InlineTable. Returns JSON-serialized Value, or null."
)]
async fn inline_table_get(
    ctx: Arc<TomlCtx>,
    p: InlineTableGetParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.inline_table_id)?;
    let inline_tables = ctx.inline_tables.lock().unwrap();
    match inline_tables.get(&id) {
        Some(it) => match it.get(&p.key) {
            Some(val) => ok_text(val.to_string()),
            None => ok_text("null"),
        },
        None => err_text(format!("inline_table_id '{}' not found", p.inline_table_id)),
    }
}

// ── InlineTable: to_string ───────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InlineTableToStringParams {
    /// UUID of the InlineTable.
    pub inline_table_id: String,
}

/// Serialize an `InlineTable` to its TOML inline representation (e.g. `{a=1, b=2}`).
#[elicit_tool(
    plugin = "toml",
    name = "toml__inline_table__to_string",
    description = "Serialize an InlineTable to its TOML inline representation, e.g. {a=1, b=2}."
)]
async fn inline_table_to_string(
    ctx: Arc<TomlCtx>,
    p: InlineTableToStringParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.inline_table_id)?;
    let inline_tables = ctx.inline_tables.lock().unwrap();
    match inline_tables.get(&id) {
        Some(it) => {
            // Render via a temporary document
            let mut doc = toml_edit::DocumentMut::new();
            doc.insert(
                "__v",
                toml_edit::Item::Value(toml_edit::Value::InlineTable(it.clone())),
            );
            let s = doc.to_string();
            // Strip the synthetic key
            let rendered = s.trim_start_matches("__v = ").trim().to_string();
            ok_text(rendered)
        }
        None => err_text(format!("inline_table_id '{}' not found", p.inline_table_id)),
    }
}

// ── InlineTable: len ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InlineTableLenParams {
    /// UUID of the InlineTable.
    pub inline_table_id: String,
}

/// Return the number of entries in an `InlineTable`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__inline_table__len",
    description = "Return the number of key-value pairs in an InlineTable."
)]
async fn inline_table_len(
    ctx: Arc<TomlCtx>,
    p: InlineTableLenParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.inline_table_id)?;
    let inline_tables = ctx.inline_tables.lock().unwrap();
    match inline_tables.get(&id) {
        Some(it) => ok_text(it.len().to_string()),
        None => err_text(format!("inline_table_id '{}' not found", p.inline_table_id)),
    }
}

// ── InlineTable: drop ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InlineTableDropParams {
    /// UUID of the InlineTable to drop.
    pub inline_table_id: String,
}

/// Drop a live `InlineTable` from the plugin context.
#[elicit_tool(
    plugin = "toml",
    name = "toml__inline_table__drop",
    description = "Drop a live InlineTable from the plugin context, freeing its memory."
)]
async fn inline_table_drop(
    ctx: Arc<TomlCtx>,
    p: InlineTableDropParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.inline_table_id)?;
    let removed = ctx.inline_tables.lock().unwrap().remove(&id).is_some();
    if removed {
        ok_text(format!("inline_table '{}' dropped", p.inline_table_id))
    } else {
        err_text(format!("inline_table_id '{}' not found", p.inline_table_id))
    }
}
