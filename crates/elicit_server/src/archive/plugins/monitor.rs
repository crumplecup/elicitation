//! `ArchiveMonitorPlugin` — live database monitoring and administration.
//!
//! Wraps [`DbMonitor`], [`DbRoleManager`], [`DbBackupManager`], and
//! [`DbServerAdmin`] methods to expose them as MCP tools.  All results carry
//! an [`Established`] proof token recording the monitoring contract.
//!
//! Primary backend: PostgreSQL.  Graceful fallback on other backends.

use elicit_db::{
    DbBackupManager, DbMonitor, DbRoleManager, DbServerAdmin, DbSessionInfo, DbStatActivity,
};
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, contracts::Established, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::archive::backend::ArchiveDbBackend;

// ── helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        serde_json::to_string(value).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

fn db_err(e: impl std::fmt::Display) -> ErrorData {
    ErrorData::internal_error(e.to_string(), None)
}

// ── propositions ──────────────────────────────────────────────────────────────

/// Proposition: a live activity snapshot was successfully read from the
/// database.
#[derive(Prop)]
pub struct ActivityRead;
impl VerifiedWorkflow for ActivityRead {}

/// Proposition: slow-query statistics were successfully read.
#[derive(Prop)]
pub struct SlowQueriesRead;
impl VerifiedWorkflow for SlowQueriesRead {}

/// Proposition: lock-wait data was successfully read.
#[derive(Prop)]
pub struct LockWaitsRead;
impl VerifiedWorkflow for LockWaitsRead {}

/// Proposition: the cache hit ratio was successfully computed.
#[derive(Prop)]
pub struct CacheHitRead;
impl VerifiedWorkflow for CacheHitRead {}

/// Proposition: table-bloat statistics were successfully read.
#[derive(Prop)]
pub struct TableBloatRead;
impl VerifiedWorkflow for TableBloatRead {}

/// Proposition: index-usage statistics were successfully read.
#[derive(Prop)]
pub struct IndexUsageRead;
impl VerifiedWorkflow for IndexUsageRead {}

/// Proposition: the role list was successfully read.
#[derive(Prop)]
pub struct RolesRead;
impl VerifiedWorkflow for RolesRead {}

/// Proposition: a backup was successfully initiated.
#[derive(Prop)]
pub struct BackupInitiated;
impl VerifiedWorkflow for BackupInitiated {}

/// Proposition: the backup list was successfully read.
#[derive(Prop)]
pub struct BackupsRead;
impl VerifiedWorkflow for BackupsRead {}

/// Proposition: server settings were successfully read.
#[derive(Prop)]
pub struct SettingsRead;
impl VerifiedWorkflow for SettingsRead {}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `archive_monitor__active_sessions`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ActiveSessionsParams {
    /// Connection URL.
    pub url: String,
}

/// Parameters for `archive_monitor__slow_queries`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SlowQueriesParams {
    /// Connection URL.
    pub url: String,
    /// Minimum query duration (milliseconds) to include in results.
    pub threshold_ms: u64,
}

/// Parameters for `archive_monitor__lock_waits`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LockWaitsParams {
    /// Connection URL.
    pub url: String,
}

/// Parameters for `archive_monitor__cache_hit_ratio`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CacheHitRatioParams {
    /// Connection URL.
    pub url: String,
}

/// Parameters for `archive_monitor__table_bloat`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableBloatParams {
    /// Connection URL.
    pub url: String,
    /// Schema to analyse for bloat.
    pub schema: String,
}

/// Parameters for `archive_monitor__index_usage`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IndexUsageParams {
    /// Connection URL.
    pub url: String,
    /// Schema to analyse index usage in.
    pub schema: String,
}

/// Parameters for `archive_monitor__list_roles`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListRolesParams {
    /// Connection URL.
    pub url: String,
}

/// Parameters for `archive_monitor__initiate_backup`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InitiateBackupParams {
    /// Connection URL.
    pub url: String,
    /// Human-readable label for this backup.
    pub label: String,
}

/// Parameters for `archive_monitor__list_backups`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListBackupsParams {
    /// Connection URL.
    pub url: String,
}

/// Parameters for `archive_monitor__list_settings`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListSettingsParams {
    /// Connection URL.
    pub url: String,
    /// Optional filter substring applied to setting names.
    pub filter: Option<String>,
}

// ── result types ──────────────────────────────────────────────────────────────

/// A single lock-wait pair: the blocking and blocked PIDs.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LockWaitPair {
    /// PID of the session holding the lock.
    pub blocking_pid: i32,
    /// PID of the session waiting for the lock.
    pub blocked_pid: i32,
}

/// A table bloat measurement.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableBloatEntry {
    /// Table name.
    pub table_name: String,
    /// Dead-tuple fraction (0.0–1.0); higher means more bloat.
    pub bloat_ratio: f64,
}

/// An index usage measurement.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IndexUsageEntry {
    /// Index name.
    pub index_name: String,
    /// Total number of index scans.
    pub scan_count: u64,
}

/// A server configuration setting.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ServerSetting {
    /// Setting name (e.g. `"max_connections"`).
    pub name: String,
    /// Current effective value.
    pub value: String,
    /// Human-readable description (empty if not available from backend).
    pub description: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

/// Return all active database sessions from `pg_stat_activity`.
#[elicit_tool(
    plugin = "archive_monitor",
    name = "archive_monitor__active_sessions",
    description = "Return all active PostgreSQL sessions from pg_stat_activity."
)]
#[instrument]
async fn active_sessions(p: ActiveSessionsParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<ActivityRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let activity: DbStatActivity = backend.active_sessions().await.map_err(db_err)?;
    json_result(&activity)
}

/// Return sessions whose current query exceeds `threshold_ms` milliseconds.
#[elicit_tool(
    plugin = "archive_monitor",
    name = "archive_monitor__slow_queries",
    description = "Return PostgreSQL sessions whose current query duration exceeds a threshold."
)]
#[instrument]
async fn slow_queries(p: SlowQueriesParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<SlowQueriesRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let sessions: Vec<DbSessionInfo> =
        backend.slow_queries(p.threshold_ms).await.map_err(db_err)?;
    json_result(&sessions)
}

/// Return `(blocking_pid, blocked_pid)` pairs for current lock waits.
#[elicit_tool(
    plugin = "archive_monitor",
    name = "archive_monitor__lock_waits",
    description = "Return current PostgreSQL lock-wait pairs (blocking PID, blocked PID)."
)]
#[instrument]
async fn lock_waits(p: LockWaitsParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<LockWaitsRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let pairs: Vec<(i32, i32)> = backend.lock_waits().await.map_err(db_err)?;
    let result: Vec<LockWaitPair> = pairs
        .into_iter()
        .map(|(blocking_pid, blocked_pid)| LockWaitPair {
            blocking_pid,
            blocked_pid,
        })
        .collect();
    json_result(&result)
}

/// Return the buffer cache hit ratio (0.0–1.0).
#[elicit_tool(
    plugin = "archive_monitor",
    name = "archive_monitor__cache_hit_ratio",
    description = "Return the PostgreSQL buffer cache hit ratio (0.0 = no hits, 1.0 = all hits)."
)]
#[instrument]
async fn cache_hit_ratio(p: CacheHitRatioParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<CacheHitRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let ratio: f64 = backend.cache_hit_ratio().await.map_err(db_err)?;
    json_result(&ratio)
}

/// Return dead-tuple bloat ratios for tables in a schema.
#[elicit_tool(
    plugin = "archive_monitor",
    name = "archive_monitor__table_bloat",
    description = "Return dead-tuple bloat ratios for tables in a schema."
)]
#[instrument]
async fn table_bloat(p: TableBloatParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<TableBloatRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let pairs: Vec<(String, f64)> = backend.table_bloat(&p.schema).await.map_err(db_err)?;
    let result: Vec<TableBloatEntry> = pairs
        .into_iter()
        .map(|(table_name, bloat_ratio)| TableBloatEntry {
            table_name,
            bloat_ratio,
        })
        .collect();
    json_result(&result)
}

/// Return scan counts for all indexes in a schema.
#[elicit_tool(
    plugin = "archive_monitor",
    name = "archive_monitor__index_usage",
    description = "Return scan counts for all indexes in a schema."
)]
#[instrument]
async fn index_usage(p: IndexUsageParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<IndexUsageRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let pairs: Vec<(String, u64)> = backend.index_usage(&p.schema).await.map_err(db_err)?;
    let result: Vec<IndexUsageEntry> = pairs
        .into_iter()
        .map(|(index_name, scan_count)| IndexUsageEntry {
            index_name,
            scan_count,
        })
        .collect();
    json_result(&result)
}

/// List all database roles.
#[elicit_tool(
    plugin = "archive_monitor",
    name = "archive_monitor__list_roles",
    description = "List all PostgreSQL roles with their attributes."
)]
#[instrument]
async fn list_roles(p: ListRolesParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<RolesRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let roles = backend.list_roles().await.map_err(db_err)?;
    json_result(&roles)
}

/// Initiate a base backup with the given label.
#[elicit_tool(
    plugin = "archive_monitor",
    name = "archive_monitor__initiate_backup",
    description = "Initiate a PostgreSQL base backup with the given label."
)]
#[instrument]
async fn initiate_backup(p: InitiateBackupParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<BackupInitiated>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let _tokens = backend.initiate_backup(&p.label).await.map_err(db_err)?;
    json_result(&format!("Backup '{}' initiated successfully.", p.label))
}

/// List recent backups.
#[elicit_tool(
    plugin = "archive_monitor",
    name = "archive_monitor__list_backups",
    description = "List recent PostgreSQL base backups."
)]
#[instrument]
async fn list_backups(p: ListBackupsParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<BackupsRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let backups = backend.list_backups().await.map_err(db_err)?;
    json_result(&backups)
}

/// Browse `pg_settings` with optional name filter.
#[elicit_tool(
    plugin = "archive_monitor",
    name = "archive_monitor__list_settings",
    description = "Browse pg_settings with optional name-filter substring."
)]
#[instrument]
async fn list_settings(p: ListSettingsParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<SettingsRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let raw: Vec<(String, String)> = backend.list_settings().await.map_err(db_err)?;

    let filter = p.filter.as_deref().unwrap_or("").to_lowercase();
    let settings: Vec<ServerSetting> = raw
        .into_iter()
        .filter(|(name, _)| filter.is_empty() || name.to_lowercase().contains(&filter))
        .map(|(name, value)| ServerSetting {
            name,
            value,
            description: String::new(),
        })
        .collect();
    json_result(&settings)
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for live database monitoring and administration.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "archive_monitor")]
pub struct ArchiveMonitorPlugin;

impl ArchiveMonitorPlugin {
    /// Create a new monitor plugin.
    pub fn new() -> Self {
        Self
    }
}

impl Default for ArchiveMonitorPlugin {
    fn default() -> Self {
        Self::new()
    }
}
