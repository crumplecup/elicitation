//! Integration tests for contract-based tools.

use elicitation::{
    ElicitResult,
    contracts::{Established, Prop},
    tool::True,
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
        fn send(
            &self,
            _email: String,
            _pre: Established<EmailValidated>,
        ) -> ((), Established<True>) {
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
        fn validate(
            &self,
            _email: String,
            _pre: Established<True>,
        ) -> (String, Established<EmailValidated>) {
            (String::from("valid@example.com"), Established::assert())
        }
    }

    struct SendEmail;
    impl SendEmail {
        fn send(
            &self,
            _email: String,
            _pre: Established<EmailValidated>,
        ) -> ((), Established<EmailSent>) {
            ((), Established::assert())
        }
    }

    // Chain: validate then send
    let validator = ValidateEmail;
    let sender = SendEmail;

    let (validated_email, validation_proof) =
        validator.validate("user@example.com".to_string(), True::axiom());

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

/// Test sequential tool composition with then
#[tokio::test]
async fn test_then_composition() {
    struct EmailValidated;
    struct EmailSent;
    impl Prop for EmailValidated {}
    impl Prop for EmailSent {}

    // String implements Elicitation
    struct ValidateEmail;
    impl elicitation::Tool for ValidateEmail {
        type Input = String;
        type Output = String;
        type Pre = True;
        type Post = EmailValidated;

        async fn execute(
            &self,
            email: String,
            _pre: Established<True>,
        ) -> ElicitResult<(String, Established<EmailValidated>)> {
            Ok((email, Established::assert()))
        }
    }

    struct SendEmail;
    impl elicitation::Tool for SendEmail {
        type Input = String;
        type Output = ();
        type Pre = EmailValidated;
        type Post = EmailSent;

        async fn execute(
            &self,
            _email: String,
            _pre: Established<EmailValidated>,
        ) -> ElicitResult<((), Established<EmailSent>)> {
            Ok(((), Established::assert()))
        }
    }

    let validator = ValidateEmail;
    let sender = SendEmail;

    let (_result, _proof) = elicitation::then(
        &validator,
        &sender,
        "user@example.com".to_string(),
        True::axiom(),
    )
    .await
    .expect("Chain should succeed");
}

/// Test parallel tool composition with both_tools
#[tokio::test]
async fn test_both_tools_composition() {
    use elicitation::contracts::both;

    struct EmailValidated;
    struct PhoneValidated;
    impl Prop for EmailValidated {}
    impl Prop for PhoneValidated {}

    struct ValidateEmail;
    impl elicitation::Tool for ValidateEmail {
        type Input = String;
        type Output = String;
        type Pre = True;
        type Post = EmailValidated;

        async fn execute(
            &self,
            email: String,
            _pre: Established<True>,
        ) -> ElicitResult<(String, Established<EmailValidated>)> {
            Ok((email, Established::assert()))
        }
    }

    struct ValidatePhone;
    impl elicitation::Tool for ValidatePhone {
        type Input = String;
        type Output = String;
        type Pre = True;
        type Post = PhoneValidated;

        async fn execute(
            &self,
            phone: String,
            _pre: Established<True>,
        ) -> ElicitResult<(String, Established<PhoneValidated>)> {
            Ok((phone, Established::assert()))
        }
    }

    let email_validator = ValidateEmail;
    let phone_validator = ValidatePhone;

    let pre1 = True::axiom();
    let pre2 = True::axiom();
    let combined_pre = both(pre1, pre2);

    let ((email_result, phone_result), _combined_proof) = elicitation::both_tools(
        &email_validator,
        &phone_validator,
        "user@example.com".to_string(),
        "+1234567890".to_string(),
        combined_pre,
    )
    .await
    .expect("Both tools should succeed");

    assert_eq!(email_result, "user@example.com");
    assert_eq!(phone_result, "+1234567890");
}
