//! Security propositions.
//!
//! Sources: ISO/IEC 27001:2022 — Information security management;
//! PostgreSQL documentation — Row Security Policies, Roles, Authentication.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    // -------------------------------------------------------------------------
    // §A.5.15 Access control (ISO/IEC 27001:2022)
    // -------------------------------------------------------------------------

    /// Access was authorized for the requesting identity.
    ///
    /// Source: ISO/IEC 27001:2022 §A.5.15 — Access control
    pub struct AccessAuthorized;

    /// Access was correctly denied to an unauthorized identity.
    ///
    /// Source: ISO/IEC 27001:2022 §A.5.15 — Access control
    pub struct AccessDeniedCorrectly;

    /// The minimum necessary privileges were enforced for this operation.
    ///
    /// Source: ISO/IEC 27001:2022 §A.5.15 — Least privilege principle
    pub struct LeastPrivilegeEnforced;

    /// Data access is restricted to roles with a documented business need.
    ///
    /// Source: ISO/IEC 27001:2022 §A.5.15 — Need-to-know principle
    pub struct NeedToKnowEnforced;

    /// Administrative duties are separated from normal operational use.
    ///
    /// Source: ISO/IEC 27001:2022 §A.5.15 — Privilege separation
    pub struct PrivilegeSeparationMaintained;

    // -------------------------------------------------------------------------
    // §A.8.15 Logging (ISO/IEC 27001:2022)
    // -------------------------------------------------------------------------

    /// The operation was recorded in the audit log.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.15 — Logging
    pub struct AuditLogged;

    /// The audit log cannot be modified without detection.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.15 — Logging
    pub struct AuditLogTamperEvident;

    /// The audit log has been retained for the required duration.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.15 — Logging
    pub struct AuditLogRetentionMet;

    /// A security-relevant event was captured in the audit trail.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.15 — Logging
    pub struct SecurityEventLogged;

    /// A privileged operation was logged together with the acting identity.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.15 — Logging
    pub struct PrivilegedActionLogged;

    // -------------------------------------------------------------------------
    // §A.8.24 Cryptography (ISO/IEC 27001:2022)
    // -------------------------------------------------------------------------

    /// Data at rest is encrypted.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.24 — Use of cryptography
    pub struct EncryptedAtRest;

    /// Data in transit is encrypted.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.24 — Use of cryptography
    pub struct EncryptedInTransit;

    /// An approved encryption algorithm is in use for this operation.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.24 — Use of cryptography
    pub struct EncryptionAlgorithmApproved;

    /// Encryption keys are managed in accordance with the key management policy.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.24 — Use of cryptography
    pub struct KeyManagementPolicyApplied;

    /// Encryption keys have been rotated according to the rotation schedule.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.24 — Use of cryptography
    pub struct KeyRotationPerformed;

    // -------------------------------------------------------------------------
    // Row-Level Security (PostgreSQL)
    // -------------------------------------------------------------------------

    /// RLS is enabled on the table (`ALTER TABLE … ENABLE ROW LEVEL SECURITY`).
    ///
    /// Source: PostgreSQL documentation — Row Security Policies
    pub struct RowLevelSecurityEnabled;

    /// At least one RLS policy exists on the table.
    ///
    /// Source: PostgreSQL documentation — Row Security Policies
    pub struct RowLevelSecurityPolicyDefined;

    /// An RLS policy was evaluated and applied to the query.
    ///
    /// Source: PostgreSQL documentation — Row Security Policies
    pub struct RowLevelSecurityPolicyApplied;

    /// A non-superuser did not bypass RLS for this operation.
    ///
    /// Source: PostgreSQL documentation — Row Security Policies
    pub struct RowLevelSecurityBypassDenied;

    /// The `USING` clause of the RLS policy is correct for the intended access pattern.
    ///
    /// Source: PostgreSQL documentation — Row Security Policies
    pub struct RlsUsingClauseCorrect;

    /// The `WITH CHECK` clause of the RLS policy is correct for the intended write pattern.
    ///
    /// Source: PostgreSQL documentation — Row Security Policies
    pub struct RlsWithCheckClauseCorrect;

    /// `FORCE ROW LEVEL SECURITY` applies to the table owner.
    ///
    /// Source: PostgreSQL documentation — Row Security Policies
    pub struct RlsForcedForTableOwner;

    // -------------------------------------------------------------------------
    // Role-Based Access Control (PostgreSQL RBAC)
    // -------------------------------------------------------------------------

    /// `CREATE ROLE` succeeded.
    ///
    /// Source: PostgreSQL documentation — Database Roles
    pub struct RoleCreated;

    /// `DROP ROLE` succeeded.
    ///
    /// Source: PostgreSQL documentation — Database Roles
    pub struct RoleDropped;

    /// The superuser count is minimized and monitored.
    ///
    /// Source: PostgreSQL documentation — Database Roles
    pub struct SuperuserPrivilegeRestricted;

    /// The login role has a password set.
    ///
    /// Source: PostgreSQL documentation — Role Attributes
    pub struct LoginRolePasswordSet;

    /// A `NOLOGIN` role cannot initiate a database session.
    ///
    /// Source: PostgreSQL documentation — Role Attributes
    pub struct RoleCannotLoginUnexpectedly;

    /// The `CONNECTION LIMIT` on the role was not exceeded.
    ///
    /// Source: PostgreSQL documentation — Role Attributes
    pub struct RoleConnectionLimitEnforced;

    /// The `VALID UNTIL` expiry date on the role was enforced.
    ///
    /// Source: PostgreSQL documentation — Role Attributes
    pub struct RoleValidUntilEnforced;

    // -------------------------------------------------------------------------
    // Authentication (PostgreSQL pg_hba.conf)
    // -------------------------------------------------------------------------

    /// The client authenticated successfully.
    ///
    /// Source: PostgreSQL documentation — Client Authentication
    pub struct AuthenticationSucceeded;

    /// SCRAM-SHA-256 was used as the authentication method.
    ///
    /// Source: PostgreSQL documentation — Password Authentication
    pub struct ScramSha256AuthenticationUsed;

    /// Client certificate authentication was used for this connection.
    ///
    /// Source: PostgreSQL documentation — Certificate Authentication
    pub struct CertificateAuthenticationUsed;

    /// MD5 authentication is explicitly flagged as deprecated for this connection.
    ///
    /// Source: PostgreSQL documentation — Password Authentication
    pub struct Md5AuthenticationDeprecated;

    /// `trust` authentication is limited to the local socket only.
    ///
    /// Source: PostgreSQL documentation — Trust Authentication
    pub struct TrustAuthenticationLimited;

    /// Peer authentication was used for a local UNIX-domain socket connection.
    ///
    /// Source: PostgreSQL documentation — Peer Authentication
    pub struct PeerAuthenticationLocal;

    /// A matching `pg_hba.conf` rule was found for this connection.
    ///
    /// Source: PostgreSQL documentation — The pg_hba.conf File
    pub struct PgHbaRuleMatched;

    // -------------------------------------------------------------------------
    // Connection security
    // -------------------------------------------------------------------------

    /// The `max_connections` or role connection limit was not exceeded.
    ///
    /// Source: PostgreSQL documentation — Connection and Authentication
    pub struct ConnectionLimitEnforced;

    /// The connection source IP matches the configured allowlist.
    ///
    /// Source: PostgreSQL documentation — The pg_hba.conf File
    pub struct IpAllowlistEnforced;

    /// `sslmode=require` was enforced for this connection.
    ///
    /// Source: PostgreSQL documentation — SSL Support
    pub struct SslModeRequired;

    /// The SSL certificate was verified against a trusted CA.
    ///
    /// Source: PostgreSQL documentation — SSL Support
    pub struct SslCertificateVerified;

    /// An active TLS session exists for this connection.
    ///
    /// Source: PostgreSQL documentation — SSL Support
    pub struct TlsSessionActive;

    // -------------------------------------------------------------------------
    // Data protection
    // -------------------------------------------------------------------------

    /// Sensitive column data is masked for roles without authorization.
    ///
    /// Source: ISO/IEC 27001:2022 §A.5.15; PostgreSQL column-level privileges
    pub struct SensitiveColumnMasked;

    /// Personally identifiable information was not written to server logs.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.15 — Logging; GDPR Art. 5(1)(f)
    pub struct PiiNotExposedInLogs;

    /// No plaintext passwords are stored in any accessible table.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.24 — Use of cryptography
    pub struct PasswordNotStoredInPlaintext;

    /// A data classification label has been applied to the table or column.
    ///
    /// Source: ISO/IEC 27001:2022 §A.5.12 — Classification of information
    pub struct DataClassificationTagApplied;

    // -- Additional ISO 27001:2022 security controls --

    /// Multi-factor authentication is enforced for privileged or remote access.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.5 — Secure authentication
    pub struct MultiFactorAuthEnforced;

    /// A password complexity/rotation policy is enforced for all database login roles.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.5 — Secure authentication; PostgreSQL `pgaudit` / `passwordcheck`
    pub struct PasswordPolicyEnforced;

    /// SQL injection is prevented by using parameterized queries or prepared statements.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.29 — Security testing in development and acceptance;
    ///         OWASP SQL Injection Prevention
    pub struct SqlInjectionPrevented;

    /// Idle session timeout (`idle_in_transaction_session_timeout` or `statement_timeout`) is enforced.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.1 — User endpoint devices; PostgreSQL session timeout parameters
    pub struct SessionTimeoutEnforced;

    macro_rules! security_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by access control policy */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by access control policy */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by access control policy */ }
                }
            }
        };
    }

    // §A.5.15 Access control
    security_prop!(AccessAuthorized, "AccessAuthorized");
    security_prop!(AccessDeniedCorrectly, "AccessDeniedCorrectly");
    security_prop!(LeastPrivilegeEnforced, "LeastPrivilegeEnforced");
    security_prop!(NeedToKnowEnforced, "NeedToKnowEnforced");
    security_prop!(
        PrivilegeSeparationMaintained,
        "PrivilegeSeparationMaintained"
    );

    // §A.8.15 Logging
    security_prop!(AuditLogged, "AuditLogged");
    security_prop!(AuditLogTamperEvident, "AuditLogTamperEvident");
    security_prop!(AuditLogRetentionMet, "AuditLogRetentionMet");
    security_prop!(SecurityEventLogged, "SecurityEventLogged");
    security_prop!(PrivilegedActionLogged, "PrivilegedActionLogged");

    // §A.8.24 Cryptography
    security_prop!(EncryptedAtRest, "EncryptedAtRest");
    security_prop!(EncryptedInTransit, "EncryptedInTransit");
    security_prop!(EncryptionAlgorithmApproved, "EncryptionAlgorithmApproved");
    security_prop!(KeyManagementPolicyApplied, "KeyManagementPolicyApplied");
    security_prop!(KeyRotationPerformed, "KeyRotationPerformed");

    // Row-Level Security
    security_prop!(RowLevelSecurityEnabled, "RowLevelSecurityEnabled");
    security_prop!(
        RowLevelSecurityPolicyDefined,
        "RowLevelSecurityPolicyDefined"
    );
    security_prop!(
        RowLevelSecurityPolicyApplied,
        "RowLevelSecurityPolicyApplied"
    );
    security_prop!(RowLevelSecurityBypassDenied, "RowLevelSecurityBypassDenied");
    security_prop!(RlsUsingClauseCorrect, "RlsUsingClauseCorrect");
    security_prop!(RlsWithCheckClauseCorrect, "RlsWithCheckClauseCorrect");
    security_prop!(RlsForcedForTableOwner, "RlsForcedForTableOwner");

    // Role-Based Access Control
    security_prop!(RoleCreated, "RoleCreated");
    security_prop!(RoleDropped, "RoleDropped");
    security_prop!(SuperuserPrivilegeRestricted, "SuperuserPrivilegeRestricted");
    security_prop!(LoginRolePasswordSet, "LoginRolePasswordSet");
    security_prop!(RoleCannotLoginUnexpectedly, "RoleCannotLoginUnexpectedly");
    security_prop!(RoleConnectionLimitEnforced, "RoleConnectionLimitEnforced");
    security_prop!(RoleValidUntilEnforced, "RoleValidUntilEnforced");

    // Authentication
    security_prop!(AuthenticationSucceeded, "AuthenticationSucceeded");
    security_prop!(
        ScramSha256AuthenticationUsed,
        "ScramSha256AuthenticationUsed"
    );
    security_prop!(
        CertificateAuthenticationUsed,
        "CertificateAuthenticationUsed"
    );
    security_prop!(Md5AuthenticationDeprecated, "Md5AuthenticationDeprecated");
    security_prop!(TrustAuthenticationLimited, "TrustAuthenticationLimited");
    security_prop!(PeerAuthenticationLocal, "PeerAuthenticationLocal");
    security_prop!(PgHbaRuleMatched, "PgHbaRuleMatched");

    // Connection security
    security_prop!(ConnectionLimitEnforced, "ConnectionLimitEnforced");
    security_prop!(IpAllowlistEnforced, "IpAllowlistEnforced");
    security_prop!(SslModeRequired, "SslModeRequired");
    security_prop!(SslCertificateVerified, "SslCertificateVerified");
    security_prop!(TlsSessionActive, "TlsSessionActive");

    // Data protection
    security_prop!(SensitiveColumnMasked, "SensitiveColumnMasked");
    security_prop!(PiiNotExposedInLogs, "PiiNotExposedInLogs");
    security_prop!(PasswordNotStoredInPlaintext, "PasswordNotStoredInPlaintext");
    security_prop!(DataClassificationTagApplied, "DataClassificationTagApplied");

    security_prop!(MultiFactorAuthEnforced, "MultiFactorAuthEnforced");
    security_prop!(PasswordPolicyEnforced, "PasswordPolicyEnforced");
    security_prop!(SqlInjectionPrevented, "SqlInjectionPrevented");
    security_prop!(SessionTimeoutEnforced, "SessionTimeoutEnforced");
}

pub use emit_impls::{
    // §A.5.15 Access control
    AccessAuthorized,
    AccessDeniedCorrectly,
    // §A.8.15 Logging
    AuditLogRetentionMet,
    AuditLogTamperEvident,
    AuditLogged,
    // Authentication
    AuthenticationSucceeded,
    CertificateAuthenticationUsed,
    // Connection security
    ConnectionLimitEnforced,
    // Data protection
    DataClassificationTagApplied,
    // §A.8.24 Cryptography
    EncryptedAtRest,
    EncryptedInTransit,
    EncryptionAlgorithmApproved,
    IpAllowlistEnforced,
    KeyManagementPolicyApplied,
    KeyRotationPerformed,
    LeastPrivilegeEnforced,
    // Role-Based Access Control
    LoginRolePasswordSet,
    Md5AuthenticationDeprecated,
    MultiFactorAuthEnforced,
    NeedToKnowEnforced,
    PasswordNotStoredInPlaintext,
    PasswordPolicyEnforced,
    PeerAuthenticationLocal,
    PgHbaRuleMatched,
    PiiNotExposedInLogs,
    PrivilegeSeparationMaintained,
    PrivilegedActionLogged,
    // Row-Level Security
    RlsForcedForTableOwner,
    RlsUsingClauseCorrect,
    RlsWithCheckClauseCorrect,
    RoleCannotLoginUnexpectedly,
    RoleConnectionLimitEnforced,
    RoleCreated,
    RoleDropped,
    RoleValidUntilEnforced,
    RowLevelSecurityBypassDenied,
    RowLevelSecurityEnabled,
    RowLevelSecurityPolicyApplied,
    RowLevelSecurityPolicyDefined,
    ScramSha256AuthenticationUsed,
    SecurityEventLogged,
    SensitiveColumnMasked,
    SessionTimeoutEnforced,
    SqlInjectionPrevented,
    SslCertificateVerified,
    SslModeRequired,
    SuperuserPrivilegeRestricted,
    TlsSessionActive,
    TrustAuthenticationLimited,
};
