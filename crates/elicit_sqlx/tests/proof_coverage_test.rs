//! Proof coverage tests — assert every Prop has non-empty kani/verus/creusot proofs,
//! and every wrapper type satisfies `ElicitComplete`.

use elicit_sqlx::{
    AnyColumn, AnyQueryResult, AnyRow, AnyTypeInfo, DbConnected, MigrateFragmentEmitted,
    QueryAsFragmentEmitted, QueryExecuted, QueryFragmentEmitted, QueryScalarFragmentEmitted,
    RowsFetched, SqlxError, TransactionCommitted, TransactionOpen, TransactionRolledBack,
};
use elicitation::{Elicitation as _, contracts::Prop};

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

// ── Compile-time ElicitComplete bound checks ────────────────────────────────

fn assert_elicit_complete<T: elicitation::ElicitComplete>() {}

#[test]
fn all_sqlx_wrapper_types_are_elicit_complete() {
    assert_elicit_complete::<AnyColumn>();
    assert_elicit_complete::<AnyQueryResult>();
    assert_elicit_complete::<AnyRow>();
    assert_elicit_complete::<AnyTypeInfo>();
    assert_elicit_complete::<SqlxError>();
}

#[test]
fn sqlx_wrapper_proofs_are_empty_by_design() {
    assert!(
        !AnyColumn::kani_proof().is_empty(),
        "AnyColumn kani_proof expected non-empty"
    );
    assert!(
        !AnyQueryResult::kani_proof().is_empty(),
        "AnyQueryResult kani_proof expected non-empty"
    );
    assert!(
        !AnyRow::kani_proof().is_empty(),
        "AnyRow kani_proof expected non-empty"
    );
    assert!(
        !AnyTypeInfo::kani_proof().is_empty(),
        "AnyTypeInfo kani_proof expected non-empty"
    );
    assert!(
        !SqlxError::kani_proof().is_empty(),
        "SqlxError kani_proof expected non-empty"
    );
}
