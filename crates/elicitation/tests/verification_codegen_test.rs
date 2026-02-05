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
struct _User {
    _name: StringNonEmpty,
    _age: I8Positive,
}

#[test]
fn test_user_struct_compiles() {
    // This test just verifies the derive macro compiles successfully
    // The verification contracts are feature-gated and only visible to Kani
}

// Note: The generated verification functions are only compiled by Kani,
// not by regular cargo test. To verify them:
//   cargo kani --harness __verify_User --features verify-kani
