//! Boolean type implementation using the Affirm pattern.

use crate::{mcp, Affirm, ElicitResult, Elicitation, Prompt};
use rmcp::service::{Peer, RoleClient};

impl Prompt for bool {
    fn prompt() -> Option<&'static str> {
        Some("Please answer yes or no:")
    }
}

impl Affirm for bool {}

impl Elicitation for bool {
    #[tracing::instrument(skip(client))]
    async fn elicit(client: &Peer<RoleClient>) -> ElicitResult<Self> {
        let prompt = Self::prompt().unwrap();
        tracing::debug!("Eliciting boolean");

        let params = mcp::bool_params(prompt);
        let result = client
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
