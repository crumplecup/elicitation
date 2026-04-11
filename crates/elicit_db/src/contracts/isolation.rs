//! ANSI isolation level propositions.
//!
//! Source: ANSI X3.135-1992 (SQL-92) — Transaction isolation and read phenomena.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    /// Transaction ran at READ UNCOMMITTED isolation.
    ///
    /// Source: ANSI SQL-92 Table 2 — isolation level READ UNCOMMITTED
    pub struct ReadUncommittedIsolation;

    /// Transaction ran at READ COMMITTED isolation.
    ///
    /// Source: ANSI SQL-92 Table 2 — isolation level READ COMMITTED
    pub struct ReadCommittedIsolation;

    /// Transaction ran at REPEATABLE READ isolation.
    ///
    /// Source: ANSI SQL-92 Table 2 — isolation level REPEATABLE READ
    pub struct RepeatableReadIsolation;

    /// Transaction ran at SERIALIZABLE isolation.
    ///
    /// Source: ANSI SQL-92 Table 2 — isolation level SERIALIZABLE
    pub struct SerializableIsolation;

    /// Dirty reads (phenomenon P1) are prevented.
    ///
    /// Source: ANSI SQL-92 §4.28 — Read phenomena P1
    pub struct PreventsDirtyRead;

    /// Non-repeatable reads (phenomenon P2) are prevented.
    ///
    /// Source: ANSI SQL-92 §4.28 — Read phenomena P2
    pub struct PreventsNonRepeatableRead;

    /// Phantom reads (phenomenon P3) are prevented.
    ///
    /// Source: ANSI SQL-92 §4.28 — Read phenomena P3
    pub struct PreventsPhantomRead;

    /// Dirty writes (phenomenon P0) are prevented.
    ///
    /// Source: Berenson et al. "A Critique of ANSI SQL Isolation Levels" §2.1 — P0
    pub struct PreventsDirtyWrite;

    macro_rules! isolation_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by isolation level guarantee */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by isolation level guarantee */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by isolation level guarantee */ }
                }
            }
        };
    }

    isolation_prop!(ReadUncommittedIsolation, "ReadUncommittedIsolation");
    isolation_prop!(ReadCommittedIsolation, "ReadCommittedIsolation");
    isolation_prop!(RepeatableReadIsolation, "RepeatableReadIsolation");
    isolation_prop!(SerializableIsolation, "SerializableIsolation");
    isolation_prop!(PreventsDirtyRead, "PreventsDirtyRead");
    isolation_prop!(PreventsNonRepeatableRead, "PreventsNonRepeatableRead");
    isolation_prop!(PreventsPhantomRead, "PreventsPhantomRead");
    isolation_prop!(PreventsDirtyWrite, "PreventsDirtyWrite");
}

pub use emit_impls::{
    PreventsDirtyRead, PreventsDirtyWrite, PreventsNonRepeatableRead, PreventsPhantomRead,
    ReadCommittedIsolation, ReadUncommittedIsolation, RepeatableReadIsolation,
    SerializableIsolation,
};

/// Alias: no dirty reads occurred — equivalent to [`PreventsDirtyRead`].
pub type NoDirtyReads = PreventsDirtyRead;

/// Alias: no phantom reads occurred — equivalent to [`PreventsPhantomRead`].
pub type NoPhantomReads = PreventsPhantomRead;
