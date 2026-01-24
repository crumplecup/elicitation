//! Test enum verification contract generation.

use elicitation::verification::types::{I8Positive, StringNonEmpty};
use elicitation::{Elicit, Prompt, Select};

/// Test enum with all variant types.
#[derive(Elicit)]
enum Status {
    Active { since: StringNonEmpty },
    Pending { count: I8Positive },
    Inactive,
}

#[test]
fn test_enum_compiles() {
    // This test verifies the enum derive macro compiles successfully
    assert!(true);
}

// Note: Generated verification harnesses are only compiled by Kani:
//   cargo kani --harness __verify_Status_Active --features verify-kani
//   cargo kani --harness __verify_Status_Pending --features verify-kani
//   cargo kani --harness __verify_Status_Inactive --features verify-kani
