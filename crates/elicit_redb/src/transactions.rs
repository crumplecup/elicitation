//! Shadow of [`redb::WriteTransaction`] and [`redb::ReadTransaction`].
//!
//! Both shadow types serialize as UUID strings.  The actual redb transactions
//! live in [`RedbCtx`] keyed by UUID.  `WriteTransaction::commit` and
//! `WriteTransaction::abort` consume ownership via `HashMap::remove`.

use std::sync::Arc;

use elicitation::elicit_tool;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    Savepoint,
    plugin::{RedbCtx, ok_json, ok_text},
};
use redb::TableHandle as _;

// ── UUID-string serialization helper ─────────────────────────────────────────

macro_rules! uuid_handle {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
        pub struct $name(pub Uuid);

        impl Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_str(&self.0.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                let s = String::deserialize(d)?;
                s.parse::<Uuid>()
                    .map($name)
                    .map_err(serde::de::Error::custom)
            }
        }
    };
}

uuid_handle!(
    WriteTransaction,
    "Shadow of `redb::WriteTransaction`. A UUID handle identifying a live write transaction in the plugin context."
);
uuid_handle!(
    ReadTransaction,
    "Shadow of `redb::ReadTransaction`. A UUID handle identifying a live read transaction in the plugin context."
);

// ── WriteTransaction params ───────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteTxnParams {
    /// Write transaction handle returned by `database__begin_write`.
    pub txn: WriteTransaction,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteTxnDurabilityParams {
    /// Write transaction handle.
    pub txn: WriteTransaction,
    /// Durability level: "Eventual", "Immediate", "None", or "Paranoid".
    pub durability: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteTxnSavepointRestoreParams {
    /// Write transaction handle.
    pub txn: WriteTransaction,
    /// Savepoint handle returned by `write_txn__persistent_savepoint` or `write_txn__ephemeral_savepoint`.
    pub savepoint: Savepoint,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteTxnRenameParams {
    /// Write transaction handle.
    pub txn: WriteTransaction,
    /// Current table name.
    pub old_name: String,
    /// New table name.
    pub new_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteTxnDeleteTableParams {
    /// Write transaction handle.
    pub txn: WriteTransaction,
    /// Table name to delete.
    pub name: String,
}

// ── ReadTransaction params ────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadTxnParams {
    /// Read transaction handle returned by `database__begin_read`.
    pub txn: ReadTransaction,
}

// ── shared result ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct ListResult {
    tables: Vec<String>,
}

// ══════════════════════════════════════════════════════════════════════════════
// WriteTransaction tools
// ══════════════════════════════════════════════════════════════════════════════

/// Mirrors `redb::WriteTransaction::commit`.
#[elicit_tool(
    plugin = "redb",
    name = "write_txn__commit",
    description = "Commit the write transaction, durably persisting all changes.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn write_txn_commit(
    ctx: Arc<RedbCtx>,
    p: WriteTxnParams,
) -> Result<CallToolResult, ErrorData> {
    let txn = ctx
        .lock_write_txns()?
        .remove(&p.txn.0)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?;
    txn.commit()
        .map_err(|e| ErrorData::internal_error(format!("commit: {e}"), None))?;
    ok_text("committed")
}

/// Mirrors `redb::WriteTransaction::abort`.
#[elicit_tool(
    plugin = "redb",
    name = "write_txn__abort",
    description = "Abort the write transaction, discarding all pending changes.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn write_txn_abort(
    ctx: Arc<RedbCtx>,
    p: WriteTxnParams,
) -> Result<CallToolResult, ErrorData> {
    let txn = ctx
        .lock_write_txns()?
        .remove(&p.txn.0)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?;
    txn.abort()
        .map_err(|e| ErrorData::internal_error(format!("abort: {e}"), None))?;
    ok_text("aborted")
}

/// Mirrors `redb::WriteTransaction::set_durability`.
#[elicit_tool(
    plugin = "redb",
    name = "write_txn__set_durability",
    description = "Set durability level: Eventual | Immediate | None | Paranoid.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn write_txn_set_durability(
    ctx: Arc<RedbCtx>,
    p: WriteTxnDurabilityParams,
) -> Result<CallToolResult, ErrorData> {
    let level = match p.durability.as_str() {
        "Eventual" => redb::Durability::None,
        "Immediate" => redb::Durability::Immediate,
        "None" => redb::Durability::None,
        "Paranoid" => redb::Durability::Immediate,
        other => {
            return Err(ErrorData::invalid_params(
                format!("unknown durability '{other}'; use Eventual | Immediate | None | Paranoid"),
                None,
            ));
        }
    };
    ctx.lock_write_txns()?
        .get_mut(&p.txn.0)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?
        .set_durability(level)
        .map_err(|e| ErrorData::internal_error(format!("set_durability: {e}"), None))?;
    ok_text("ok")
}

/// Mirrors `redb::WriteTransaction::list_tables`.
#[elicit_tool(
    plugin = "redb",
    name = "write_txn__list_tables",
    description = "List all table names visible in the write transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn write_txn_list_tables(
    ctx: Arc<RedbCtx>,
    p: WriteTxnParams,
) -> Result<CallToolResult, ErrorData> {
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&p.txn.0)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?;
    let tables: Vec<String> = txn
        .list_tables()
        .map_err(|e| ErrorData::internal_error(format!("list_tables: {e}"), None))?
        .map(|h| h.name().to_owned())
        .collect();
    ok_json(&ListResult { tables })
}

/// Mirrors `redb::WriteTransaction::delete_table`.
#[elicit_tool(
    plugin = "redb",
    name = "write_txn__delete_table",
    description = "Delete a table from the database. Returns true if the table existed.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn write_txn_delete_table(
    ctx: Arc<RedbCtx>,
    p: WriteTxnDeleteTableParams,
) -> Result<CallToolResult, ErrorData> {
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&p.txn.0)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?;
    // Use a typed TableDefinition as impl TableHandle — K/V types are irrelevant for delete
    let def = redb::TableDefinition::<u64, u64>::new(&p.name);
    let existed = txn
        .delete_table(def)
        .map_err(|e| ErrorData::internal_error(format!("delete_table: {e}"), None))?;
    ok_text(if existed { "true" } else { "false" })
}

/// Mirrors `redb::WriteTransaction::rename_table`.
#[elicit_tool(
    plugin = "redb",
    name = "write_txn__rename_table",
    description = "Rename a table within the write transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn write_txn_rename_table(
    ctx: Arc<RedbCtx>,
    p: WriteTxnRenameParams,
) -> Result<CallToolResult, ErrorData> {
    let txns = ctx.lock_write_txns()?;
    let txn = txns
        .get(&p.txn.0)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?;
    // Use typed TableDefinitions as impl TableHandle — K/V types irrelevant for rename
    let old_def = redb::TableDefinition::<u64, u64>::new(&p.old_name);
    let new_def = redb::TableDefinition::<u64, u64>::new(&p.new_name);
    txn.rename_table(old_def, new_def)
        .map_err(|e| ErrorData::internal_error(format!("rename_table: {e}"), None))?;
    ok_text("renamed")
}

/// Mirrors `redb::WriteTransaction::persistent_savepoint`.
#[elicit_tool(
    plugin = "redb",
    name = "write_txn__persistent_savepoint",
    description = "Create a persistent savepoint. Returns a Savepoint handle.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn write_txn_persistent_savepoint(
    ctx: Arc<RedbCtx>,
    p: WriteTxnParams,
) -> Result<CallToolResult, ErrorData> {
    let sp_id = {
        let txns = ctx.lock_write_txns()?;
        let txn = txns
            .get(&p.txn.0)
            .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?;
        txn.persistent_savepoint()
            .map_err(|e| ErrorData::internal_error(format!("persistent_savepoint: {e}"), None))?
    };
    // Retrieve the Savepoint object via get_persistent_savepoint
    let sp = {
        let txns = ctx.lock_write_txns()?;
        let txn = txns.get(&p.txn.0).unwrap();
        txn.get_persistent_savepoint(sp_id).map_err(|e| {
            ErrorData::internal_error(format!("get_persistent_savepoint: {e}"), None)
        })?
    };
    let id = Uuid::new_v4();
    ctx.lock_savepoints()?.insert(id, sp);
    ok_json(&Savepoint(id))
}

/// Mirrors `redb::WriteTransaction::ephemeral_savepoint`.
#[elicit_tool(
    plugin = "redb",
    name = "write_txn__ephemeral_savepoint",
    description = "Create an ephemeral savepoint (lasts only until the transaction ends). Returns a Savepoint handle.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn write_txn_ephemeral_savepoint(
    ctx: Arc<RedbCtx>,
    p: WriteTxnParams,
) -> Result<CallToolResult, ErrorData> {
    let sp = {
        let txns = ctx.lock_write_txns()?;
        let txn = txns
            .get(&p.txn.0)
            .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?;
        txn.ephemeral_savepoint()
            .map_err(|e| ErrorData::internal_error(format!("ephemeral_savepoint: {e}"), None))?
    };
    let id = Uuid::new_v4();
    ctx.lock_savepoints()?.insert(id, sp);
    ok_json(&Savepoint(id))
}

/// Mirrors `redb::WriteTransaction::restore_savepoint`.
#[elicit_tool(
    plugin = "redb",
    name = "write_txn__restore_savepoint",
    description = "Restore the write transaction to a previously created savepoint.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn write_txn_restore_savepoint(
    ctx: Arc<RedbCtx>,
    p: WriteTxnSavepointRestoreParams,
) -> Result<CallToolResult, ErrorData> {
    // Lock order: savepoints first, then write_txns
    let sp_ref = {
        let sps = ctx.lock_savepoints()?;
        sps.get(&p.savepoint.0).ok_or_else(|| {
            ErrorData::invalid_params(format!("unknown savepoint: {}", p.savepoint.0), None)
        })?;
        // Can't hold the guard across the write_txns lock; we need &Savepoint
        // Clone the UUID, look up again inside write_txns lock scope
        p.savepoint.0
    };
    // Acquire both locks
    let sps = ctx.lock_savepoints()?;
    let sp = sps
        .get(&sp_ref)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown savepoint: {sp_ref}"), None))?;
    let mut txns = ctx.lock_write_txns()?;
    let txn = txns
        .get_mut(&p.txn.0)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?;
    txn.restore_savepoint(sp)
        .map_err(|e| ErrorData::internal_error(format!("restore_savepoint: {e}"), None))?;
    ok_text("restored")
}

// ══════════════════════════════════════════════════════════════════════════════
// ReadTransaction tools
// ══════════════════════════════════════════════════════════════════════════════

/// Mirrors `redb::ReadTransaction::list_tables`.
#[elicit_tool(
    plugin = "redb",
    name = "read_txn__list_tables",
    description = "List all table names visible in the read transaction.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn read_txn_list_tables(
    ctx: Arc<RedbCtx>,
    p: ReadTxnParams,
) -> Result<CallToolResult, ErrorData> {
    let txns = ctx.lock_read_txns()?;
    let txn = txns
        .get(&p.txn.0)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?;
    let tables: Vec<String> = txn
        .list_tables()
        .map_err(|e| ErrorData::internal_error(format!("list_tables: {e}"), None))?
        .map(|h| h.name().to_owned())
        .collect();
    ok_json(&ListResult { tables })
}

/// Close (drop) a read transaction.
#[elicit_tool(
    plugin = "redb",
    name = "read_txn__close",
    description = "Close the read transaction, releasing its snapshot.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn read_txn_close(
    ctx: Arc<RedbCtx>,
    p: ReadTxnParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.lock_read_txns()?
        .remove(&p.txn.0)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown txn: {}", p.txn.0), None))?;
    ok_text("closed")
}
