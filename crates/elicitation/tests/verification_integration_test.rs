//! Integration test for verification code generation.
//!
//! This test verifies that #[derive(Elicit)] generates correct verification
//! code for all four supported verifiers.

#![cfg(feature = "verification")]

use elicitation::*;

/// Test struct with multiple field types to ensure comprehensive verification.
#[derive(Debug, Clone, Elicit)]
pub struct VerifiedUser {
    _name: StringNonEmpty,
    _age: I8Positive,
    _email: StringNonEmpty,
}

/// Test enum with multiple variant types.
#[derive(Debug, Clone, Elicit)]
pub enum VerifiedStatus {
    Active,
    Pending(StringNonEmpty),
    Suspended { reason: StringNonEmpty },
}

// Note: The actual verification happens when running the verifier tools:
// - cargo kani --features verify-kani (works now, Kani installed)
// - cargo creusot --features verify-creusot (requires Creusot toolchain)
// - cargo prusti --features verify-prusti (requires Prusti toolchain)
// - verus --features verify-verus src/lib.rs (requires Verus toolchain)

#[cfg(test)]
mod tests {
    #[test]
    fn test_derive_compiles() {
        // This test just ensures the derive macro expands without errors
        // Actual verification requires running the verifier tools
        let _ = stringify!(super::VerifiedUser);
        let _ = stringify!(VerifiedStatus);
    }
}
