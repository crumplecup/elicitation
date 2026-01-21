//! Kani proofs for string contract types.

use crate::StringNonEmpty;

// ============================================================================
// String Contract Proofs
// ============================================================================

#[kani::proof]
fn verify_string_non_empty() {
    // Kani can't handle arbitrary strings, so we test with bounded strings
    let len: usize = kani::any();
    kani::assume(len < 10); // Bound the string length
    
    let mut s = String::new();
    for _ in 0..len {
        s.push('a');
    }
    
    match StringNonEmpty::new(s.clone()) {
        Ok(non_empty) => {
            assert!(!s.is_empty(), "StringNonEmpty invariant: not empty");
            assert!(non_empty.get().len() > 0, "get() returns non-empty");
        }
        Err(_) => {
            assert!(s.is_empty(), "Construction rejects empty string");
        }
    }
}

// ============================================================================
// Bool Contract Proofs
