//! [`DbServerAdmin`] — server-level administration.
//!
//! Source: PostgreSQL docs §20 — Server Configuration; §54.16 — pg_settings.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{AuditLogged, DbResult};

/// Manages server-level settings, extensions, and configuration.
///
/// Source: PostgreSQL docs §20 — Server Configuration
pub trait DbServerAdmin: Send + Sync {
    /// Return the server version string.
    ///
    /// Source: PostgreSQL docs §9.26 — `version()`
    fn server_version(&self) -> BoxFuture<'_, DbResult<String>>;

    /// List all GUC settings from `pg_settings`.
    ///
    /// Source: PostgreSQL docs §54.16 — pg_settings
    fn list_settings(&self) -> BoxFuture<'_, DbResult<Vec<(String, String)>>>;

    /// List installed extensions from `pg_available_extensions`.
    ///
    /// Source: PostgreSQL docs §54.6 — pg_available_extensions
    fn list_extensions(&self) -> BoxFuture<'_, DbResult<Vec<String>>>;

    /// Install an extension via `CREATE EXTENSION`.
    ///
    /// Source: PostgreSQL docs §45.1 — `CREATE EXTENSION`
    fn install_extension(&self, name: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>>;

    /// Reload server configuration without restart via `pg_reload_conf()`.
    ///
    /// Source: PostgreSQL docs §9.27 — pg_reload_conf()
    fn reload_config(&self) -> BoxFuture<'_, DbResult<Established<AuditLogged>>>;
}
