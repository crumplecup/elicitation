//! Test enum verification contract generation.

use elicitation::verification::types::{I8Positive, StringNonEmpty};
use elicitation::{Elicit, Prompt, Select};

/// Test enum with all variant types.
#[derive(Elicit)]
#[allow(dead_code)] // Test enum
#[allow(clippy::large_enum_variant)] // Test code
enum _Status {
    _Active { since: StringNonEmpty },
    _Pending { count: I8Positive },
    _Inactive,
}

#[test]
fn test_enum_compiles() {
    // This test verifies the enum derive macro compiles successfully
}

// Note: Generated verification harnesses are only compiled by Kani:
//   cargo kani --harness __verify_Status_Active --features verify-kani
//   cargo kani --harness __verify_Status_Pending --features verify-kani
//   cargo kani --harness __verify_Status_Inactive --features verify-kani
