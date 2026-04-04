//! Char type implementation.

use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt, mcp};

// Generate default-only style enum
crate::default_style!(char => CharStyle);

impl Prompt for char {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a single character:")
    }
}

impl Elicitation for char {
    type Style = CharStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>("value", "char", &crate::style::PromptContext::new(0, 1))
            .unwrap_or(None)
            .unwrap_or_else(|| Self::prompt().unwrap().to_string());
        tracing::debug!(%prompt, "Eliciting char");

        let params = mcp::text_params(&prompt);
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(params),
            )
            .await?;

        let value = mcp::extract_value(result)?;
        let string = mcp::parse_string(value)?;

        // Get first character from string
        string.chars().next().ok_or_else(|| {
            crate::ElicitError::new(crate::ElicitErrorKind::InvalidFormat {
                expected: "non-empty string with at least one character".to_string(),
                received: "empty string".to_string(),
            })
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_char()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_char()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_char()
    }
}

impl crate::ElicitIntrospect for char {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }
    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "char",
            description: <char as crate::Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}
