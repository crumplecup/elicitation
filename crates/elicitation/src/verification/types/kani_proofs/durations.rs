//! Kani proofs for duration contract types.

use crate::{DurationPositive, DurationNonZero};
use std::time::Duration;

// Duration Contract Proofs
// ============================================================================

#[kani::proof]
fn verify_duration_positive() {
    let secs: u64 = kani::any();
    let nanos: u32 = kani::any();
    kani::assume(nanos < 1_000_000_000); // Valid nanos range
    
    let duration = std::time::Duration::new(secs, nanos);
    
    match DurationPositive::new(duration) {
        Ok(positive) => {
            kani::assert(duration.as_nanos() > 0, "DurationPositive invariant");
            kani::assert(positive.get().as_nanos() > 0, "get() preserves invariant");
        }
        Err(_) => {
            kani::assert(duration.as_nanos() == 0, "Construction rejects zero duration");
        }
    }
}

// ============================================================================
// Compositional Proofs (Tuples)
// ============================================================================

#[kani::proof]
fn verify_tuple2_composition() {
    // If both elements are valid contracts, tuple is valid
    let v1: i8 = kani::any();
    let v2: i8 = kani::any();
    
    kani::assume(v1 > 0); // Assume first is positive
    kani::assume(v2 > 0); // Assume second is positive
    
    let first = I8Positive::new(v1).unwrap();
    let second = I8Positive::new(v2).unwrap();
    
    let tuple = Tuple2::new(first, second);
    
    // Both elements remain positive after tuple construction
    kani::assert(tuple.first().get() > 0, "First element preserves contract");
    kani::assert(tuple.second().get() > 0, "Second element preserves contract");
}

// ============================================================================
// Collection Proofs
// ============================================================================

#[kani::proof]
fn verify_option_some() {
    let value: i32 = kani::any();
    let opt = Some(value);
    
    match OptionSome::new(opt) {
        Ok(some) => {
            kani::assert(*some.get() == value, "OptionSome unwraps correctly");
        }
        Err(_) => {
            unreachable!("OptionSome::new(Some) should never fail");
        }
    }
}

#[kani::proof]
fn verify_option_some_rejects_none() {
    let opt: Option<i32> = None;
    
    match OptionSome::new(opt) {
        Ok(_) => {
            unreachable!("OptionSome::new(None) should never succeed");
        }
        Err(_) => {
            // Expected: construction rejects None
        }
    }
}

// ============================================================================
// Trenchcoat Pattern Proof
// ============================================================================

/// Master proof: The trenchcoat pattern preserves type safety.
///
/// Proves that wrapping a value in a contract type and then unwrapping
/// it yields a validated value.
#[kani::proof]
fn verify_trenchcoat_pattern() {
    let value: i8 = kani::any();
    
    // Assume we have a positive value
    kani::assume(value > 0);
    
    // STEP 1: Put on the trenchcoat (wrap in contract type)
    let wrapped = I8Positive::new(value).unwrap();
    
    // STEP 2: Contract guarantees hold
    kani::assert(wrapped.get() > 0, "Contract guarantees positive");
    
    // STEP 3: Take off the trenchcoat (unwrap)
    let unwrapped = wrapped.into_inner();
    
    // STEP 4: Unwrapped value still satisfies contract
    kani::assert(unwrapped > 0, "Unwrapped value remains positive");
    kani::assert(unwrapped == value, "Unwrap preserves value identity");
}

// ============================================================================
// Phase 1: Complete Primitive Type Proofs
// ============================================================================

// ----------------------------------------------------------------------------
// Float Proofs: NonNegative variants
// ----------------------------------------------------------------------------

#[kani::proof]
fn verify_f32_non_negative() {
    let value: f32 = kani::any();
    kani::assume(value.is_finite());
    
    match F32NonNegative::new(value) {
        Ok(_non_neg) => {
            kani::assert(value >= 0.0, "F32NonNegative invariant: value >= 0");
            kani::assert(value.is_finite(), "NonNegative implies finite");
        }
        Err(_) => {
            kani::assert(value < 0.0, "Construction rejects negative");
        }
    }
}

#[kani::proof]
fn verify_f64_non_negative() {
    let value: f64 = kani::any();
    kani::assume(value.is_finite());
    
    match F64NonNegative::new(value) {
        Ok(_non_neg) => {
            kani::assert(value >= 0.0, "F64NonNegative invariant: value >= 0");
            kani::assert(value.is_finite(), "NonNegative implies finite");
        }
        Err(_) => {
            kani::assert(value < 0.0, "Construction rejects negative");
        }
    }
}

#[kani::proof]
fn verify_f32_positive() {
    let value: f32 = kani::any();
    kani::assume(value.is_finite());
    
    match F32Positive::new(value) {
        Ok(_positive) => {
            kani::assert(value > 0.0, "F32Positive invariant: value > 0");
            kani::assert(value.is_finite(), "Positive implies finite");
        }
        Err(_) => {
            kani::assert(value <= 0.0, "Construction rejects non-positive");
        }
    }
}

// ----------------------------------------------------------------------------
// Char Proofs: Complete coverage
// ----------------------------------------------------------------------------

#[kani::proof]
fn verify_char_alphanumeric() {
    let value: char = kani::any();
    
    match CharAlphanumeric::new(value) {
        Ok(alphanumeric) => {
            kani::assert(value.is_alphanumeric(), "CharAlphanumeric invariant");
            kani::assert(alphanumeric.get().is_alphanumeric(), "Accessor preserves");
            kani::assert(alphanumeric.into_inner().is_alphanumeric(), "Unwrap preserves");
        }
        Err(_) => {
            kani::assert(!value.is_alphanumeric(), "Construction rejects non-alphanumeric");
        }
    }
}

// ----------------------------------------------------------------------------
// Integer Proofs: More sizes (i32, i64, i128, u32, u64, u128, isize, usize)
// ----------------------------------------------------------------------------

// Note: Range types use const generics, harder to prove exhaustively
// Focus on Positive/NonNegative/NonZero variants for remaining sizes


// ============================================================================
// Phase 2: Specialized Type Proofs
// ============================================================================

// ----------------------------------------------------------------------------
// Network Proofs
