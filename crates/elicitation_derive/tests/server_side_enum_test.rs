//! Regression test: Enum derive MUST use send_prompt(), not call_tool().
//!
//! This test would have caught the bug where enum derives were generating
//! call_tool() instead of send_prompt(), which broke server-side elicitation.

use elicitation::{ElicitCommunicator, ElicitError, Elicitation, Prompt, Select};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::cell::RefCell;

///Mock communicator that tracks which methods are called.
#[derive(Clone)]
struct MockCommunicator {
    send_prompt_called: Arc<AtomicBool>,
    response: String,
    style_context: elicitation::StyleContext,
}

impl MockCommunicator {
    fn new(response: impl Into<String>) -> Self {
        Self {
            send_prompt_called: Arc::new(AtomicBool::new(false)),
            response: response.into(),
            style_context: elicitation::StyleContext::default(),
        }
    }

    fn was_send_prompt_called(&self) -> bool {
        self.send_prompt_called.load(Ordering::SeqCst)
    }
}

impl ElicitCommunicator for MockCommunicator {
    async fn send_prompt(&self, _prompt: &str) -> Result<String, ElicitError> {
        self.send_prompt_called.store(true, Ordering::SeqCst);
        Ok(self.response.clone())
    }

    async fn call_tool(
        &self,
        _params: rmcp::model::CallToolRequestParams,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ServiceError> {
        panic!("BUG: Enum derive must NOT call call_tool() - use send_prompt() instead!");
    }

    fn style_context(&self) -> &elicitation::StyleContext {
        &self.style_context
    }

    fn with_style<T: 'static, S: elicitation::ElicitationStyle>(&self, _style: S) -> Self {
        self.clone()
    }
}

// Test enums
#[derive(Debug, Clone, Copy, PartialEq, elicitation::Elicit)]
enum SimpleEnum {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, Copy, PartialEq, elicitation::Elicit)]
#[prompt("Choose a color:")]
enum ColorEnum {
    Red,
    Green,
    Blue,
}

#[tokio::test]
async fn test_simple_enum_uses_send_prompt() {
    let mock = MockCommunicator::new("1"); // Select first option
    
    let result = SimpleEnum::elicit(&mock).await;
    
    // Verify send_prompt was called (not call_tool)
    assert!(
        mock.was_send_prompt_called(),
        "Enum derive MUST use send_prompt(), not call_tool()"
    );
    
    // Verify it actually worked
    assert!(result.is_ok(), "Elicitation should succeed");
    assert_eq!(result.unwrap(), SimpleEnum::First);
}

#[tokio::test]
async fn test_enum_with_custom_prompt_uses_send_prompt() {
    let mock = MockCommunicator::new("2"); // Select second option
    
    let result = ColorEnum::elicit(&mock).await;
    
    // Verify send_prompt was called (not call_tool)
    assert!(
        mock.was_send_prompt_called(),
        "Enum derive MUST use send_prompt(), not call_tool()"
    );
    
    // Verify it actually worked
    assert!(result.is_ok(), "Elicitation should succeed");
    assert_eq!(result.unwrap(), ColorEnum::Green);
}

#[tokio::test]
async fn test_enum_label_response_uses_send_prompt() {
    let mock = MockCommunicator::new("Blue"); // Select by label
    
    let result = ColorEnum::elicit(&mock).await;
    
    // Verify send_prompt was called (not call_tool)
    assert!(
        mock.was_send_prompt_called(),
        "Enum derive MUST use send_prompt(), not call_tool()"
    );
    
    // Verify it actually worked
    assert!(result.is_ok(), "Elicitation should succeed");
    assert_eq!(result.unwrap(), ColorEnum::Blue);
}

#[tokio::test]
async fn test_enum_invalid_response_still_uses_send_prompt() {
    let mock = MockCommunicator::new("99"); // Invalid selection
    
    let result = ColorEnum::elicit(&mock).await;
    
    // Even on error, verify send_prompt was called (not call_tool)
    assert!(
        mock.was_send_prompt_called(),
        "Enum derive MUST use send_prompt(), not call_tool() (even on error)"
    );
    
    // Should fail with invalid selection
    assert!(result.is_err(), "Invalid selection should error");
}
