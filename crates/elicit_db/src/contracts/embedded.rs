//! Propositions for embedded key-value store operations.
//!
//! These propositions apply to backends like `redb` that expose a typed,
//! embedded KV store rather than a relational SQL engine.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! embedded_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by embedded store operation */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by embedded store operation */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by embedded store operation */ }
                }
            }
        };
    }

    // ── KV mutations ─────────────────────────────────────────────────────────

    /// A key-value pair was successfully inserted or replaced in a KV table.
    pub struct KvKeyInserted;

    /// A key was successfully removed from a KV table.
    pub struct KvKeyDeleted;

    // ── Maintenance ──────────────────────────────────────────────────────────

    /// The embedded database file was compacted; fragmented space was reclaimed.
    pub struct KvTableCompacted;

    /// Structural integrity of all database pages was verified without error.
    pub struct KvIntegrityVerified;

    // ── Snapshots ────────────────────────────────────────────────────────────

    /// A durable snapshot of the database state was created successfully.
    pub struct KvSnapshotCreated;

    /// The database was successfully restored to a previously created snapshot.
    pub struct KvSnapshotRestored;

    embedded_prop!(KvKeyInserted, "KvKeyInserted");
    embedded_prop!(KvKeyDeleted, "KvKeyDeleted");
    embedded_prop!(KvTableCompacted, "KvTableCompacted");
    embedded_prop!(KvIntegrityVerified, "KvIntegrityVerified");
    embedded_prop!(KvSnapshotCreated, "KvSnapshotCreated");
    embedded_prop!(KvSnapshotRestored, "KvSnapshotRestored");
}

pub use emit_impls::{
    KvIntegrityVerified, KvKeyDeleted, KvKeyInserted, KvSnapshotCreated, KvSnapshotRestored,
    KvTableCompacted,
};
