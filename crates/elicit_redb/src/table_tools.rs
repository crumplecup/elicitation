//! Typed table CRUD tools for the three primary `(K, V)` combinations.
//!
//! Combinations covered:
//! - `(u64, u64)` — integer tables: tool prefix `redb__table_u64_u64__*`
//! - `(&str, &str)` — string tables: tool prefix `redb__table_str_str__*`
//! - `(&[u8], &[u8])` — binary tables (hex-encoded): tool prefix `redb__table_bytes_bytes__*`
//!
//! Each combination provides: insert, get, remove, len, pop_first, pop_last,
//! read_get (read-only transaction), read_len (read-only transaction).

use std::sync::Arc;

use elicitation::elicit_tool;
use redb::{ReadableTable as _, ReadableTableMetadata as _};
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::plugin::{RedbCtx, ok_json, ok_text, parse_uuid};

// ── shared output types ───────────────────────────────────────────────────────

#[derive(Serialize)]
struct MaybeEntry {
    entry: Option<KvEntry>,
}

/// A serialized key-value pair.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KvEntry {
    /// Serialized key.
    pub key: String,
    /// Serialized value.
    pub value: String,
}

#[derive(Serialize)]
struct LenResult {
    len: u64,
}

fn err_parse(s: &str, e: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(format!("parse error for {s:?}: {e}"), None)
}

// ══════════════════════════════════════════════════════════════════════════════
// (u64, u64) tables
// ══════════════════════════════════════════════════════════════════════════════

/// Parameters for `table_u64_u64__insert`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableU64U64InsertParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
    /// Decimal u64 key.
    pub key: String,
    /// Decimal u64 value.
    pub value: String,
}

/// Parameters for `table_u64_u64__get`, `table_u64_u64__remove`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableU64U64KeyParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
    /// Decimal u64 key.
    pub key: String,
}

/// Parameters for `table_u64_u64__len`, `table_u64_u64__pop_first`, `table_u64_u64__pop_last`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableU64U64TableParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
}

/// Parameters for read-only u64 table operations.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableU64U64ReadKeyParams {
    /// UUID returned by `redb__database_begin_read`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
    /// Decimal u64 key.
    pub key: String,
}

/// Parameters for read-only u64 table length.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableU64U64ReadTableParams {
    /// UUID returned by `redb__database_begin_read`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
}

#[elicit_tool(
    plugin = "redb",
    name = "table_u64_u64__insert",
    description = "Insert a (u64, u64) entry into a named table within a write transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_u64_u64_insert(
    ctx: Arc<RedbCtx>,
    p: TableU64U64InsertParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k: u64 = p.key.parse().map_err(|e| err_parse(&p.key, e))?;
    let v: u64 = p.value.parse().map_err(|e| err_parse(&p.value, e))?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<u64, u64>::new(&p.table_name);
    let mut table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    table
        .insert(k, v)
        .map_err(|e| ErrorData::internal_error(format!("insert: {e}"), None))?;
    ok_text("inserted")
}

#[elicit_tool(
    plugin = "redb",
    name = "table_u64_u64__get",
    description = "Get a u64 value by u64 key from a named table within a write transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_u64_u64_get(
    ctx: Arc<RedbCtx>,
    p: TableU64U64KeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k: u64 = p.key.parse().map_err(|e| err_parse(&p.key, e))?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<u64, u64>::new(&p.table_name);
    let table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let entry = table
        .get(k)
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|g| KvEntry {
            key: k.to_string(),
            value: g.value().to_string(),
        });
    ok_json(&MaybeEntry { entry })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_u64_u64__remove",
    description = "Remove a u64 key from a named table within a write transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_u64_u64_remove(
    ctx: Arc<RedbCtx>,
    p: TableU64U64KeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k: u64 = p.key.parse().map_err(|e| err_parse(&p.key, e))?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<u64, u64>::new(&p.table_name);
    let mut table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let removed = table
        .remove(k)
        .map_err(|e| ErrorData::internal_error(format!("remove: {e}"), None))?;
    ok_text(if removed.is_some() {
        "removed"
    } else {
        "not_found"
    })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_u64_u64__len",
    description = "Return the number of entries in a (u64, u64) table (write transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_u64_u64_len(
    ctx: Arc<RedbCtx>,
    p: TableU64U64TableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<u64, u64>::new(&p.table_name);
    let table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let len = table
        .len()
        .map_err(|e| ErrorData::internal_error(format!("len: {e}"), None))?;
    ok_json(&LenResult { len })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_u64_u64__pop_first",
    description = "Remove and return the first (lowest key) (u64, u64) entry from a named table.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_u64_u64_pop_first(
    ctx: Arc<RedbCtx>,
    p: TableU64U64TableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<u64, u64>::new(&p.table_name);
    let mut table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let entry = table
        .pop_first()
        .map_err(|e| ErrorData::internal_error(format!("pop_first: {e}"), None))?
        .map(|(k, v)| KvEntry {
            key: k.value().to_string(),
            value: v.value().to_string(),
        });
    ok_json(&MaybeEntry { entry })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_u64_u64__pop_last",
    description = "Remove and return the last (highest key) (u64, u64) entry from a named table.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_u64_u64_pop_last(
    ctx: Arc<RedbCtx>,
    p: TableU64U64TableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<u64, u64>::new(&p.table_name);
    let mut table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let entry = table
        .pop_last()
        .map_err(|e| ErrorData::internal_error(format!("pop_last: {e}"), None))?
        .map(|(k, v)| KvEntry {
            key: k.value().to_string(),
            value: v.value().to_string(),
        });
    ok_json(&MaybeEntry { entry })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_u64_u64__read_get",
    description = "Get a u64 value by u64 key from a named table within a read-only transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_u64_u64_read_get(
    ctx: Arc<RedbCtx>,
    p: TableU64U64ReadKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k: u64 = p.key.parse().map_err(|e| err_parse(&p.key, e))?;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<u64, u64>::new(&p.table_name);
    let table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let entry = table
        .get(k)
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|g| KvEntry {
            key: k.to_string(),
            value: g.value().to_string(),
        });
    ok_json(&MaybeEntry { entry })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_u64_u64__read_len",
    description = "Return the number of entries in a (u64, u64) table (read-only transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_u64_u64_read_len(
    ctx: Arc<RedbCtx>,
    p: TableU64U64ReadTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<u64, u64>::new(&p.table_name);
    let table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let len = table
        .len()
        .map_err(|e| ErrorData::internal_error(format!("len: {e}"), None))?;
    ok_json(&LenResult { len })
}

// ══════════════════════════════════════════════════════════════════════════════
// (&str, &str) tables
// ══════════════════════════════════════════════════════════════════════════════

/// Parameters for `table_str_str__insert`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableStrStrInsertParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
    /// String key.
    pub key: String,
    /// String value.
    pub value: String,
}

/// Parameters for `table_str_str__get`, `table_str_str__remove`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableStrStrKeyParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
    /// String key.
    pub key: String,
}

/// Parameters for `table_str_str__len`, `table_str_str__pop_first`, `table_str_str__pop_last`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableStrStrTableParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
}

/// Parameters for read-only str table operations.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableStrStrReadKeyParams {
    /// UUID returned by `redb__database_begin_read`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
    /// String key.
    pub key: String,
}

/// Parameters for read-only str table length.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableStrStrReadTableParams {
    /// UUID returned by `redb__database_begin_read`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
}

#[elicit_tool(
    plugin = "redb",
    name = "table_str_str__insert",
    description = "Insert a (&str, &str) entry into a named table within a write transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_str_str_insert(
    ctx: Arc<RedbCtx>,
    p: TableStrStrInsertParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k: &str = &p.key;
    let v: &str = &p.value;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&str, &str>::new(&p.table_name);
    let mut table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    table
        .insert(k, v)
        .map_err(|e| ErrorData::internal_error(format!("insert: {e}"), None))?;
    ok_text("inserted")
}

#[elicit_tool(
    plugin = "redb",
    name = "table_str_str__get",
    description = "Get a string value by string key from a named table within a write transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_str_str_get(
    ctx: Arc<RedbCtx>,
    p: TableStrStrKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k: &str = &p.key;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&str, &str>::new(&p.table_name);
    let table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let entry = table
        .get(k)
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|g| KvEntry {
            key: k.to_owned(),
            value: g.value().to_owned(),
        });
    ok_json(&MaybeEntry { entry })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_str_str__remove",
    description = "Remove a string key from a named (&str, &str) table within a write transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_str_str_remove(
    ctx: Arc<RedbCtx>,
    p: TableStrStrKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k: &str = &p.key;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&str, &str>::new(&p.table_name);
    let mut table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let removed = table
        .remove(k)
        .map_err(|e| ErrorData::internal_error(format!("remove: {e}"), None))?;
    ok_text(if removed.is_some() {
        "removed"
    } else {
        "not_found"
    })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_str_str__len",
    description = "Return the number of entries in a (&str, &str) table (write transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_str_str_len(
    ctx: Arc<RedbCtx>,
    p: TableStrStrTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&str, &str>::new(&p.table_name);
    let table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let len = table
        .len()
        .map_err(|e| ErrorData::internal_error(format!("len: {e}"), None))?;
    ok_json(&LenResult { len })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_str_str__pop_first",
    description = "Remove and return the lexicographically first (&str, &str) entry from a named table.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_str_str_pop_first(
    ctx: Arc<RedbCtx>,
    p: TableStrStrTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&str, &str>::new(&p.table_name);
    let mut table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let entry = table
        .pop_first()
        .map_err(|e| ErrorData::internal_error(format!("pop_first: {e}"), None))?
        .map(|(k, v)| KvEntry {
            key: k.value().to_owned(),
            value: v.value().to_owned(),
        });
    ok_json(&MaybeEntry { entry })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_str_str__pop_last",
    description = "Remove and return the lexicographically last (&str, &str) entry from a named table.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_str_str_pop_last(
    ctx: Arc<RedbCtx>,
    p: TableStrStrTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&str, &str>::new(&p.table_name);
    let mut table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let entry = table
        .pop_last()
        .map_err(|e| ErrorData::internal_error(format!("pop_last: {e}"), None))?
        .map(|(k, v)| KvEntry {
            key: k.value().to_owned(),
            value: v.value().to_owned(),
        });
    ok_json(&MaybeEntry { entry })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_str_str__read_get",
    description = "Get a string value by string key from a named (&str, &str) table (read-only transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_str_str_read_get(
    ctx: Arc<RedbCtx>,
    p: TableStrStrReadKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k: &str = &p.key;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&str, &str>::new(&p.table_name);
    let table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let entry = table
        .get(k)
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|g| KvEntry {
            key: k.to_owned(),
            value: g.value().to_owned(),
        });
    ok_json(&MaybeEntry { entry })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_str_str__read_len",
    description = "Return the number of entries in a (&str, &str) table (read-only transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_str_str_read_len(
    ctx: Arc<RedbCtx>,
    p: TableStrStrReadTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&str, &str>::new(&p.table_name);
    let table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let len = table
        .len()
        .map_err(|e| ErrorData::internal_error(format!("len: {e}"), None))?;
    ok_json(&LenResult { len })
}

// ══════════════════════════════════════════════════════════════════════════════
// (&[u8], &[u8]) tables — keys/values are hex-encoded in JSON
// ══════════════════════════════════════════════════════════════════════════════

/// Parameters for `table_bytes_bytes__insert`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableBytesBytesInsertParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
    /// Hex-encoded key bytes.
    pub key: String,
    /// Hex-encoded value bytes.
    pub value: String,
}

/// Parameters for `table_bytes_bytes__get`, `table_bytes_bytes__remove`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableBytesBytesKeyParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
    /// Hex-encoded key bytes.
    pub key: String,
}

/// Parameters for `table_bytes_bytes__len`, pop_first, pop_last.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableBytesBytesTableParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
}

/// Parameters for read-only bytes table operations.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableBytesBytesReadKeyParams {
    /// UUID returned by `redb__database_begin_read`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
    /// Hex-encoded key bytes.
    pub key: String,
}

/// Parameters for read-only bytes table length.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableBytesBytesReadTableParams {
    /// UUID returned by `redb__database_begin_read`.
    pub txn_id: String,
    /// Table name.
    pub table_name: String,
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ErrorData> {
    hex::decode(s).map_err(|e| err_parse(s, e))
}

#[elicit_tool(
    plugin = "redb",
    name = "table_bytes_bytes__insert",
    description = "Insert a hex-encoded (&[u8], &[u8]) entry into a named table within a write transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_bytes_bytes_insert(
    ctx: Arc<RedbCtx>,
    p: TableBytesBytesInsertParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k_vec = decode_hex(&p.key)?;
    let v_vec = decode_hex(&p.value)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let mut table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    table
        .insert(k_vec.as_slice(), v_vec.as_slice())
        .map_err(|e| ErrorData::internal_error(format!("insert: {e}"), None))?;
    ok_text("inserted")
}

#[elicit_tool(
    plugin = "redb",
    name = "table_bytes_bytes__get",
    description = "Get hex-encoded bytes by hex key from a (&[u8], &[u8]) table (write transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_bytes_bytes_get(
    ctx: Arc<RedbCtx>,
    p: TableBytesBytesKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k_vec = decode_hex(&p.key)?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let entry = table
        .get(k_vec.as_slice())
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|g| KvEntry {
            key: hex::encode(&k_vec),
            value: hex::encode(g.value()),
        });
    ok_json(&MaybeEntry { entry })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_bytes_bytes__remove",
    description = "Remove a hex-encoded key from a (&[u8], &[u8]) table within a write transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_bytes_bytes_remove(
    ctx: Arc<RedbCtx>,
    p: TableBytesBytesKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k_vec = decode_hex(&p.key)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let mut table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let removed = table
        .remove(k_vec.as_slice())
        .map_err(|e| ErrorData::internal_error(format!("remove: {e}"), None))?;
    ok_text(if removed.is_some() {
        "removed"
    } else {
        "not_found"
    })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_bytes_bytes__len",
    description = "Return the number of entries in a (&[u8], &[u8]) table (write transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_bytes_bytes_len(
    ctx: Arc<RedbCtx>,
    p: TableBytesBytesTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let len = table
        .len()
        .map_err(|e| ErrorData::internal_error(format!("len: {e}"), None))?;
    ok_json(&LenResult { len })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_bytes_bytes__pop_first",
    description = "Remove and return the first (lowest key) (&[u8], &[u8]) entry, hex-encoded.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_bytes_bytes_pop_first(
    ctx: Arc<RedbCtx>,
    p: TableBytesBytesTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let mut table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let entry = table
        .pop_first()
        .map_err(|e| ErrorData::internal_error(format!("pop_first: {e}"), None))?
        .map(|(k, v)| KvEntry {
            key: hex::encode(k.value()),
            value: hex::encode(v.value()),
        });
    ok_json(&MaybeEntry { entry })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_bytes_bytes__pop_last",
    description = "Remove and return the last (highest key) (&[u8], &[u8]) entry, hex-encoded.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_bytes_bytes_pop_last(
    ctx: Arc<RedbCtx>,
    p: TableBytesBytesTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let mut table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let entry = table
        .pop_last()
        .map_err(|e| ErrorData::internal_error(format!("pop_last: {e}"), None))?
        .map(|(k, v)| KvEntry {
            key: hex::encode(k.value()),
            value: hex::encode(v.value()),
        });
    ok_json(&MaybeEntry { entry })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_bytes_bytes__read_get",
    description = "Get hex-encoded bytes by hex key from a (&[u8], &[u8]) table (read-only transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_bytes_bytes_read_get(
    ctx: Arc<RedbCtx>,
    p: TableBytesBytesReadKeyParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let k_vec = decode_hex(&p.key)?;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let entry = table
        .get(k_vec.as_slice())
        .map_err(|e| ErrorData::internal_error(format!("get: {e}"), None))?
        .map(|g| KvEntry {
            key: hex::encode(&k_vec),
            value: hex::encode(g.value()),
        });
    ok_json(&MaybeEntry { entry })
}

#[elicit_tool(
    plugin = "redb",
    name = "table_bytes_bytes__read_len",
    description = "Return the number of entries in a (&[u8], &[u8]) table (read-only transaction).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn table_bytes_bytes_read_len(
    ctx: Arc<RedbCtx>,
    p: TableBytesBytesReadTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let def = redb::TableDefinition::<&[u8], &[u8]>::new(&p.table_name);
    let table = txn
        .open_table(def)
        .map_err(|e| ErrorData::internal_error(format!("open_table: {e}"), None))?;
    let len = table
        .len()
        .map_err(|e| ErrorData::internal_error(format!("len: {e}"), None))?;
    ok_json(&LenResult { len })
}
