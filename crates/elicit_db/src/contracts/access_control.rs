//! Access control propositions.
//!
//! Source: ISO/IEC 9075-2 §12 — Access control; PostgreSQL §5.7 — Privileges.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
            }
        };
    }

    // -- Table-level privileges (§12.3) --

    /// SELECT privilege on table granted to role.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — GRANT statement: table privilege SELECT
    pub struct TableSelectPrivilegeGranted;
    structural_prop!(TableSelectPrivilegeGranted, "TableSelectPrivilegeGranted");

    /// INSERT privilege on table granted.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — GRANT statement: table privilege INSERT
    pub struct TableInsertPrivilegeGranted;
    structural_prop!(TableInsertPrivilegeGranted, "TableInsertPrivilegeGranted");

    /// UPDATE privilege on table granted.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — GRANT statement: table privilege UPDATE
    pub struct TableUpdatePrivilegeGranted;
    structural_prop!(TableUpdatePrivilegeGranted, "TableUpdatePrivilegeGranted");

    /// DELETE privilege on table granted.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — GRANT statement: table privilege DELETE
    pub struct TableDeletePrivilegeGranted;
    structural_prop!(TableDeletePrivilegeGranted, "TableDeletePrivilegeGranted");

    /// TRUNCATE privilege on table granted.
    ///
    /// Source: PostgreSQL §5.7 — TRUNCATE table privilege
    pub struct TableTruncatePrivilegeGranted;
    structural_prop!(
        TableTruncatePrivilegeGranted,
        "TableTruncatePrivilegeGranted"
    );

    /// REFERENCES privilege on table granted.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — GRANT statement: table privilege REFERENCES
    pub struct TableReferencesPrivilegeGranted;
    structural_prop!(
        TableReferencesPrivilegeGranted,
        "TableReferencesPrivilegeGranted"
    );

    /// TRIGGER privilege on table granted.
    ///
    /// Source: PostgreSQL §5.7 — TRIGGER table privilege
    pub struct TableTriggerPrivilegeGranted;
    structural_prop!(TableTriggerPrivilegeGranted, "TableTriggerPrivilegeGranted");

    /// A table privilege was revoked from a role.
    ///
    /// Source: ISO/IEC 9075-2 §12.6 — REVOKE statement: table privilege
    pub struct TablePrivilegeRevoked;
    structural_prop!(TablePrivilegeRevoked, "TablePrivilegeRevoked");

    /// ALL PRIVILEGES on table granted to role.
    ///
    /// Source: PostgreSQL §5.7 — GRANT ALL PRIVILEGES ON TABLE
    pub struct TableAllPrivilegesGranted;
    structural_prop!(TableAllPrivilegesGranted, "TableAllPrivilegesGranted");

    // -- Column-level privileges (§12.3) --

    /// SELECT privilege on specific column granted.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — Column privilege: SELECT
    pub struct ColumnSelectPrivilegeGranted;
    structural_prop!(ColumnSelectPrivilegeGranted, "ColumnSelectPrivilegeGranted");

    /// INSERT privilege on specific column granted.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — Column privilege: INSERT
    pub struct ColumnInsertPrivilegeGranted;
    structural_prop!(ColumnInsertPrivilegeGranted, "ColumnInsertPrivilegeGranted");

    /// UPDATE privilege on specific column granted.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — Column privilege: UPDATE
    pub struct ColumnUpdatePrivilegeGranted;
    structural_prop!(ColumnUpdatePrivilegeGranted, "ColumnUpdatePrivilegeGranted");

    /// A column-level privilege was revoked.
    ///
    /// Source: ISO/IEC 9075-2 §12.6 — REVOKE statement: column privilege
    pub struct ColumnPrivilegeRevoked;
    structural_prop!(ColumnPrivilegeRevoked, "ColumnPrivilegeRevoked");

    // -- Schema privileges (PostgreSQL) --

    /// USAGE privilege on schema granted.
    ///
    /// Source: PostgreSQL §5.7 — Schema privilege: USAGE
    pub struct SchemaUsagePrivilegeGranted;
    structural_prop!(SchemaUsagePrivilegeGranted, "SchemaUsagePrivilegeGranted");

    /// CREATE privilege on schema granted.
    ///
    /// Source: PostgreSQL §5.7 — Schema privilege: CREATE
    pub struct SchemaCreatePrivilegeGranted;
    structural_prop!(SchemaCreatePrivilegeGranted, "SchemaCreatePrivilegeGranted");

    /// A schema privilege was revoked.
    ///
    /// Source: PostgreSQL §5.7 — REVOKE on schema
    pub struct SchemaPrivilegeRevoked;
    structural_prop!(SchemaPrivilegeRevoked, "SchemaPrivilegeRevoked");

    // -- Sequence privileges --

    /// USAGE privilege on sequence granted.
    ///
    /// Source: PostgreSQL §5.7 — Sequence privilege: USAGE
    pub struct SequenceUsagePrivilegeGranted;
    structural_prop!(
        SequenceUsagePrivilegeGranted,
        "SequenceUsagePrivilegeGranted"
    );

    /// SELECT privilege on sequence granted.
    ///
    /// Source: PostgreSQL §5.7 — Sequence privilege: SELECT (currval)
    pub struct SequenceSelectPrivilegeGranted;
    structural_prop!(
        SequenceSelectPrivilegeGranted,
        "SequenceSelectPrivilegeGranted"
    );

    /// UPDATE privilege on sequence granted.
    ///
    /// Source: PostgreSQL §5.7 — Sequence privilege: UPDATE (nextval/setval)
    pub struct SequenceUpdatePrivilegeGranted;
    structural_prop!(
        SequenceUpdatePrivilegeGranted,
        "SequenceUpdatePrivilegeGranted"
    );

    // -- Function/procedure privileges --

    /// EXECUTE privilege on function/procedure granted.
    ///
    /// Source: PostgreSQL §5.7 — Function/procedure privilege: EXECUTE
    pub struct FunctionExecutePrivilegeGranted;
    structural_prop!(
        FunctionExecutePrivilegeGranted,
        "FunctionExecutePrivilegeGranted"
    );

    /// EXECUTE privilege revoked.
    ///
    /// Source: PostgreSQL §5.7 — REVOKE EXECUTE ON FUNCTION
    pub struct FunctionExecutePrivilegeRevoked;
    structural_prop!(
        FunctionExecutePrivilegeRevoked,
        "FunctionExecutePrivilegeRevoked"
    );

    // -- Type privileges --

    /// USAGE privilege on a user-defined type granted.
    ///
    /// Source: PostgreSQL §5.7 — Type privilege: USAGE
    pub struct TypeUsagePrivilegeGranted;
    structural_prop!(TypeUsagePrivilegeGranted, "TypeUsagePrivilegeGranted");

    // -- Database privileges --

    /// CONNECT privilege on database granted.
    ///
    /// Source: PostgreSQL §5.7 — Database privilege: CONNECT
    pub struct DatabaseConnectPrivilegeGranted;
    structural_prop!(
        DatabaseConnectPrivilegeGranted,
        "DatabaseConnectPrivilegeGranted"
    );

    /// CREATE privilege on database granted.
    ///
    /// Source: PostgreSQL §5.7 — Database privilege: CREATE
    pub struct DatabaseCreatePrivilegeGranted;
    structural_prop!(
        DatabaseCreatePrivilegeGranted,
        "DatabaseCreatePrivilegeGranted"
    );

    /// TEMP privilege on database granted.
    ///
    /// Source: PostgreSQL §5.7 — Database privilege: TEMP
    pub struct DatabaseTempPrivilegeGranted;
    structural_prop!(DatabaseTempPrivilegeGranted, "DatabaseTempPrivilegeGranted");

    // -- Role membership and hierarchy --

    /// GRANT role_name TO grantee succeeded.
    ///
    /// Source: PostgreSQL §5.7 — GRANT role membership
    pub struct RoleMembershipGranted;
    structural_prop!(RoleMembershipGranted, "RoleMembershipGranted");

    /// REVOKE role_name FROM grantee succeeded.
    ///
    /// Source: PostgreSQL §5.7 — REVOKE role membership
    pub struct RoleMembershipRevoked;
    structural_prop!(RoleMembershipRevoked, "RoleMembershipRevoked");

    /// WITH ADMIN OPTION granted for role membership.
    ///
    /// Source: PostgreSQL §5.7 — GRANT ... WITH ADMIN OPTION
    pub struct RoleAdminOptionGranted;
    structural_prop!(RoleAdminOptionGranted, "RoleAdminOptionGranted");

    /// WITH GRANT OPTION cascaded to grantee.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — WITH GRANT OPTION inheritance
    pub struct RoleGrantOptionInherited;
    structural_prop!(RoleGrantOptionInherited, "RoleGrantOptionInherited");

    /// INHERIT attribute causes grantee to inherit role privileges.
    ///
    /// Source: PostgreSQL §5.7 — Role attribute: INHERIT
    pub struct RoleInheritanceActive;
    structural_prop!(RoleInheritanceActive, "RoleInheritanceActive");

    /// SET ROLE successfully assumed a granted role.
    ///
    /// Source: PostgreSQL §5.7 — SET ROLE statement
    pub struct RoleSetRoleApplied;
    structural_prop!(RoleSetRoleApplied, "RoleSetRoleApplied");

    /// RESET ROLE returned to original role.
    ///
    /// Source: PostgreSQL §5.7 — RESET ROLE statement
    pub struct RoleResetApplied;
    structural_prop!(RoleResetApplied, "RoleResetApplied");

    // -- Object ownership --

    /// The operating role owns or has been granted privileges on the object.
    ///
    /// Source: PostgreSQL §5.7 — Object ownership and privilege checks
    pub struct ObjectOwnershipVerified;
    structural_prop!(ObjectOwnershipVerified, "ObjectOwnershipVerified");

    /// ALTER ... OWNER TO reassigned ownership.
    ///
    /// Source: PostgreSQL §5.7 — ALTER TABLE/SEQUENCE/FUNCTION ... OWNER TO
    pub struct ObjectOwnershipTransferred;
    structural_prop!(ObjectOwnershipTransferred, "ObjectOwnershipTransferred");

    /// ALTER DEFAULT PRIVILEGES rules applied to new objects.
    ///
    /// Source: PostgreSQL §5.7 — ALTER DEFAULT PRIVILEGES
    pub struct DefaultPrivilegesApplied;
    structural_prop!(DefaultPrivilegesApplied, "DefaultPrivilegesApplied");

    // -- Public role management --

    /// Excessive default PUBLIC privileges have been revoked.
    ///
    /// Source: PostgreSQL §5.7 — Security hardening: revoke from PUBLIC
    pub struct PublicRolePrivilegeLimited;
    structural_prop!(PublicRolePrivilegeLimited, "PublicRolePrivilegeLimited");

    /// REVOKE CONNECT ON DATABASE FROM PUBLIC applied.
    ///
    /// Source: PostgreSQL §5.7 — REVOKE CONNECT ON DATABASE ... FROM PUBLIC
    pub struct PublicConnectRevokedFromDatabase;
    structural_prop!(
        PublicConnectRevokedFromDatabase,
        "PublicConnectRevokedFromDatabase"
    );

    // -- Row-level security in access control context --

    /// SELECT RLS policy applied; user sees only permitted rows.
    ///
    /// Source: PostgreSQL §5.8 — Row security policies: SELECT USING
    pub struct RlsSelectPolicyApplied;
    structural_prop!(RlsSelectPolicyApplied, "RlsSelectPolicyApplied");

    /// INSERT WITH CHECK policy applied.
    ///
    /// Source: PostgreSQL §5.8 — Row security policies: INSERT WITH CHECK
    pub struct RlsInsertPolicyApplied;
    structural_prop!(RlsInsertPolicyApplied, "RlsInsertPolicyApplied");

    /// UPDATE RLS policy applied (both USING and WITH CHECK).
    ///
    /// Source: PostgreSQL §5.8 — Row security policies: UPDATE USING/WITH CHECK
    pub struct RlsUpdatePolicyApplied;
    structural_prop!(RlsUpdatePolicyApplied, "RlsUpdatePolicyApplied");

    /// DELETE RLS policy applied.
    ///
    /// Source: PostgreSQL §5.8 — Row security policies: DELETE USING
    pub struct RlsDeletePolicyApplied;
    structural_prop!(RlsDeletePolicyApplied, "RlsDeletePolicyApplied");

    /// BYPASSRLS role is not inadvertently granted.
    ///
    /// Source: PostgreSQL §5.8 — BYPASSRLS role attribute security consideration
    pub struct RlsBypassRoleExcluded;
    structural_prop!(RlsBypassRoleExcluded, "RlsBypassRoleExcluded");

    // -- Grant option and revoke cascade/restrict --

    /// Privilege was granted `WITH GRANT OPTION`, allowing the grantee to re-grant it.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — `<grant privilege statement>` WITH GRANT OPTION
    pub struct PrivilegeGrantedWithGrantOption;
    structural_prop!(
        PrivilegeGrantedWithGrantOption,
        "PrivilegeGrantedWithGrantOption"
    );

    /// Privilege was revoked with `CASCADE`, removing all dependent grants transitively.
    ///
    /// Source: ISO/IEC 9075-2 §12.6 — `<revoke privilege statement>` CASCADE
    pub struct PrivilegeRevokedCascade;
    structural_prop!(PrivilegeRevokedCascade, "PrivilegeRevokedCascade");

    /// Privilege was revoked with `RESTRICT`, failing if any dependent grants exist.
    ///
    /// Source: ISO/IEC 9075-2 §12.6 — `<revoke privilege statement>` RESTRICT
    pub struct PrivilegeRevokedRestrict;
    structural_prop!(PrivilegeRevokedRestrict, "PrivilegeRevokedRestrict");

    /// Database-level privilege was explicitly revoked from a role or user.
    ///
    /// Source: ISO/IEC 9075-2 §12.6 — database privilege revocation
    pub struct DatabasePrivilegeRevoked;
    structural_prop!(DatabasePrivilegeRevoked, "DatabasePrivilegeRevoked");

    /// USAGE privilege on a foreign server was granted to a role or user.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — foreign server USAGE privilege; SQL/MED extension
    pub struct ForeignServerUsagePrivilegeGranted;
    structural_prop!(
        ForeignServerUsagePrivilegeGranted,
        "ForeignServerUsagePrivilegeGranted"
    );

    /// USAGE privilege on a foreign-data wrapper was granted to a role or user.
    ///
    /// Source: ISO/IEC 9075-2 §12.3 — foreign-data wrapper USAGE privilege; SQL/MED extension
    pub struct ForeignDataWrapperUsagePrivilegeGranted;
    structural_prop!(
        ForeignDataWrapperUsagePrivilegeGranted,
        "ForeignDataWrapperUsagePrivilegeGranted"
    );
}

pub use emit_impls::{
    ColumnInsertPrivilegeGranted, ColumnPrivilegeRevoked, ColumnSelectPrivilegeGranted,
    ColumnUpdatePrivilegeGranted, DatabaseConnectPrivilegeGranted, DatabaseCreatePrivilegeGranted,
    DatabasePrivilegeRevoked, DatabaseTempPrivilegeGranted, DefaultPrivilegesApplied,
    ForeignDataWrapperUsagePrivilegeGranted, ForeignServerUsagePrivilegeGranted,
    FunctionExecutePrivilegeGranted, FunctionExecutePrivilegeRevoked, ObjectOwnershipTransferred,
    ObjectOwnershipVerified, PrivilegeGrantedWithGrantOption, PrivilegeRevokedCascade,
    PrivilegeRevokedRestrict, PublicConnectRevokedFromDatabase, PublicRolePrivilegeLimited,
    RlsBypassRoleExcluded, RlsDeletePolicyApplied, RlsInsertPolicyApplied, RlsSelectPolicyApplied,
    RlsUpdatePolicyApplied, RoleAdminOptionGranted, RoleGrantOptionInherited,
    RoleInheritanceActive, RoleMembershipGranted, RoleMembershipRevoked, RoleResetApplied,
    RoleSetRoleApplied, SchemaCreatePrivilegeGranted, SchemaPrivilegeRevoked,
    SchemaUsagePrivilegeGranted, SequenceSelectPrivilegeGranted, SequenceUpdatePrivilegeGranted,
    SequenceUsagePrivilegeGranted, TableAllPrivilegesGranted, TableDeletePrivilegeGranted,
    TableInsertPrivilegeGranted, TablePrivilegeRevoked, TableReferencesPrivilegeGranted,
    TableSelectPrivilegeGranted, TableTriggerPrivilegeGranted, TableTruncatePrivilegeGranted,
    TableUpdatePrivilegeGranted, TypeUsagePrivilegeGranted,
};
