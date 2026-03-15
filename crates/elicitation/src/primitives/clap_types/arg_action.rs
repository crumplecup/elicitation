//! [`clap::ArgAction`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use clap::ArgAction;

impl Prompt for ArgAction {
    fn prompt() -> Option<&'static str> {
        Some("Choose how this argument behaves when provided:")
    }
}

impl Select for ArgAction {
    fn options() -> Vec<Self> {
        vec![
            ArgAction::Set,
            ArgAction::Append,
            ArgAction::SetTrue,
            ArgAction::SetFalse,
            ArgAction::Count,
            ArgAction::Help,
            ArgAction::HelpShort,
            ArgAction::Version,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Set (store single value)".to_string(),
            "Append (accumulate values)".to_string(),
            "SetTrue (flag → true)".to_string(),
            "SetFalse (flag → false)".to_string(),
            "Count (tally occurrences)".to_string(),
            "Help (print help)".to_string(),
            "HelpShort (print short help)".to_string(),
            "Version (print version)".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Set (store single value)" => Some(ArgAction::Set),
            "Append (accumulate values)" => Some(ArgAction::Append),
            "SetTrue (flag → true)" => Some(ArgAction::SetTrue),
            "SetFalse (flag → false)" => Some(ArgAction::SetFalse),
            "Count (tally occurrences)" => Some(ArgAction::Count),
            "Help (print help)" => Some(ArgAction::Help),
            "HelpShort (print short help)" => Some(ArgAction::HelpShort),
            "Version (print version)" => Some(ArgAction::Version),
            _ => None,
        }
    }
}

crate::default_style!(ArgAction => ArgActionStyle);

impl Elicitation for ArgAction {
    type Style = ArgActionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ArgAction");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose argument action:"),
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
                "Invalid ArgAction: {}",
                label
            )))
        })
    }
}

impl ElicitIntrospect for ArgAction {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "clap::ArgAction",
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
