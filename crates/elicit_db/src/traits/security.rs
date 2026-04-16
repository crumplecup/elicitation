//! `DbSecurityFactory` — security posture factory (Role 1a).
//! `DbSecurityMeta`    — security audit reporter (Role 2).
//!
//! # Three-role taxonomy
//!
//! | Role | Trait | Purpose |
//! |------|-------|---------|
//! | 1a (leaf factory) | [`DbSecurityFactory`] | Establishes security controls; returns proof tokens |
//! | 2 (reporter) | [`DbSecurityMeta`] | Queries pg_hba_file_rules, pg_stat_activity; no proof tokens |
//!
//! This trait is orthogonal to [`crate::DbRoleManager`]: `DbRoleManager` covers
//! role and privilege management (GRANT/REVOKE), while `DbSecurityFactory` covers
//! transport security, authentication policy, and audit integrity controls.
//!
//! Source: ISO/IEC 27001:2022 §A.5 — Organizational controls;
//!         §A.8 — Technological controls;
//!         PostgreSQL docs §21 — Client Authentication.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{
    AuditLogRetentionMet, AuditLogTamperEvident, AuditLogged, DbResult, EncryptedAtRest,
    EncryptedInTransit, LeastPrivilegeEnforced, MultiFactorAuthEnforced, PasswordPolicyEnforced,
    RowLevelSecurityEnabled, RowLevelSecurityPolicyDefined, SessionTimeoutEnforced,
    SqlInjectionPrevented, SslModeRequired,
};

// ── Role 1a: security posture factory ────────────────────────────────────────

/// Establishes security controls and returns compile-time-verifiable proof tokens.
///
/// Each method asserts that a specific ISO/IEC 27001:2022 security control
/// has been activated on this backend.  Proof tokens can be composed into
/// aggregate proofs via `contracts::proof_composition`.
///
/// Source: ISO/IEC 27001:2022; PostgreSQL docs §19.9 — SSL; §21 — Auth.
pub trait DbSecurityFactory: Send + Sync {
    // ── Transport security ────────────────────────────────────────────────────

    /// Enforce TLS for all client connections (`ssl = on`, `sslmode = require`).
    ///
    /// Returns `SslModeRequired` + `EncryptedInTransit`.
    ///
    /// Source: PostgreSQL docs §19.9 — SSL Support; ISO/IEC 27001:2022 §A.8.24
    fn enforce_tls(
        &self,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<SslModeRequired>,
            Established<EncryptedInTransit>,
        )>,
    >;

    /// Configure transparent data encryption at rest.
    ///
    /// Returns `EncryptedAtRest`.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.24 — Use of cryptography
    fn configure_encryption_at_rest(&self)
    -> BoxFuture<'_, DbResult<Established<EncryptedAtRest>>>;

    // ── Row-level security ────────────────────────────────────────────────────

    /// Enable row-level security on a table (`ALTER TABLE … ENABLE ROW LEVEL SECURITY`).
    ///
    /// Returns `RowLevelSecurityEnabled` + `AuditLogged`.
    ///
    /// Source: PostgreSQL docs §5.8 — Row Security Policies;
    ///         ISO/IEC 27001:2022 §A.5.15
    fn enable_row_level_security(
        &self,
        schema: &str,
        table: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<RowLevelSecurityEnabled>,
            Established<AuditLogged>,
        )>,
    >;

    /// Create a row-level security policy on a table (`CREATE POLICY`).
    ///
    /// Returns `RowLevelSecurityPolicyDefined` + `AuditLogged`.
    ///
    /// Source: PostgreSQL docs §5.8 — Row Security Policies
    fn define_rls_policy(
        &self,
        schema: &str,
        table: &str,
        policy_name: &str,
        using_expr: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<RowLevelSecurityPolicyDefined>,
            Established<AuditLogged>,
        )>,
    >;

    // ── Authentication policy ─────────────────────────────────────────────────

    /// Enforce multi-factor authentication requirements.
    ///
    /// Returns `MultiFactorAuthEnforced`.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.5 — Secure authentication
    fn enforce_mfa(&self) -> BoxFuture<'_, DbResult<Established<MultiFactorAuthEnforced>>>;

    /// Enforce a password strength and rotation policy.
    ///
    /// Returns `PasswordPolicyEnforced`.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.5; PostgreSQL `pg_hba.conf`
    fn enforce_password_policy(
        &self,
    ) -> BoxFuture<'_, DbResult<Established<PasswordPolicyEnforced>>>;

    /// Enforce session timeout for idle connections (`idle_session_timeout`).
    ///
    /// Returns `SessionTimeoutEnforced`.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.5;
    ///         PostgreSQL docs §20.11 — `idle_session_timeout`
    fn enforce_session_timeout(
        &self,
        timeout_ms: u64,
    ) -> BoxFuture<'_, DbResult<Established<SessionTimeoutEnforced>>>;

    /// Enable query parameterization enforcement to prevent SQL injection.
    ///
    /// Returns `SqlInjectionPrevented`.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.26 — Application security
    fn enforce_parameterized_queries(
        &self,
    ) -> BoxFuture<'_, DbResult<Established<SqlInjectionPrevented>>>;

    // ── Audit and access control ──────────────────────────────────────────────

    /// Verify that audit logging is tamper-evident and meets the retention requirement.
    ///
    /// Returns `AuditLogTamperEvident` + `AuditLogRetentionMet`.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.15 — Logging
    fn verify_audit_log_integrity(
        &self,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<AuditLogTamperEvident>,
            Established<AuditLogRetentionMet>,
        )>,
    >;

    /// Apply the least-privilege principle: revoke PUBLIC grants, restrict defaults.
    ///
    /// Returns `LeastPrivilegeEnforced`.
    ///
    /// Source: ISO/IEC 27001:2022 §A.5.15; NIST SP 800-53 AC-6
    fn apply_least_privilege(&self)
    -> BoxFuture<'_, DbResult<Established<LeastPrivilegeEnforced>>>;
}

// ── Role 2: security audit reporter ──────────────────────────────────────────

/// Orthogonal reporter for the security posture of the database cluster.
///
/// Queries `pg_hba_file_rules`, `pg_settings`, `pg_stat_activity`.
/// No proof tokens are consumed or produced.
///
/// Source: PostgreSQL docs §21 — Client Authentication.
pub trait DbSecurityMeta: Send + Sync {
    /// Return whether TLS is active for the current connection.
    ///
    /// Source: PostgreSQL docs §9.27 — `ssl_is_used()`
    fn tls_status(&self) -> BoxFuture<'_, DbResult<bool>>;

    /// Return all `pg_hba.conf` entries as `(type, database, user, method)` tuples.
    ///
    /// Source: PostgreSQL docs §54.2 — pg_hba_file_rules
    fn hba_rules(&self) -> BoxFuture<'_, DbResult<Vec<(String, String, String, String)>>>;

    /// Return PIDs of sessions that have been idle-in-transaction beyond `threshold_ms`.
    ///
    /// Source: PostgreSQL docs §28.2 — pg_stat_activity (`idle_in_transaction_session_timeout`)
    fn idle_transaction_sessions(&self, threshold_ms: u64) -> BoxFuture<'_, DbResult<Vec<i32>>>;

    /// Return current values of security-relevant GUC parameters.
    ///
    /// Returns `(name, value)` pairs for parameters such as `ssl`, `log_connections`,
    /// `idle_session_timeout`, `password_encryption`.
    ///
    /// Source: PostgreSQL docs §54.16 — pg_settings
    fn security_settings(&self) -> BoxFuture<'_, DbResult<Vec<(String, String)>>>;
}
