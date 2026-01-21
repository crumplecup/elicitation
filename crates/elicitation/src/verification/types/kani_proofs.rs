//! Kani proof harnesses for contract types.
//!
//! This module contains formal verification proofs using the Kani model checker.
//! Each proof harness verifies that contract invariants hold by construction.
//!
//! # Running Proofs
//!
//! ```bash
//! # Run all Kani proofs
//! cargo kani --package elicitation
//!
//! # Run specific proof
//! cargo kani --package elicitation --harness verify_i8_positive
//! ```
//!
//! # Proof Strategy
//!
//! For each contract type T, we prove:
//! 1. **Construction Safety**: `T::new(x)` succeeds ⟹ invariant holds
//! 2. **Invalid Rejection**: `T::new(x)` fails ⟹ invariant violated
//! 3. **Accessor Correctness**: `t.get()` returns validated value
//! 4. **Unwrap Correctness**: `t.into_inner()` returns validated value

#![cfg(kani)]

use crate::verification::types::*;

// ============================================================================
// Integer Contract Proofs
// ============================================================================

#[kani::proof]
fn verify_i8_positive() {
    let value: i8 = kani::any();
    
    match I8Positive::new(value) {
        Ok(positive) => {
            // If construction succeeds, value must be positive
            kani::assert(value > 0, "I8Positive invariant: value > 0");
            kani::assert(positive.get() > 0, "get() returns positive value");
            kani::assert(positive.into_inner() > 0, "into_inner() returns positive value");
        }
        Err(_) => {
            // If construction fails, value must be <= 0
            kani::assert(value <= 0, "Construction fails when value <= 0");
        }
    }
}

#[kani::proof]
fn verify_i8_non_negative() {
    let value: i8 = kani::any();
    
    match I8NonNegative::new(value) {
        Ok(non_neg) => {
            kani::assert(value >= 0, "I8NonNegative invariant: value >= 0");
            kani::assert(non_neg.get() >= 0, "get() returns non-negative value");
        }
        Err(_) => {
            kani::assert(value < 0, "Construction fails when value < 0");
        }
    }
}

#[kani::proof]
fn verify_u8_non_zero() {
    let value: u8 = kani::any();
    
    match U8NonZero::new(value) {
        Ok(non_zero) => {
            kani::assert(value != 0, "U8NonZero invariant: value != 0");
            kani::assert(non_zero.get() != 0, "get() returns non-zero value");
        }
        Err(_) => {
            kani::assert(value == 0, "Construction fails when value == 0");
        }
    }
}

#[kani::proof]
fn verify_i16_positive() {
    let value: i16 = kani::any();
    
    match I16Positive::new(value) {
        Ok(positive) => {
            kani::assert(value > 0, "I16Positive invariant: value > 0");
            kani::assert(positive.get() > 0, "get() preserves invariant");
        }
        Err(_) => {
            kani::assert(value <= 0, "Construction rejects non-positive");
        }
    }
}

#[kani::proof]
fn verify_u16_non_zero() {
    let value: u16 = kani::any();
    
    match U16NonZero::new(value) {
        Ok(non_zero) => {
            kani::assert(value != 0, "U16NonZero invariant");
            kani::assert(non_zero.get() != 0, "get() preserves invariant");
        }
        Err(_) => {
            kani::assert(value == 0, "Construction rejects zero");
        }
    }
}

// ============================================================================
// Float Contract Proofs
// ============================================================================

#[kani::proof]
fn verify_f32_finite() {
    let value: f32 = kani::any();
    
    match F32Finite::new(value) {
        Ok(finite) => {
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
        Ok(positive) => {
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
            kani::assert(!s.is_empty(), "StringNonEmpty invariant: not empty");
            kani::assert(non_empty.get().len() > 0, "get() returns non-empty");
        }
        Err(_) => {
            kani::assert(s.is_empty(), "Construction rejects empty string");
        }
    }
}

// ============================================================================
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
            kani::assert(numeric.get().is_numeric(), "get() preserves invariant");
        }
        Err(_) => {
            kani::assert(!value.is_numeric(), "Construction rejects non-numeric");
        }
    }
}

// ============================================================================
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
            kani::unreachable(); // Should never fail for Some
        }
    }
}

#[kani::proof]
fn verify_option_some_rejects_none() {
    let opt: Option<i32> = None;
    
    match OptionSome::new(opt) {
        Ok(_) => {
            kani::unreachable(); // Should never succeed for None
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
