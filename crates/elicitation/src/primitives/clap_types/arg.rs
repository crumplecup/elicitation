//! [`clap::Arg`] elicitation.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata, mcp,
};
use clap::Arg;

use super::{ArgActionStyle, ValueHintStyle};

crate::default_style!(Arg => ArgStyle);

impl Prompt for Arg {
    fn prompt() -> Option<&'static str> {
        Some("Define a CLI argument:")
    }
}

impl Elicitation for Arg {
    type Style = ArgStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Arg");

        let id_params = mcp::text_params("Enter the argument identifier (e.g. 'output-file'):");
        let id_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(id_params),
            )
            .await?;
        let id_value = mcp::extract_value(id_result)?;
        let id_str = mcp::parse_string(id_value)?;
        let mut arg = Arg::new(id_str.trim().to_string());

        let help_params = mcp::text_params("Enter help text (optional, leave empty to skip):");
        let help_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(help_params),
            )
            .await?;
        let help_value = mcp::extract_value(help_result)?;
        let help_str = mcp::parse_string(help_value)?;
        if !help_str.trim().is_empty() {
            arg = arg.help(help_str.trim().to_string());
        }

        let action = clap::ArgAction::elicit(
            &communicator.with_style::<clap::ArgAction, ArgActionStyle>(ArgActionStyle::default()),
        )
        .await?;
        arg = arg.action(action);

        let hint = clap::ValueHint::elicit(
            &communicator.with_style::<clap::ValueHint, ValueHintStyle>(ValueHintStyle::default()),
        )
        .await?;
        arg = arg.value_hint(hint);

        tracing::debug!("Elicited Arg");
        Ok(arg)
    }
}

impl ElicitIntrospect for Arg {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "clap::Arg",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    crate::FieldInfo {
                        name: "id",
                        type_name: "clap::Id",
                        prompt: Some("Enter the argument identifier:"),
                    },
                    crate::FieldInfo {
                        name: "help",
                        type_name: "str",
                        prompt: Some("Enter help text (optional):"),
                    },
                    crate::FieldInfo {
                        name: "action",
                        type_name: "clap::ArgAction",
                        prompt: Some("Choose how this argument behaves:"),
                    },
                    crate::FieldInfo {
                        name: "value_hint",
                        type_name: "clap::ValueHint",
                        prompt: Some("Choose a shell completion hint:"),
                    },
                ],
            },
        }
    }
}
