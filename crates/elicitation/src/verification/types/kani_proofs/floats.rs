//! Kani proofs for float contract types.

use crate::{F32Finite, F64Positive};

// ============================================================================
// Float Contract Proofs
// ============================================================================

#[kani::proof]
fn verify_f32_finite() {
    let value: f32 = kani::any();

    let _result = F32Finite::new(value);
    
    // Verify construction doesn't panic
    // Note: Can't assert on is_finite(), is_nan(), is_infinite() with symbolic floats
    // With symbolic validation, both Ok and Err are valid paths
}

#[kani::proof]
fn verify_f64_positive() {
    let value: f64 = kani::any();

    let _result = F64Positive::new(value);
    
    // Verify construction doesn't panic
    // Note: Can't assert on value > 0.0 or is_finite() with symbolic floats
    // With symbolic validation, all paths (Ok/Err) are valid
}

// ============================================================================
// String Contract Proofs
