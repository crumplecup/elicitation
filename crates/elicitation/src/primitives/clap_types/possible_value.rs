//! [`clap::builder::PossibleValue`] elicitation.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata, mcp,
};
use clap::builder::PossibleValue;

crate::default_style!(PossibleValue => PossibleValueStyle);

impl Prompt for PossibleValue {
    fn prompt() -> Option<&'static str> {
        Some("Enter a possible value name:")
    }
}

impl Elicitation for PossibleValue {
    type Style = PossibleValueStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PossibleValue");

        let name_params = mcp::text_params("Enter the possible value name (required):");
        let name_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(name_params),
            )
            .await?;
        let name_value = mcp::extract_value(name_result)?;
        let name = mcp::parse_string(name_value)?;

        let help_params =
            mcp::text_params("Enter help text for this value (optional, leave empty to skip):");
        let help_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(help_params),
            )
            .await?;
        let help_value = mcp::extract_value(help_result)?;
        let help = mcp::parse_string(help_value)?;

        tracing::debug!(name = %name, "Elicited PossibleValue");
        let mut pv = PossibleValue::new(name);
        if !help.trim().is_empty() {
            pv = pv.help(help);
        }
        Ok(pv)
    }
}

impl ElicitIntrospect for PossibleValue {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "clap::builder::PossibleValue",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    crate::FieldInfo {
                        name: "name",
                        type_name: "str",
                        prompt: Some("Enter the possible value name (required):"),
                    },
                    crate::FieldInfo {
                        name: "help",
                        type_name: "str",
                        prompt: Some("Enter help text for this value (optional):"),
                    },
                ],
            },
        }
    }
}
