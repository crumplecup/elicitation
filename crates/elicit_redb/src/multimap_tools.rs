//! `#[elicit_tool]` functions for `redb::MultimapTable` CRUD operations.
//!
//! Combinations covered:
//! - `(u64, u64)` — integer multimaps: tool prefix `redb__multimap_u64_u64__*`
//! - `(&str, &str)` — string multimaps: tool prefix `redb__multimap_str_str__*`
//! - `(&[u8], &[u8])` — binary multimaps (hex-encoded): tool prefix `redb__multimap_bytes_bytes__*`
//!
//! Each combination provides: insert, get (returns all values for a key), remove
//! (single key-value pair), remove_all (all values for a key), len, iter,
//! read_get, read_len.

use std::sync::Arc;

use elicitation::elicit_tool;
use redb::{ReadableMultimapTable as _, ReadableTableMetadata as _};
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::plugin::{RedbCtx, ok_json, ok_text, parse_uuid};

// ── shared output types ───────────────────────────────────────────────────────

#[derive(Serialize)]
struct ValuesResult {
    values: Vec<String>,
}

#[derive(Serialize)]
struct MultiEntryList {
    entries: Vec<MultiEntry>,
}

/// A serialized multimap key with all its associated values.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultiEntry {
    /// Serialized key.
    pub key: String,
    /// All values associated with this key.
    pub values: Vec<String>,
}

#[derive(Serialize)]
struct LenResult {
    len: u64,
}

fn err_parse(s: &str, e: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(format!("parse error for {s:?}: {e}"), None)
}

fn hex_decode(s: &str) -> Result<Vec<u8>, ErrorData> {
    hex::decode(s).map_err(|e| err_parse(s, e))
}

// ══════════════════════════════════════════════════════════════════════════════
// (u64, u64) multimaps
// ══════════════════════════════════════════════════════════════════════════════

/// Parameters for `multimap_u64_u64__insert`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmU64U64InsertParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
    /// Decimal u64 key.
    pub key: String,
    /// Decimal u64 value.
    pub value: String,
}

/// Parameters for key-only multimap operations (get, remove_all, read_get, read_len).
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmU64U64KeyParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
    /// Decimal u64 key.
    pub key: String,
}

/// Parameters for `multimap_u64_u64__remove` (key + value pair).
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmU64U64RemoveParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
    /// Decimal u64 key.
    pub key: String,
    /// Decimal u64 value to remove.
    pub value: String,
}

/// Parameters for table-level operations (len, iter).
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmU64U64TableParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
}

/// Parameters for read-only key-level operations.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmU64U64ReadKeyParams {
    /// UUID returned by `redb__database_begin_read`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
    /// Decimal u64 key.
    pub key: String,
}

/// Parameters for read-only table-level operations.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmU64U64ReadTableParams {
    /// UUID returned by `redb__database_begin_read`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_u64_u64__insert",
    description = "Insert a (u64, u64) value into a multimap table. Returns true if the value was newly inserted.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_u64_u64_insert(
    ctx: Arc<RedbCtx>,
    p: MmU64U64InsertParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k: u64 = p.key.parse().map_err(|e| err_parse(&p.key, e))?;
    let v: u64 = p.value.parse().map_err(|e| err_parse(&p.value, e))?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<u64, u64>::new(&p.table_name);
    let mut table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let inserted = table
        .insert(k, v)
        .map_err(|e| ErrorData::internal_error(format!("insert: {e}"), None))?;
    ok_text(if inserted {
        "inserted"
    } else {
        "already_present"
    })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_u64_u64__get",
    description = "Get all u64 values associated with a u64 key in a multimap table (write transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_u64_u64_get(
    ctx: Arc<RedbCtx>,
    p: MmU64U64KeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k: u64 = p.key.parse().map_err(|e| err_parse(&p.key, e))?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<u64, u64>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let values: Vec<String> = table
        .get(k)
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|r| r.map(|g| g.value().to_string()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| ErrorData::internal_error(format!("value iter: {e}"), None))?;
    ok_json(&ValuesResult { values })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_u64_u64__remove",
    description = "Remove a specific (u64, u64) key-value pair from a multimap table.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_u64_u64_remove(
    ctx: Arc<RedbCtx>,
    p: MmU64U64RemoveParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k: u64 = p.key.parse().map_err(|e| err_parse(&p.key, e))?;
    let v: u64 = p.value.parse().map_err(|e| err_parse(&p.value, e))?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<u64, u64>::new(&p.table_name);
    let mut table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let removed = table
        .remove(k, v)
        .map_err(|e| ErrorData::internal_error(format!("remove: {e}"), None))?;
    ok_text(if removed { "removed" } else { "not_found" })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_u64_u64__remove_all",
    description = "Remove all values associated with a u64 key from a multimap table.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_u64_u64_remove_all(
    ctx: Arc<RedbCtx>,
    p: MmU64U64KeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k: u64 = p.key.parse().map_err(|e| err_parse(&p.key, e))?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<u64, u64>::new(&p.table_name);
    let mut table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    table
        .remove_all(k)
        .map_err(|e| ErrorData::internal_error(format!("remove_all: {e}"), None))?;
    ok_text("removed")
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_u64_u64__len",
    description = "Return the total number of key-value pairs in a (u64, u64) multimap table.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_u64_u64_len(
    ctx: Arc<RedbCtx>,
    p: MmU64U64TableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<u64, u64>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let len = table
        .len()
        .map_err(|e| ErrorData::internal_error(format!("len: {e}"), None))?;
    ok_json(&LenResult { len })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_u64_u64__iter",
    description = "Iterate all (u64, u64) entries in a multimap table, grouped by key.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_u64_u64_iter(
    ctx: Arc<RedbCtx>,
    p: MmU64U64TableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<u64, u64>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let mut entries: Vec<MultiEntry> = Vec::new();
    for r in table
        .iter()
        .map_err(|e| ErrorData::internal_error(format!("iter: {e}"), None))?
    {
        let (k_guard, vs) =
            r.map_err(|e| ErrorData::internal_error(format!("iter entry: {e}"), None))?;
        let key = k_guard.value().to_string();
        let values = vs
            .map(|rv| rv.map(|g| g.value().to_string()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ErrorData::internal_error(format!("value iter: {e}"), None))?;
        entries.push(MultiEntry { key, values });
    }
    ok_json(&MultiEntryList { entries })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_u64_u64__read_get",
    description = "Get all u64 values for a u64 key from a multimap table (read-only transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_u64_u64_read_get(
    ctx: Arc<RedbCtx>,
    p: MmU64U64ReadKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k: u64 = p.key.parse().map_err(|e| err_parse(&p.key, e))?;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<u64, u64>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let values: Vec<String> = table
        .get(k)
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|r| r.map(|g| g.value().to_string()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| ErrorData::internal_error(format!("value iter: {e}"), None))?;
    ok_json(&ValuesResult { values })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_u64_u64__read_len",
    description = "Return the total number of key-value pairs in a (u64, u64) multimap table (read-only transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_u64_u64_read_len(
    ctx: Arc<RedbCtx>,
    p: MmU64U64ReadTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<u64, u64>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let len = table
        .len()
        .map_err(|e| ErrorData::internal_error(format!("len: {e}"), None))?;
    ok_json(&LenResult { len })
}

// ══════════════════════════════════════════════════════════════════════════════
// (&str, &str) multimaps
// ══════════════════════════════════════════════════════════════════════════════

/// Parameters for `multimap_str_str__insert`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmStrStrInsertParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
    /// String key.
    pub key: String,
    /// String value.
    pub value: String,
}

/// Parameters for key-only str multimap operations.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmStrStrKeyParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
    /// String key.
    pub key: String,
}

/// Parameters for `multimap_str_str__remove`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmStrStrRemoveParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
    /// String key.
    pub key: String,
    /// String value to remove.
    pub value: String,
}

/// Parameters for table-level str multimap operations.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmStrStrTableParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
}

/// Parameters for read-only str multimap key operations.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmStrStrReadKeyParams {
    /// UUID returned by `redb__database_begin_read`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
    /// String key.
    pub key: String,
}

/// Parameters for read-only str multimap table operations.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmStrStrReadTableParams {
    /// UUID returned by `redb__database_begin_read`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_str_str__insert",
    description = "Insert a (&str, &str) value into a multimap table. Returns true if newly inserted.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_str_str_insert(
    ctx: Arc<RedbCtx>,
    p: MmStrStrInsertParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&str, &str>::new(&p.table_name);
    let mut table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let inserted = table
        .insert(p.key.as_str(), p.value.as_str())
        .map_err(|e| ErrorData::internal_error(format!("insert: {e}"), None))?;
    ok_text(if inserted {
        "inserted"
    } else {
        "already_present"
    })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_str_str__get",
    description = "Get all string values associated with a string key in a multimap table (write transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_str_str_get(
    ctx: Arc<RedbCtx>,
    p: MmStrStrKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&str, &str>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let values: Vec<String> = table
        .get(p.key.as_str())
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|r| r.map(|g| g.value().to_owned()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| ErrorData::internal_error(format!("value iter: {e}"), None))?;
    ok_json(&ValuesResult { values })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_str_str__remove",
    description = "Remove a specific (&str, &str) key-value pair from a multimap table.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_str_str_remove(
    ctx: Arc<RedbCtx>,
    p: MmStrStrRemoveParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&str, &str>::new(&p.table_name);
    let mut table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let removed = table
        .remove(p.key.as_str(), p.value.as_str())
        .map_err(|e| ErrorData::internal_error(format!("remove: {e}"), None))?;
    ok_text(if removed { "removed" } else { "not_found" })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_str_str__remove_all",
    description = "Remove all values associated with a string key from a multimap table.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_str_str_remove_all(
    ctx: Arc<RedbCtx>,
    p: MmStrStrKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&str, &str>::new(&p.table_name);
    let mut table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    table
        .remove_all(p.key.as_str())
        .map_err(|e| ErrorData::internal_error(format!("remove_all: {e}"), None))?;
    ok_text("removed")
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_str_str__len",
    description = "Return the total number of key-value pairs in a (&str, &str) multimap table.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_str_str_len(
    ctx: Arc<RedbCtx>,
    p: MmStrStrTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&str, &str>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let len = table
        .len()
        .map_err(|e| ErrorData::internal_error(format!("len: {e}"), None))?;
    ok_json(&LenResult { len })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_str_str__iter",
    description = "Iterate all (&str, &str) entries in a multimap table, grouped by key.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_str_str_iter(
    ctx: Arc<RedbCtx>,
    p: MmStrStrTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&str, &str>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let mut entries: Vec<MultiEntry> = Vec::new();
    for r in table
        .iter()
        .map_err(|e| ErrorData::internal_error(format!("iter: {e}"), None))?
    {
        let (k_guard, vs) =
            r.map_err(|e| ErrorData::internal_error(format!("iter entry: {e}"), None))?;
        let key = k_guard.value().to_owned();
        let values = vs
            .map(|rv| rv.map(|g| g.value().to_owned()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ErrorData::internal_error(format!("value iter: {e}"), None))?;
        entries.push(MultiEntry { key, values });
    }
    ok_json(&MultiEntryList { entries })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_str_str__read_get",
    description = "Get all string values for a string key from a multimap table (read-only transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_str_str_read_get(
    ctx: Arc<RedbCtx>,
    p: MmStrStrReadKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&str, &str>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let values: Vec<String> = table
        .get(p.key.as_str())
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|r| r.map(|g| g.value().to_owned()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| ErrorData::internal_error(format!("value iter: {e}"), None))?;
    ok_json(&ValuesResult { values })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_str_str__read_len",
    description = "Return the total number of key-value pairs in a (&str, &str) multimap table (read-only transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_str_str_read_len(
    ctx: Arc<RedbCtx>,
    p: MmStrStrReadTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&str, &str>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let len = table
        .len()
        .map_err(|e| ErrorData::internal_error(format!("len: {e}"), None))?;
    ok_json(&LenResult { len })
}

// ══════════════════════════════════════════════════════════════════════════════
// (&[u8], &[u8]) multimaps (hex-encoded)
// ══════════════════════════════════════════════════════════════════════════════

/// Parameters for `multimap_bytes_bytes__insert`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmBytesBytesInsertParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
    /// Hex-encoded key bytes.
    pub key: String,
    /// Hex-encoded value bytes.
    pub value: String,
}

/// Parameters for key-only bytes multimap operations.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmBytesBytesKeyParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
    /// Hex-encoded key bytes.
    pub key: String,
}

/// Parameters for `multimap_bytes_bytes__remove`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmBytesBytesRemoveParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
    /// Hex-encoded key bytes.
    pub key: String,
    /// Hex-encoded value bytes to remove.
    pub value: String,
}

/// Parameters for table-level bytes multimap operations.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmBytesBytesTableParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
}

/// Parameters for read-only bytes multimap key operations.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmBytesBytesReadKeyParams {
    /// UUID returned by `redb__database_begin_read`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
    /// Hex-encoded key bytes.
    pub key: String,
}

/// Parameters for read-only bytes multimap table operations.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MmBytesBytesReadTableParams {
    /// UUID returned by `redb__database_begin_read`.
    pub txn_id: String,
    /// Multimap table name.
    pub table_name: String,
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_bytes_bytes__insert",
    description = "Insert a (&[u8], &[u8]) value into a multimap table (hex-encoded). Returns true if newly inserted.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_bytes_bytes_insert(
    ctx: Arc<RedbCtx>,
    p: MmBytesBytesInsertParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k = hex_decode(&p.key)?;
    let v = hex_decode(&p.value)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let mut table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let inserted = table
        .insert(k.as_slice(), v.as_slice())
        .map_err(|e| ErrorData::internal_error(format!("insert: {e}"), None))?;
    ok_text(if inserted {
        "inserted"
    } else {
        "already_present"
    })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_bytes_bytes__get",
    description = "Get all byte values for a hex-encoded key in a multimap table (write transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_bytes_bytes_get(
    ctx: Arc<RedbCtx>,
    p: MmBytesBytesKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k = hex_decode(&p.key)?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let values: Vec<String> = table
        .get(k.as_slice())
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|r| r.map(|g| hex::encode(g.value())))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| ErrorData::internal_error(format!("value iter: {e}"), None))?;
    ok_json(&ValuesResult { values })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_bytes_bytes__remove",
    description = "Remove a specific hex-encoded key-value pair from a bytes multimap table.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_bytes_bytes_remove(
    ctx: Arc<RedbCtx>,
    p: MmBytesBytesRemoveParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k = hex_decode(&p.key)?;
    let v = hex_decode(&p.value)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let mut table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let removed = table
        .remove(k.as_slice(), v.as_slice())
        .map_err(|e| ErrorData::internal_error(format!("remove: {e}"), None))?;
    ok_text(if removed { "removed" } else { "not_found" })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_bytes_bytes__remove_all",
    description = "Remove all values for a hex-encoded key from a bytes multimap table.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_bytes_bytes_remove_all(
    ctx: Arc<RedbCtx>,
    p: MmBytesBytesKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k = hex_decode(&p.key)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let mut table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    table
        .remove_all(k.as_slice())
        .map_err(|e| ErrorData::internal_error(format!("remove_all: {e}"), None))?;
    ok_text("removed")
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_bytes_bytes__len",
    description = "Return the total number of key-value pairs in a (&[u8], &[u8]) multimap table.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_bytes_bytes_len(
    ctx: Arc<RedbCtx>,
    p: MmBytesBytesTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let len = table
        .len()
        .map_err(|e| ErrorData::internal_error(format!("len: {e}"), None))?;
    ok_json(&LenResult { len })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_bytes_bytes__iter",
    description = "Iterate all (&[u8], &[u8]) entries in a multimap table, grouped by key (hex-encoded).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_bytes_bytes_iter(
    ctx: Arc<RedbCtx>,
    p: MmBytesBytesTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let mut entries: Vec<MultiEntry> = Vec::new();
    for r in table
        .iter()
        .map_err(|e| ErrorData::internal_error(format!("iter: {e}"), None))?
    {
        let (k_guard, vs) =
            r.map_err(|e| ErrorData::internal_error(format!("iter entry: {e}"), None))?;
        let key = hex::encode(k_guard.value());
        let values = vs
            .map(|rv| rv.map(|g| hex::encode(g.value())))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ErrorData::internal_error(format!("value iter: {e}"), None))?;
        entries.push(MultiEntry { key, values });
    }
    ok_json(&MultiEntryList { entries })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_bytes_bytes__read_get",
    description = "Get all byte values for a hex-encoded key from a bytes multimap table (read-only transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_bytes_bytes_read_get(
    ctx: Arc<RedbCtx>,
    p: MmBytesBytesReadKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k = hex_decode(&p.key)?;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let values: Vec<String> = table
        .get(k.as_slice())
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|r| r.map(|g| hex::encode(g.value())))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| ErrorData::internal_error(format!("value iter: {e}"), None))?;
    ok_json(&ValuesResult { values })
}

#[elicit_tool(
    plugin = "redb",
    name = "multimap_bytes_bytes__read_len",
    description = "Return the total number of key-value pairs in a (&[u8], &[u8]) multimap table (read-only transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn multimap_bytes_bytes_read_len(
    ctx: Arc<RedbCtx>,
    p: MmBytesBytesReadTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::MultimapTableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let table = txn
        .open_multimap_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_multimap_table: {e}"), None))?;
    let len = table
        .len()
        .map_err(|e| ErrorData::internal_error(format!("len: {e}"), None))?;
    ok_json(&LenResult { len })
}
