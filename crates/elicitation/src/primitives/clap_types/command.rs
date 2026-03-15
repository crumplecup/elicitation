//! [`clap::Command`] elicitation.
//!
//! This is a partial implementation covering core fields only (name, about).
//! Full `Command` configuration requires many optional fields and programmatic
//! setup that is better handled directly in application code.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata, mcp,
};
use clap::Command;

crate::default_style!(Command => CommandStyle);

impl Prompt for Command {
    fn prompt() -> Option<&'static str> {
        Some("Define a CLI command (core fields only):")
    }
}

impl Elicitation for Command {
    type Style = CommandStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Command");

        let name_params = mcp::text_params("Enter the command name (e.g. 'my-app'):");
        let name_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(name_params),
            )
            .await?;
        let name_value = mcp::extract_value(name_result)?;
        let name_str = mcp::parse_string(name_value)?;
        let mut cmd = Command::new(name_str.trim().to_string());

        let about_params = mcp::text_params(
            "Enter a short description (about text, optional — leave empty to skip):",
        );
        let about_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(about_params),
            )
            .await?;
        let about_value = mcp::extract_value(about_result)?;
        let about_str = mcp::parse_string(about_value)?;
        if !about_str.trim().is_empty() {
            cmd = cmd.about(about_str.trim().to_string());
        }

        let version_params = mcp::text_params(
            "Enter a version string (optional — leave empty to skip, e.g. '1.0.0'):",
        );
        let version_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(version_params),
            )
            .await?;
        let version_value = mcp::extract_value(version_result)?;
        let version_str = mcp::parse_string(version_value)?;
        if !version_str.trim().is_empty() {
            cmd = cmd.version(version_str.trim().to_string());
        }

        tracing::debug!("Elicited Command");
        Ok(cmd)
    }
}

impl ElicitIntrospect for Command {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "clap::Command",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    crate::FieldInfo {
                        name: "name",
                        type_name: "str",
                        prompt: Some("Enter the command name:"),
                    },
                    crate::FieldInfo {
                        name: "about",
                        type_name: "str",
                        prompt: Some("Enter a short description (optional):"),
                    },
                    crate::FieldInfo {
                        name: "version",
                        type_name: "str",
                        prompt: Some("Enter a version string (optional):"),
                    },
                ],
            },
        }
    }
}
