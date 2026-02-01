//! Prusti proofs for integer contract types.

#![cfg(feature = "verify-prusti")]
#![allow(unused_imports)]

use crate::*;
use prusti_contracts::*;

// Integer Contract Proofs
// ============================================================================

// ----------------------------------------------------------------------------
// I8 Contracts
// ----------------------------------------------------------------------------

#[cfg(feature = "verify-prusti")]
use prusti_contracts::*;

/// Prove that I8Positive construction succeeds for positive values.
#[cfg(feature = "verify-prusti")]
#[requires(value > 0)]
#[ensures(result.is_ok())]
pub fn verify_i8_positive_valid(value: i8) -> Result<I8Positive, ValidationError> {
    I8Positive::new(value)
}

/// Prove that I8Positive construction fails for non-positive values.
#[cfg(feature = "verify-prusti")]
#[requires(value <= 0)]
#[ensures(result.is_err())]
pub fn verify_i8_positive_invalid(value: i8) -> Result<I8Positive, ValidationError> {
    I8Positive::new(value)
}

/// Prove that I8Positive::get() returns the wrapped value.
#[cfg(feature = "verify-prusti")]
#[requires(value > 0)]
#[ensures(match result {
    Ok(ref wrapped) => wrapped.get() == value,
    Err(_) => false,
})]
pub fn verify_i8_positive_accessor(value: i8) -> Result<I8Positive, ValidationError> {
    I8Positive::new(value)
}

/// Prove that I8NonNegative construction succeeds for non-negative values.
#[cfg(feature = "verify-prusti")]
#[requires(value >= 0)]
#[ensures(result.is_ok())]
pub fn verify_i8_non_negative_valid(value: i8) -> Result<I8NonNegative, ValidationError> {
    I8NonNegative::new(value)
}

/// Prove that I8NonNegative construction fails for negative values.
#[cfg(feature = "verify-prusti")]
#[requires(value < 0)]
#[ensures(result.is_err())]
pub fn verify_i8_non_negative_invalid(value: i8) -> Result<I8NonNegative, ValidationError> {
    I8NonNegative::new(value)
}

/// Prove that I8NonZero construction succeeds for non-zero values.
#[cfg(feature = "verify-prusti")]
#[requires(value != 0)]
#[ensures(result.is_ok())]
pub fn verify_i8_non_zero_valid(value: i8) -> Result<I8NonZero, ValidationError> {
    I8NonZero::new(value)
}

/// Prove that I8NonZero construction fails for zero.
#[cfg(feature = "verify-prusti")]
#[requires(value == 0)]
#[ensures(result.is_err())]
pub fn verify_i8_non_zero_invalid(value: i8) -> Result<I8NonZero, ValidationError> {
    I8NonZero::new(value)
}

// ----------------------------------------------------------------------------
// U8 Contracts
// ----------------------------------------------------------------------------

/// Prove that U8Positive construction succeeds for positive values.
#[cfg(feature = "verify-prusti")]
#[requires(value > 0)]
#[ensures(result.is_ok())]
pub fn verify_u8_positive_valid(value: u8) -> Result<U8Positive, ValidationError> {
    U8Positive::new(value)
}

/// Prove that U8Positive construction fails for zero.
#[cfg(feature = "verify-prusti")]
#[requires(value == 0)]
#[ensures(result.is_err())]
pub fn verify_u8_positive_invalid(value: u8) -> Result<U8Positive, ValidationError> {
    U8Positive::new(value)
}

/// Prove that U8NonZero construction succeeds for non-zero values.
#[cfg(feature = "verify-prusti")]
#[requires(value != 0)]
#[ensures(result.is_ok())]
pub fn verify_u8_non_zero_valid(value: u8) -> Result<U8NonZero, ValidationError> {
    U8NonZero::new(value)
}

/// Prove that U8NonZero construction fails for zero.
#[cfg(feature = "verify-prusti")]
#[requires(value == 0)]
#[ensures(result.is_err())]
pub fn verify_u8_non_zero_invalid(value: u8) -> Result<U8NonZero, ValidationError> {
    U8NonZero::new(value)
}

// ----------------------------------------------------------------------------
// I16 Contracts
// ----------------------------------------------------------------------------

/// Prove that I16Positive construction succeeds for positive values.
#[cfg(feature = "verify-prusti")]
#[requires(value > 0)]
#[ensures(result.is_ok())]
pub fn verify_i16_positive_valid(value: i16) -> Result<I16Positive, ValidationError> {
    I16Positive::new(value)
}

/// Prove that I16NonNegative construction succeeds for non-negative values.
#[cfg(feature = "verify-prusti")]
#[requires(value >= 0)]
#[ensures(result.is_ok())]
pub fn verify_i16_non_negative_valid(value: i16) -> Result<I16NonNegative, ValidationError> {
    I16NonNegative::new(value)
}

/// Prove that I16NonZero construction succeeds for non-zero values.
#[cfg(feature = "verify-prusti")]
#[requires(value != 0)]
#[ensures(result.is_ok())]
pub fn verify_i16_non_zero_valid(value: i16) -> Result<I16NonZero, ValidationError> {
    I16NonZero::new(value)
}

// ----------------------------------------------------------------------------
// U16 Contracts
// ----------------------------------------------------------------------------

/// Prove that U16Positive construction succeeds for positive values.
#[cfg(feature = "verify-prusti")]
#[requires(value > 0)]
#[ensures(result.is_ok())]
pub fn verify_u16_positive_valid(value: u16) -> Result<U16Positive, ValidationError> {
    U16Positive::new(value)
}

/// Prove that U16NonZero construction succeeds for non-zero values.
#[cfg(feature = "verify-prusti")]
#[requires(value != 0)]
#[ensures(result.is_ok())]
pub fn verify_u16_non_zero_valid(value: u16) -> Result<U16NonZero, ValidationError> {
    U16NonZero::new(value)
}

// ----------------------------------------------------------------------------
// I32 Contracts
// ----------------------------------------------------------------------------

/// Prove that I32Positive construction succeeds for positive values.
#[cfg(feature = "verify-prusti")]
#[requires(value > 0)]
#[ensures(result.is_ok())]
pub fn verify_i32_positive_valid(value: i32) -> Result<I32Positive, ValidationError> {
    I32Positive::new(value)
}

/// Prove that I32NonNegative construction succeeds for non-negative values.
#[cfg(feature = "verify-prusti")]
#[requires(value >= 0)]
#[ensures(result.is_ok())]
pub fn verify_i32_non_negative_valid(value: i32) -> Result<I32NonNegative, ValidationError> {
    I32NonNegative::new(value)
}

/// Prove that I32NonZero construction succeeds for non-zero values.
#[cfg(feature = "verify-prusti")]
#[requires(value != 0)]
#[ensures(result.is_ok())]
pub fn verify_i32_non_zero_valid(value: i32) -> Result<I32NonZero, ValidationError> {
    I32NonZero::new(value)
}

// ----------------------------------------------------------------------------
// U32 Contracts
// ----------------------------------------------------------------------------

/// Prove that U32Positive construction succeeds for positive values.
#[cfg(feature = "verify-prusti")]
#[requires(value > 0)]
#[ensures(result.is_ok())]
pub fn verify_u32_positive_valid(value: u32) -> Result<U32Positive, ValidationError> {
    U32Positive::new(value)
}

/// Prove that U32NonZero construction succeeds for non-zero values.
#[cfg(feature = "verify-prusti")]
#[requires(value != 0)]
#[ensures(result.is_ok())]
pub fn verify_u32_non_zero_valid(value: u32) -> Result<U32NonZero, ValidationError> {
    U32NonZero::new(value)
}

// ----------------------------------------------------------------------------
// I64 Contracts
// ----------------------------------------------------------------------------

/// Prove that I64Positive construction succeeds for positive values.
#[cfg(feature = "verify-prusti")]
#[requires(value > 0)]
#[ensures(result.is_ok())]
pub fn verify_i64_positive_valid(value: i64) -> Result<I64Positive, ValidationError> {
    I64Positive::new(value)
}

/// Prove that I64NonNegative construction succeeds for non-negative values.
#[cfg(feature = "verify-prusti")]
#[requires(value >= 0)]
#[ensures(result.is_ok())]
pub fn verify_i64_non_negative_valid(value: i64) -> Result<I64NonNegative, ValidationError> {
    I64NonNegative::new(value)
}

/// Prove that I64NonZero construction succeeds for non-zero values.
#[cfg(feature = "verify-prusti")]
#[requires(value != 0)]
#[ensures(result.is_ok())]
pub fn verify_i64_non_zero_valid(value: i64) -> Result<I64NonZero, ValidationError> {
    I64NonZero::new(value)
}

// ----------------------------------------------------------------------------
// U64 Contracts
// ----------------------------------------------------------------------------

/// Prove that U64Positive construction succeeds for positive values.
#[cfg(feature = "verify-prusti")]
#[requires(value > 0)]
#[ensures(result.is_ok())]
pub fn verify_u64_positive_valid(value: u64) -> Result<U64Positive, ValidationError> {
    U64Positive::new(value)
}

/// Prove that U64NonZero construction succeeds for non-zero values.
#[cfg(feature = "verify-prusti")]
#[requires(value != 0)]
#[ensures(result.is_ok())]
pub fn verify_u64_non_zero_valid(value: u64) -> Result<U64NonZero, ValidationError> {
    U64NonZero::new(value)
}

// ----------------------------------------------------------------------------
// I128 Contracts
// ----------------------------------------------------------------------------

/// Prove that I128Positive construction succeeds for positive values.
#[cfg(feature = "verify-prusti")]
#[requires(value > 0)]
#[ensures(result.is_ok())]
pub fn verify_i128_positive_valid(value: i128) -> Result<I128Positive, ValidationError> {
    I128Positive::new(value)
}

/// Prove that I128NonNegative construction succeeds for non-negative values.
#[cfg(feature = "verify-prusti")]
#[requires(value >= 0)]
#[ensures(result.is_ok())]
pub fn verify_i128_non_negative_valid(value: i128) -> Result<I128NonNegative, ValidationError> {
    I128NonNegative::new(value)
}

/// Prove that I128NonZero construction succeeds for non-zero values.
#[cfg(feature = "verify-prusti")]
#[requires(value != 0)]
#[ensures(result.is_ok())]
pub fn verify_i128_non_zero_valid(value: i128) -> Result<I128NonZero, ValidationError> {
    I128NonZero::new(value)
}

// ----------------------------------------------------------------------------
// U128 Contracts
// ----------------------------------------------------------------------------

/// Prove that U128Positive construction succeeds for positive values.
#[cfg(feature = "verify-prusti")]
#[requires(value > 0)]
#[ensures(result.is_ok())]
pub fn verify_u128_positive_valid(value: u128) -> Result<U128Positive, ValidationError> {
    U128Positive::new(value)
}

/// Prove that U128NonZero construction succeeds for non-zero values.
#[cfg(feature = "verify-prusti")]
#[requires(value != 0)]
#[ensures(result.is_ok())]
pub fn verify_u128_non_zero_valid(value: u128) -> Result<U128NonZero, ValidationError> {
    U128NonZero::new(value)
}

// ----------------------------------------------------------------------------
// Isize Contracts
// ----------------------------------------------------------------------------

/// Prove that IsizePositive construction succeeds for positive values.
#[cfg(feature = "verify-prusti")]
#[requires(value > 0)]
#[ensures(result.is_ok())]
pub fn verify_isize_positive_valid(value: isize) -> Result<IsizePositive, ValidationError> {
    IsizePositive::new(value)
}

/// Prove that IsizeNonNegative construction succeeds for non-negative values.
#[cfg(feature = "verify-prusti")]
#[requires(value >= 0)]
#[ensures(result.is_ok())]
pub fn verify_isize_non_negative_valid(value: isize) -> Result<IsizeNonNegative, ValidationError> {
    IsizeNonNegative::new(value)
}

/// Prove that IsizeNonZero construction succeeds for non-zero values.
#[cfg(feature = "verify-prusti")]
#[requires(value != 0)]
#[ensures(result.is_ok())]
pub fn verify_isize_non_zero_valid(value: isize) -> Result<IsizeNonZero, ValidationError> {
    IsizeNonZero::new(value)
}

// ----------------------------------------------------------------------------
// Usize Contracts
// ----------------------------------------------------------------------------

/// Prove that UsizePositive construction succeeds for positive values.
#[cfg(feature = "verify-prusti")]
#[requires(value > 0)]
#[ensures(result.is_ok())]
pub fn verify_usize_positive_valid(value: usize) -> Result<UsizePositive, ValidationError> {
    UsizePositive::new(value)
}

/// Prove that UsizeNonZero construction succeeds for non-zero values.
#[cfg(feature = "verify-prusti")]
#[requires(value != 0)]
#[ensures(result.is_ok())]
pub fn verify_usize_non_zero_valid(value: usize) -> Result<UsizeNonZero, ValidationError> {
    UsizeNonZero::new(value)
}

// ----------------------------------------------------------------------------
// Range Type Contracts
// ----------------------------------------------------------------------------

/// Prove that I8Range construction succeeds for values in range.
#[cfg(feature = "verify-prusti")]
#[requires(-10 <= value && value <= 10)]
#[ensures(result.is_ok())]
pub fn verify_i8_range_valid(value: i8) -> Result<I8Range<-10, 10>, ValidationError> {
    I8Range::<-10, 10>::new(value)
}

/// Prove that I8Range construction fails for values out of range.
#[cfg(feature = "verify-prusti")]
#[requires(value < -10 || value > 10)]
#[ensures(result.is_err())]
pub fn verify_i8_range_invalid(value: i8) -> Result<I8Range<-10, 10>, ValidationError> {
    I8Range::<-10, 10>::new(value)
}

/// Prove that U8Range construction succeeds for values in range.
#[cfg(feature = "verify-prusti")]
#[requires(10 <= value && value <= 250)]
#[ensures(result.is_ok())]
pub fn verify_u8_range_valid(value: u8) -> Result<U8Range<10, 250>, ValidationError> {
    U8Range::<10, 250>::new(value)
}

/// Prove that I16Range construction succeeds for values in range.
#[cfg(feature = "verify-prusti")]
#[requires(-1000 <= value && value <= 1000)]
#[ensures(result.is_ok())]
pub fn verify_i16_range_valid(value: i16) -> Result<I16Range<-1000, 1000>, ValidationError> {
    I16Range::<-1000, 1000>::new(value)
}

/// Prove that U16Range construction succeeds for values in range.
#[cfg(feature = "verify-prusti")]
#[requires(100 <= value && value <= 65000)]
#[ensures(result.is_ok())]
pub fn verify_u16_range_valid(value: u16) -> Result<U16Range<100, 65000>, ValidationError> {
    U16Range::<100, 65000>::new(value)
}

/// Prove that I32Range construction succeeds for values in range.
#[cfg(feature = "verify-prusti")]
#[requires(-100000 <= value && value <= 100000)]
#[ensures(result.is_ok())]
pub fn verify_i32_range_valid(value: i32) -> Result<I32Range<-100000, 100000>, ValidationError> {
    I32Range::<-100000, 100000>::new(value)
}

/// Prove that U32Range construction succeeds for values in range.
#[cfg(feature = "verify-prusti")]
#[requires(1000 <= value && value <= 4000000000)]
#[ensures(result.is_ok())]
pub fn verify_u32_range_valid(value: u32) -> Result<U32Range<1000, 4000000000>, ValidationError> {
    U32Range::<1000, 4000000000>::new(value)
}

/// Prove that I64Range construction succeeds for values in range.
#[cfg(feature = "verify-prusti")]
#[requires(-1000000000 <= value && value <= 1000000000)]
#[ensures(result.is_ok())]
pub fn verify_i64_range_valid(
    value: i64,
) -> Result<I64Range<-1000000000, 1000000000>, ValidationError> {
    I64Range::<-1000000000, 1000000000>::new(value)
}

/// Prove that U64Range construction succeeds for values in range.
#[cfg(feature = "verify-prusti")]
#[requires(1000000 <= value && value <= 18000000000000000000)]
#[ensures(result.is_ok())]
pub fn verify_u64_range_valid(
    value: u64,
) -> Result<U64Range<1000000, 18000000000000000000>, ValidationError> {
    U64Range::<1000000, 18000000000000000000>::new(value)
}

/// Prove that IsizeRange construction succeeds for values in range.
#[cfg(feature = "verify-prusti")]
#[requires(-500 <= value && value <= 500)]
#[ensures(result.is_ok())]
pub fn verify_isize_range_valid(value: isize) -> Result<IsizeRange<-500, 500>, ValidationError> {
    IsizeRange::<-500, 500>::new(value)
}

/// Prove that UsizeRange construction succeeds for values in range.
#[cfg(feature = "verify-prusti")]
#[requires(100 <= value && value <= 10000)]
#[ensures(result.is_ok())]
pub fn verify_usize_range_valid(value: usize) -> Result<UsizeRange<100, 10000>, ValidationError> {
    UsizeRange::<100, 10000>::new(value)
}

// ============================================================================
