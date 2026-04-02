//! `VerifiedWorkflow` validation tests for elicit_sqlx propositions.

use elicit_sqlx::{
    DbConnected, MigrateFragmentEmitted, QueryAsFragmentEmitted, QueryExecuted,
    QueryFragmentEmitted, QueryScalarFragmentEmitted, RowsFetched, TransactionCommitted,
    TransactionOpen, TransactionRolledBack,
};
use elicitation::VerifiedWorkflow;
use elicitation::contracts::And;

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn sqlx_workflow_props_non_empty() {
    assert_verified::<DbConnected>("DbConnected");
    assert_verified::<QueryExecuted>("QueryExecuted");
    assert_verified::<RowsFetched>("RowsFetched");
    assert_verified::<TransactionOpen>("TransactionOpen");
    assert_verified::<TransactionCommitted>("TransactionCommitted");
    assert_verified::<TransactionRolledBack>("TransactionRolledBack");
}

#[test]
fn sqlx_frag_props_non_empty() {
    assert_verified::<QueryFragmentEmitted>("QueryFragmentEmitted");
    assert_verified::<QueryAsFragmentEmitted>("QueryAsFragmentEmitted");
    assert_verified::<QueryScalarFragmentEmitted>("QueryScalarFragmentEmitted");
    assert_verified::<MigrateFragmentEmitted>("MigrateFragmentEmitted");
}

#[test]
fn sqlx_and_contains_constituents() {
    type ConnQuery = And<DbConnected, QueryExecuted>;
    type ConnRows = And<DbConnected, RowsFetched>;
    type OpenCommit = And<TransactionOpen, TransactionCommitted>;

    assert!(ConnQuery::kani_proof_contains::<DbConnected>());
    assert!(ConnQuery::kani_proof_contains::<QueryExecuted>());
    assert!(ConnRows::kani_proof_contains::<RowsFetched>());
    assert!(OpenCommit::kani_proof_contains::<TransactionOpen>());
    assert!(OpenCommit::kani_proof_contains::<TransactionCommitted>());
}
