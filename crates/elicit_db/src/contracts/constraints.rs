//! Constraint propositions.
//!
//! Source: ISO/IEC 9075-2 — Schema definition statements (§11); PostgreSQL §5.4–5.5.

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

    // -- NOT NULL constraints (§11.4) --

    /// Column has a NOT NULL constraint defined in its DDL.
    ///
    /// Source: ISO/IEC 9075-2 §11.4 — Column constraints: NOT NULL
    pub struct NotNullConstraintDefined;
    structural_prop!(NotNullConstraintDefined, "NotNullConstraintDefined");

    /// NULL value insertion was correctly rejected by NOT NULL.
    ///
    /// Source: ISO/IEC 9075-2 §11.4 — Violation of NOT NULL constraint
    pub struct NotNullConstraintEnforced;
    structural_prop!(NotNullConstraintEnforced, "NotNullConstraintEnforced");

    /// NOT NULL constraint was removed with ALTER TABLE.
    ///
    /// Source: ISO/IEC 9075-2 §11.4 — DROP CONSTRAINT / ALTER COLUMN DROP NOT NULL
    pub struct NotNullConstraintDropped;
    structural_prop!(NotNullConstraintDropped, "NotNullConstraintDropped");

    // -- UNIQUE constraints (§11.6) --

    /// A UNIQUE constraint is defined on the column set.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — Unique constraint definition
    pub struct UniqueConstraintDefined;
    structural_prop!(UniqueConstraintDefined, "UniqueConstraintDefined");

    /// Duplicate value insertion was rejected by UNIQUE.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — Enforcement of uniqueness
    pub struct UniqueConstraintEnforced;
    structural_prop!(UniqueConstraintEnforced, "UniqueConstraintEnforced");

    /// UNIQUE constraint was dropped.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — DROP CONSTRAINT
    pub struct UniqueConstraintDropped;
    structural_prop!(UniqueConstraintDropped, "UniqueConstraintDropped");

    // -- Primary key constraints (§11.7) --

    /// Table has a PRIMARY KEY constraint.
    ///
    /// Source: ISO/IEC 9075-2 §11.7 — Primary key constraint definition
    pub struct PrimaryKeyDefined;
    structural_prop!(PrimaryKeyDefined, "PrimaryKeyDefined");

    /// PRIMARY KEY covers exactly one column.
    ///
    /// Source: ISO/IEC 9075-2 §11.7 — Single-column primary key
    pub struct PrimaryKeySingleColumn;
    structural_prop!(PrimaryKeySingleColumn, "PrimaryKeySingleColumn");

    /// PRIMARY KEY covers two or more columns.
    ///
    /// Source: ISO/IEC 9075-2 §11.7 — Composite primary key
    pub struct PrimaryKeyMultiColumn;
    structural_prop!(PrimaryKeyMultiColumn, "PrimaryKeyMultiColumn");

    /// Duplicate or NULL PK insertion was rejected.
    ///
    /// Source: ISO/IEC 9075-2 §11.7 — Enforcement of primary key uniqueness and NOT NULL
    pub struct PrimaryKeyEnforced;
    structural_prop!(PrimaryKeyEnforced, "PrimaryKeyEnforced");

    /// PRIMARY KEY constraint was dropped.
    ///
    /// Source: ISO/IEC 9075-2 §11.7 — DROP CONSTRAINT on primary key
    pub struct PrimaryKeyDropped;
    structural_prop!(PrimaryKeyDropped, "PrimaryKeyDropped");

    // -- Foreign key constraints (§11.8) --

    /// FOREIGN KEY constraint is defined referencing another table.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — Referential constraint definition
    pub struct ForeignKeyDefined;
    structural_prop!(ForeignKeyDefined, "ForeignKeyDefined");

    /// ON DELETE CASCADE action is defined.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — Referential action: CASCADE
    pub struct ForeignKeyOnDeleteCascade;
    structural_prop!(ForeignKeyOnDeleteCascade, "ForeignKeyOnDeleteCascade");

    /// ON DELETE SET NULL action is defined.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — Referential action: SET NULL
    pub struct ForeignKeyOnDeleteSetNull;
    structural_prop!(ForeignKeyOnDeleteSetNull, "ForeignKeyOnDeleteSetNull");

    /// ON DELETE SET DEFAULT action is defined.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — Referential action: SET DEFAULT
    pub struct ForeignKeyOnDeleteSetDefault;
    structural_prop!(ForeignKeyOnDeleteSetDefault, "ForeignKeyOnDeleteSetDefault");

    /// ON DELETE RESTRICT action is defined.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — Referential action: RESTRICT
    pub struct ForeignKeyOnDeleteRestrict;
    structural_prop!(ForeignKeyOnDeleteRestrict, "ForeignKeyOnDeleteRestrict");

    /// ON DELETE NO ACTION (default) is defined.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — Referential action: NO ACTION
    pub struct ForeignKeyOnDeleteNoAction;
    structural_prop!(ForeignKeyOnDeleteNoAction, "ForeignKeyOnDeleteNoAction");

    /// ON UPDATE CASCADE action is defined.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — Referential action on UPDATE: CASCADE
    pub struct ForeignKeyOnUpdateCascade;
    structural_prop!(ForeignKeyOnUpdateCascade, "ForeignKeyOnUpdateCascade");

    /// ON UPDATE SET NULL action is defined.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — Referential action on UPDATE: SET NULL
    pub struct ForeignKeyOnUpdateSetNull;
    structural_prop!(ForeignKeyOnUpdateSetNull, "ForeignKeyOnUpdateSetNull");

    /// ON UPDATE RESTRICT action is defined.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — Referential action on UPDATE: RESTRICT
    pub struct ForeignKeyOnUpdateRestrict;
    structural_prop!(ForeignKeyOnUpdateRestrict, "ForeignKeyOnUpdateRestrict");

    /// ON UPDATE SET DEFAULT action is defined.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — Referential action on UPDATE: SET DEFAULT
    pub struct ForeignKeyOnUpdateSetDefault;
    structural_prop!(ForeignKeyOnUpdateSetDefault, "ForeignKeyOnUpdateSetDefault");

    /// ON UPDATE NO ACTION (default) is defined.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — Referential action on UPDATE: NO ACTION
    pub struct ForeignKeyOnUpdateNoAction;
    structural_prop!(ForeignKeyOnUpdateNoAction, "ForeignKeyOnUpdateNoAction");

    /// Referential integrity violation was rejected.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — Enforcement of referential constraints
    pub struct ForeignKeyEnforced;
    structural_prop!(ForeignKeyEnforced, "ForeignKeyEnforced");

    /// FOREIGN KEY constraint was dropped.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — DROP CONSTRAINT on foreign key
    pub struct ForeignKeyDropped;
    structural_prop!(ForeignKeyDropped, "ForeignKeyDropped");

    // -- CHECK constraints (§11.6) --

    /// A CHECK constraint is defined on the table or column.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — Check constraint definition
    pub struct CheckConstraintDefined;
    structural_prop!(CheckConstraintDefined, "CheckConstraintDefined");

    /// CHECK constraint evaluated to TRUE for the inserted/updated row.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — Check constraint search condition: TRUE
    pub struct CheckConstraintEvaluatesTrue;
    structural_prop!(CheckConstraintEvaluatesTrue, "CheckConstraintEvaluatesTrue");

    /// CHECK constraint evaluated to FALSE and the operation was rejected.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — Constraint violation: CHECK condition FALSE
    pub struct CheckConstraintViolationRejected;
    structural_prop!(
        CheckConstraintViolationRejected,
        "CheckConstraintViolationRejected"
    );

    /// CHECK constraint was dropped.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — DROP CONSTRAINT on check constraint
    pub struct CheckConstraintDropped;
    structural_prop!(CheckConstraintDropped, "CheckConstraintDropped");

    // -- Deferrable constraints (§11.6, §17.5) --

    /// Constraint is declared DEFERRABLE.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — Constraint characteristics: DEFERRABLE
    pub struct ConstraintDeferrable;
    structural_prop!(ConstraintDeferrable, "ConstraintDeferrable");

    /// Constraint starts DEFERRED within transactions.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — Constraint characteristics: INITIALLY DEFERRED
    pub struct ConstraintInitiallyDeferred;
    structural_prop!(ConstraintInitiallyDeferred, "ConstraintInitiallyDeferred");

    /// Constraint starts IMMEDIATE within transactions.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — Constraint characteristics: INITIALLY IMMEDIATE
    pub struct ConstraintInitiallyImmediate;
    structural_prop!(ConstraintInitiallyImmediate, "ConstraintInitiallyImmediate");

    /// SET CONSTRAINTS DEFERRED deferred this constraint.
    ///
    /// Source: ISO/IEC 9075-2 §17.5 — SET CONSTRAINTS statement
    pub struct ConstraintDeferredInTransaction;
    structural_prop!(
        ConstraintDeferredInTransaction,
        "ConstraintDeferredInTransaction"
    );

    /// Deferred constraint was checked at transaction COMMIT.
    ///
    /// Source: ISO/IEC 9075-2 §17.5 — Deferred constraint checking at COMMIT
    pub struct ConstraintCheckedAtCommit;
    structural_prop!(ConstraintCheckedAtCommit, "ConstraintCheckedAtCommit");

    // -- Exclusion constraints (PostgreSQL extension) --

    /// EXCLUDE constraint is defined using GiST/SP-GiST.
    ///
    /// Source: PostgreSQL §5.4 — Exclusion constraints
    pub struct ExclusionConstraintDefined;
    structural_prop!(ExclusionConstraintDefined, "ExclusionConstraintDefined");

    /// Exclusion violation was correctly rejected.
    ///
    /// Source: PostgreSQL §5.4 — Enforcement of exclusion constraints
    pub struct ExclusionConstraintEnforced;
    structural_prop!(ExclusionConstraintEnforced, "ExclusionConstraintEnforced");

    // -- Partial indexes as constraints (PostgreSQL) --

    /// A partial UNIQUE index implements a conditional constraint.
    ///
    /// Source: PostgreSQL §5.5 — Partial indexes as conditional unique constraints
    pub struct PartialIndexConstraintDefined;
    structural_prop!(
        PartialIndexConstraintDefined,
        "PartialIndexConstraintDefined"
    );

    /// The WHERE predicate of the partial index is correct.
    ///
    /// Source: PostgreSQL §5.5 — Partial index predicate validity
    pub struct PartialIndexConstraintPredicateValid;
    structural_prop!(
        PartialIndexConstraintPredicateValid,
        "PartialIndexConstraintPredicateValid"
    );

    // -- Identity columns (ISO/IEC 9075-2:2016 §11.4) --

    /// Column is defined as `GENERATED ALWAYS AS IDENTITY` or `GENERATED BY DEFAULT AS IDENTITY`.
    ///
    /// Source: ISO/IEC 9075-2:2016 §11.4 — `<identity column specification>`
    pub struct IdentityColumnDefined;
    structural_prop!(IdentityColumnDefined, "IdentityColumnDefined");

    /// `GENERATED ALWAYS` variant: manual `INSERT OVERRIDING USER VALUE` is rejected.
    ///
    /// Source: ISO/IEC 9075-2:2016 §11.4 — GENERATED ALWAYS AS IDENTITY enforcement
    pub struct IdentityAlwaysEnforced;
    structural_prop!(IdentityAlwaysEnforced, "IdentityAlwaysEnforced");

    /// `GENERATED BY DEFAULT` variant: explicit value is accepted; sequence used as fallback.
    ///
    /// Source: ISO/IEC 9075-2:2016 §11.4 — GENERATED BY DEFAULT AS IDENTITY semantics
    pub struct IdentityByDefaultDefined;
    structural_prop!(IdentityByDefaultDefined, "IdentityByDefaultDefined");

    // -- Generated columns (ISO/IEC 9075-2:2011 §11.4 / PostgreSQL §5.3) --

    /// Column is defined as `GENERATED ALWAYS AS (expr) STORED`.
    ///
    /// Source: ISO/IEC 9075-2:2011 §11.4 — `<generation clause>`; PostgreSQL §5.3
    pub struct GeneratedColumnDefined;
    structural_prop!(GeneratedColumnDefined, "GeneratedColumnDefined");

    /// Generated column expression was evaluated and stored on INSERT.
    ///
    /// Source: ISO/IEC 9075-2:2011 §11.4 — generated column computation on insert
    pub struct GeneratedColumnComputedOnInsert;
    structural_prop!(
        GeneratedColumnComputedOnInsert,
        "GeneratedColumnComputedOnInsert"
    );

    /// Generated column expression was re-evaluated and stored on UPDATE.
    ///
    /// Source: ISO/IEC 9075-2:2011 §11.4 — generated column recomputation on update
    pub struct GeneratedColumnComputedOnUpdate;
    structural_prop!(
        GeneratedColumnComputedOnUpdate,
        "GeneratedColumnComputedOnUpdate"
    );
}

pub use emit_impls::{
    CheckConstraintDefined, CheckConstraintDropped, CheckConstraintEvaluatesTrue,
    CheckConstraintViolationRejected, ConstraintCheckedAtCommit, ConstraintDeferrable,
    ConstraintDeferredInTransaction, ConstraintInitiallyDeferred, ConstraintInitiallyImmediate,
    ExclusionConstraintDefined, ExclusionConstraintEnforced, ForeignKeyDefined, ForeignKeyDropped,
    ForeignKeyEnforced, ForeignKeyOnDeleteCascade, ForeignKeyOnDeleteNoAction,
    ForeignKeyOnDeleteRestrict, ForeignKeyOnDeleteSetDefault, ForeignKeyOnDeleteSetNull,
    ForeignKeyOnUpdateCascade, ForeignKeyOnUpdateNoAction, ForeignKeyOnUpdateRestrict,
    ForeignKeyOnUpdateSetDefault, ForeignKeyOnUpdateSetNull, GeneratedColumnComputedOnInsert,
    GeneratedColumnComputedOnUpdate, GeneratedColumnDefined, IdentityAlwaysEnforced,
    IdentityByDefaultDefined, IdentityColumnDefined, NotNullConstraintDefined,
    NotNullConstraintDropped, NotNullConstraintEnforced, PartialIndexConstraintDefined,
    PartialIndexConstraintPredicateValid, PrimaryKeyDefined, PrimaryKeyDropped, PrimaryKeyEnforced,
    PrimaryKeyMultiColumn, PrimaryKeySingleColumn, UniqueConstraintDefined,
    UniqueConstraintDropped, UniqueConstraintEnforced,
};
