//! Kani proofs for string contract types.

use crate::StringNonEmpty;

// ============================================================================
// String Contract Proofs
// ============================================================================

#[kani::proof]
fn verify_string_non_empty() {
    // Reduced buffer size for Kani
    const MAX_LEN: usize = 2;
    
    // Empty string case
    let empty = String::new();
    let result = StringNonEmpty::<MAX_LEN>::new(empty);
    assert!(result.is_err(), "Construction rejects empty string");

    // Non-empty string case
    let non_empty = String::from("a");
    let result = StringNonEmpty::<MAX_LEN>::new(non_empty);
    
    // Verify construction doesn't panic and respects invariant
    if let Ok(contract) = result {
        assert!(!contract.is_empty(), "Non-empty invariant");
    }
}

// ============================================================================
// Bool Contract Proofs
