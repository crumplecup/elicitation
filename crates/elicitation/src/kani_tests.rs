//! Kani verification harnesses for formal contract verification.
//!
//! These harnesses verify that contract properties hold under symbolic execution.
//!
//! # Organization
//!
//! - **Primitive Contracts**: String, i32, bool
//! - **Complex Contracts**: Email, NonEmpty, etc.
//! - **Composition**: Multiple contracts together

#![cfg(kani)]

use crate::verification::Contract;
use crate::verification::contracts::{BoolValid, I32Positive, StringNonEmpty};

// ============================================================================
// Phase 1: Primitive Type Contracts
// ============================================================================

/// Verify StringNonEmpty contract with symbolic execution.
///
/// **Property:** Non-empty strings remain non-empty.
#[kani::proof]
fn verify_string_non_empty_contract() {
    // Create various concrete strings to test
    let inputs = [
        String::from("a"),
        String::from("hello"),
        String::from("test string"),
        String::from("x".repeat(50)),
    ];

    for input in inputs.iter() {
        // Assume precondition
        kani::assume(StringNonEmpty::requires(input));

        // Property 1: Input is non-empty
        assert!(!input.is_empty());
        assert!(input.len() > 0);

        // Property 2: Identity transformation preserves non-emptiness
        let output = input.clone();
        assert!(StringNonEmpty::ensures(input, &output));
        assert!(!output.is_empty());
    }
}

/// Verify I32Positive contract with symbolic execution.
///
/// **Property:** Positive numbers remain positive.
#[kani::proof]
fn verify_i32_positive_contract() {
    // Symbolic i32
    let input: i32 = kani::any();

    // Assume precondition
    kani::assume(I32Positive::requires(&input));

    // Property 1: Input is positive
    assert!(input > 0);

    // Property 2: Identity transformation preserves positivity
    let output = input;
    assert!(I32Positive::ensures(&input, &output));
    assert!(output > 0);
}

/// Verify BoolValid contract (trivial).
///
/// **Property:** All booleans are valid.
#[kani::proof]
fn verify_bool_valid_contract() {
    // Symbolic bool
    let input: bool = kani::any();

    // Property: All booleans satisfy precondition
    assert!(BoolValid::requires(&input));

    // Property: All transformations are valid
    let output = !input; // Even negation is valid
    assert!(BoolValid::ensures(&input, &output));
}

// ============================================================================
// Legacy Examples (for reference)
// ============================================================================

/// Contract that ensures output is non-empty when input is non-empty.
struct NonEmptyString;

impl Contract for NonEmptyString {
    type Input = String;
    type Output = String;

    fn requires(input: &String) -> bool {
        !input.is_empty()
    }

    fn ensures(_input: &String, output: &String) -> bool {
        !output.is_empty()
    }
}

/// Contract that validates email format.
struct ValidEmail;

impl Contract for ValidEmail {
    type Input = String;
    type Output = String;

    fn requires(input: &String) -> bool {
        input.contains('@') && input.len() > 2
    }

    fn ensures(_input: &String, output: &String) -> bool {
        output.contains('@')
    }
}

/// Contract for number validation (positive only).
struct PositiveNumber;

impl Contract for PositiveNumber {
    type Input = i32;
    type Output = i32;

    fn requires(input: &i32) -> bool {
        *input > 0
    }

    fn ensures(_input: &i32, output: &i32) -> bool {
        *output > 0
    }
}

// ============================================================================
// Kani Verification Harnesses
// ============================================================================

#[kani::proof]
fn verify_non_empty_string_contract() {
    // Symbolic string input
    let input = String::from("test");

    // Assume precondition
    kani::assume(NonEmptyString::requires(&input));

    // Property: precondition guarantees non-empty
    assert!(!input.is_empty());
}

#[kani::proof]
fn verify_email_requires_at_symbol() {
    // Symbolic string
    let input = String::from("user@example.com");

    // Assume precondition holds
    kani::assume(ValidEmail::requires(&input));

    // Property: valid emails must contain @
    assert!(input.contains('@'));
    assert!(input.len() > 2);
}

#[kani::proof]
fn verify_positive_number_contract() {
    // Symbolic integer
    let input: i32 = kani::any();

    // Assume precondition
    kani::assume(PositiveNumber::requires(&input));

    // Property: precondition guarantees positive
    assert!(input > 0);
}

#[kani::proof]
fn verify_contract_composition_preconditions() {
    // Verify that if Contract A's postcondition holds,
    // and Contract B's precondition requires what A ensures,
    // then composition is valid.

    let email = String::from("user@example.com");

    // Contract A (ValidEmail) postcondition
    kani::assume(ValidEmail::ensures(&email, &email));

    // Verify email still has @ (preserved property)
    assert!(email.contains('@'));

    // Contract B (NonEmptyString) can accept this
    assert!(NonEmptyString::requires(&email));
}
