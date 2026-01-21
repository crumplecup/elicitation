//! Char type implementation.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt, mcp};

// Generate default-only style enum
crate::default_style!(char => CharStyle);

impl Prompt for char {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a single character:")
    }
}

impl Elicitation for char {
    type Style = CharStyle;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        let prompt = Self::prompt().unwrap();
        tracing::debug!("Eliciting char");

        let params = mcp::text_params(prompt);
        let result = client
            .peer()
            .call_tool(rmcp::model::CallToolRequestParam {
                name: mcp::tool_names::elicit_text().into(),
                arguments: Some(params),
                task: None,
            })
            .await?;

        let value = mcp::extract_value(result)?;
        let string = mcp::parse_string(value)?;
        
        // Get first character from string
        string
            .chars()
            .next()
            .ok_or_else(|| crate::ElicitError::ParseError("Empty string, expected character".into()))
    }
}
