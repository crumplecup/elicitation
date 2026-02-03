//! Creusot verification proofs for elicitation contract types.
//!
//! This crate contains pure Rust proofs that can be verified by Creusot.
//! It imports contract types from the main elicitation crate but avoids
//! async code that Creusot cannot handle.

#![forbid(unsafe_code)]

use creusot_std::prelude::*;

// Import contract types from elicitation
use elicitation::{
    // Bool types
    BoolTrue, BoolFalse,
    // Signed integer types
    I8Positive, I8NonNegative, I8NonZero,
    I16Positive, I16NonNegative, I16NonZero,
    I32Positive, I32NonNegative, I32NonZero,
    I64Positive, I64NonNegative, I64NonZero,
    I128Positive, I128NonNegative, I128NonZero,
    IsizePositive, IsizeNonNegative, IsizeNonZero,
    // Unsigned integer types
    U8Positive, U8NonZero,
    U16Positive, U16NonZero,
    U32Positive, U32NonZero,
    U64Positive, U64NonZero,
    U128Positive, U128NonZero,
    UsizePositive, UsizeNonZero,
    // Range types
    I8Range, I16Range, I32Range, I64Range, IsizeRange,
    U8Range, U16Range, U32Range, U64Range, UsizeRange,
    // Error type
    ValidationError,
};

pub mod bools;
pub mod integers;
