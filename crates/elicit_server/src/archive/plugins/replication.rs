//! `ArchiveReplicationPlugin` — replication topology inspection via [`DbReplicationMeta`].
//!
//! Showcases the [`elicit_db`] trait abstraction: callers work through the
//! `DbReplicationMeta` trait to inspect replication slots, publications,
//! subscriptions, and streaming replication status.

use elicit_db::{
    DbPublicationDescriptor, DbReplicationMeta, DbReplicationSlotDescriptor,
    DbSubscriptionDescriptor,
};
use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::archive::ArchiveDbBackend;

// ── helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        serde_json::to_string(value).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `archive_replication__slot_lag`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SlotLagParams {
    /// Database connection URL.
    pub url: String,
}

/// Parameters for `archive_replication__list_publications`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListPublicationsParams {
    /// Database connection URL.
    pub url: String,
}

/// Parameters for `archive_replication__list_subscriptions`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListSubscriptionsParams {
    /// Database connection URL.
    pub url: String,
}

/// Parameters for `archive_replication__streaming_status`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StreamingStatusParams {
    /// Database connection URL.
    pub url: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "archive_replication",
    name = "archive_replication__slot_lag",
    description = "List replication slots with their WAL lag in bytes. Uses DbReplicationMeta \
                   trait against pg_replication_slots. Returns Vec<(DbReplicationSlotDescriptor, lag_bytes)>."
)]
#[instrument]
async fn slot_lag(p: SlotLagParams) -> Result<CallToolResult, ErrorData> {
    let backend = ArchiveDbBackend::connect(&p.url)
        .await
        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
    let lags: Vec<(DbReplicationSlotDescriptor, u64)> = backend
        .replication_slot_lag()
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&lags)
}

#[elicit_tool(
    plugin = "archive_replication",
    name = "archive_replication__list_publications",
    description = "List logical replication publications defined on this server. Uses \
                   DbReplicationMeta trait against pg_publication. Returns Vec<DbPublicationDescriptor>."
)]
#[instrument]
async fn list_publications(p: ListPublicationsParams) -> Result<CallToolResult, ErrorData> {
    let backend = ArchiveDbBackend::connect(&p.url)
        .await
        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
    let publications: Vec<DbPublicationDescriptor> = backend
        .list_publications()
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&publications)
}

#[elicit_tool(
    plugin = "archive_replication",
    name = "archive_replication__list_subscriptions",
    description = "List logical replication subscriptions on this server. Uses DbReplicationMeta \
                   trait against pg_subscription. Returns Vec<DbSubscriptionDescriptor>."
)]
#[instrument]
async fn list_subscriptions(p: ListSubscriptionsParams) -> Result<CallToolResult, ErrorData> {
    let backend = ArchiveDbBackend::connect(&p.url)
        .await
        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
    let subscriptions: Vec<DbSubscriptionDescriptor> = backend
        .list_subscriptions()
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&subscriptions)
}

#[elicit_tool(
    plugin = "archive_replication",
    name = "archive_replication__streaming_status",
    description = "List active streaming replication standbys and their state. Uses \
                   DbReplicationMeta trait against pg_stat_replication. Returns Vec<(application_name, state)>."
)]
#[instrument]
async fn streaming_status(p: StreamingStatusParams) -> Result<CallToolResult, ErrorData> {
    let backend = ArchiveDbBackend::connect(&p.url)
        .await
        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
    let status: Vec<(String, String)> = backend
        .streaming_replication_status()
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&status)
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for replication topology inspection via the `DbReplicationMeta` trait.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "archive_replication")]
pub struct ArchiveReplicationPlugin;

impl ArchiveReplicationPlugin {
    /// Create a new replication plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for ArchiveReplicationPlugin {
    fn default() -> Self {
        Self::new()
    }
}
