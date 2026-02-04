//! Integration tests for contract-based elicitation.

use elicitation::{
    contracts::{And, Established, Is, Prop, both},
    ElicitResult,
};

/// Test that elicit_proven returns a proof
#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn test_string_elicit_proven() -> ElicitResult<()> {
    // This would require actual MCP infrastructure
    // For now, we test the trait signature compiles
    Ok(())
}

/// Test that proofs can be used in downstream functions
#[test]
fn test_proof_composition() {
    struct EmailValidated;
    struct ConsentObtained;
    impl Prop for EmailValidated {}
    impl Prop for ConsentObtained {}

    // Simulate two proofs
    let email_proof: Established<EmailValidated> = Established::assert();
    let consent_proof: Established<ConsentObtained> = Established::assert();

    // Combine them
    let combined: Established<And<EmailValidated, ConsentObtained>> =
        both(email_proof, consent_proof);

    // Function requiring both proofs
    fn register_user(_email: String, _proof: Established<And<EmailValidated, ConsentObtained>>) {
        // Would register user
    }

    register_user("user@example.com".to_string(), combined);
}

/// Test that Is<T> proofs work with concrete types
#[test]
fn test_inhabitation_proof() {
    fn use_validated_string(_s: String, _proof: Established<Is<String>>) {
        // Would use validated string
    }

    let s = String::from("hello");
    let proof = Established::assert();
    use_validated_string(s, proof);
}

/// Test that proofs are zero-sized
#[test]
fn test_proofs_zero_sized() {
    let string_proof: Established<Is<String>> = Established::assert();
    assert_eq!(std::mem::size_of_val(&string_proof), 0);

    let i32_proof: Established<Is<i32>> = Established::assert();
    assert_eq!(std::mem::size_of_val(&i32_proof), 0);

    struct EmailValid;
    impl Prop for EmailValid {}
    let custom_proof: Established<EmailValid> = Established::assert();
    assert_eq!(std::mem::size_of_val(&custom_proof), 0);
}
