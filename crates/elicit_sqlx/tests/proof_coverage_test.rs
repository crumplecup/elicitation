//! Proof coverage tests — assert every Prop has non-empty kani/verus/creusot proofs.

#![cfg(feature = "proofs")]

use elicit_sqlx::{
    DbConnected, MigrateFragmentEmitted, QueryAsFragmentEmitted, QueryExecuted,
    QueryFragmentEmitted, QueryScalarFragmentEmitted, RowsFetched, TransactionCommitted,
    TransactionOpen, TransactionRolledBack,
};
use elicitation::contracts::Prop;

macro_rules! assert_prop_proofs {
    ($($T:ty),+ $(,)?) => {
        $(
            assert!(!<$T as Prop>::kani_proof().is_empty(),
                "{} missing kani_proof", stringify!($T));
            assert!(!<$T as Prop>::verus_proof().is_empty(),
                "{} missing verus_proof", stringify!($T));
            assert!(!<$T as Prop>::creusot_proof().is_empty(),
                "{} missing creusot_proof", stringify!($T));
        )+
    };
}

#[test]
fn all_sqlx_props_have_proof_coverage() {
    assert_prop_proofs!(
        DbConnected,
        QueryExecuted,
        RowsFetched,
        TransactionOpen,
        TransactionCommitted,
        TransactionRolledBack,
        QueryFragmentEmitted,
        QueryAsFragmentEmitted,
        QueryScalarFragmentEmitted,
        MigrateFragmentEmitted,
    );
}
