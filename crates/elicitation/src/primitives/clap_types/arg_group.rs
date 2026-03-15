//! [`clap::ArgGroup`] elicitation.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata, mcp,
};
use clap::ArgGroup;

crate::default_style!(ArgGroup => ArgGroupStyle);

impl Prompt for ArgGroup {
    fn prompt() -> Option<&'static str> {
        Some("Define a CLI argument group:")
    }
}

impl Elicitation for ArgGroup {
    type Style = ArgGroupStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ArgGroup");

        let id_params = mcp::text_params("Enter the group identifier (e.g. 'output-options'):");
        let id_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(id_params),
            )
            .await?;
        let id_value = mcp::extract_value(id_result)?;
        let id_str = mcp::parse_string(id_value)?;
        let mut group = ArgGroup::new(id_str.trim().to_string());

        let args_params = mcp::text_params(
            "Enter argument IDs to include in this group (comma-separated, e.g. 'arg1,arg2'), \
             or leave empty to skip:",
        );
        let args_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(args_params),
            )
            .await?;
        let args_value = mcp::extract_value(args_result)?;
        let args_str = mcp::parse_string(args_value)?;
        if !args_str.trim().is_empty() {
            let arg_ids: Vec<String> = args_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            group = group.args(arg_ids);
        }

        tracing::debug!("Elicited ArgGroup");
        Ok(group)
    }
}

impl ElicitIntrospect for ArgGroup {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "clap::ArgGroup",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    crate::FieldInfo {
                        name: "id",
                        type_name: "clap::Id",
                        prompt: Some("Enter the group identifier:"),
                    },
                    crate::FieldInfo {
                        name: "args",
                        type_name: "Vec<clap::Id>",
                        prompt: Some("Enter argument IDs in this group (comma-separated):"),
                    },
                ],
            },
        }
    }
}
