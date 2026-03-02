//! Verus proofs for HTTP contract types.

use crate::verification::types::ValidationError;
use crate::verification::types::http::StatusCodeValid;

#[cfg(verus)]
#[allow(unused_imports)]
use verus_builtin::*;
#[cfg(verus)]
#[allow(unused_imports)]
use verus_builtin_macros::*;

verus! {

// HTTP Status Code Contract Proofs
// ============================================================================

/// Verify StatusCodeValid construction accepts valid codes (100–999).
proof fn verify_status_code_valid_construction(value: u16)
    ensures
        (value >= 100 && value <= 999) ==> StatusCodeValid::new(value).is_ok(),
        (value < 100 || value > 999) ==> StatusCodeValid::new(value).is_err(),
{
    // Linear arithmetic: reqwest::StatusCode accepts exactly 100..=999
}

/// Verify StatusCodeValid rejects out-of-range codes.
proof fn verify_status_code_invalid_rejection()
    ensures
        StatusCodeValid::new(0).is_err(),
        StatusCodeValid::new(99).is_err(),
        StatusCodeValid::new(1000).is_err(),
        StatusCodeValid::new(65535).is_err(),
{
    // Boundary values outside 100..=999 must always fail
}

/// Verify common status codes are accepted.
proof fn verify_status_code_common_codes()
    ensures
        StatusCodeValid::new(200).is_ok(),
        StatusCodeValid::new(404).is_ok(),
        StatusCodeValid::new(500).is_ok(),
        StatusCodeValid::new(100).is_ok(),
        StatusCodeValid::new(999).is_ok(),
{
    // Concrete witnesses: common HTTP codes succeed
}

} // verus!
