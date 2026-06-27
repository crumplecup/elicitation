//! Regression tests: field-level `#[prompt]` text must reach `send_prompt`.
//!
//! Before the fix, `generate_elicit_impl_simple` ignored `FieldInfo::default_prompt`
//! and always called `<T>::elicit(communicator)`, which used each type's hardcoded
//! default ("Please provide a text value:", "Please enter a u8:", etc.).
//! The field's annotation was captured in metadata but silently dropped at runtime.

use elicitation::{ElicitCommunicator, ElicitError, Elicitation};
use std::sync::{
    Arc, RwLock,
    atomic::{AtomicUsize, Ordering},
};

/// Communicator that records the exact prompt text passed to each `send_prompt` call.
#[derive(Clone)]
struct CapturingCommunicator {
    responses: Arc<Vec<String>>,
    call_index: Arc<AtomicUsize>,
    received_prompts: Arc<RwLock<Vec<String>>>,
    style_context: elicitation::StyleContext,
    elicitation_context: elicitation::ElicitationContext,
}

impl CapturingCommunicator {
    fn new(responses: Vec<&str>) -> Self {
        Self {
            responses: Arc::new(responses.into_iter().map(str::to_string).collect()),
            call_index: Arc::new(AtomicUsize::new(0)),
            received_prompts: Arc::new(RwLock::new(Vec::new())),
            style_context: elicitation::StyleContext::default(),
            elicitation_context: elicitation::ElicitationContext::default(),
        }
    }

    fn received_prompts(&self) -> Vec<String> {
        self.received_prompts.read().unwrap().clone()
    }
}

impl ElicitCommunicator for CapturingCommunicator {
    async fn send_prompt(&self, prompt: &str) -> Result<String, ElicitError> {
        self.received_prompts
            .write()
            .unwrap()
            .push(prompt.to_string());
        let idx = self.call_index.fetch_add(1, Ordering::SeqCst);
        self.responses.get(idx).cloned().ok_or_else(|| {
            ElicitError::new(elicitation::ElicitErrorKind::InvalidFormat {
                expected: "response".to_string(),
                received: "no more responses".to_string(),
            })
        })
    }

    async fn call_tool(
        &self,
        _params: rmcp::model::CallToolRequestParams,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ServiceError> {
        panic!("call_tool must not be called in prompt propagation tests");
    }

    fn style_context(&self) -> &elicitation::StyleContext {
        &self.style_context
    }

    fn with_style<
        T: 'static,
        S: elicitation::StyleMarker + elicitation::style::ElicitationStyle + 'static,
    >(
        &self,
        _style: S,
    ) -> Self {
        self.clone()
    }

    fn elicitation_context(&self) -> &elicitation::ElicitationContext {
        &self.elicitation_context
    }
}

// ── Struct under test ─────────────────────────────────────────────────────────

#[derive(
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    Debug,
    PartialEq,
    elicitation::Elicit,
)]
struct PromptedPrimitives {
    #[prompt("What is your name?")]
    name: String,
    #[prompt("How old are you?")]
    age: u8,
    #[prompt("Are you active?")]
    active: bool,
}

// Enum used as a complex (Select) field type
#[derive(
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    Debug,
    Clone,
    Copy,
    PartialEq,
    elicitation::Elicit,
)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    Debug,
    PartialEq,
    elicitation::Elicit,
)]
struct PromptedEnumField {
    #[prompt("Which direction?")]
    direction: Direction,
}

// ── Primitive field tests ─────────────────────────────────────────────────────

#[tokio::test]
async fn string_field_uses_field_prompt() {
    let mock = CapturingCommunicator::new(vec!["Alice", "30", "true"]);
    let result = PromptedPrimitives::elicit(&mock)
        .await
        .expect("elicit failed");
    assert_eq!(result.name, "Alice");

    let prompts = mock.received_prompts();
    assert_eq!(
        prompts[0], "What is your name?",
        "String field #[prompt] must reach send_prompt; got {:?}",
        prompts[0]
    );
}

#[tokio::test]
async fn string_field_bypasses_style_elicitation() {
    // With the field prompt override active, String::elicit must NOT trigger
    // StringStyle selection — only one send_prompt call for the name field.
    let mock = CapturingCommunicator::new(vec!["Alice", "30", "true"]);
    PromptedPrimitives::elicit(&mock)
        .await
        .expect("elicit failed");

    let prompts = mock.received_prompts();
    // Exactly 3 calls: one per field, none for style selection
    assert_eq!(
        prompts.len(),
        3,
        "Exactly one send_prompt per field expected; got: {prompts:?}"
    );
}

#[tokio::test]
async fn u8_field_uses_field_prompt() {
    let mock = CapturingCommunicator::new(vec!["Alice", "30", "true"]);
    let result = PromptedPrimitives::elicit(&mock)
        .await
        .expect("elicit failed");
    assert_eq!(result.age, 30);

    let prompts = mock.received_prompts();
    assert_eq!(
        prompts[1], "How old are you?",
        "u8 field #[prompt] must reach send_prompt; got {:?}",
        prompts[1]
    );
}

#[tokio::test]
async fn bool_field_uses_field_prompt() {
    let mock = CapturingCommunicator::new(vec!["Alice", "30", "true"]);
    let result = PromptedPrimitives::elicit(&mock)
        .await
        .expect("elicit failed");
    assert!(result.active);

    let prompts = mock.received_prompts();
    assert_eq!(
        prompts[2], "Are you active?",
        "bool field #[prompt] must reach send_prompt; got {:?}",
        prompts[2]
    );
}

// ── Enum field test ───────────────────────────────────────────────────────────

#[tokio::test]
async fn enum_field_prompt_becomes_base_prompt() {
    // The generated enum elicit combines base_prompt + options into one message.
    // With the field prompt override, base_prompt must be the field's text.
    let mock = CapturingCommunicator::new(vec!["North"]);
    let result = PromptedEnumField::elicit(&mock)
        .await
        .expect("elicit failed");
    assert_eq!(result.direction, Direction::North);

    let prompts = mock.received_prompts();
    assert_eq!(prompts.len(), 1);
    assert!(
        prompts[0].starts_with("Which direction?"),
        "Enum field #[prompt] must be the base_prompt; got {:?}",
        prompts[0]
    );
}

// ── Isolation test ────────────────────────────────────────────────────────────

#[tokio::test]
async fn field_prompt_does_not_leak_to_subsequent_fields() {
    // Each field's override must be consumed before the next field is elicited.
    let mock = CapturingCommunicator::new(vec!["Alice", "30", "true"]);
    PromptedPrimitives::elicit(&mock)
        .await
        .expect("elicit failed");

    let prompts = mock.received_prompts();
    assert_eq!(
        prompts,
        vec!["What is your name?", "How old are you?", "Are you active?"],
        "Each field must use its own prompt; got: {prompts:?}"
    );
}
