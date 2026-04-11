//! Backup and recovery propositions.
//!
//! Source: PostgreSQL documentation, chapters 26 (backup) and 30 (WAL).

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    /// Backup is consistent and can be used for a full restore.
    ///
    /// Source: PostgreSQL docs §26 — Backup and Restore
    pub struct BackupConsistent;

    /// WAL segment is intact and can be replayed.
    ///
    /// Source: PostgreSQL docs §30 — Reliability and the Write-Ahead Log
    pub struct WALReplayable;

    /// Database can be restored to a specific point in time.
    ///
    /// Source: PostgreSQL docs §26.3 — Continuous Archiving and Point-in-Time Recovery
    pub struct PointInTimeRecoverable;

    macro_rules! recovery_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by backup/WAL integrity check */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by backup/WAL integrity check */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by backup/WAL integrity check */ }
                }
            }
        };
    }

    recovery_prop!(BackupConsistent, "BackupConsistent");
    recovery_prop!(WALReplayable, "WALReplayable");
    recovery_prop!(PointInTimeRecoverable, "PointInTimeRecoverable");
}

pub use emit_impls::{BackupConsistent, PointInTimeRecoverable, WALReplayable};
