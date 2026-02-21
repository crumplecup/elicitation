//! Verus proofs for duration contract types.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    DurationNotPositive,
}

// ============================================================================
// DurationPositive - positive duration validation
// ============================================================================

/// Contract type for positive (non-zero) durations.
///
/// We abstract the Duration and its properties.
/// Verifies wrapper logic, not stdlib Duration implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurationPositive {
    pub validated: bool,
}

impl DurationPositive {
    /// Creates a DurationPositive given validation result.
    ///
    /// Parameters abstract the Duration properties we'd check:
    /// - is_positive: result of duration.as_nanos() > 0
    ///
    /// This verifies our wrapper logic is correct.
    pub fn new(is_positive: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_positive ==> (result matches Ok(d) && d.validated == true),
            !is_positive ==> (result matches Err(ValidationError::DurationNotPositive)),
    {
        if is_positive {
            Ok(DurationPositive { validated: true })
        } else {
            Err(ValidationError::DurationNotPositive)
        }
    }

    /// Check if validated (always true for valid instances).
    pub fn is_validated(&self) -> (result: bool)
        requires self.validated == true,
        ensures result == true,
    {
        self.validated
    }
}

} // verus!
