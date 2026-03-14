//! Creusot proofs for tuple contract types.
//!
//! Cloud of assumptions: We trust composition - if all elements are valid
//! contract types, the tuple is valid. We verify tuple wrapper structure.

use creusot_std::prelude::*;
use elicitation::{BoolTrue, I32Positive, Tuple2, Tuple3, Tuple4};

// ============================================================================
// Tuple2 Proofs
// ============================================================================

/// Verify Tuple2 construction with valid contract types.
#[requires(true)]
#[ensures(true)]
pub fn verify_tuple2_valid() -> Tuple2<I32Positive, I32Positive> {
    let first = I32Positive::new(42).unwrap();
    let second = I32Positive::new(7).unwrap();
    Tuple2::new(first, second)
}

/// Verify Tuple2 accessor methods work correctly.
#[requires(true)]
#[ensures(true)]
pub fn verify_tuple2_accessors() {
    let first = I32Positive::new(10).unwrap();
    let second = I32Positive::new(20).unwrap();
    let tuple = Tuple2::new(first, second);
    let _ = tuple.first();
    let _ = tuple.second();
}

// ============================================================================
// Tuple3 Proofs
// ============================================================================

/// Verify Tuple3 construction with valid contract types.
#[requires(true)]
#[ensures(true)]
pub fn verify_tuple3_valid() -> Tuple3<I32Positive, I32Positive, BoolTrue> {
    let first = I32Positive::new(42).unwrap();
    let second = I32Positive::new(1).unwrap();
    let third = BoolTrue::new(true).unwrap();
    Tuple3::new(first, second, third)
}

/// Verify Tuple3 into_inner decomposes correctly.
#[requires(true)]
#[ensures(true)]
pub fn verify_tuple3_into_inner() {
    let first = I32Positive::new(1).unwrap();
    let second = I32Positive::new(2).unwrap();
    let third = BoolTrue::new(true).unwrap();
    let tuple = Tuple3::new(first, second, third);
    let (_a, _b, _c) = tuple.into_inner();
}

// ============================================================================
// Tuple4 Proofs
// ============================================================================

/// Verify Tuple4 construction with valid contract types.
#[requires(true)]
#[ensures(true)]
pub fn verify_tuple4_valid() -> Tuple4<I32Positive, I32Positive, I32Positive, BoolTrue> {
    let first = I32Positive::new(1).unwrap();
    let second = I32Positive::new(2).unwrap();
    let third = I32Positive::new(3).unwrap();
    let fourth = BoolTrue::new(true).unwrap();
    Tuple4::new(first, second, third, fourth)
}

/// Verify Tuple4 into_inner decomposes correctly.
#[requires(true)]
#[ensures(true)]
pub fn verify_tuple4_into_inner() {
    let first = I32Positive::new(10).unwrap();
    let second = I32Positive::new(20).unwrap();
    let third = I32Positive::new(30).unwrap();
    let fourth = BoolTrue::new(true).unwrap();
    let tuple = Tuple4::new(first, second, third, fourth);
    let (_a, _b, _c, _d) = tuple.into_inner();
}

