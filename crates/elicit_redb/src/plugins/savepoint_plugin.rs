//! `RedbSavepointPlugin` — savepoint create / restore / delete patterns.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

fn ok(text: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(text.into())]))
}

fn default_write_txn() -> String {
    "write_txn".into()
}
fn default_sp_var() -> String {
    "sp".into()
}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `redb_savepoint__create_persistent`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SavepointCreatePersistentParams {
    /// Write-transaction variable name.
    #[serde(default = "default_write_txn")]
    pub txn_var: String,
    /// Variable name for the resulting savepoint.
    #[serde(default = "default_sp_var")]
    pub sp_var: String,
}

/// Parameters for `redb_savepoint__create_ephemeral`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SavepointCreateEphemeralParams {
    /// Write-transaction variable name.
    #[serde(default = "default_write_txn")]
    pub txn_var: String,
    /// Variable name for the resulting savepoint.
    #[serde(default = "default_sp_var")]
    pub sp_var: String,
}

/// Parameters for `redb_savepoint__restore`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SavepointRestoreParams {
    /// Write-transaction variable name.
    #[serde(default = "default_write_txn")]
    pub txn_var: String,
    /// Savepoint variable name.
    #[serde(default = "default_sp_var")]
    pub sp_var: String,
}

/// Parameters for `redb_savepoint__list`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SavepointListParams {
    /// Write-transaction variable name.
    #[serde(default = "default_write_txn")]
    pub txn_var: String,
}

/// Parameters for `redb_savepoint__delete`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SavepointDeleteParams {
    /// Write-transaction variable name.
    #[serde(default = "default_write_txn")]
    pub txn_var: String,
    /// Savepoint variable name to delete.
    #[serde(default = "default_sp_var")]
    pub sp_var: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "redb_savepoint",
    name = "redb_savepoint__create_persistent",
    description = "Emit a persistent `create_savepoint()` snippet to create a savepoint that survives commit."
)]
#[instrument]
async fn redb_savepoint_create_persistent(
    p: SavepointCreatePersistentParams,
) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let {sp} = {txn}.create_savepoint()?;",
        sp = p.sp_var,
        txn = p.txn_var,
    ))
}

#[elicit_tool(
    plugin = "redb_savepoint",
    name = "redb_savepoint__create_ephemeral",
    description = "Emit an ephemeral `ephemeral_savepoint()` snippet to create a savepoint deleted on commit."
)]
#[instrument]
async fn redb_savepoint_create_ephemeral(
    p: SavepointCreateEphemeralParams,
) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let {sp} = {txn}.ephemeral_savepoint()?;",
        sp = p.sp_var,
        txn = p.txn_var,
    ))
}

#[elicit_tool(
    plugin = "redb_savepoint",
    name = "redb_savepoint__restore",
    description = "Emit a `restore_savepoint(&sp)` snippet to roll back a write transaction to a savepoint."
)]
#[instrument]
async fn redb_savepoint_restore(p: SavepointRestoreParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "{txn}.restore_savepoint(&{sp})?;",
        txn = p.txn_var,
        sp = p.sp_var,
    ))
}

#[elicit_tool(
    plugin = "redb_savepoint",
    name = "redb_savepoint__list",
    description = "Emit a `list_persistent_savepoints()` iteration snippet to enumerate all persistent savepoints."
)]
#[instrument]
async fn redb_savepoint_list(p: SavepointListParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "for sp_id in {txn}.list_persistent_savepoints()? {{\n    println!(\"savepoint id: {{sp_id}}\");\n}}",
        txn = p.txn_var,
    ))
}

#[elicit_tool(
    plugin = "redb_savepoint",
    name = "redb_savepoint__delete",
    description = "Emit a `delete_savepoint(sp)` snippet to remove a persistent savepoint from a redb database."
)]
#[instrument]
async fn redb_savepoint_delete(p: SavepointDeleteParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "{txn}.delete_savepoint({sp})?;",
        txn = p.txn_var,
        sp = p.sp_var,
    ))
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// Plugin providing redb savepoint code-generation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "redb_savepoint")]
pub struct RedbSavepointPlugin;
