//! Test that verus_proof() method works for user-defined types.
//!
//! This test verifies that the #[derive(Elicit)] macro correctly generates
//! verus_proof() methods that call field proofs, enabling compositional
//! verification.

use elicitation::{Elicit, Prompt, Select};

#[cfg(verus)]
use elicitation::Elicitation;

/// Simple struct for testing.
#[derive(Debug, Clone, Elicit, schemars::JsonSchema)]
pub struct TestStruct {
    pub name: String,
    pub count: u32,
}

/// Enum with variants for testing.
#[derive(Debug, Clone, Elicit, schemars::JsonSchema)]
pub enum TestEnum {
    Simple,
    WithData { value: String },
}

/// Nested struct for testing.
#[derive(Debug, Clone, Elicit, schemars::JsonSchema)]
pub struct NestedStruct {
    pub inner: TestStruct,
    pub mode: TestEnum,
}

#[test]
fn test_verus_proof_trait_exists() {
    // This test verifies that verus_proof() exists on user types.
    // The #[cfg(verus)] gate means it won't be called in normal tests,
    // but we can verify it compiles.

    #[cfg(verus)]
    {
        TestStruct::verus_proof();
        TestEnum::verus_proof();
        NestedStruct::verus_proof();
    }

    // In non-verus builds, just verify types implement Elicitation
    fn assert_elicitation<T: elicitation::Elicitation>() {}

    assert_elicitation::<TestStruct>();
    assert_elicitation::<TestEnum>();
    assert_elicitation::<NestedStruct>();
}

#[test]
fn test_compositional_chain() {
    // Verify the types compose correctly
    #[cfg(verus)]
    {
        // Layer 1: primitives (String, u32) - handled by Rust
        // Layer 2: TestStruct, TestEnum - derive generates verus_proof()
        TestStruct::verus_proof();
        TestEnum::verus_proof();

        // Layer 3: NestedStruct - compose Layer 2 proofs
        NestedStruct::verus_proof();
    }

    // The test passes if it compiles - the trait bounds ensure correctness
}

#[test]
fn test_contract_types_have_verus_proof() {
    // Verify that contract types from elicitation also have verus_proof()
    #[cfg(verus)]
    {
        use elicitation::verification::types::{BoolFalse, BoolTrue};
        BoolTrue::verus_proof();
        BoolFalse::verus_proof();
    }
}
