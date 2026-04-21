//! `RedbTablePlugin` — `TableDefinition`, typed CRUD, and iteration patterns.

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

// ── default helpers ───────────────────────────────────────────────────────────

fn default_table_const() -> String {
    "TABLE".into()
}
fn default_table_var() -> String {
    "table".into()
}
fn default_write_txn() -> String {
    "write_txn".into()
}
fn default_read_txn() -> String {
    "read_txn".into()
}
fn default_k() -> String {
    "key".into()
}
fn default_v() -> String {
    "value".into()
}
fn default_true() -> bool {
    true
}
fn default_predicate() -> String {
    "true".into()
}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `redb_table__define`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableDefineParams {
    /// Constant name for the table definition.
    #[serde(default = "default_table_const")]
    pub const_name: String,
    /// String name of the table stored in the database.
    pub table_name: String,
    /// Rust type for the key (e.g. `u64`, `&str`).
    pub key_type: String,
    /// Rust type for the value (e.g. `&str`, `MyStruct`).
    pub value_type: String,
}

/// Parameters for `redb_table__open_write`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableOpenWriteParams {
    /// Variable name for the mutable table handle.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Write-transaction variable name.
    #[serde(default = "default_write_txn")]
    pub txn_var: String,
    /// `TableDefinition` constant name.
    #[serde(default = "default_table_const")]
    pub const_name: String,
}

/// Parameters for `redb_table__open_read`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableOpenReadParams {
    /// Variable name for the read-only table handle.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Read-transaction variable name.
    #[serde(default = "default_read_txn")]
    pub txn_var: String,
    /// `TableDefinition` constant name.
    #[serde(default = "default_table_const")]
    pub const_name: String,
}

/// Parameters for `redb_table__insert`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableInsertParams {
    /// Table variable name.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Key expression (e.g. `&42u64`).
    pub key_expr: String,
    /// Value expression (e.g. `&"hello"`).
    pub value_expr: String,
    /// Capture the old value returned by `insert`.
    #[serde(default)]
    pub capture_old: bool,
}

/// Parameters for `redb_table__get`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableGetParams {
    /// Table variable name.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Key expression.
    pub key_expr: String,
}

/// Parameters for `redb_table__remove`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableRemoveParams {
    /// Table variable name.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Key expression.
    pub key_expr: String,
    /// Capture the old value returned by `remove`.
    #[serde(default)]
    pub capture_old: bool,
}

/// Parameters for `redb_table__iter`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableIterParams {
    /// Table variable name.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Variable name for key in the loop.
    #[serde(default = "default_k")]
    pub key_var: String,
    /// Variable name for value in the loop.
    #[serde(default = "default_v")]
    pub value_var: String,
}

/// Parameters for `redb_table__range`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableRangeParams {
    /// Table variable name.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Lower bound expression (e.g. `&0u64`).
    pub lo: String,
    /// Upper bound expression (e.g. `&100u64`).
    pub hi: String,
    /// Use inclusive (`..=`) upper bound.
    #[serde(default = "default_true")]
    pub inclusive: bool,
}

/// Parameters for `redb_table__pop`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TablePopParams {
    /// Table variable name.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Use `pop_last` instead of `pop_first`.
    #[serde(default)]
    pub pop_last: bool,
}

/// Parameters for `redb_table__retain`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableRetainParams {
    /// Table variable name.
    #[serde(default = "default_table_var")]
    pub table_var: String,
    /// Predicate body (e.g. `k > &10 && v.as_str() != ""`).
    #[serde(default = "default_predicate")]
    pub predicate: String,
}

/// Parameters for `redb_table__len`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableLenParams {
    /// Table variable name.
    #[serde(default = "default_table_var")]
    pub table_var: String,
}

/// Parameters for `redb_table__rename`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableRenameParams {
    /// Write-transaction variable name.
    #[serde(default = "default_write_txn")]
    pub txn_var: String,
    /// Old table definition constant name.
    pub old_const: String,
    /// New table definition constant name.
    pub new_const: String,
}

/// Parameters for `redb_table__delete`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableDeleteParams {
    /// Write-transaction variable name.
    #[serde(default = "default_write_txn")]
    pub txn_var: String,
    /// `TableDefinition` constant name.
    #[serde(default = "default_table_const")]
    pub const_name: String,
}

/// Parameters for `redb_table__list`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableListParams {
    /// Write-transaction variable name.
    #[serde(default = "default_write_txn")]
    pub txn_var: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "redb_table",
    name = "redb_table__define",
    description = "Emit a `const TABLE: TableDefinition<K, V>` declaration for a redb table."
)]
#[instrument]
async fn redb_table_define(p: TableDefineParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "const {c}: redb::TableDefinition<{k}, {v}> = redb::TableDefinition::new({n:?});",
        c = p.const_name,
        k = p.key_type,
        v = p.value_type,
        n = p.table_name,
    ))
}

#[elicit_tool(
    plugin = "redb_table",
    name = "redb_table__open_write",
    description = "Emit `write_txn.open_table(TABLE)?` to get a mutable table handle inside a write transaction."
)]
#[instrument]
async fn redb_table_open_write(p: TableOpenWriteParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let mut {tv} = {txn}.open_table({cn})?;",
        tv = p.table_var,
        txn = p.txn_var,
        cn = p.const_name,
    ))
}

#[elicit_tool(
    plugin = "redb_table",
    name = "redb_table__open_read",
    description = "Emit `read_txn.open_table(TABLE)?` to get a read-only table handle inside a read transaction."
)]
#[instrument]
async fn redb_table_open_read(p: TableOpenReadParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let {tv} = {txn}.open_table({cn})?;",
        tv = p.table_var,
        txn = p.txn_var,
        cn = p.const_name,
    ))
}

#[elicit_tool(
    plugin = "redb_table",
    name = "redb_table__insert",
    description = "Emit a `table.insert(key, value)?` snippet for inserting into a redb table."
)]
#[instrument]
async fn redb_table_insert(p: TableInsertParams) -> Result<CallToolResult, ErrorData> {
    if p.capture_old {
        ok(format!(
            "let old = {t}.insert({k}, {v})?;",
            t = p.table_var,
            k = p.key_expr,
            v = p.value_expr,
        ))
    } else {
        ok(format!(
            "{t}.insert({k}, {v})?;",
            t = p.table_var,
            k = p.key_expr,
            v = p.value_expr,
        ))
    }
}

#[elicit_tool(
    plugin = "redb_table",
    name = "redb_table__get",
    description = "Emit a `table.get(key)?` → `Option<AccessGuard>` snippet with an if-let pattern."
)]
#[instrument]
async fn redb_table_get(p: TableGetParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "if let Some(guard) = {t}.get({k})? {{\n    let value = guard.value();\n    println!(\"{{value:?}}\");\n}}",
        t = p.table_var,
        k = p.key_expr,
    ))
}

#[elicit_tool(
    plugin = "redb_table",
    name = "redb_table__remove",
    description = "Emit a `table.remove(key)?` snippet for deleting an entry from a redb table."
)]
#[instrument]
async fn redb_table_remove(p: TableRemoveParams) -> Result<CallToolResult, ErrorData> {
    if p.capture_old {
        ok(format!(
            "let old = {t}.remove({k})?;",
            t = p.table_var,
            k = p.key_expr,
        ))
    } else {
        ok(format!(
            "{t}.remove({k})?;",
            t = p.table_var,
            k = p.key_expr,
        ))
    }
}

#[elicit_tool(
    plugin = "redb_table",
    name = "redb_table__iter",
    description = "Emit a `for entry in table.iter()?` loop skeleton for iterating all entries in a redb table."
)]
#[instrument]
async fn redb_table_iter(p: TableIterParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "for item in {t}.iter()? {{\n    let ({k}, {v}) = item?;\n    let {k} = {k}.value();\n    let {v} = {v}.value();\n    // use {k} and {v} here\n}}",
        t = p.table_var,
        k = p.key_var,
        v = p.value_var,
    ))
}

#[elicit_tool(
    plugin = "redb_table",
    name = "redb_table__range",
    description = "Emit a `table.range(lo..=hi)?` snippet for iterating a key range in a redb table."
)]
#[instrument]
async fn redb_table_range(p: TableRangeParams) -> Result<CallToolResult, ErrorData> {
    let range = if p.inclusive {
        format!("{}..={}", p.lo, p.hi)
    } else {
        format!("{}..{}", p.lo, p.hi)
    };
    ok(format!(
        "for item in {t}.range({range})? {{\n    let (key, value) = item?;\n    let key = key.value();\n    let value = value.value();\n    // use key and value here\n}}",
        t = p.table_var,
    ))
}

#[elicit_tool(
    plugin = "redb_table",
    name = "redb_table__pop",
    description = "Emit a `table.pop_first()` or `table.pop_last()` snippet for removing and returning the first or last entry."
)]
#[instrument]
async fn redb_table_pop(p: TablePopParams) -> Result<CallToolResult, ErrorData> {
    let method = if p.pop_last { "pop_last" } else { "pop_first" };
    ok(format!(
        "if let Some((key, value)) = {t}.{method}()? {{\n    let key = key.value();\n    let value = value.value();\n    println!(\"popped: {{key:?}} → {{value:?}}\");\n}}",
        t = p.table_var,
    ))
}

#[elicit_tool(
    plugin = "redb_table",
    name = "redb_table__retain",
    description = "Emit a `table.retain(|k, v| predicate)?` snippet to remove entries that fail a predicate."
)]
#[instrument]
async fn redb_table_retain(p: TableRetainParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "{t}.retain(|k, v| {pred})?;",
        t = p.table_var,
        pred = p.predicate,
    ))
}

#[elicit_tool(
    plugin = "redb_table",
    name = "redb_table__len",
    description = "Emit `table.len()` and `table.is_empty()` snippets for checking the size of a redb table."
)]
#[instrument]
async fn redb_table_len(p: TableLenParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let len = {t}.len()?;\nlet is_empty = {t}.is_empty()?;\nprintln!(\"len={{len}}, is_empty={{is_empty}}\");",
        t = p.table_var,
    ))
}

#[elicit_tool(
    plugin = "redb_table",
    name = "redb_table__rename",
    description = "Emit a `write_txn.rename_table(old, new)?` snippet to rename a redb table."
)]
#[instrument]
async fn redb_table_rename(p: TableRenameParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "{txn}.rename_table(&{old}, &{new})?;",
        txn = p.txn_var,
        old = p.old_const,
        new = p.new_const,
    ))
}

#[elicit_tool(
    plugin = "redb_table",
    name = "redb_table__delete",
    description = "Emit a `write_txn.delete_table(TABLE)?` snippet to permanently delete a redb table and all its data."
)]
#[instrument]
async fn redb_table_delete(p: TableDeleteParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let existed = {txn}.delete_table({cn})?;",
        txn = p.txn_var,
        cn = p.const_name,
    ))
}

#[elicit_tool(
    plugin = "redb_table",
    name = "redb_table__list",
    description = "Emit a `write_txn.list_tables()` iteration snippet to enumerate all tables in a redb database."
)]
#[instrument]
async fn redb_table_list(p: TableListParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "for handle in {txn}.list_tables()? {{\n    println!(\"{{}}\", handle.name());\n}}",
        txn = p.txn_var,
    ))
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// Plugin providing redb `TableDefinition`, typed CRUD, and iteration tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "redb_table")]
pub struct RedbTablePlugin;
