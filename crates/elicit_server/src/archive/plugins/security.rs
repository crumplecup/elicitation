//! `ArchiveSecurityPlugin` — security posture inspection via [`DbSecurityMeta`] and [`DbSecurityFactory`].
//!
//! Showcases the [`elicit_db`] trait abstraction: callers work through the
//! security traits to read server security settings and enable Row-Level
//! Security, receiving formal proof tokens on success.

use elicit_db::{AuditLogged, DbSecurityFactory, DbSecurityMeta, RowLevelSecurityEnabled};
use elicitation::{ElicitPlugin, contracts::Established, elicit_tool};
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

/// Parameters for `archive_security__tls_status`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TlsStatusParams {
    /// Database connection URL.
    pub url: String,
}

/// Parameters for `archive_security__security_settings`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SecuritySettingsParams {
    /// Database connection URL.
    pub url: String,
}

/// Parameters for `archive_security__enable_rls`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EnableRlsParams {
    /// Database connection URL.
    pub url: String,
    /// Schema containing the table.
    pub schema: String,
    /// Table to enable Row-Level Security on.
    pub table: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "archive_security",
    name = "archive_security__tls_status",
    description = "Check whether the current connection is using TLS/SSL. Uses DbSecurityMeta \
                   trait via ssl_is_used(). Returns bool."
)]
#[instrument]
async fn tls_status(p: TlsStatusParams) -> Result<CallToolResult, ErrorData> {
    let backend = ArchiveDbBackend::connect(&p.url)
        .await
        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
    let status: bool = backend
        .tls_status()
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&status)
}

#[elicit_tool(
    plugin = "archive_security",
    name = "archive_security__security_settings",
    description = "Retrieve key security-related PostgreSQL settings (ssl, log_connections, \
                   password_encryption, etc.). Uses DbSecurityMeta trait. Returns Vec<(name, value)>."
)]
#[instrument]
async fn security_settings(p: SecuritySettingsParams) -> Result<CallToolResult, ErrorData> {
    let backend = ArchiveDbBackend::connect(&p.url)
        .await
        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
    let settings: Vec<(String, String)> = backend
        .security_settings()
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&settings)
}

#[elicit_tool(
    plugin = "archive_security",
    name = "archive_security__enable_rls",
    description = "Enable Row-Level Security on a table. Uses DbSecurityFactory trait. \
                   Returns confirmation with the table name on success."
)]
#[instrument]
async fn enable_rls(p: EnableRlsParams) -> Result<CallToolResult, ErrorData> {
    let backend = ArchiveDbBackend::connect(&p.url)
        .await
        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
    // Obtain proof tokens via DbSecurityFactory — the trait layer ensures formal
    // guarantees that RLS was enabled and the action was audit-logged.
    let _proofs: (
        Established<RowLevelSecurityEnabled>,
        Established<AuditLogged>,
    ) = backend
        .enable_row_level_security(&p.schema, &p.table)
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&format!(
        "row_level_security_enabled on {}.{}",
        p.schema, p.table
    ))
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for security posture inspection via the `DbSecurityMeta` and `DbSecurityFactory` traits.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "archive_security")]
pub struct ArchiveSecurityPlugin;

impl ArchiveSecurityPlugin {
    /// Create a new security plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for ArchiveSecurityPlugin {
    fn default() -> Self {
        Self::new()
    }
}
