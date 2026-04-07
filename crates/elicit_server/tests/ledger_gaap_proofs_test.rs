//! Tests for GAAP proposition types - verifying zero-cost proofs and basic usage.
//!
//! These tests verify that:
//! 1. All GAAP propositions are zero-sized (compile to nothing)
//! 2. Established<P> proofs are zero-sized
//! 3. Proofs can be constructed and used
//! 4. Composite proofs maintain zero-cost property

use elicit_server::ledger::{
    AccrualBasis, ConservatismPrinciple, DoubleEntryBookkeeping, EconomicEntityAssumption,
    GoingConcernAssumption, HistoricalCostPrinciple, MatchingPrinciple, MaterialityPrinciple,
    MonetaryUnitAssumption,
};
use elicitation::contracts::Established;

// ─────────────────────────────────────────────────────────────
//  P0: Critical - Core Ledger Operations
// ─────────────────────────────────────────────────────────────

#[test]
fn test_double_entry_bookkeeping_zero_sized() {
    let proof: Established<DoubleEntryBookkeeping> = Established::assert();
    assert_eq!(
        std::mem::size_of_val(&proof),
        0,
        "DoubleEntryBookkeeping proof must be zero-sized"
    );
}

#[test]
fn test_double_entry_bookkeeping_copy() {
    let proof: Established<DoubleEntryBookkeeping> = Established::assert();
    let proof2 = proof; // Copy
    let _proof3 = proof; // Can still use original
    let _proof4 = proof2; // Can use copy
}

#[test]
fn test_accrual_basis_zero_sized() {
    let proof: Established<AccrualBasis> = Established::assert();
    assert_eq!(
        std::mem::size_of_val(&proof),
        0,
        "AccrualBasis proof must be zero-sized"
    );
}

#[test]
fn test_accrual_basis_copy() {
    let proof: Established<AccrualBasis> = Established::assert();
    let proof2 = proof;
    let _proof3 = proof;
    let _proof4 = proof2;
}

#[test]
fn test_monetary_unit_assumption_zero_sized() {
    let proof: Established<MonetaryUnitAssumption> = Established::assert();
    assert_eq!(
        std::mem::size_of_val(&proof),
        0,
        "MonetaryUnitAssumption proof must be zero-sized"
    );
}

#[test]
fn test_monetary_unit_assumption_copy() {
    let proof: Established<MonetaryUnitAssumption> = Established::assert();
    let proof2 = proof;
    let _proof3 = proof;
    let _proof4 = proof2;
}

// ─────────────────────────────────────────────────────────────
//  P1: Enhanced Compliance - Audit-Ready Operations
// ─────────────────────────────────────────────────────────────

#[test]
fn test_matching_principle_zero_sized() {
    let proof: Established<MatchingPrinciple> = Established::assert();
    assert_eq!(
        std::mem::size_of_val(&proof),
        0,
        "MatchingPrinciple proof must be zero-sized"
    );
}

#[test]
fn test_matching_principle_copy() {
    let proof: Established<MatchingPrinciple> = Established::assert();
    let proof2 = proof;
    let _proof3 = proof;
    let _proof4 = proof2;
}

#[test]
fn test_economic_entity_assumption_zero_sized() {
    let proof: Established<EconomicEntityAssumption> = Established::assert();
    assert_eq!(
        std::mem::size_of_val(&proof),
        0,
        "EconomicEntityAssumption proof must be zero-sized"
    );
}

#[test]
fn test_economic_entity_assumption_copy() {
    let proof: Established<EconomicEntityAssumption> = Established::assert();
    let proof2 = proof;
    let _proof3 = proof;
    let _proof4 = proof2;
}

#[test]
fn test_historical_cost_principle_zero_sized() {
    let proof: Established<HistoricalCostPrinciple> = Established::assert();
    assert_eq!(
        std::mem::size_of_val(&proof),
        0,
        "HistoricalCostPrinciple proof must be zero-sized"
    );
}

#[test]
fn test_historical_cost_principle_copy() {
    let proof: Established<HistoricalCostPrinciple> = Established::assert();
    let proof2 = proof;
    let _proof3 = proof;
    let _proof4 = proof2;
}

// ─────────────────────────────────────────────────────────────
//  P2: Policy - Configuration and Error Handling
// ─────────────────────────────────────────────────────────────

#[test]
fn test_conservatism_principle_zero_sized() {
    let proof: Established<ConservatismPrinciple> = Established::assert();
    assert_eq!(
        std::mem::size_of_val(&proof),
        0,
        "ConservatismPrinciple proof must be zero-sized"
    );
}

#[test]
fn test_conservatism_principle_copy() {
    let proof: Established<ConservatismPrinciple> = Established::assert();
    let proof2 = proof;
    let _proof3 = proof;
    let _proof4 = proof2;
}

#[test]
fn test_going_concern_assumption_zero_sized() {
    let proof: Established<GoingConcernAssumption> = Established::assert();
    assert_eq!(
        std::mem::size_of_val(&proof),
        0,
        "GoingConcernAssumption proof must be zero-sized"
    );
}

#[test]
fn test_going_concern_assumption_copy() {
    let proof: Established<GoingConcernAssumption> = Established::assert();
    let proof2 = proof;
    let _proof3 = proof;
    let _proof4 = proof2;
}

#[test]
fn test_materiality_principle_zero_sized() {
    let proof: Established<MaterialityPrinciple> = Established::assert();
    assert_eq!(
        std::mem::size_of_val(&proof),
        0,
        "MaterialityPrinciple proof must be zero-sized"
    );
}

#[test]
fn test_materiality_principle_copy() {
    let proof: Established<MaterialityPrinciple> = Established::assert();
    let proof2 = proof;
    let _proof3 = proof;
    let _proof4 = proof2;
}

// ─────────────────────────────────────────────────────────────
//  Composite Proofs
// ─────────────────────────────────────────────────────────────

#[test]
fn test_composite_gaap_core_zero_sized() {
    use elicitation::contracts::both;

    // GaapCoreCompliant = And<DoubleEntryBookkeeping, And<AccrualBasis, MonetaryUnitAssumption>>
    let double_entry: Established<DoubleEntryBookkeeping> = Established::assert();
    let accrual: Established<AccrualBasis> = Established::assert();
    let monetary: Established<MonetaryUnitAssumption> = Established::assert();

    let accrual_and_monetary = both(accrual, monetary);
    let core_gaap = both(double_entry, accrual_and_monetary);

    assert_eq!(
        std::mem::size_of_val(&core_gaap),
        0,
        "Composite GAAP Core proof must be zero-sized"
    );
}

#[test]
fn test_composite_gaap_enhanced_zero_sized() {
    use elicitation::contracts::both;

    // Enhanced = Core + Matching + Economic Entity + Historical Cost
    let double_entry: Established<DoubleEntryBookkeeping> = Established::assert();
    let accrual: Established<AccrualBasis> = Established::assert();
    let monetary: Established<MonetaryUnitAssumption> = Established::assert();
    let matching: Established<MatchingPrinciple> = Established::assert();
    let entity: Established<EconomicEntityAssumption> = Established::assert();
    let historical: Established<HistoricalCostPrinciple> = Established::assert();

    // Build core
    let accrual_and_monetary = both(accrual, monetary);
    let core = both(double_entry, accrual_and_monetary);

    // Build P1 additions
    let entity_and_historical = both(entity, historical);
    let p1 = both(matching, entity_and_historical);

    // Combine
    let enhanced = both(core, p1);

    assert_eq!(
        std::mem::size_of_val(&enhanced),
        0,
        "Composite GAAP Enhanced proof must be zero-sized"
    );
}

#[test]
fn test_composite_gaap_full_zero_sized() {
    use elicitation::contracts::both;

    // All 9 principles
    let p0_double: Established<DoubleEntryBookkeeping> = Established::assert();
    let p0_accrual: Established<AccrualBasis> = Established::assert();
    let p0_monetary: Established<MonetaryUnitAssumption> = Established::assert();
    let p1_matching: Established<MatchingPrinciple> = Established::assert();
    let p1_entity: Established<EconomicEntityAssumption> = Established::assert();
    let p1_historical: Established<HistoricalCostPrinciple> = Established::assert();
    let p2_conservatism: Established<ConservatismPrinciple> = Established::assert();
    let p2_going: Established<GoingConcernAssumption> = Established::assert();
    let p2_materiality: Established<MaterialityPrinciple> = Established::assert();

    // Build P0
    let p0_a = both(p0_accrual, p0_monetary);
    let p0 = both(p0_double, p0_a);

    // Build P1
    let p1_a = both(p1_entity, p1_historical);
    let p1 = both(p1_matching, p1_a);

    // Build P2
    let p2_a = both(p2_going, p2_materiality);
    let p2 = both(p2_conservatism, p2_a);

    // Combine all
    let p01 = both(p0, p1);
    let full = both(p01, p2);

    assert_eq!(
        std::mem::size_of_val(&full),
        0,
        "Full GAAP compliance proof must be zero-sized"
    );
}

// ─────────────────────────────────────────────────────────────
//  Function Parameter Usage
// ─────────────────────────────────────────────────────────────

#[test]
fn test_function_requires_proof() {
    // Functions can require GAAP proofs as parameters
    fn requires_double_entry(_proof: Established<DoubleEntryBookkeeping>) {
        // Function body
    }

    fn requires_gaap_core(
        _proof: Established<
            elicitation::contracts::And<
                DoubleEntryBookkeeping,
                elicitation::contracts::And<AccrualBasis, MonetaryUnitAssumption>,
            >,
        >,
    ) {
        // Function body
    }

    // Can call with proofs
    let double_entry = Established::assert();
    requires_double_entry(double_entry);

    // Composite proof
    use elicitation::contracts::both;
    let de = Established::assert();
    let ac = Established::assert();
    let mon = Established::assert();
    let core = both(de, both(ac, mon));
    requires_gaap_core(core);
}

#[test]
fn test_cannot_call_without_proof() {
    // This is a compile-time test - the following would NOT compile:
    //
    // fn requires_proof(_p: Established<DoubleEntryBookkeeping>) {}
    // requires_proof();  // ERROR: missing argument
    //
    // Type system enforces proof requirement
}
