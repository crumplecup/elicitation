//! Kani proofs for char contract types.

use crate::{CharAlphabetic, CharNumeric, ValidationError};

// Char Contract Proofs
// ============================================================================

// Proof: Wrapper logic for alphabetic chars
#[kani::proof]
fn verify_char_alphabetic_accepts() {
    let value: char = kani::any();

    // Trust stdlib: if construction succeeds, the char is alphabetic
    let result = CharAlphabetic::new(value);

    if let Ok(alphabetic) = result {
        // Verify: get() returns the original value
        assert_eq!(alphabetic.get(), value);
        assert_eq!(alphabetic.into_inner(), value);
    }
}

// Proof: Wrapper logic for non-alphabetic chars
#[kani::proof]
fn verify_char_alphabetic_rejects() {
    let value: char = kani::any();

    let result = CharAlphabetic::new(value);

    // Verify: if it fails, we get the correct error
    if let Err(e) = result {
        match e {
            ValidationError::NotAlphabetic(_) => {
                // Correct error type
            }
            _ => panic!("Wrong error type"),
        }
    }
}

// Proof: Wrapper logic for numeric chars
#[kani::proof]
fn verify_char_numeric_accepts() {
    let value: char = kani::any();

    let result = CharNumeric::new(value);

    if let Ok(numeric) = result {
        assert_eq!(numeric.get(), value);
        assert_eq!(numeric.into_inner(), value);
    }
}

// Proof: Wrapper logic for non-numeric chars
#[kani::proof]
fn verify_char_numeric_rejects() {
    let value: char = kani::any();

    let result = CharNumeric::new(value);

    if let Err(e) = result {
        match e {
            ValidationError::NotNumeric(_) => {
                // Correct error type
            }
            _ => panic!("Wrong error type"),
        }
    }
}

// ============================================================================
// Duration Contract Proofs
