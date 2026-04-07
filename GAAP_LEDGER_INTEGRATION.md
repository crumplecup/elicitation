# GAAP Integration for Typestate Ledger

> Applying Generally Accepted Accounting Principles (GAAP) to the ledger typestate system through proof-carrying propositions with formal spec references.

## Overview

Extend the typestate ledger with GAAP-compliant propositions that:
1. Reference specific FASB Accounting Standards Codification (ASC) sections
2. Establish compile-time proof chains for accounting compliance
3. Demonstrate audit-traceable workflows
4. Mirror the WCAG constraint pattern from `elicit_ui`

## Motivation

The UI system applies WCAG standards to accessibility verification:
```rust
// elicit_ui pattern
pub struct HasLabelConstraint;  // WCAG 4.1.2 Level A
pub struct NoOverflowConstraint; // WCAG 1.4.10 Level AA
```

The ledger should apply GAAP standards to accounting verification:
```rust
// ledger pattern (proposed)
#[derive(elicitation::Prop)]
pub struct MatchingPrinciple;  // ASC 606-10-25-23

#[derive(elicitation::Prop)]
pub struct AccrualBasis;  // ASC 606-10-25-1
```

## Current State

**Existing propositions:**
- `AmountPositive` - Basic validation (amount > 0)
- `SufficientFunds` - Balance check
- `AccountsDistinct` - Source ≠ destination
- `BalancedEntries` - Double-entry invariant (debit + credit = 0)

**Missing:**
- GAAP standard references
- Formal accounting principle documentation
- Audit-traceable compliance

## Implementation Plan

### Phase 1: Research GAAP Principles (Days 1-2)

**Goal:** Identify applicable GAAP principles for double-entry bookkeeping.

**Research tasks:**
1. Review FASB ASC Topic 606 (Revenue Recognition)
2. Review FASB ASC Topic 820 (Fair Value Measurement)
3. Review FASB ASC Topic 842 (Leases) - if applicable
4. Identify core principles applicable to ledger operations:
   - Matching Principle
   - Accrual Basis
   - Historical Cost Principle
   - Monetary Unit Assumption
   - Economic Entity Assumption
   - Going Concern Principle
   - Full Disclosure Principle
   - Materiality Principle
   - Conservatism Principle

**Deliverables:**
- [ ] Document: `GAAP_PRINCIPLES_RESEARCH.md`
- [ ] Table mapping GAAP principles → ASC references
- [ ] Applicability analysis for each principle to ledger operations

### Phase 2: Define Proposition Types (Days 3-4)

**Goal:** Create proposition types with accurate documentation.

**Tasks:**
1. Create `src/ledger/gaap.rs` module
2. Define proposition structs with `#[derive(elicitation::Prop)]`
3. Write comprehensive doc comments with:
   - GAAP principle name
   - ASC section reference
   - Plain-language explanation
   - How it applies to ledger operations
   - Examples

**Example structure:**
```rust
/// GAAP Matching Principle (ASC 606-10-25-23)
///
/// Revenue and associated expenses must be recognized in the same
/// accounting period to accurately reflect the relationship between
/// costs incurred and revenues earned.
///
/// # Application to Ledger
///
/// For transfers, this principle ensures that:
/// - Transaction timestamp is recorded at occurrence time
/// - Related debits and credits share the same `transfer_id`
/// - Both entries have identical `created_at` timestamps
///
/// # References
///
/// - FASB ASC 606-10-25-23: Revenue Recognition - When to Recognize
/// - GAAP Conceptual Framework: Matching Principle
#[derive(elicitation::Prop)]
pub struct MatchingPrinciple;
```

**Deliverables:**
- [ ] `crates/elicit_server/src/ledger/gaap.rs`
- [ ] Proposition types (8-12 expected)
- [ ] Comprehensive doc comments
- [ ] Public exports in `ledger/mod.rs`

### Phase 3: Validation Functions (Days 5-6)

**Goal:** Implement validation logic that establishes GAAP propositions.

**Tasks:**
1. Create validation functions returning `Result<Established<P>, ValidationError>`
2. Implement business logic checking GAAP compliance
3. Add new `ValidationError` variants for GAAP violations
4. Document validation criteria

**Example:**
```rust
/// Validate that a transfer follows the matching principle.
///
/// Checks:
/// - Both debit and credit have same `transfer_id`
/// - Both entries have same `created_at` timestamp
/// - Transaction is atomic (both or neither)
fn validate_matching_principle(
    transfer: &Transfer<Pending>,
) -> Result<Established<MatchingPrinciple>, ValidationError> {
    // Validation logic here
    Ok(Established::assert())
}
```

**Deliverables:**
- [ ] Validation functions in `gaap.rs`
- [ ] New `ValidationError::GaapViolation` variant
- [ ] Unit tests for each validation function

### Phase 4: Integration with Transfer Typestate (Days 7-8)

**Goal:** Integrate GAAP propositions into Transfer validation workflow.

**Options:**

**Option A: Composite Proof (Recommended)**
```rust
// All GAAP principles satisfied
pub type GaapCompliant = And<
    MatchingPrinciple,
    And<AccrualBasis, HistoricalCost>
>;

// Extend Transfer::validate() to return composite proof
impl Transfer<Pending> {
    pub async fn validate_gaap(
        self,
        pool: &AnyPool,
    ) -> Result<(Transfer<Validated>, Established<GaapCompliant>), ValidationError> {
        // Validate all GAAP principles
        let matching = validate_matching_principle(&self)?;
        let accrual = validate_accrual_basis(&self)?;
        let historical = validate_historical_cost(&self)?;

        let gaap_proof = both(matching, both(accrual, historical));

        // Existing validation
        let validated = self.validate(pool).await?;

        Ok((validated, gaap_proof))
    }
}
```

**Option B: Separate Validation**
```rust
impl Transfer<Validated> {
    pub fn verify_gaap_compliance(
        &self,
    ) -> Result<Established<GaapCompliant>, ValidationError> {
        // Post-validation GAAP check
    }
}
```

**Deliverables:**
- [ ] Integration method on `Transfer<T>`
- [ ] Composite proof type definitions
- [ ] Updated documentation
- [ ] Integration tests

### Phase 5: Testing & Documentation (Days 9-10)

**Goal:** Comprehensive testing and documentation updates.

**Testing tasks:**
1. Unit tests for each proposition validation
2. Integration tests for composite proofs
3. Test GAAP violation scenarios
4. Document failure cases

**Documentation tasks:**
1. Update `ledger/README.md` with GAAP section
2. Add GAAP examples to ledger tests
3. Create `GAAP_COMPLIANCE_GUIDE.md` user documentation
4. Update workspace README with GAAP mention

**Test structure:**
```rust
#[test]
fn test_matching_principle_valid() {
    let transfer = Transfer::new(...);
    let proof = validate_matching_principle(&transfer).unwrap();
    assert_eq!(std::mem::size_of_val(&proof), 0); // Zero-cost
}

#[test]
fn test_matching_principle_violation() {
    let transfer = Transfer::new(...); // Invalid state
    let err = validate_matching_principle(&transfer).unwrap_err();
    assert!(matches!(err, ValidationError::GaapViolation(_)));
}
```

**Deliverables:**
- [ ] Test suite (15-20 tests expected)
- [ ] Updated documentation
- [ ] User guide with examples
- [ ] Changelog entry

## Success Criteria

- [ ] All GAAP propositions have accurate ASC references
- [ ] Zero-cost proofs (verified with `size_of`)
- [ ] Validation functions establish proofs correctly
- [ ] Integration with Transfer typestate is clean
- [ ] Comprehensive test coverage
- [ ] Documentation is audit-quality
- [ ] No clippy warnings or errors
- [ ] All tests pass: `just test-package elicit_server`

## Future Extensions

**Potential additions:**
1. **Materiality thresholds** - Configurable significance levels
2. **Period-end adjustments** - Accrual entries, deferrals
3. **Financial statement generation** - Balance sheet, income statement
4. **Audit trail queries** - GAAP compliance reports
5. **Multi-currency support** - ASC 830 foreign currency
6. **Revenue recognition** - ASC 606 five-step model

## References

**Primary sources:**
- FASB Accounting Standards Codification (ASC)
- GAAP Conceptual Framework
- AICPA Professional Standards

**Key ASC topics:**
- ASC 606: Revenue Recognition
- ASC 820: Fair Value Measurement
- ASC 830: Foreign Currency
- ASC 842: Leases

**Implementation references:**
- `crates/elicit_ui/src/constraints.rs` - Constraint pattern
- `crates/elicit_server/src/ledger/README.md` - Ledger documentation
- `crates/elicitation/src/contracts.rs` - Prop trait, Established<P>

## Timeline

**Total estimated time:** 10 days

- Phase 1 (Research): 2 days
- Phase 2 (Propositions): 2 days
- Phase 3 (Validation): 2 days
- Phase 4 (Integration): 2 days
- Phase 5 (Testing/Docs): 2 days

## Notes

- Follow CLAUDE.md guidelines throughout
- Use `just check-all elicit_server` before each commit
- Add planning doc to PLANNING_INDEX.md
- Verify zero-cost proofs with `std::mem::size_of`
- Ensure all doc comments have proper ASC references
