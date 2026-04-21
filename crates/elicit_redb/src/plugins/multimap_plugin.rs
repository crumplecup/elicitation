//! `RedbMultimapPlugin` — `MultimapTableDefinition` and multimap CRUD patterns.

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

fn default_const_name() -> String {
    "MM".into()
}
fn default_table_var() -> String {
    "mm_table".into()
}
fn default_write_txn() -> String {
    "write_txn".into()
}
fn default_read_txn() -> String {
    "read_txn".into()
}
fn default_key_var() -> String {
    "key".into()
}
fn default_value_var() -> String {
    "value".into()
}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `redb_multimap__define`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MmDefineParams {
    /// Constant name for the multimap table definition.
    #[serde(default = "default_const_name")]
    pub const_name: String,
    /// String name of the table stored in the database.
    pub table_name: String,
    /// Rust type for the key (e.g. `u64`, `&str`).
    pub key_type: String,
    /// Rust type for values (e.g. `&str`, `u64`).
    pub value_type: String,
}

/// Parameters for `redb_multimap__open_write`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MmOpenWriteParams {
    /// Variable name for the mutable multimap handle.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Write-transaction variable name.
    #[serde(default = "default_write_txn")]
    pub txn_var: String,
    /// `MultimapTableDefinition` constant name.
    #[serde(default = "default_const_name")]
    pub const_name: String,
}

/// Parameters for `redb_multimap__open_read`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MmOpenReadParams {
    /// Variable name for the read-only multimap handle.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Read-transaction variable name.
    #[serde(default = "default_read_txn")]
    pub txn_var: String,
    /// `MultimapTableDefinition` constant name.
    #[serde(default = "default_const_name")]
    pub const_name: String,
}

/// Parameters for `redb_multimap__insert`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MmInsertParams {
    /// Multimap table variable name.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Key expression (e.g. `&42u64`).
    pub key_expr: String,
    /// Value expression (e.g. `&"hello"`).
    pub value_expr: String,
}

/// Parameters for `redb_multimap__get`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MmGetParams {
    /// Multimap table variable name.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Key expression.
    pub key_expr: String,
    /// Variable name for the value in the iteration loop.
    #[serde(default = "default_value_var")]
    pub value_var: String,
}

/// Parameters for `redb_multimap__remove`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MmRemoveParams {
    /// Multimap table variable name.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Key expression.
    pub key_expr: String,
    /// Value expression to remove.
    pub value_expr: String,
}

/// Parameters for `redb_multimap__remove_all`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MmRemoveAllParams {
    /// Multimap table variable name.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Key expression whose values should all be removed.
    pub key_expr: String,
}

/// Parameters for `redb_multimap__iter`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MmIterParams {
    /// Multimap table variable name.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Variable name for the key in the outer loop.
    #[serde(default = "default_key_var")]
    pub key_var: String,
    /// Variable name for each value in the inner iteration.
    #[serde(default = "default_value_var")]
    pub value_var: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "redb_multimap",
    name = "redb_multimap__define",
    description = "Emit a `const MM: MultimapTableDefinition<K, V>` declaration for a redb multimap table."
)]
#[instrument]
async fn redb_multimap_define(p: MmDefineParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "const {c}: redb::MultimapTableDefinition<{k}, {v}> = redb::MultimapTableDefinition::new({n:?});",
        c = p.const_name,
        k = p.key_type,
        v = p.value_type,
        n = p.table_name,
    ))
}

#[elicit_tool(
    plugin = "redb_multimap",
    name = "redb_multimap__open_write",
    description = "Emit `write_txn.open_multimap_table(MM)?` to get a mutable multimap handle."
)]
#[instrument]
async fn redb_multimap_open_write(p: MmOpenWriteParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let mut {tv} = {txn}.open_multimap_table({cn})?;",
        tv = p.table_var,
        txn = p.txn_var,
        cn = p.const_name,
    ))
}

#[elicit_tool(
    plugin = "redb_multimap",
    name = "redb_multimap__open_read",
    description = "Emit `read_txn.open_multimap_table(MM)?` to get a read-only multimap handle."
)]
#[instrument]
async fn redb_multimap_open_read(p: MmOpenReadParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let {tv} = {txn}.open_multimap_table({cn})?;",
        tv = p.table_var,
        txn = p.txn_var,
        cn = p.const_name,
    ))
}

#[elicit_tool(
    plugin = "redb_multimap",
    name = "redb_multimap__insert",
    description = "Emit a `table.insert(&key, &value)?` snippet for inserting a value into a redb multimap."
)]
#[instrument]
async fn redb_multimap_insert(p: MmInsertParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "{t}.insert({k}, {v})?;",
        t = p.table_var,
        k = p.key_expr,
        v = p.value_expr,
    ))
}

#[elicit_tool(
    plugin = "redb_multimap",
    name = "redb_multimap__get",
    description = "Emit an iteration snippet for all values associated with a key in a redb multimap."
)]
#[instrument]
async fn redb_multimap_get(p: MmGetParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "for {vv} in {t}.get({k})? {{\n    let v = {vv}?.value();\n    println!(\"{{v:?}}\");\n}}",
        t = p.table_var,
        k = p.key_expr,
        vv = p.value_var,
    ))
}

#[elicit_tool(
    plugin = "redb_multimap",
    name = "redb_multimap__remove",
    description = "Emit a `table.remove(&key, &value)?` snippet for removing a specific value from a redb multimap key."
)]
#[instrument]
async fn redb_multimap_remove(p: MmRemoveParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let removed = {t}.remove({k}, {v})?;",
        t = p.table_var,
        k = p.key_expr,
        v = p.value_expr,
    ))
}

#[elicit_tool(
    plugin = "redb_multimap",
    name = "redb_multimap__remove_all",
    description = "Emit a `table.remove_all(&key)?` snippet to remove all values for a key in a redb multimap."
)]
#[instrument]
async fn redb_multimap_remove_all(p: MmRemoveAllParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let removed_iter = {t}.remove_all({k})?;\n// Consume or ignore removed_iter to complete removal.",
        t = p.table_var,
        k = p.key_expr,
    ))
}

#[elicit_tool(
    plugin = "redb_multimap",
    name = "redb_multimap__iter",
    description = "Emit a `for item in table.iter()?` loop skeleton for iterating all key-value pairs in a redb multimap."
)]
#[instrument]
async fn redb_multimap_iter(p: MmIterParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "for item in {t}.iter()? {{\n    let ({k}, values) = item?;\n    let {k} = {k}.value();\n    for {vv} in values {{\n        let {vv} = {vv}?.value();\n        println!(\"  {{key:?}} => {{{vv}:?}}\");\n    }}\n}}",
        t = p.table_var,
        k = p.key_var,
        vv = p.value_var,
    ))
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// Plugin providing redb `MultimapTableDefinition` and multimap CRUD tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "redb_multimap")]
pub struct RedbMultimapPlugin;
