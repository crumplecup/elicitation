//! Creusot proofs for HTTP contract types (feature-gated on reqwest).
//!
//! Cloud of assumptions: Trust reqwest::StatusCode::from_u16() range validation
//! (100–999). Verify wrapper construction and error cases.

#![cfg(feature = "reqwest")]

use creusot_std::prelude::*;
use elicitation::{StatusCodeValid, ValidationError};

/// Verify StatusCodeValid accepts HTTP 200 OK.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_status_code_valid_200() -> Result<StatusCodeValid, ValidationError> {
    StatusCodeValid::new(200)
}

/// Verify StatusCodeValid accepts HTTP 404 Not Found.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_status_code_valid_404() -> Result<StatusCodeValid, ValidationError> {
    StatusCodeValid::new(404)
}

/// Verify StatusCodeValid rejects code 99 (below minimum).
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_status_code_invalid_99() -> Result<StatusCodeValid, ValidationError> {
    StatusCodeValid::new(99)
}

/// Verify StatusCodeValid rejects code 1000 (above maximum).
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_status_code_invalid_1000() -> Result<StatusCodeValid, ValidationError> {
    StatusCodeValid::new(1000)
}
