//! Kani proofs for bool contract types.

use crate::{BoolTrue, BoolFalse};

// Bool Contract Proofs
// ============================================================================

#[kani::proof]
fn verify_bool_true() {
    let value: bool = kani::any();
    
    match BoolTrue::new(value) {
        Ok(bool_true) => {
            assert!(value == true, "BoolTrue invariant: value is true");
            assert!(bool_true.get() == true, "get() returns true");
        }
        Err(_) => {
            assert!(value == false, "Construction rejects false");
        }
    }
}

#[kani::proof]
fn verify_bool_false() {
    let value: bool = kani::any();
    
    match BoolFalse::new(value) {
        Ok(bool_false) => {
            assert!(value == false, "BoolFalse invariant: value is false");
            let val: bool = bool_false.get();
            assert!(val == false, "get() returns false");
        }
        Err(_) => {
            assert!(value == true, "Construction rejects true");
        }
    }
}

// ============================================================================
// Char Contract Proofs
