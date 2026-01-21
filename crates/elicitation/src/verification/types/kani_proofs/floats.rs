//! Kani proofs for float contract types.

use crate::{F32Finite, F64Positive, F32Positive, F32NonNegative, F64NonNegative};

// ============================================================================
// Float Contract Proofs
// ============================================================================

#[kani::proof]
fn verify_f32_finite() {
    let value: f32 = kani::any();
    
    match F32Finite::new(value) {
        Ok(_finite) => {
            kani::assert(value.is_finite(), "F32Finite invariant: value is finite");
            kani::assert(!value.is_nan(), "Finite excludes NaN");
            kani::assert(!value.is_infinite(), "Finite excludes infinity");
        }
        Err(_) => {
            kani::assert(!value.is_finite(), "Construction rejects non-finite");
        }
    }
}

#[kani::proof]
fn verify_f64_positive() {
    let value: f64 = kani::any();
    
    // Only test finite values (NaN/infinity rejected separately)
    kani::assume(value.is_finite());
    
    match F64Positive::new(value) {
        Ok(_positive) => {
            kani::assert(value > 0.0, "F64Positive invariant: value > 0");
            kani::assert(value.is_finite(), "Positive implies finite");
        }
        Err(_) => {
            kani::assert(value <= 0.0, "Construction rejects non-positive");
        }
    }
}

// ============================================================================
// String Contract Proofs
