//! [`DbRoleManager`] — role and privilege management.
//!
//! Source: PostgreSQL docs §22.1 — Database Roles; ISO/IEC 27001:2022 §A.5.15.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{AccessAuthorized, AuditLogged, DbResult, DbRoleInfo, LeastPrivilegeEnforced};

/// Creates roles and manages privileges.
///
/// Source: PostgreSQL docs §22.1 — Database Roles
pub trait DbRoleManager: Send + Sync {
    /// Create a new database role.
    ///
    /// Source: PostgreSQL docs §22.1 — `CREATE ROLE`
    fn create_role(
        &self,
        name: &str,
        can_login: bool,
        superuser: bool,
    ) -> BoxFuture<'_, DbResult<(Established<AccessAuthorized>, Established<AuditLogged>)>>;

    /// Drop a role.
    ///
    /// Source: PostgreSQL docs §22.4 — `DROP ROLE`
    fn drop_role(&self, name: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>>;

    /// List all roles in the cluster.
    ///
    /// Source: PostgreSQL docs §54.43 — pg_roles
    fn list_roles(&self) -> BoxFuture<'_, DbResult<Vec<DbRoleInfo>>>;

    /// Grant a privilege on an object to a role.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — `<grant statement>`
    fn grant(
        &self,
        privilege: &str,
        on: &str,
        to: &str,
    ) -> BoxFuture<'_, DbResult<(Established<AccessAuthorized>, Established<AuditLogged>)>>;

    /// Revoke a privilege on an object from a role.
    ///
    /// Source: ISO/IEC 9075-2 §12.6 — `<revoke statement>`
    fn revoke(
        &self,
        privilege: &str,
        on: &str,
        from: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<LeastPrivilegeEnforced>,
            Established<AuditLogged>,
        )>,
    >;
}
