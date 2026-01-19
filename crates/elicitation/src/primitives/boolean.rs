//! Boolean type implementation using the Affirm pattern.

use crate::{Affirm, ElicitClient, ElicitResult, Elicitation, Prompt, mcp};

// Generate default-only style enum
crate::default_style!(bool => BoolStyle);

impl Prompt for bool {
    fn prompt() -> Option<&'static str> {
        Some("Please answer yes or no:")
    }
}

impl Affirm for bool {}

impl Elicitation for bool {
    type Style = BoolStyle;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        let prompt = Self::prompt().unwrap();
        tracing::debug!("Eliciting boolean");

        let params = mcp::bool_params(prompt);
        let result = client
            .peer()
            .call_tool(rmcp::model::CallToolRequestParam {
                name: mcp::tool_names::elicit_bool().into(),
                arguments: Some(params),
                task: None,
            })
            .await?;

        let value = mcp::extract_value(result)?;
        mcp::parse_bool(value)
    }
}
