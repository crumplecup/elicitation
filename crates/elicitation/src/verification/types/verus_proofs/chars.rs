//! Verus proofs for char contract types.


use crate::verification::types::chars::{CharAlphabetic, CharAlphanumeric, CharNumeric};
use crate::verification::types::ValidationError;

#[cfg(verus)]
#[allow(unused_imports)]
use verus_builtin::*;
#[cfg(verus)]
#[allow(unused_imports)]
use verus_builtin_macros::*;

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
