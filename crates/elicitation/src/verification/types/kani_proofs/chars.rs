//! Kani proofs for char contract types.

use crate::{CharAlphabetic, CharAlphanumeric, CharNumeric};

// Char Contract Proofs
// ============================================================================

#[kani::proof]
fn verify_char_alphabetic() {
    let value: char = kani::any();
    
    match CharAlphabetic::new(value) {
        Ok(alphabetic) => {
            kani::assert(value.is_alphabetic(), "CharAlphabetic invariant");
            kani::assert(alphabetic.get().is_alphabetic(), "get() preserves invariant");
        }
        Err(_) => {
            kani::assert(!value.is_alphabetic(), "Construction rejects non-alphabetic");
        }
    }
}

#[kani::proof]
fn verify_char_numeric() {
    let value: char = kani::any();
    
    match CharNumeric::new(value) {
        Ok(numeric) => {
            kani::assert(value.is_numeric(), "CharNumeric invariant");
            let val: char = numeric.get();
            kani::assert(val.is_numeric(), "get() preserves invariant");
        }
        Err(_) => {
            kani::assert(!value.is_numeric(), "Construction rejects non-numeric");
        }
    }
}

// ============================================================================
// Duration Contract Proofs
