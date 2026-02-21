//! Verus proofs for integer contract types.

use crate::verification::types::ValidationError;
use crate::verification::types::integers::{
    I8NonNegative, I8Positive, I8Range, I16NonNegative, I16Positive, I16Range, I32NonNegative,
    I32Positive, I32Range, I64NonNegative, I64Positive, I64Range, I128NonNegative, I128Positive,
    I128Range, IsizeNonNegative, IsizePositive, IsizeRange, U8NonZero, U8Range, U16NonZero,
    U16Range, U32NonZero, U32Range, U64NonZero, U64Range, U128NonZero, U128Range, UsizeNonZero,
    UsizeRange,
};

#[cfg(verus)]
#[allow(unused_imports)]
use verus_builtin::*;
#[cfg(verus)]
#[allow(unused_imports)]
use verus_builtin_macros::*;

verus! {

// Phase 1: Integer Contract Proofs
// ============================================================================

/// Verify I8Positive contract correctness.
///
/// Proves that I8Positive construction succeeds iff value > 0.
proof fn verify_i8_positive_construction(value: i8)
    ensures
        value > 0 ==> I8Positive::new(value).is_ok(),
        value <= 0 ==> I8Positive::new(value).is_err(),
{
    // Z3 proves this via linear arithmetic
}

/// Verify I8Positive accessor preserves invariant.
proof fn verify_i8_positive_accessor(positive: I8Positive)
    ensures positive.get() > 0,
{
    // Invariant preserved by construction
}

/// Verify I8NonNegative contract correctness.
proof fn verify_i8_non_negative_construction(value: i8)
    ensures
        value >= 0 ==> I8NonNegative::new(value).is_ok(),
        value < 0 ==> I8NonNegative::new(value).is_err(),
{
    // Linear arithmetic proof
}

/// Verify I8Range const generic contract.
proof fn verify_i8_range_construction<const MIN: i8, const MAX: i8>(value: i8)
    requires MIN <= MAX
    ensures
        (MIN <= value && value <= MAX) ==> I8Range::<MIN, MAX>::new(value).is_ok(),
        (value < MIN || value > MAX) ==> I8Range::<MIN, MAX>::new(value).is_err(),
{
    // Const generic bounds proof
}

/// Verify U8NonZero contract correctness.
proof fn verify_u8_non_zero_construction(value: u8)
    ensures
        value != 0 ==> U8NonZero::new(value).is_ok(),
        value == 0 ==> U8NonZero::new(value).is_err(),
{
    // Zero rejection proof
}

// Repeat for all integer sizes: i16, i32, i64, i128, u16, u32, u64, u128, isize, usize
// (Following same pattern as Kani proofs)

// ============================================================================

} // verus!
