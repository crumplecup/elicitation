//! `RedbDatabasePlugin` — redb `Database`, `ReadOnlyDatabase`, and `Builder` snippets.

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

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `redb_database__create`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DbCreateParams {
    /// Variable name for the `Database` binding.
    #[serde(default = "default_db_var")]
    pub db_var: String,
    /// File-system path to the database file.
    pub path: String,
}
fn default_db_var() -> String {
    "db".into()
}

/// Parameters for `redb_database__open`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DbOpenParams {
    /// Variable name for the `Database` binding.
    #[serde(default = "default_db_var")]
    pub db_var: String,
    /// File-system path to the database file.
    pub path: String,
}

/// Parameters for `redb_database__open_read_only`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DbOpenReadOnlyParams {
    /// Variable name for the `ReadOnlyDatabase` binding.
    #[serde(default = "default_db_var")]
    pub db_var: String,
    /// File-system path to the database file.
    pub path: String,
}

/// Parameters for `redb_database__builder_new`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DbBuilderParams {
    /// Variable name for the resulting `Database`.
    #[serde(default = "default_db_var")]
    pub db_var: String,
    /// File-system path to the database file.
    pub path: String,
    /// Optional page size in bytes (must be a power of 2, ≥512).
    #[serde(default)]
    pub page_size: Option<usize>,
    /// Optional in-memory cache size in bytes.
    #[serde(default)]
    pub cache_size: Option<usize>,
    /// Emit as `create` (new file, error if exists) or `open` (error if missing).
    #[serde(default = "default_create_mode")]
    pub create_mode: bool,
}
fn default_create_mode() -> bool {
    true
}

/// Parameters for `redb_database__begin_write`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DbBeginWriteParams {
    /// Variable name for the `WriteTransaction`.
    #[serde(default = "default_write_txn")]
    pub txn_var: String,
    /// Database variable name.
    #[serde(default = "default_db_var")]
    pub db_var: String,
    /// Include a placeholder `// … operations …` comment inside the block.
    #[serde(default = "default_true")]
    pub include_placeholder: bool,
}
fn default_write_txn() -> String {
    "write_txn".into()
}
fn default_true() -> bool {
    true
}

/// Parameters for `redb_database__begin_read`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DbBeginReadParams {
    /// Variable name for the `ReadTransaction`.
    #[serde(default = "default_read_txn")]
    pub txn_var: String,
    /// Database variable name.
    #[serde(default = "default_db_var")]
    pub db_var: String,
}
fn default_read_txn() -> String {
    "read_txn".into()
}

/// Parameters for `redb_database__compact`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DbCompactParams {
    /// Database variable name.
    #[serde(default = "default_db_var")]
    pub db_var: String,
}

/// Parameters for `redb_database__check_integrity`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DbCheckIntegrityParams {
    /// Database variable name.
    #[serde(default = "default_db_var")]
    pub db_var: String,
    /// Use `QuickRepair` instead of the default `SavepointRepair`.
    #[serde(default)]
    pub quick_repair: bool,
}

/// Parameters for `redb_database__stats`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DbStatsParams {
    /// Write-transaction variable name.
    #[serde(default = "default_write_txn")]
    pub txn_var: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "redb_database",
    name = "redb_database__create",
    description = "Emit a `Database::create(path)` snippet for creating a new redb database file."
)]
#[instrument]
async fn redb_database_create(p: DbCreateParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let {v} = redb::Database::create({path:?})?;",
        v = p.db_var,
        path = p.path,
    ))
}

#[elicit_tool(
    plugin = "redb_database",
    name = "redb_database__open",
    description = "Emit a `Database::open(path)` snippet for opening an existing redb database file."
)]
#[instrument]
async fn redb_database_open(p: DbOpenParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let {v} = redb::Database::open({path:?})?;",
        v = p.db_var,
        path = p.path,
    ))
}

#[elicit_tool(
    plugin = "redb_database",
    name = "redb_database__open_read_only",
    description = "Emit a `ReadOnlyDatabase::open(path)` snippet for read-only access to a redb file."
)]
#[instrument]
async fn redb_database_open_read_only(
    p: DbOpenReadOnlyParams,
) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let {v} = redb::ReadOnlyDatabase::open({path:?})?;",
        v = p.db_var,
        path = p.path,
    ))
}

#[elicit_tool(
    plugin = "redb_database",
    name = "redb_database__builder_new",
    description = "Emit a `Builder::new()` chain with optional `set_page_size` and `set_cache_size` calls."
)]
#[instrument]
async fn redb_database_builder_new(p: DbBuilderParams) -> Result<CallToolResult, ErrorData> {
    let mut chain = "let mut builder = redb::Builder::new();\n".to_string();
    if let Some(ps) = p.page_size {
        chain.push_str(&format!("builder.set_page_size({ps});\n"));
    }
    if let Some(cs) = p.cache_size {
        chain.push_str(&format!("builder.set_cache_size({cs});\n"));
    }
    let method = if p.create_mode { "create" } else { "open" };
    chain.push_str(&format!(
        "let {v} = builder.{method}({path:?})?;",
        v = p.db_var,
        path = p.path,
    ));
    ok(chain)
}

#[elicit_tool(
    plugin = "redb_database",
    name = "redb_database__begin_write",
    description = "Emit a complete `begin_write` / `commit` / `abort` block for a redb write transaction."
)]
#[instrument]
async fn redb_database_begin_write(p: DbBeginWriteParams) -> Result<CallToolResult, ErrorData> {
    let placeholder = if p.include_placeholder {
        "\n    // … operations …\n"
    } else {
        "\n"
    };
    ok(format!(
        "let {txn} = {db}.begin_write()?;\n{{{placeholder}    {txn}.commit()?;\n}}\n// To abort instead:  {txn}.abort();",
        txn = p.txn_var,
        db = p.db_var,
    ))
}

#[elicit_tool(
    plugin = "redb_database",
    name = "redb_database__begin_read",
    description = "Emit a `begin_read` snippet for starting a redb read transaction."
)]
#[instrument]
async fn redb_database_begin_read(p: DbBeginReadParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let {txn} = {db}.begin_read()?;",
        txn = p.txn_var,
        db = p.db_var,
    ))
}

#[elicit_tool(
    plugin = "redb_database",
    name = "redb_database__compact",
    description = "Emit a `db.compact()` snippet to reclaim unused space in a redb database."
)]
#[instrument]
async fn redb_database_compact(p: DbCompactParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let compacted = {db}.compact()?;\n// `compacted` is `true` if the database was actually compacted.",
        db = p.db_var,
    ))
}

#[elicit_tool(
    plugin = "redb_database",
    name = "redb_database__check_integrity",
    description = "Emit a `db.check_integrity(repair)` snippet with SavepointRepair or QuickRepair."
)]
#[instrument]
async fn redb_database_check_integrity(
    p: DbCheckIntegrityParams,
) -> Result<CallToolResult, ErrorData> {
    let repair = if p.quick_repair {
        "QuickRepair"
    } else {
        "SavepointRepair"
    };
    ok(format!(
        "{db}.check_integrity(|repair| repair.{repair}())?;",
        db = p.db_var,
    ))
}

#[elicit_tool(
    plugin = "redb_database",
    name = "redb_database__stats",
    description = "Emit a `write_txn.stats()` usage snippet showing tree_height, allocated_pages, leaf_pages, and stored_bytes."
)]
#[instrument]
async fn redb_database_stats(p: DbStatsParams) -> Result<CallToolResult, ErrorData> {
    ok(format!(
        "let stats = {txn}.stats()?;\nprintln!(\"tree_height={{}}, allocated_pages={{}}, leaf_pages={{}}, stored_bytes={{}}\",\n    stats.tree_height(), stats.allocated_pages(), stats.leaf_pages(), stats.stored_bytes());",
        txn = p.txn_var,
    ))
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// Plugin providing redb `Database` / `ReadOnlyDatabase` / `Builder` code-generation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "redb_database")]
pub struct RedbDatabasePlugin;
