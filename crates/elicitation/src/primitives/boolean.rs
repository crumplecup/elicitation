//! Boolean type implementation using the Affirm pattern.

use crate::{mcp, Affirm, ElicitResult, Elicitation, Prompt};

impl Prompt for bool {
    fn prompt() -> Option<&'static str> {
        Some("Please answer yes or no:")
    }
}

impl Affirm for bool {}

impl Elicitation for bool {
    #[tracing::instrument(skip(client))]
    async fn elicit<T: pmcp::shared::transport::Transport>(
        client: &pmcp::Client<T>,
    ) -> ElicitResult<Self> {
        let prompt = Self::prompt().unwrap();
        tracing::debug!("Eliciting boolean");

        let params = mcp::bool_params(prompt);
        let result = client
            .call_tool(mcp::tool_names::elicit_bool(), params)
            .await?;

        let value = mcp::extract_value(result)?;
        mcp::parse_bool(value)
    }
}
