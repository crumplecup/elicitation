//! Verus proofs for bool contract types.
//!
//! These proofs verify that BoolTrue and BoolFalse correctly validate boolean values.

#[cfg(verus)]
use verus_builtin::*;
#[cfg(verus)]
use verus_builtin_macros::*;
#[cfg(verus)]
use vstd::prelude::*;

use crate::verification::types::bools::{BoolTrue, BoolFalse};
use crate::verification::types::ValidationError;

#[cfg(verus)]
verus! {

/// Verify that BoolTrue::new correctly accepts true and rejects false
pub fn verify_bool_true_new(value: bool) -> (result: Result<BoolTrue, ValidationError>)
    ensures
        value == true ==> (result matches Ok(_)),
        value == false ==> (result matches Err(_)),
{
    BoolTrue::new(value)
}

/// Verify that BoolFalse::new correctly accepts false and rejects true
pub fn verify_bool_false_new(value: bool) -> (result: Result<BoolFalse, ValidationError>)
    ensures
        value == false ==> (result matches Ok(_)),
        value == true ==> (result matches Err(_)),
{
    BoolFalse::new(value)
}

} // verus!
