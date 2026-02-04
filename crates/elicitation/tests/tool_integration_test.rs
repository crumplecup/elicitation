//! Integration tests for contract-based tools.

use elicitation::{
    contracts::{Established, Prop},
    tool::True,
    ElicitResult,
};

/// Test tool with no preconditions
#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn test_unconstrained_tool() -> ElicitResult<()> {
    // This would require actual MCP infrastructure
    Ok(())
}

/// Test that True is zero-sized
#[test]
fn test_true_zero_sized() {
    let proof = True::axiom();
    assert_eq!(std::mem::size_of_val(&proof), 0);
}

/// Test tool with precondition
#[test]
fn test_tool_with_precondition() {
    struct EmailValidated;
    impl Prop for EmailValidated {}

    struct MockSendEmail;
    
    // Simulate tool implementation
    impl MockSendEmail {
        fn send(&self, _email: String, _pre: Established<EmailValidated>) -> ((), Established<True>) {
            ((), True::axiom())
        }
    }

    let tool = MockSendEmail;
    let email_proof: Established<EmailValidated> = Established::assert();
    let (_output, _post_proof) = tool.send("user@example.com".to_string(), email_proof);
}

/// Test tool chain with proof composition
#[test]
fn test_tool_chain() {
    struct EmailValidated;
    struct EmailSent;
    impl Prop for EmailValidated {}
    impl Prop for EmailSent {}

    struct ValidateEmail;
    impl ValidateEmail {
        fn validate(&self, _email: String, _pre: Established<True>) 
            -> (String, Established<EmailValidated>) 
        {
            (String::from("valid@example.com"), Established::assert())
        }
    }

    struct SendEmail;
    impl SendEmail {
        fn send(&self, _email: String, _pre: Established<EmailValidated>) 
            -> ((), Established<EmailSent>) 
        {
            ((), Established::assert())
        }
    }

    // Chain: validate then send
    let validator = ValidateEmail;
    let sender = SendEmail;

    let (validated_email, validation_proof) = validator.validate(
        "user@example.com".to_string(),
        True::axiom(),
    );

    let (_result, _sent_proof) = sender.send(validated_email, validation_proof);
}

/// Test that tool without precondition proof doesn't compile
#[test]
fn test_cannot_call_without_proof() {
    // Compile-time enforcement test
    // If this compiles, the following would NOT compile:
    
    // struct EmailValidated;
    // impl Prop for EmailValidated {}
    // 
    // struct SendEmail;
    // impl SendEmail {
    //     fn send(&self, _email: String, _pre: Established<EmailValidated>) {}
    // }
    // 
    // let tool = SendEmail;
    // tool.send("user@example.com".to_string()); // ERROR: missing proof!
}

/// Test tool with multiple preconditions
#[test]
fn test_tool_with_multiple_preconditions() {
    use elicitation::contracts::{And, both};

    struct EmailValidated;
    struct ConsentObtained;
    impl Prop for EmailValidated {}
    impl Prop for ConsentObtained {}

    struct RegisterUser;
    impl RegisterUser {
        fn register(
            &self,
            _email: String,
            _pre: Established<And<EmailValidated, ConsentObtained>>,
        ) -> ((), Established<True>) {
            ((), True::axiom())
        }
    }

    let tool = RegisterUser;
    let email_proof: Established<EmailValidated> = Established::assert();
    let consent_proof: Established<ConsentObtained> = Established::assert();
    let combined_proof = both(email_proof, consent_proof);

    let (_output, _post) = tool.register("user@example.com".to_string(), combined_proof);
}
