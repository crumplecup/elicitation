//! String type implementation.

use crate::{mcp, ElicitResult, Elicitation, Prompt};

impl Prompt for String {
    fn prompt() -> Option<&'static str> {
        Some("Please enter text:")
    }
}

impl Elicitation for String {
    #[tracing::instrument(skip(client))]
    async fn elicit<T: pmcp::shared::transport::Transport>(
        client: &pmcp::Client<T>,
    ) -> ElicitResult<Self> {
        let prompt = Self::prompt().unwrap();
        tracing::debug!("Eliciting string");

        let params = mcp::text_params(prompt);
        let result = client
            .call_tool(mcp::tool_names::elicit_text(), params)
            .await?;

        let value = mcp::extract_value(result)?;
        mcp::parse_string(value)
    }
}
