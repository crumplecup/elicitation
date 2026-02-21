//! Verus proof for BoolTrue contract type
//!
//! This file contains verification that BoolTrue correctly validates boolean values.

#![allow(unused_imports)]

#[cfg(verus)]
use verus_builtin::*;
#[cfg(verus)]
use verus_builtin_macros::*;
#[cfg(verus)]
use vstd::prelude::*;

// Import the actual BoolTrue type
use crate::verification::types::bools::BoolTrue;
use crate::verification::types::ValidationError;

#[cfg(verus)]
verus! {

// Verification function: proves BoolTrue::new works correctly
// Verus will verify that the implementation satisfies this specification
pub fn verify_bool_true_new(value: bool) -> (result: Result<BoolTrue, ValidationError>)
    ensures
        value == true ==> (result matches Ok(_)),
        value == false ==> (result matches Err(_)),
{
    BoolTrue::new(value)
}

} // verus!
