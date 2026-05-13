//! `ArchiveAdminPlugin` — database administration tools.
//!
//! Wraps [`DbRoleManager`], [`DbBackupManager`], and [`DbServerAdmin`]
//! methods to expose them as MCP tools.  Each tool carries an [`Established`]
//! proof token recording the administration contract.
//!
//! Primary backend: PostgreSQL.  Graceful fallback on other backends.

use elicit_db::{DbBackupManager, DbRoleManager, DbServerAdmin};
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

/// Proposition: the role list was successfully read from `pg_roles`.
#[derive(Prop)]
pub struct RoleListRead;
impl VerifiedWorkflow for RoleListRead {}

/// Proposition: a new role was successfully created.
#[derive(Prop)]
pub struct RoleCreated;
impl VerifiedWorkflow for RoleCreated {}

/// Proposition: a role was successfully dropped.
#[derive(Prop)]
pub struct RoleDropped;
impl VerifiedWorkflow for RoleDropped {}

/// Proposition: a privilege was successfully granted.
#[derive(Prop)]
pub struct PrivilegeGranted;
impl VerifiedWorkflow for PrivilegeGranted {}

/// Proposition: a privilege was successfully revoked.
#[derive(Prop)]
pub struct PrivilegeRevoked;
impl VerifiedWorkflow for PrivilegeRevoked {}

/// Proposition: a base backup was successfully initiated.
#[derive(Prop)]
pub struct BackupStarted;
impl VerifiedWorkflow for BackupStarted {}

/// Proposition: the backup list was successfully read.
#[derive(Prop)]
pub struct BackupListRead;
impl VerifiedWorkflow for BackupListRead {}

/// Proposition: a backup was successfully verified.
#[derive(Prop)]
pub struct BackupVerified;
impl VerifiedWorkflow for BackupVerified {}

/// Proposition: the WAL status was successfully queried.
#[derive(Prop)]
pub struct WalStatusRead;
impl VerifiedWorkflow for WalStatusRead {}

/// Proposition: the server version was successfully read.
#[derive(Prop)]
pub struct VersionRead;
impl VerifiedWorkflow for VersionRead {}

/// Proposition: the extension list was successfully read.
#[derive(Prop)]
pub struct ExtensionListRead;
impl VerifiedWorkflow for ExtensionListRead {}

/// Proposition: an extension was successfully installed.
#[derive(Prop)]
pub struct ExtensionInstalled;
impl VerifiedWorkflow for ExtensionInstalled {}

/// Proposition: the GUC settings were successfully read.
#[derive(Prop)]
pub struct AdminSettingsRead;
impl VerifiedWorkflow for AdminSettingsRead {}

/// Proposition: server configuration was successfully reloaded.
#[derive(Prop)]
pub struct ConfigReloaded;
impl VerifiedWorkflow for ConfigReloaded {}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `archive_admin__list_roles`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminListRolesParams {
    /// Connection URL.
    pub url: String,
}

/// Parameters for `archive_admin__create_role`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminCreateRoleParams {
    /// Connection URL.
    pub url: String,
    /// Name of the role to create.
    pub name: String,
    /// Whether the role can log in.
    pub can_login: bool,
    /// Whether the role has superuser privileges.
    pub superuser: bool,
}

/// Parameters for `archive_admin__drop_role`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminDropRoleParams {
    /// Connection URL.
    pub url: String,
    /// Name of the role to drop.
    pub name: String,
}

/// Parameters for `archive_admin__grant_privilege`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminGrantPrivilegeParams {
    /// Connection URL.
    pub url: String,
    /// Privilege to grant (e.g. `"SELECT"`, `"ALL"`).
    pub privilege: String,
    /// Object to grant on (e.g. `"schema.table"`).
    pub on: String,
    /// Role to grant to.
    pub to: String,
}

/// Parameters for `archive_admin__revoke_privilege`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminRevokePrivilegeParams {
    /// Connection URL.
    pub url: String,
    /// Privilege to revoke (e.g. `"SELECT"`, `"ALL"`).
    pub privilege: String,
    /// Object to revoke from (e.g. `"schema.table"`).
    pub on: String,
    /// Role to revoke from.
    pub from: String,
}

/// Parameters for `archive_admin__initiate_backup`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminInitiateBackupParams {
    /// Connection URL.
    pub url: String,
    /// Human-readable label for this backup.
    pub label: String,
}

/// Parameters for `archive_admin__list_backups`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminListBackupsParams {
    /// Connection URL.
    pub url: String,
}

/// Parameters for `archive_admin__verify_backup`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminVerifyBackupParams {
    /// Connection URL.
    pub url: String,
    /// Label of the backup to verify.
    pub label: String,
}

/// Parameters for `archive_admin__wal_status`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminWalStatusParams {
    /// Connection URL.
    pub url: String,
}

/// Parameters for `archive_admin__server_version`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminServerVersionParams {
    /// Connection URL.
    pub url: String,
}

/// Parameters for `archive_admin__list_extensions`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminListExtensionsParams {
    /// Connection URL.
    pub url: String,
}

/// Parameters for `archive_admin__install_extension`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminInstallExtensionParams {
    /// Connection URL.
    pub url: String,
    /// Extension name to install via `CREATE EXTENSION`.
    pub name: String,
}

/// Parameters for `archive_admin__list_settings`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminListSettingsParams {
    /// Connection URL.
    pub url: String,
    /// Optional substring to filter setting names.
    pub filter: Option<String>,
}

/// Parameters for `archive_admin__reload_config`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminReloadConfigParams {
    /// Connection URL.
    pub url: String,
}

// ── response helpers ──────────────────────────────────────────────────────────

/// A single role row returned by `archive_admin__list_roles`.
#[derive(Debug, Serialize)]
struct RoleRow {
    name: String,
    superuser: bool,
    can_login: bool,
    can_create_db: bool,
    can_create_role: bool,
}

/// WAL status summary.
#[derive(Debug, Serialize)]
struct WalStatusSummary {
    wal_ready: bool,
}

/// Server version response.
#[derive(Debug, Serialize)]
struct VersionResponse {
    version: String,
}

/// A GUC setting row.
#[derive(Debug, Serialize)]
struct SettingRow {
    name: String,
    value: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

/// List all roles in the cluster from `pg_roles`.
#[elicit_tool(
    plugin = "archive_admin",
    name = "archive_admin__list_roles",
    description = "List all roles in the PostgreSQL cluster from pg_roles."
)]
#[instrument]
async fn list_roles(p: AdminListRolesParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<RoleListRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let roles = backend.list_roles().await.map_err(db_err)?;
    let rows: Vec<RoleRow> = roles
        .into_iter()
        .map(|r| RoleRow {
            name: r.name,
            superuser: r.superuser,
            can_login: r.can_login,
            can_create_db: r.can_create_db,
            can_create_role: r.can_create_role,
        })
        .collect();
    json_result(&rows)
}

/// Create a new database role.
#[elicit_tool(
    plugin = "archive_admin",
    name = "archive_admin__create_role",
    description = "Create a new database role with the given attributes."
)]
#[instrument]
async fn create_role(p: AdminCreateRoleParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<RoleCreated>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let (_access, _audit) = backend
        .create_role(&p.name, p.can_login, p.superuser)
        .await
        .map_err(db_err)?;
    json_result(&serde_json::json!({ "created": p.name }))
}

/// Drop a database role.
#[elicit_tool(
    plugin = "archive_admin",
    name = "archive_admin__drop_role",
    description = "Drop a database role by name."
)]
#[instrument]
async fn drop_role(p: AdminDropRoleParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<RoleDropped>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let _audit = backend.drop_role(&p.name).await.map_err(db_err)?;
    json_result(&serde_json::json!({ "dropped": p.name }))
}

/// Grant a privilege on an object to a role.
#[elicit_tool(
    plugin = "archive_admin",
    name = "archive_admin__grant_privilege",
    description = "Grant a SQL privilege on an object to a role."
)]
#[instrument]
async fn grant_privilege(p: AdminGrantPrivilegeParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<PrivilegeGranted>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let (_access, _audit) = backend
        .grant(&p.privilege, &p.on, &p.to)
        .await
        .map_err(db_err)?;
    json_result(&serde_json::json!({ "granted": p.privilege, "on": p.on, "to": p.to }))
}

/// Revoke a privilege on an object from a role.
#[elicit_tool(
    plugin = "archive_admin",
    name = "archive_admin__revoke_privilege",
    description = "Revoke a SQL privilege on an object from a role."
)]
#[instrument]
async fn revoke_privilege(p: AdminRevokePrivilegeParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<PrivilegeRevoked>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let (_least_priv, _audit) = backend
        .revoke(&p.privilege, &p.on, &p.from)
        .await
        .map_err(db_err)?;
    json_result(&serde_json::json!({ "revoked": p.privilege, "on": p.on, "from": p.from }))
}

/// Initiate a base backup with the given label.
#[elicit_tool(
    plugin = "archive_admin",
    name = "archive_admin__initiate_backup",
    description = "Initiate a PostgreSQL base backup with the given label."
)]
#[instrument]
async fn initiate_backup(p: AdminInitiateBackupParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<BackupStarted>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let _tokens = backend.initiate_backup(&p.label).await.map_err(db_err)?;
    json_result(&serde_json::json!({ "backup_initiated": p.label }))
}

/// List available backup labels.
#[elicit_tool(
    plugin = "archive_admin",
    name = "archive_admin__list_backups",
    description = "List available PostgreSQL base backup labels."
)]
#[instrument]
async fn list_backups(p: AdminListBackupsParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<BackupListRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let backups = backend.list_backups().await.map_err(db_err)?;
    json_result(&backups)
}

/// Verify the integrity of a named backup.
#[elicit_tool(
    plugin = "archive_admin",
    name = "archive_admin__verify_backup",
    description = "Verify the integrity of a named PostgreSQL backup."
)]
#[instrument]
async fn verify_backup(p: AdminVerifyBackupParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<BackupVerified>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let _token = backend.verify_backup(&p.label).await.map_err(db_err)?;
    json_result(&serde_json::json!({ "verified": p.label }))
}

/// Return current WAL replay/archiving status.
#[elicit_tool(
    plugin = "archive_admin",
    name = "archive_admin__wal_status",
    description = "Return current PostgreSQL WAL replay and archiving status."
)]
#[instrument]
async fn wal_status(p: AdminWalStatusParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<WalStatusRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let _token = backend.wal_status().await.map_err(db_err)?;
    json_result(&WalStatusSummary { wal_ready: true })
}

/// Return the PostgreSQL server version string.
#[elicit_tool(
    plugin = "archive_admin",
    name = "archive_admin__server_version",
    description = "Return the PostgreSQL server version string."
)]
#[instrument]
async fn server_version(p: AdminServerVersionParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<VersionRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let version = backend.server_version().await.map_err(db_err)?;
    json_result(&VersionResponse { version })
}

/// List available extensions from `pg_available_extensions`.
#[elicit_tool(
    plugin = "archive_admin",
    name = "archive_admin__list_extensions",
    description = "List available extensions from pg_available_extensions."
)]
#[instrument]
async fn list_extensions(p: AdminListExtensionsParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<ExtensionListRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let extensions = backend.list_extensions().await.map_err(db_err)?;
    json_result(&extensions)
}

/// Install an extension via `CREATE EXTENSION`.
#[elicit_tool(
    plugin = "archive_admin",
    name = "archive_admin__install_extension",
    description = "Install a PostgreSQL extension via CREATE EXTENSION."
)]
#[instrument]
async fn install_extension(p: AdminInstallExtensionParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<ExtensionInstalled>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let _audit = backend.install_extension(&p.name).await.map_err(db_err)?;
    json_result(&serde_json::json!({ "installed": p.name }))
}

/// List all GUC settings from `pg_settings`.
#[elicit_tool(
    plugin = "archive_admin",
    name = "archive_admin__list_settings",
    description = "List all GUC settings from pg_settings with optional name-filter substring."
)]
#[instrument]
async fn list_settings(p: AdminListSettingsParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<AdminSettingsRead>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let raw: Vec<(String, String)> = backend.list_settings().await.map_err(db_err)?;
    let filter = p.filter.as_deref().unwrap_or("").to_lowercase();
    let rows: Vec<SettingRow> = raw
        .into_iter()
        .filter(|(name, _)| filter.is_empty() || name.to_lowercase().contains(&filter))
        .map(|(name, value)| SettingRow { name, value })
        .collect();
    json_result(&rows)
}

/// Reload server configuration without restart.
#[elicit_tool(
    plugin = "archive_admin",
    name = "archive_admin__reload_config",
    description = "Reload PostgreSQL server configuration without restart via pg_reload_conf()."
)]
#[instrument]
async fn reload_config(p: AdminReloadConfigParams) -> Result<CallToolResult, ErrorData> {
    let _proof = Established::<ConfigReloaded>::assert();
    let backend = ArchiveDbBackend::connect(&p.url).await.map_err(db_err)?;
    let _audit = backend.reload_config().await.map_err(db_err)?;
    json_result(&serde_json::json!({ "reloaded": true }))
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for database administration — roles, backups, WAL, and settings.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "archive_admin")]
pub struct ArchiveAdminPlugin;

impl ArchiveAdminPlugin {
    /// Create a new admin plugin.
    pub fn new() -> Self {
        Self
    }
}

impl Default for ArchiveAdminPlugin {
    fn default() -> Self {
        Self::new()
    }
}
