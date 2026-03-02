//! HTTP status code contract type.

use crate::verification::types::ValidationError;

/// A valid HTTP status code (100–999).
///
/// Validates that a `u16` can be parsed as a legitimate HTTP status code
/// via `reqwest::StatusCode::from_u16()`.
///
/// # Kani Verification
///
/// In Kani mode, uses symbolic boolean to verify wrapper invariants without
/// invoking the reqwest runtime.
#[cfg(not(kani))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StatusCodeValid(reqwest::StatusCode);

#[cfg(kani)]
#[derive(Debug, Clone)]
pub struct StatusCodeValid(std::marker::PhantomData<reqwest::StatusCode>);

#[cfg(not(kani))]
impl StatusCodeValid {
    /// Create a new `StatusCodeValid` from a `u16`.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::StatusCodeInvalid` if the value is outside
    /// the range accepted by `reqwest::StatusCode::from_u16()` (100–999).
    pub fn new(value: u16) -> Result<Self, ValidationError> {
        reqwest::StatusCode::from_u16(value)
            .map(Self)
            .map_err(|_| ValidationError::StatusCodeInvalid(value))
    }

    /// Get a reference to the wrapped status code.
    pub fn get(&self) -> reqwest::StatusCode {
        self.0
    }

    #[cfg(verus)]
    fn verus_proof() {
        // Verus proof exists in elicitation_verus crate
    }

    #[cfg(prusti)]
    fn prusti_proof() {
        // Prusti proof exists in elicitation_prusti crate
        // Verifies: wrapper structure with separation logic
    }

    /// Unwrap to the inner status code.
    pub fn into_inner(self) -> reqwest::StatusCode {
        self.0
    }

    #[cfg(verus)]
    fn verus_proof_into_inner() {
        // Verus proof exists in elicitation_verus crate
    }

    #[cfg(prusti)]
    fn prusti_proof_into_inner() {
        // Prusti proof exists in elicitation_prusti crate
    }
}

#[cfg(kani)]
impl StatusCodeValid {
    /// Create a new `StatusCodeValid` (Kani mode).
    pub fn new(value: u16) -> Result<Self, ValidationError> {
        // 100–999 is the range accepted by http::StatusCode
        if value >= 100 && value <= 999 {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::StatusCodeInvalid(value))
        }
    }

    /// Get the wrapped status code (Kani mode).
    pub fn get(&self) -> reqwest::StatusCode {
        panic!("StatusCodeValid::get() not available in Kani mode")
    }

    /// Unwrap (Kani mode).
    pub fn into_inner(self) -> reqwest::StatusCode {
        panic!("StatusCodeValid::into_inner() not available in Kani mode")
    }
}
