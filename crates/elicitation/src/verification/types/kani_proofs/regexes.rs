//! Kani proofs for Regex contract types.

#[cfg(feature = "regex")]
use crate::{RegexCaseInsensitive, RegexMultiline, RegexSetNonEmpty, RegexSetValid, RegexValid};

// ============================================================================
// Regex Contract Proofs - Wrapper Logic Only
// ============================================================================
//
// These proofs verify ONLY the wrapper logic, not regex compilation.
// We trust the regex crate's correctness and verify our contract enforcement.

#[cfg(feature = "regex")]
#[kani::proof]
fn verify_regex_valid_wrapper() {
    // Test wrapper logic: new() returns Result with correct variants
    let result = RegexValid::new(r"test_pattern");

    // Verify Result type behavior
    match result {
        Ok(_valid) => {
            // If Ok, wrapper successfully constructed
            // Cannot access internal Regex in kani mode (PhantomData)
        }
        Err(e) => {
            // If Err, correct error variant returned
            assert!(matches!(e, crate::ValidationError::RegexInvalid));
        }
    }
}

#[cfg(feature = "regex")]
#[kani::proof]
fn verify_regex_set_valid_wrapper() {
    // Test wrapper logic for set
    let result = RegexSetValid::new(&[r"pattern1", r"pattern2"]);

    match result {
        Ok(_set) => {
            // Wrapper constructed successfully
        }
        Err(e) => {
            assert!(matches!(e, crate::ValidationError::RegexInvalid));
        }
    }
}

#[cfg(feature = "regex")]
#[kani::proof]
fn verify_regex_case_insensitive_wrapper() {
    // Test wrapper construction
    let result = RegexCaseInsensitive::new(r"test");

    match result {
        Ok(_re) => {
            // Case-insensitive wrapper constructed
        }
        Err(e) => {
            assert!(matches!(e, crate::ValidationError::RegexInvalid));
        }
    }
}

#[cfg(feature = "regex")]
#[kani::proof]
fn verify_regex_multiline_wrapper() {
    // Test wrapper construction
    let result = RegexMultiline::new(r"^test$");

    match result {
        Ok(_re) => {
            // Multiline wrapper constructed
        }
        Err(e) => {
            assert!(matches!(e, crate::ValidationError::RegexInvalid));
        }
    }
}

#[cfg(feature = "regex")]
#[kani::proof]
fn verify_regex_set_non_empty_wrapper() {
    // Test non-empty constraint
    let single_result = RegexSetNonEmpty::new(&[r"pattern"]);

    match single_result {
        Ok(_set) => {
            // Non-empty set constructed
        }
        Err(e) => {
            // Could fail on regex invalid OR empty collection
            assert!(
                matches!(e, crate::ValidationError::RegexInvalid)
                    || matches!(e, crate::ValidationError::EmptyCollection)
            );
        }
    }

    // Test empty set - must return EmptyCollection error
    let empty_result = RegexSetNonEmpty::new::<&[&str], _>(&[]);
    assert!(empty_result.is_err(), "Empty set must be rejected");
    if let Err(e) = empty_result {
        assert!(matches!(e, crate::ValidationError::EmptyCollection));
    }
}
