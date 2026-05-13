//! `#[elicit_tool]` functions for `redb::WriteTransaction` and `redb::ReadTransaction`.

use std::sync::Arc;

use elicitation::elicit_tool;
use redb::TableHandle as _;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::plugin::{RedbCtx, ok_json, ok_text, parse_uuid};

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for single-transaction operations (commit, abort, list tables, etc.)
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TxnIdParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
}

/// Parameters for `redb__write_txn_set_durability`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteTxnSetDurabilityParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Durability level: `"none"` or `"immediate"`.
    pub durability: String,
}

/// Parameters for `redb__write_txn_delete_table` and `redb__write_txn_rename_table`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteTxnDeleteTableParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Table name to delete.
    pub table_name: String,
}

/// Parameters for `redb__write_txn_rename_table`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteTxnRenameTableParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// Current table name.
    pub old_name: String,
    /// New table name.
    pub new_name: String,
}

/// Parameters for `redb__write_txn_savepoint_persistent` and `redb__write_txn_savepoint_ephemeral`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SavepointCreateParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
}

/// Parameters for `redb__write_txn_savepoint_restore`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SavepointRestoreParams {
    /// UUID returned by `redb__database_begin_write`.
    pub txn_id: String,
    /// UUID returned by a prior savepoint create call.
    pub savepoint_id: String,
}

/// Parameters for read transaction operations.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadTxnIdParams {
    /// UUID returned by `redb__database_begin_read`.
    pub txn_id: String,
}

// ── result types ──────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct IdResult {
    id: String,
}

#[derive(Serialize)]
struct TableListResult {
    tables: Vec<String>,
}

// ── write transaction tools ───────────────────────────────────────────────────

#[elicit_tool(
    plugin = "redb",
    name = "write_txn_commit",
    description = "Commit a write transaction, making all changes permanent and removing it from the session.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn write_txn_commit(ctx: Arc<RedbCtx>, p: TxnIdParams) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txn = ctx
        .lock_write_txns()?
        .remove(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    txn.commit()
        .map_err(|e| ErrorData::internal_error(format!("commit error: {e}"), None))?;
    ok_text("committed")
}

#[elicit_tool(
    plugin = "redb",
    name = "write_txn_abort",
    description = "Abort a write transaction, discarding all changes and removing it from the session.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn write_txn_abort(ctx: Arc<RedbCtx>, p: TxnIdParams) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    ctx.lock_write_txns()?
        .remove(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    ok_text("aborted")
}

#[elicit_tool(
    plugin = "redb",
    name = "write_txn_set_durability",
    description = "Change the durability level of an open write transaction (none or immediate).",
    emit = None
)]
#[instrument(skip(ctx))]
async fn write_txn_set_durability(
    ctx: Arc<RedbCtx>,
    p: WriteTxnSetDurabilityParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let durability = match p.durability.as_str() {
        "none" => redb::Durability::None,
        "immediate" => redb::Durability::Immediate,
        other => {
            return Err(ErrorData::invalid_params(
                format!("unknown durability level: {other}; valid: none, immediate"),
                None,
            ));
        }
    };
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    txn.set_durability(durability)
        .map_err(|e| ErrorData::internal_error(format!("set_durability error: {e}"), None))?;
    ok_text("ok")
}

#[elicit_tool(
    plugin = "redb",
    name = "write_txn_list_tables",
    description = "List all table names visible in the write transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn write_txn_list_tables(
    ctx: Arc<RedbCtx>,
    p: TxnIdParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let tables: Vec<String> = txn
        .list_tables()
        .map_err(|e| ErrorData::internal_error(format!("list_tables error: {e}"), None))?
        .map(|h| h.name().to_owned())
        .collect();
    ok_json(&TableListResult { tables })
}

#[elicit_tool(
    plugin = "redb",
    name = "write_txn_delete_table",
    description = "Delete a table within an open write transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn write_txn_delete_table(
    ctx: Arc<RedbCtx>,
    p: WriteTxnDeleteTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let table_def = redb::TableDefinition::<'_, &[u8], &[u8]>::new(&p.table_name);
    let deleted = txn
        .delete_table(table_def)
        .map_err(|e| ErrorData::internal_error(format!("delete_table error: {e}"), None))?;
    ok_text(if deleted { "deleted" } else { "not_found" })
}

#[elicit_tool(
    plugin = "redb",
    name = "write_txn_rename_table",
    description = "Rename a table within an open write transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn write_txn_rename_table(
    ctx: Arc<RedbCtx>,
    p: WriteTxnRenameTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let old_def = redb::TableDefinition::<'_, &[u8], &[u8]>::new(&p.old_name);
    let new_def = redb::TableDefinition::<'_, &[u8], &[u8]>::new(&p.new_name);
    txn.rename_table(old_def, new_def)
        .map_err(|e| ErrorData::internal_error(format!("rename_table error: {e}"), None))?;
    ok_text("renamed")
}

#[elicit_tool(
    plugin = "redb",
    name = "write_txn_savepoint_persistent",
    description = "Create a named persistent savepoint within the write transaction. Returns a savepoint_id UUID.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn write_txn_savepoint_persistent(
    ctx: Arc<RedbCtx>,
    p: SavepointCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let sp = {
        let txns = ctx.lock_write_txns()?;
        let txn = txns.get(&txn_id).ok_or_else(|| {
            ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None)
        })?;
        let id = txn.persistent_savepoint().map_err(|e| {
            ErrorData::internal_error(format!("persistent_savepoint error: {e}"), None)
        })?;
        txn.get_persistent_savepoint(id).map_err(|e| {
            ErrorData::internal_error(format!("get_persistent_savepoint error: {e}"), None)
        })?
    };
    let sp_id = uuid::Uuid::new_v4();
    ctx.lock_savepoints()?.insert(sp_id, sp);
    ok_json(&IdResult {
        id: sp_id.to_string(),
    })
}

#[elicit_tool(
    plugin = "redb",
    name = "write_txn_savepoint_ephemeral",
    description = "Create an ephemeral savepoint within the write transaction. Returns a savepoint_id UUID.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn write_txn_savepoint_ephemeral(
    ctx: Arc<RedbCtx>,
    p: SavepointCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let sp = {
        let txns = ctx.lock_write_txns()?;
        let txn = txns.get(&txn_id).ok_or_else(|| {
            ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None)
        })?;
        txn.ephemeral_savepoint().map_err(|e| {
            ErrorData::internal_error(format!("ephemeral_savepoint error: {e}"), None)
        })?
    };
    let sp_id = uuid::Uuid::new_v4();
    ctx.lock_savepoints()?.insert(sp_id, sp);
    ok_json(&IdResult {
        id: sp_id.to_string(),
    })
}

#[elicit_tool(
    plugin = "redb",
    name = "write_txn_savepoint_restore",
    description = "Restore a write transaction to a previously created savepoint.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn write_txn_savepoint_restore(
    ctx: Arc<RedbCtx>,
    p: SavepointRestoreParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let sp_id = parse_uuid(&p.savepoint_id)?;
    // Acquire savepoints first, then write_txns (consistent lock ordering)
    let savepoints = ctx.lock_savepoints()?;
    let sp = savepoints.get(&sp_id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown savepoint_id: {}", p.savepoint_id), None)
    })?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    txn.restore_savepoint(sp)
        .map_err(|e| ErrorData::internal_error(format!("restore_savepoint error: {e}"), None))?;
    ok_text("restored")
}

// ── read transaction tools ────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "redb",
    name = "read_txn_list_tables",
    description = "List all table names visible in the read transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn read_txn_list_tables(
    ctx: Arc<RedbCtx>,
    p: ReadTxnIdParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    let tables: Vec<String> = txn
        .list_tables()
        .map_err(|e| ErrorData::internal_error(format!("list_tables error: {e}"), None))?
        .map(|h| h.name().to_owned())
        .collect();
    ok_json(&TableListResult { tables })
}

#[elicit_tool(
    plugin = "redb",
    name = "read_txn_close",
    description = "Close and remove a read transaction from the session context.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn read_txn_close(
    ctx: Arc<RedbCtx>,
    p: ReadTxnIdParams,
) -> Result<CallToolResult, ErrorData> {
    let txn_id = parse_uuid(&p.txn_id)?;
    ctx.lock_read_txns()?
        .remove(&txn_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn_id: {}", p.txn_id), None))?;
    ok_text("closed")
}
