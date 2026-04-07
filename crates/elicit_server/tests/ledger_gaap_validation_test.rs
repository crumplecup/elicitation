//! Tests for GAAP validation functions.
//!
//! These tests verify that the validation functions correctly establish
//! GAAP propositions for valid transfers and return appropriate errors
//! for violations.

use elicit_server::ledger::{
    AccountId, Amount, Transfer, TransferId, validate_accrual_basis,
    validate_conservatism_principle, validate_double_entry_bookkeeping,
    validate_economic_entity_assumption, validate_going_concern_assumption,
    validate_historical_cost_principle, validate_matching_principle,
    validate_materiality_principle, validate_monetary_unit_assumption,
};

// ─────────────────────────────────────────────────────────────
//  P0: Critical Validations
// ─────────────────────────────────────────────────────────────

#[test]
fn test_validate_double_entry_bookkeeping_valid() {
    let transfer = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(100).unwrap(),
        TransferId::new("tx1"),
    );

    let result = validate_double_entry_bookkeeping(&transfer);
    assert!(
        result.is_ok(),
        "Valid transfer should pass double-entry check"
    );

    // Verify proof is zero-sized
    let proof = result.unwrap();
    assert_eq!(std::mem::size_of_val(&proof), 0);
}

#[test]
fn test_validate_double_entry_bookkeeping_zero_amount() {
    // Amount validation happens at Amount::new, but we can test the principle
    // This test documents that zero amounts violate double-entry
    let result = Amount::new(0);
    assert!(result.is_err(), "Zero amount should fail at construction");
}

#[test]
fn test_validate_accrual_basis_valid() {
    let transfer = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(100).unwrap(),
        TransferId::new("tx1"),
    );

    let result = validate_accrual_basis(&transfer);
    assert!(result.is_ok(), "Transfer should satisfy accrual basis");

    let proof = result.unwrap();
    assert_eq!(std::mem::size_of_val(&proof), 0);
}

#[test]
fn test_validate_monetary_unit_assumption_valid() {
    let transfer = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(100).unwrap(),
        TransferId::new("tx1"),
    );

    let result = validate_monetary_unit_assumption(&transfer);
    assert!(
        result.is_ok(),
        "Valid amount should satisfy monetary unit assumption"
    );

    let proof = result.unwrap();
    assert_eq!(std::mem::size_of_val(&proof), 0);
}

#[test]
fn test_validate_monetary_unit_assumption_large_amounts() {
    // Test with various amount sizes
    let amounts = vec![1, 100, 10000, 1000000, 100000000];

    for amt in amounts {
        let transfer = Transfer::new(
            AccountId::new("Alice"),
            AccountId::new("Bob"),
            Amount::new(amt).unwrap(),
            TransferId::new("tx1"),
        );

        let result = validate_monetary_unit_assumption(&transfer);
        assert!(
            result.is_ok(),
            "Amount {} should satisfy monetary unit assumption",
            amt
        );
    }
}

// ─────────────────────────────────────────────────────────────
//  P1: Enhanced Validations
// ─────────────────────────────────────────────────────────────

#[test]
fn test_validate_matching_principle_valid() {
    let transfer = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(100).unwrap(),
        TransferId::new("tx1"),
    );

    let result = validate_matching_principle(&transfer);
    assert!(result.is_ok(), "Transfer should satisfy matching principle");

    let proof = result.unwrap();
    assert_eq!(std::mem::size_of_val(&proof), 0);
}

#[test]
fn test_validate_economic_entity_assumption_valid() {
    let transfer = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(100).unwrap(),
        TransferId::new("tx1"),
    );

    let result = validate_economic_entity_assumption(&transfer);
    assert!(
        result.is_ok(),
        "Valid accounts should satisfy economic entity assumption"
    );

    let proof = result.unwrap();
    assert_eq!(std::mem::size_of_val(&proof), 0);
}

#[test]
fn test_validate_economic_entity_assumption_empty_account() {
    let transfer = Transfer::new(
        AccountId::new(""),
        AccountId::new("Bob"),
        Amount::new(100).unwrap(),
        TransferId::new("tx1"),
    );

    let result = validate_economic_entity_assumption(&transfer);
    assert!(
        result.is_err(),
        "Empty account name should violate economic entity assumption"
    );
}

#[test]
fn test_validate_historical_cost_principle_valid() {
    let transfer = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(100).unwrap(),
        TransferId::new("tx1"),
    );

    let result = validate_historical_cost_principle(&transfer);
    assert!(
        result.is_ok(),
        "Transfer should satisfy historical cost principle"
    );

    let proof = result.unwrap();
    assert_eq!(std::mem::size_of_val(&proof), 0);
}

// ─────────────────────────────────────────────────────────────
//  P2: Policy Validations
// ─────────────────────────────────────────────────────────────

#[test]
fn test_validate_conservatism_principle_valid() {
    let transfer = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(100).unwrap(),
        TransferId::new("tx1"),
    );

    let result = validate_conservatism_principle(&transfer);
    assert!(
        result.is_ok(),
        "Transfer should satisfy conservatism principle"
    );

    let proof = result.unwrap();
    assert_eq!(std::mem::size_of_val(&proof), 0);
}

#[test]
fn test_validate_going_concern_assumption_valid() {
    let transfer = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(100).unwrap(),
        TransferId::new("tx1"),
    );

    let result = validate_going_concern_assumption(&transfer);
    assert!(
        result.is_ok(),
        "Transfer should satisfy going concern assumption"
    );

    let proof = result.unwrap();
    assert_eq!(std::mem::size_of_val(&proof), 0);
}

#[test]
fn test_validate_materiality_principle_no_threshold() {
    let transfer = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(100).unwrap(),
        TransferId::new("tx1"),
    );

    let result = validate_materiality_principle(&transfer, None);
    assert!(
        result.is_ok(),
        "Transfer should satisfy materiality principle without threshold"
    );

    let proof = result.unwrap();
    assert_eq!(std::mem::size_of_val(&proof), 0);
}

#[test]
fn test_validate_materiality_principle_with_threshold_material() {
    let transfer = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(1000).unwrap(),
        TransferId::new("tx1"),
    );

    // Threshold is 500, amount is 1000 - material
    let result = validate_materiality_principle(&transfer, Some(500));
    assert!(
        result.is_ok(),
        "Amount exceeding threshold should be material"
    );
}

#[test]
fn test_validate_materiality_principle_with_threshold_immaterial() {
    let transfer = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(100).unwrap(),
        TransferId::new("tx1"),
    );

    // Threshold is 500, amount is 100 - immaterial but still valid
    let result = validate_materiality_principle(&transfer, Some(500));
    assert!(
        result.is_ok(),
        "Immaterial amounts should still pass validation"
    );
}

// ─────────────────────────────────────────────────────────────
//  Composite Validation
// ─────────────────────────────────────────────────────────────

#[test]
fn test_validate_all_gaap_principles() {
    use elicitation::contracts::both;

    let transfer = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(100).unwrap(),
        TransferId::new("tx1"),
    );

    // Validate all P0 principles
    let double_entry = validate_double_entry_bookkeeping(&transfer).unwrap();
    let accrual = validate_accrual_basis(&transfer).unwrap();
    let monetary = validate_monetary_unit_assumption(&transfer).unwrap();

    // Validate all P1 principles
    let matching = validate_matching_principle(&transfer).unwrap();
    let entity = validate_economic_entity_assumption(&transfer).unwrap();
    let historical = validate_historical_cost_principle(&transfer).unwrap();

    // Validate all P2 principles
    let conservatism = validate_conservatism_principle(&transfer).unwrap();
    let going_concern = validate_going_concern_assumption(&transfer).unwrap();
    let materiality = validate_materiality_principle(&transfer, None).unwrap();

    // Build P0 core composite
    let p0_a = both(accrual, monetary);
    let p0 = both(double_entry, p0_a);

    // Build P1 enhanced composite
    let p1_a = both(entity, historical);
    let p1 = both(matching, p1_a);

    // Build P2 policy composite
    let p2_a = both(going_concern, materiality);
    let p2 = both(conservatism, p2_a);

    // Combine all into full GAAP compliance proof
    let p01 = both(p0, p1);
    let full_gaap = both(p01, p2);

    // Verify composite proof is zero-sized
    assert_eq!(
        std::mem::size_of_val(&full_gaap),
        0,
        "Full GAAP compliance proof must be zero-sized"
    );
}

#[test]
fn test_validate_gaap_core_compliance() {
    use elicitation::contracts::both;

    let transfer = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(100).unwrap(),
        TransferId::new("tx1"),
    );

    // Validate P0 core principles
    let double_entry = validate_double_entry_bookkeeping(&transfer).unwrap();
    let accrual = validate_accrual_basis(&transfer).unwrap();
    let monetary = validate_monetary_unit_assumption(&transfer).unwrap();

    // Compose into GaapCoreCompliant
    let core = both(double_entry, both(accrual, monetary));

    assert_eq!(
        std::mem::size_of_val(&core),
        0,
        "GAAP core compliance proof must be zero-sized"
    );
}
