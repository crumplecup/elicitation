//! Shadow of [`redb::Database`].
//!
//! `elicit_redb::Database` mirrors `redb::Database` with identical method names.
//! The actual `redb::Database` is held in [`RedbCtx`] keyed by UUID.

use std::sync::Arc;

use elicitation::elicit_tool;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use redb::ReadableDatabase as _;

use crate::{
    ReadTransaction, WriteTransaction,
    plugin::{RedbCtx, ok_json, ok_text},
};

// ── shadow type ───────────────────────────────────────────────────────────────

/// Shadow of `redb::Database`.
///
/// A UUID handle identifying a live `redb::Database` in the plugin context.
#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct Database(pub Uuid);

impl Serialize for Database {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Database {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse::<Uuid>()
            .map(Database)
            .map_err(serde::de::Error::custom)
    }
}

// ── tools — mirrors redb::Database API ───────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DatabaseCreateParams {
    /// Filesystem path for the new database file.
    pub path: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DatabaseOpenParams {
    /// Filesystem path of an existing database file.
    pub path: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DatabaseBuilderCreateParams {
    /// Filesystem path for the new database file.
    pub path: String,
    /// Optional in-memory cache size in bytes.
    #[serde(default)]
    pub cache_size: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DatabaseBuilderOpenParams {
    /// Filesystem path of an existing database file.
    pub path: String,
    /// Optional in-memory cache size in bytes.
    #[serde(default)]
    pub cache_size: Option<usize>,
}

/// Params for tools that operate on an existing database handle.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DatabaseSelfParams {
    /// Database handle returned by `create` or `open`.
    pub db: Database,
}

/// Mirrors `redb::Database::create`.
#[elicit_tool(
    plugin = "redb",
    name = "database__create",
    description = "Create a new redb database at path. Returns a Database handle.",
    emit = None
)]
#[instrument(skip(ctx), fields(path = %p.path))]
pub async fn database_create(
    ctx: Arc<RedbCtx>,
    p: DatabaseCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let db = redb::Database::create(&p.path)
        .map_err(|e| ErrorData::internal_error(format!("create: {e}"), None))?;
    let id = Uuid::new_v4();
    ctx.lock_databases()?.insert(id, db);
    ok_json(&Database(id))
}

/// Mirrors `redb::Database::open`.
#[elicit_tool(
    plugin = "redb",
    name = "database__open",
    description = "Open an existing redb database at path. Returns a Database handle.",
    emit = None
)]
#[instrument(skip(ctx), fields(path = %p.path))]
pub async fn database_open(
    ctx: Arc<RedbCtx>,
    p: DatabaseOpenParams,
) -> Result<CallToolResult, ErrorData> {
    let db = redb::Database::open(&p.path)
        .map_err(|e| ErrorData::internal_error(format!("open: {e}"), None))?;
    let id = Uuid::new_v4();
    ctx.lock_databases()?.insert(id, db);
    ok_json(&Database(id))
}

/// Create via `redb::Builder` with optional cache size.
#[elicit_tool(
    plugin = "redb",
    name = "database__builder_create",
    description = "Create a redb database via Builder with optional cache_size. Returns a Database handle.",
    emit = None
)]
#[instrument(skip(ctx), fields(path = %p.path))]
pub async fn database_builder_create(
    ctx: Arc<RedbCtx>,
    p: DatabaseBuilderCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let mut b = redb::Builder::new();
    if let Some(cs) = p.cache_size {
        b.set_cache_size(cs);
    }
    let db = b
        .create(&p.path)
        .map_err(|e| ErrorData::internal_error(format!("builder create: {e}"), None))?;
    let id = Uuid::new_v4();
    ctx.lock_databases()?.insert(id, db);
    ok_json(&Database(id))
}

/// Open via `redb::Builder` with optional cache size.
#[elicit_tool(
    plugin = "redb",
    name = "database__builder_open",
    description = "Open a redb database via Builder with optional cache_size. Returns a Database handle.",
    emit = None
)]
#[instrument(skip(ctx), fields(path = %p.path))]
pub async fn database_builder_open(
    ctx: Arc<RedbCtx>,
    p: DatabaseBuilderOpenParams,
) -> Result<CallToolResult, ErrorData> {
    let mut b = redb::Builder::new();
    if let Some(cs) = p.cache_size {
        b.set_cache_size(cs);
    }
    let db = b
        .open(&p.path)
        .map_err(|e| ErrorData::internal_error(format!("builder open: {e}"), None))?;
    let id = Uuid::new_v4();
    ctx.lock_databases()?.insert(id, db);
    ok_json(&Database(id))
}

/// Mirrors `redb::Database::begin_write`.
#[elicit_tool(
    plugin = "redb",
    name = "database__begin_write",
    description = "Begin a write transaction on the database. Returns a WriteTransaction handle.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn database_begin_write(
    ctx: Arc<RedbCtx>,
    p: DatabaseSelfParams,
) -> Result<CallToolResult, ErrorData> {
    let txn = {
        let dbs = ctx.lock_databases()?;
        let db = dbs
            .get(&p.db.0)
            .ok_or_else(|| ErrorData::invalid_params(format!("unknown db: {}", p.db.0), None))?;
        db.begin_write()
            .map_err(|e| ErrorData::internal_error(format!("begin_write: {e}"), None))?
    };
    let id = Uuid::new_v4();
    ctx.lock_write_txns()?.insert(id, txn);
    ok_json(&WriteTransaction(id))
}

/// Mirrors `redb::Database::begin_read`.
#[elicit_tool(
    plugin = "redb",
    name = "database__begin_read",
    description = "Begin a read-only transaction on the database. Returns a ReadTransaction handle.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn database_begin_read(
    ctx: Arc<RedbCtx>,
    p: DatabaseSelfParams,
) -> Result<CallToolResult, ErrorData> {
    let txn = {
        let dbs = ctx.lock_databases()?;
        let db = dbs
            .get(&p.db.0)
            .ok_or_else(|| ErrorData::invalid_params(format!("unknown db: {}", p.db.0), None))?;
        db.begin_read()
            .map_err(|e| ErrorData::internal_error(format!("begin_read: {e}"), None))?
    };
    let id = Uuid::new_v4();
    ctx.lock_read_txns()?.insert(id, txn);
    ok_json(&ReadTransaction(id))
}

/// Mirrors `redb::Database::compact`.
#[elicit_tool(
    plugin = "redb",
    name = "database__compact",
    description = "Compact the database, reclaiming free pages. Returns true if pages were freed.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn database_compact(
    ctx: Arc<RedbCtx>,
    p: DatabaseSelfParams,
) -> Result<CallToolResult, ErrorData> {
    let freed = ctx
        .lock_databases()?
        .get_mut(&p.db.0)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown db: {}", p.db.0), None))?
        .compact()
        .map_err(|e| ErrorData::internal_error(format!("compact: {e}"), None))?;
    ok_text(if freed { "true" } else { "false" })
}

/// Mirrors `redb::Database::check_integrity`.
#[elicit_tool(
    plugin = "redb",
    name = "database__check_integrity",
    description = "Verify database integrity. Returns true if consistent.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn database_check_integrity(
    ctx: Arc<RedbCtx>,
    p: DatabaseSelfParams,
) -> Result<CallToolResult, ErrorData> {
    let ok = ctx
        .lock_databases()?
        .get_mut(&p.db.0)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown db: {}", p.db.0), None))?
        .check_integrity()
        .map_err(|e| ErrorData::internal_error(format!("check_integrity: {e}"), None))?;
    ok_text(if ok { "true" } else { "false" })
}

/// Close the database and release its file handle.
#[elicit_tool(
    plugin = "redb",
    name = "database__close",
    description = "Close the database, releasing its file handle.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn database_close(
    ctx: Arc<RedbCtx>,
    p: DatabaseSelfParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.lock_databases()?
        .remove(&p.db.0)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown db: {}", p.db.0), None))?;
    ok_text("closed")
}
