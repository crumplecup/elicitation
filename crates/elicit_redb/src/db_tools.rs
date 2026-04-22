//! `#[elicit_tool]` functions for `redb::Database` and `redb::Builder` operations.

use std::sync::Arc;

use elicitation::elicit_tool;
use redb::ReadableDatabase as _;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::plugin::{RedbCtx, ok_json, ok_text, parse_uuid};

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `redb__database_create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DatabaseCreateParams {
    /// Filesystem path for the new database file (must not already exist as a
    /// redb database, though the file may be absent).
    pub path: String,
}

/// Parameters for `redb__database_open`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DatabaseOpenParams {
    /// Filesystem path of an existing redb database file.
    pub path: String,
}

/// Parameters for `redb__database_open_read_only`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DatabaseOpenReadOnlyParams {
    /// Filesystem path of an existing redb database file to open read-only.
    pub path: String,
}

/// Parameters for `redb__database_builder_create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DatabaseBuilderCreateParams {
    /// Filesystem path for the new database file.
    pub path: String,
    /// Optional in-memory cache size in bytes.
    #[serde(default)]
    pub cache_size: Option<usize>,
}

/// Parameters for `redb__database_builder_open`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DatabaseBuilderOpenParams {
    /// Filesystem path of an existing redb database file.
    pub path: String,
    /// Optional in-memory cache size in bytes.
    #[serde(default)]
    pub cache_size: Option<usize>,
}

/// Parameters for `redb__database_compact` and other single-database tools.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DatabaseIdParams {
    /// UUID returned by a previous `redb__database_*` create/open call.
    pub db_id: String,
}

/// Parameters for `redb__database_begin_write` and `redb__database_begin_read`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DatabaseBeginParams {
    /// UUID returned by a previous `redb__database_*` create/open call.
    pub db_id: String,
}

// ── result types ──────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct IdResult {
    id: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "redb",
    name = "database_create",
    description = "Create a new redb database at the given path. Returns a db_id UUID for subsequent operations.",
    emit = None
)]
#[instrument(skip(ctx), fields(path = %p.path))]
async fn database_create(
    ctx: Arc<RedbCtx>,
    p: DatabaseCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let db = redb::Database::create(&p.path)
        .map_err(|e| ErrorData::internal_error(format!("redb create error: {e}"), None))?;
    let id = uuid::Uuid::new_v4();
    ctx.lock_databases()?.insert(id, db);
    ok_json(&IdResult { id: id.to_string() })
}

#[elicit_tool(
    plugin = "redb",
    name = "database_open",
    description = "Open an existing redb database at the given path. Returns a db_id UUID.",
    emit = None
)]
#[instrument(skip(ctx), fields(path = %p.path))]
async fn database_open(
    ctx: Arc<RedbCtx>,
    p: DatabaseOpenParams,
) -> Result<CallToolResult, ErrorData> {
    let db = redb::Database::open(&p.path)
        .map_err(|e| ErrorData::internal_error(format!("redb open error: {e}"), None))?;
    let id = uuid::Uuid::new_v4();
    ctx.lock_databases()?.insert(id, db);
    ok_json(&IdResult { id: id.to_string() })
}

#[elicit_tool(
    plugin = "redb",
    name = "database_open_read_only",
    description = "Open an existing redb database in read-only mode. Returns a db_id UUID.",
    emit = None
)]
#[instrument(skip(ctx), fields(path = %p.path))]
async fn database_open_read_only(
    ctx: Arc<RedbCtx>,
    p: DatabaseOpenReadOnlyParams,
) -> Result<CallToolResult, ErrorData> {
    let db = redb::Builder::new()
        .open_read_only(&p.path)
        .map_err(|e| ErrorData::internal_error(format!("redb open_read_only error: {e}"), None))?;
    // `ReadOnlyDatabase` doesn't support write transactions.
    // For MCP use, we open it as read-only and immediately begin a read transaction.
    // Store the underlying database (ReadOnlyDatabase is a separate type; store via begin_read).
    // Since ReadOnlyDatabase != Database, we record a read txn directly:
    let txn = redb::ReadableDatabase::begin_read(&db)
        .map_err(|e| ErrorData::internal_error(format!("begin_read error: {e}"), None))?;
    let txn_id = uuid::Uuid::new_v4();
    ctx.lock_read_txns()?.insert(txn_id, txn);
    ok_json(&IdResult {
        id: txn_id.to_string(),
    })
}

#[elicit_tool(
    plugin = "redb",
    name = "database_builder_create",
    description = "Create a redb database using Builder with optional page_size and cache_size. Returns a db_id UUID.",
    emit = None
)]
#[instrument(skip(ctx), fields(path = %p.path))]
async fn database_builder_create(
    ctx: Arc<RedbCtx>,
    p: DatabaseBuilderCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let mut builder = redb::Builder::new();
    if let Some(cs) = p.cache_size {
        builder.set_cache_size(cs);
    }
    let db = builder
        .create(&p.path)
        .map_err(|e| ErrorData::internal_error(format!("redb builder create error: {e}"), None))?;
    let id = uuid::Uuid::new_v4();
    ctx.lock_databases()?.insert(id, db);
    ok_json(&IdResult { id: id.to_string() })
}

#[elicit_tool(
    plugin = "redb",
    name = "database_builder_open",
    description = "Open an existing redb database using Builder with optional cache_size. Returns a db_id UUID.",
    emit = None
)]
#[instrument(skip(ctx), fields(path = %p.path))]
async fn database_builder_open(
    ctx: Arc<RedbCtx>,
    p: DatabaseBuilderOpenParams,
) -> Result<CallToolResult, ErrorData> {
    let mut builder = redb::Builder::new();
    if let Some(cs) = p.cache_size {
        builder.set_cache_size(cs);
    }
    let db = builder
        .open(&p.path)
        .map_err(|e| ErrorData::internal_error(format!("redb builder open error: {e}"), None))?;
    let id = uuid::Uuid::new_v4();
    ctx.lock_databases()?.insert(id, db);
    ok_json(&IdResult { id: id.to_string() })
}

#[elicit_tool(
    plugin = "redb",
    name = "database_begin_write",
    description = "Begin a write transaction on the given database. Returns a txn_id UUID.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn database_begin_write(
    ctx: Arc<RedbCtx>,
    p: DatabaseBeginParams,
) -> Result<CallToolResult, ErrorData> {
    let db_id = parse_uuid(&p.db_id)?;
    let txn = {
        let dbs = ctx.lock_databases()?;
        let db = dbs.get(&db_id).ok_or_else(|| {
            ErrorData::invalid_params(format!("unknown db_id: {}", p.db_id), None)
        })?;
        db.begin_write()
            .map_err(|e| ErrorData::internal_error(format!("begin_write error: {e}"), None))?
    };
    let txn_id = uuid::Uuid::new_v4();
    ctx.lock_write_txns()?.insert(txn_id, txn);
    ok_json(&IdResult {
        id: txn_id.to_string(),
    })
}

#[elicit_tool(
    plugin = "redb",
    name = "database_begin_read",
    description = "Begin a read-only transaction on the given database. Returns a txn_id UUID.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn database_begin_read(
    ctx: Arc<RedbCtx>,
    p: DatabaseBeginParams,
) -> Result<CallToolResult, ErrorData> {
    let db_id = parse_uuid(&p.db_id)?;
    let txn = {
        let dbs = ctx.lock_databases()?;
        let db = dbs.get(&db_id).ok_or_else(|| {
            ErrorData::invalid_params(format!("unknown db_id: {}", p.db_id), None)
        })?;
        db.begin_read()
            .map_err(|e| ErrorData::internal_error(format!("begin_read error: {e}"), None))?
    };
    let txn_id = uuid::Uuid::new_v4();
    ctx.lock_read_txns()?.insert(txn_id, txn);
    ok_json(&IdResult {
        id: txn_id.to_string(),
    })
}

#[elicit_tool(
    plugin = "redb",
    name = "database_compact",
    description = "Compact the database, reclaiming free pages. Returns true if compaction freed space.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn database_compact(
    ctx: Arc<RedbCtx>,
    p: DatabaseIdParams,
) -> Result<CallToolResult, ErrorData> {
    let db_id = parse_uuid(&p.db_id)?;
    let mut dbs = ctx.lock_databases()?;
    let db = dbs
        .get_mut(&db_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown db_id: {}", p.db_id), None))?;
    let freed = db
        .compact()
        .map_err(|e| ErrorData::internal_error(format!("compact error: {e}"), None))?;
    ok_text(if freed { "true" } else { "false" })
}

#[elicit_tool(
    plugin = "redb",
    name = "database_check_integrity",
    description = "Verify database integrity. Returns true if the database is consistent.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn database_check_integrity(
    ctx: Arc<RedbCtx>,
    p: DatabaseIdParams,
) -> Result<CallToolResult, ErrorData> {
    let db_id = parse_uuid(&p.db_id)?;
    let mut dbs = ctx.lock_databases()?;
    let db = dbs
        .get_mut(&db_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown db_id: {}", p.db_id), None))?;
    let ok = db
        .check_integrity()
        .map_err(|e| ErrorData::internal_error(format!("check_integrity error: {e}"), None))?;
    ok_text(if ok { "true" } else { "false" })
}

#[elicit_tool(
    plugin = "redb",
    name = "database_close",
    description = "Remove the database from the session context, releasing the file handle.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn database_close(
    ctx: Arc<RedbCtx>,
    p: DatabaseIdParams,
) -> Result<CallToolResult, ErrorData> {
    let db_id = parse_uuid(&p.db_id)?;
    ctx.lock_databases()?
        .remove(&db_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown db_id: {}", p.db_id), None))?;
    ok_text("closed")
}
