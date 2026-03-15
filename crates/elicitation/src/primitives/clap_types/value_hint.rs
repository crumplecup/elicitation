//! [`clap::ValueHint`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use clap::ValueHint;

impl Prompt for ValueHint {
    fn prompt() -> Option<&'static str> {
        Some("Choose a shell completion hint for this argument's value:")
    }
}

impl Select for ValueHint {
    fn options() -> Vec<Self> {
        vec![
            ValueHint::Unknown,
            ValueHint::Other,
            ValueHint::AnyPath,
            ValueHint::FilePath,
            ValueHint::DirPath,
            ValueHint::ExecutablePath,
            ValueHint::CommandName,
            ValueHint::CommandString,
            ValueHint::CommandWithArguments,
            ValueHint::Username,
            ValueHint::Hostname,
            ValueHint::Url,
            ValueHint::EmailAddress,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Unknown".to_string(),
            "Other".to_string(),
            "AnyPath (file or directory)".to_string(),
            "FilePath".to_string(),
            "DirPath".to_string(),
            "ExecutablePath".to_string(),
            "CommandName".to_string(),
            "CommandString".to_string(),
            "CommandWithArguments".to_string(),
            "Username".to_string(),
            "Hostname".to_string(),
            "Url".to_string(),
            "EmailAddress".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Unknown" => Some(ValueHint::Unknown),
            "Other" => Some(ValueHint::Other),
            "AnyPath (file or directory)" => Some(ValueHint::AnyPath),
            "FilePath" => Some(ValueHint::FilePath),
            "DirPath" => Some(ValueHint::DirPath),
            "ExecutablePath" => Some(ValueHint::ExecutablePath),
            "CommandName" => Some(ValueHint::CommandName),
            "CommandString" => Some(ValueHint::CommandString),
            "CommandWithArguments" => Some(ValueHint::CommandWithArguments),
            "Username" => Some(ValueHint::Username),
            "Hostname" => Some(ValueHint::Hostname),
            "Url" => Some(ValueHint::Url),
            "EmailAddress" => Some(ValueHint::EmailAddress),
            _ => None,
        }
    }
}

crate::default_style!(ValueHint => ValueHintStyle);

impl Elicitation for ValueHint {
    type Style = ValueHintStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ValueHint");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose value hint:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid ValueHint: {}",
                label
            )))
        })
    }
}

impl ElicitIntrospect for ValueHint {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "clap::ValueHint",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}
