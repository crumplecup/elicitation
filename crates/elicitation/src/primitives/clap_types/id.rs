//! [`clap::Id`] elicitation.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata, mcp,
};
use clap::Id;

crate::default_style!(Id => IdStyle);

impl Prompt for Id {
    fn prompt() -> Option<&'static str> {
        Some("Enter an argument/group identifier (e.g. 'output-file'):")
    }
}

impl Elicitation for Id {
    type Style = IdStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Id");
        let params = mcp::text_params(Self::prompt().unwrap());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let s = mcp::parse_string(value)?;
        tracing::debug!(id = %s, "Elicited Id");
        Ok(Id::from(s.trim().to_string()))
    }
}

impl ElicitIntrospect for Id {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "clap::Id",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}
