//! Kani proofs for float contract types.

use crate::{F32Finite, F64Positive};

// ============================================================================
// Float Contract Proofs
// ============================================================================

#[kani::proof]
fn verify_f32_finite() {
    let value: f32 = kani::any();

    match F32Finite::new(value) {
        Ok(_finite) => {
            assert!(value.is_finite(), "F32Finite invariant: value is finite");
            assert!(!value.is_nan(), "Finite excludes NaN");
            assert!(!value.is_infinite(), "Finite excludes infinity");
        }
        Err(_) => {
            assert!(!value.is_finite(), "Construction rejects non-finite");
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
            assert!(value > 0.0, "F64Positive invariant: value > 0");
            assert!(value.is_finite(), "Positive implies finite");
        }
        Err(_) => {
            assert!(value <= 0.0, "Construction rejects non-positive");
        }
    }
}

// ============================================================================
// String Contract Proofs
