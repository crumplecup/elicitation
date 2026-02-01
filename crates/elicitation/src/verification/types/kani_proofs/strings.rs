//! Kani proofs for string contract types.

use crate::StringNonEmpty;

// ============================================================================
// String Contract Proofs
// ============================================================================

#[kani::proof]
#[kani::unwind(1)] // No loops, string checks
fn verify_string_non_empty() {
    // Test with concrete strings only (no symbolic construction)
    // Empty string case
    let empty = String::new();
    let result: Result<StringNonEmpty, _> = StringNonEmpty::new(empty);
    assert!(result.is_err(), "Construction rejects empty string");

    // Non-empty string case
    let non_empty = String::from("a");
    match StringNonEmpty::<4096>::new(non_empty.clone()) {
        Ok(contract) => {
            assert!(!non_empty.is_empty(), "StringNonEmpty invariant: not empty");
            assert!(contract.get().len() > 0, "get() returns non-empty");
        }
        Err(_) => {
            panic!("Non-empty string should be accepted");
        }
    }
}

// ============================================================================
// Bool Contract Proofs
