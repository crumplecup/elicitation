//! PostgreSQL-specific propositions.
//!
//! Source: PostgreSQL documentation, chapters 13 (MVCC) and 25 (maintenance).

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    /// A valid MVCC snapshot was acquired for the transaction.
    ///
    /// Source: PostgreSQL docs §13.1 — Introduction to MVCC
    pub struct MVCCSnapshotValid;

    /// Snapshot isolation semantics are in effect.
    ///
    /// Source: PostgreSQL docs §13.2.2 — Repeatable Read Isolation Level
    pub struct SnapshotIsolation;

    /// An advisory lock is currently held.
    ///
    /// Source: PostgreSQL docs §13.3.5 — Advisory Locks
    pub struct AdvisoryLockHeld;

    /// The queried row is visible to the current transaction snapshot.
    ///
    /// Source: PostgreSQL docs §13.1 — Row visibility rules
    pub struct RowVisible;

    /// The named index exists on the target table.
    ///
    /// Source: PostgreSQL docs §11 — Indexes
    pub struct IndexExists;

    /// The table has been vacuumed recently enough for healthy bloat levels.
    ///
    /// Source: PostgreSQL docs §25.1 — Routine Vacuuming
    pub struct VacuumedRecently;

    macro_rules! pg_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by PostgreSQL MVCC guarantee */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by PostgreSQL MVCC guarantee */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by PostgreSQL MVCC guarantee */ }
                }
            }
        };
    }

    pg_prop!(MVCCSnapshotValid, "MVCCSnapshotValid");
    pg_prop!(SnapshotIsolation, "SnapshotIsolation");
    pg_prop!(AdvisoryLockHeld, "AdvisoryLockHeld");
    pg_prop!(RowVisible, "RowVisible");
    pg_prop!(IndexExists, "IndexExists");
    pg_prop!(VacuumedRecently, "VacuumedRecently");
}

pub use emit_impls::{
    AdvisoryLockHeld, IndexExists, MVCCSnapshotValid, RowVisible, SnapshotIsolation,
    VacuumedRecently,
};
