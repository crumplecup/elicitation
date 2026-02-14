//! Regression test: Struct derive MUST use send_prompt(), not call_tool().
//!
//! This test would have caught the bug where struct field derives were
//! generating call_tool() instead of send_prompt(), breaking server-side.

use elicitation::{ElicitCommunicator, ElicitError, Elicitation, Prompt, Select};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Mock communicator that tracks method calls and provides sequential responses.
#[derive(Clone)]
struct MockCommunicator {
    send_prompt_count: Arc<AtomicUsize>,
    responses: Arc<Vec<String>>,
    style_context: elicitation::StyleContext,
}

impl MockCommunicator {
    fn new(responses: Vec<String>) -> Self {
        let mut style_context = elicitation::StyleContext::default();
        // Pre-set StringStyle to Human to avoid extra elicitation
        style_context
            .set_style::<String, elicitation::StringStyle>(elicitation::StringStyle::Human);

        Self {
            send_prompt_count: Arc::new(AtomicUsize::new(0)),
            responses: Arc::new(responses),
            style_context,
        }
    }

    fn send_prompt_call_count(&self) -> usize {
        self.send_prompt_count.load(Ordering::SeqCst)
    }
}

impl ElicitCommunicator for MockCommunicator {
    async fn send_prompt(&self, _prompt: &str) -> Result<String, ElicitError> {
        let count = self.send_prompt_count.fetch_add(1, Ordering::SeqCst);

        if count < self.responses.len() {
            Ok(self.responses[count].clone())
        } else {
            Err(ElicitError::new(
                elicitation::ElicitErrorKind::InvalidFormat {
                    expected: "mock response".to_string(),
                    received: "no more responses".to_string(),
                },
            ))
        }
    }

    async fn call_tool(
        &self,
        _params: rmcp::model::CallToolRequestParams,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ServiceError> {
        panic!("BUG: Struct derive must NOT call call_tool() - use send_prompt() instead!");
    }

    fn style_context(&self) -> &elicitation::StyleContext {
        &self.style_context
    }

    fn with_style<T: 'static, S: elicitation::ElicitationStyle>(&self, _style: S) -> Self {
        self.clone()
    }
}

// Test structs
#[derive(Debug, PartialEq, elicitation::Elicit)]
struct SimpleStruct {
    name: String,
    age: u8,
}

#[derive(Debug, PartialEq, elicitation::Elicit)]
#[prompt("Configure your settings:")]
struct ConfigStruct {
    host: String,
    port: u16,
}

#[derive(Debug, PartialEq, elicitation::Elicit)]
struct BoolStruct {
    enabled: bool,
}

#[tokio::test]
async fn test_struct_string_field_uses_send_prompt() {
    let mock = MockCommunicator::new(vec!["Alice".to_string(), "25".to_string()]);

    let result = SimpleStruct::elicit(&mock).await;

    // Verify send_prompt was called for each field
    assert_eq!(
        mock.send_prompt_call_count(),
        2,
        "Should call send_prompt once per field"
    );

    // Verify it actually worked
    assert!(result.is_ok(), "Elicitation should succeed");
    let value = result.unwrap();
    assert_eq!(value.name, "Alice");
    assert_eq!(value.age, 25);
}

#[tokio::test]
async fn test_struct_with_custom_prompt_uses_send_prompt() {
    let mock = MockCommunicator::new(vec!["localhost".to_string(), "8080".to_string()]);

    let result = ConfigStruct::elicit(&mock).await;

    // Verify send_prompt was called for each field
    assert_eq!(
        mock.send_prompt_call_count(),
        2,
        "Should call send_prompt once per field"
    );

    // Verify it actually worked
    assert!(result.is_ok(), "Elicitation should succeed");
    let value = result.unwrap();
    assert_eq!(value.host, "localhost");
    assert_eq!(value.port, 8080);
}

#[tokio::test]
async fn test_struct_bool_field_uses_send_prompt() {
    let mock = MockCommunicator::new(vec!["true".to_string()]);

    let result = BoolStruct::elicit(&mock).await;

    // Verify send_prompt was called
    assert_eq!(
        mock.send_prompt_call_count(),
        1,
        "Should call send_prompt for bool field"
    );

    // Verify it actually worked
    assert!(result.is_ok(), "Elicitation should succeed");
    assert!(result.unwrap().enabled);
}

#[tokio::test]
async fn test_struct_parse_error_still_uses_send_prompt() {
    let mock = MockCommunicator::new(vec![
        "Alice".to_string(),
        "not_a_number".to_string(), // Invalid age
    ]);

    let result = SimpleStruct::elicit(&mock).await;

    // Even on parse error, verify send_prompt was called (not call_tool)
    assert!(
        mock.send_prompt_call_count() >= 1,
        "Should call send_prompt even if parsing fails"
    );

    // Should fail with parse error
    assert!(result.is_err(), "Invalid parse should error");
}

// Test nested types (enum in struct)
#[derive(Debug, Clone, Copy, PartialEq, elicitation::Elicit)]
enum Status {
    Active,
    Inactive,
}

#[derive(Debug, PartialEq, elicitation::Elicit)]
struct UserWithStatus {
    name: String,
    status: Status,
}

#[tokio::test]
async fn test_struct_with_enum_field_uses_send_prompt() {
    let mock = MockCommunicator::new(vec![
        "Bob".to_string(),
        "1".to_string(), // Select first enum option
    ]);

    let result = UserWithStatus::elicit(&mock).await;

    // Verify send_prompt was called for each field (string + enum)
    assert_eq!(
        mock.send_prompt_call_count(),
        2,
        "Should call send_prompt for string field AND enum field"
    );

    // Verify it actually worked
    assert!(result.is_ok(), "Elicitation should succeed");
    let value = result.unwrap();
    assert_eq!(value.name, "Bob");
    assert_eq!(value.status, Status::Active);
}
