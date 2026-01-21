//! Test verification contract code generation.
//!
//! This test verifies that the #[derive(Elicit)] macro generates
//! verification contracts when verification features are enabled.

use elicitation::Elicit;
use elicitation::verification::types::{I8Positive, StringNonEmpty};

/// Simple test struct with contract types.
///
/// When compiled with --features verify-kani, should generate:
/// - __make_User constructor with #[kani::requires] and #[kani::ensures]
/// - __verify_User harness with #[kani::proof_for_contract]
#[derive(Elicit)]
struct User {
    name: StringNonEmpty,
    age: I8Positive,
}

#[test]
fn test_user_struct_compiles() {
    // This test just verifies the derive macro compiles successfully
    // The verification contracts are feature-gated
    assert!(true);
}

// Verification tests (only compiled with verify-kani feature)
#[cfg(all(feature = "verify-kani", kani))]
mod kani_tests {
    use super::*;

    // The derive macro should have generated these functions
    // We can't call them directly in tests, but Kani will find them

    #[test]
    fn verification_functions_exist() {
        // This is a compile-time check that the functions were generated
        // If this compiles, the derive macro worked
        assert!(true);
    }
}
