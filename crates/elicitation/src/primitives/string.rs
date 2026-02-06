//! String type implementation.

use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt, Select};

/// Elicitation style variants for String.
///
/// Demonstrates Agent vs Human prompting strategies:
/// - Agent: Terse, assumes technical context
/// - Human: Friendly, more explanatory
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StringStyle {
    /// Human-friendly prompting (default).
    #[default]
    Human,
    /// Terse agent-oriented prompting.
    Agent,
}

impl Prompt for StringStyle {
    fn prompt() -> Option<&'static str> {
        Some("Select elicitation style:")
    }
}

impl Select for StringStyle {
    fn options() -> &'static [Self] {
        &[Self::Human, Self::Agent]
    }

    fn labels() -> &'static [&'static str] {
        &["human", "agent"]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "human" => Some(Self::Human),
            "agent" => Some(Self::Agent),
            _ => None,
        }
    }
}

impl Elicitation for StringStyle {
    type Style = ();

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let prompt = <Self as Prompt>::prompt().unwrap();
        let labels = <Self as Select>::labels();

        let params = crate::mcp::select_params(prompt, labels);
        let result = communicator
            .call_tool(crate::rmcp::model::CallToolRequestParams {
                meta: None,
                name: crate::mcp::tool_names::elicit_select().into(),
                arguments: Some(params),
                task: None,
            })
            .await?;

        let value = crate::mcp::extract_value(result)?;
        let label = crate::mcp::parse_string(value)?;

        <Self as Select>::from_label(&label).ok_or_else(|| {
            crate::ElicitError::from(crate::ElicitErrorKind::ParseError(format!(
                "Invalid style selection: {}",
                label
            )))
        })
    }
}

impl Prompt for String {
    fn prompt() -> Option<&'static str> {
        Some("Please enter text:")
    }
}

impl Elicitation for String {
    type Style = StringStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let style = communicator.style_or_elicit::<Self>().await?;

        tracing::debug!(?style, "Eliciting String with style");

        // Style-specific prompting
        let prompt = match style {
            StringStyle::Human => "Please provide a text value:",
            StringStyle::Agent => "Value?",
        };

        // Elicit directly with custom prompt
        communicator.send_prompt(prompt).await
    }
}
