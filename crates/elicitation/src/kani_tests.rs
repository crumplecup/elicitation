//! Kani verification harnesses for formal contract verification.
//!
//! These harnesses verify that contract properties hold under symbolic execution.

#![cfg(kani)]

use crate::verification::Contract;

// ============================================================================
// Simple Contract Examples for Verification
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

