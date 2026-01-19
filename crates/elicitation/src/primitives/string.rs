//! String type implementation.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt, mcp};

// Generate default-only style enum
crate::default_style!(String => StringStyle);

impl Prompt for String {
    fn prompt() -> Option<&'static str> {
        Some("Please enter text:")
    }
}

impl Elicitation for String {
    type Style = StringStyle;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        let prompt = Self::prompt().unwrap();
        tracing::debug!("Eliciting string");

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
        mcp::parse_string(value)
    }
}
