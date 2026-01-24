//! Verus proofs for char contract types.

#![cfg(all(feature = "verify-verus", not(kani)))]
#![allow(unused_imports)]

use crate::*;

#[allow(unused_imports)]
use builtin::*;
#[allow(unused_imports)]
use builtin_macros::*;

verus! {

// Phase 9: Char Proofs
// ============================================================================

proof fn verify_char_alphabetic_construction(c: char)
    ensures
        c.is_alphabetic() ==> CharAlphabetic::new(c).is_ok(),
        !c.is_alphabetic() ==> CharAlphabetic::new(c).is_err(),
{
}

proof fn verify_char_numeric_construction(c: char)
    ensures
        c.is_numeric() ==> CharNumeric::new(c).is_ok(),
        !c.is_numeric() ==> CharNumeric::new(c).is_err(),
{
}

proof fn verify_char_alphanumeric_construction(c: char)
    ensures
        c.is_alphanumeric() ==> CharAlphanumeric::new(c).is_ok(),
        !c.is_alphanumeric() ==> CharAlphanumeric::new(c).is_err(),
{
}

// ============================================================================

} // verus!
