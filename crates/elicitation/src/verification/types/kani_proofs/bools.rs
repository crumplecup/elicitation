//! Kani proofs for bool contract types.

use crate::BoolTrue;

// Bool Contract Proofs
// ============================================================================

#[kani::proof]
fn verify_bool_true() {
    let value: bool = kani::any();
    
    match BoolTrue::new(value) {
        Ok(bool_true) => {
            kani::assert(value == true, "BoolTrue invariant: value is true");
            kani::assert(bool_true.get() == true, "get() returns true");
        }
        Err(_) => {
            kani::assert(value == false, "Construction rejects false");
        }
    }
}

#[kani::proof]
fn verify_bool_false() {
    let value: bool = kani::any();
    
    match BoolFalse::new(value) {
        Ok(bool_false) => {
            kani::assert(value == false, "BoolFalse invariant: value is false");
            kani::assert(bool_false.get() == false, "get() returns false");
        }
        Err(_) => {
            kani::assert(value == true, "Construction rejects true");
        }
    }
}

// ============================================================================
// Char Contract Proofs
