//! [`DbSessionManager`] — connection and session lifecycle.
//!
//! Source: PostgreSQL docs §33 — Client Interfaces; §28.2 — pg_stat_activity.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{AuditLogged, ConnectionEstablished, ConnectionId, DbResult, DbStatActivity};

/// Manages database sessions: connecting, disconnecting, and monitoring.
///
/// Source: PostgreSQL docs §55.2 — Connection Setup
pub trait DbSessionManager: Send + Sync {
    /// Establish a new connection and return its identifier.
    ///
    /// Source: PostgreSQL Protocol §55.2 — Connection Setup
    fn connect(
        &self,
        url: &str,
    ) -> BoxFuture<'_, DbResult<(ConnectionId, Established<ConnectionEstablished>)>>;

    /// Gracefully close an existing connection.
    ///
    /// Source: PostgreSQL Protocol §55.2 — Termination
    fn disconnect(&self, id: ConnectionId) -> BoxFuture<'_, DbResult<()>>;

    /// List all current sessions from `pg_stat_activity`.
    ///
    /// Source: PostgreSQL docs §28.2 — pg_stat_activity
    fn list_sessions(&self) -> BoxFuture<'_, DbResult<DbStatActivity>>;

    /// Terminate a backend process by PID.
    ///
    /// Source: PostgreSQL docs §9.27 — pg_terminate_backend()
    fn terminate_session(&self, pid: i32) -> BoxFuture<'_, DbResult<Established<AuditLogged>>>;
}
