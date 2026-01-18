//! String type implementation.

use crate::{ElicitResult, Elicitation, Prompt, mcp};
use rmcp::service::{Peer, RoleClient};

impl Prompt for String {
    fn prompt() -> Option<&'static str> {
        Some("Please enter text:")
    }
}

impl Elicitation for String {
    #[tracing::instrument(skip(client))]
    async fn elicit(client: &Peer<RoleClient>) -> ElicitResult<Self> {
        let prompt = Self::prompt().unwrap();
        tracing::debug!("Eliciting string");

        let params = mcp::text_params(prompt);
        let result = client
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
