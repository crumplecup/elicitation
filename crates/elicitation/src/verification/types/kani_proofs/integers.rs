//! Kani proofs for integer contract types.

use crate::{
    I8Positive, U8Positive, I16Positive, U16Positive,
    I8NonNegative, U8NonZero, U16NonZero,
};

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
            let val: i8 = non_neg.get();
            kani::assert(val >= 0, "get() returns non-negative value");
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
            let val: u8 = non_zero.get();
            kani::assert(val != 0, "get() returns non-zero value");
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
            let val: u16 = non_zero.get();
            kani::assert(val != 0, "get() preserves invariant");
        }
        Err(_) => {
            kani::assert(value == 0, "Construction rejects zero");
        }
    }
}

// ============================================================================
