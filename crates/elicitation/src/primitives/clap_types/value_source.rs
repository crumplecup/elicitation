//! [`clap::ValueSource`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use clap::parser::ValueSource;

impl Prompt for ValueSource {
    fn prompt() -> Option<&'static str> {
        Some("Choose where this argument's value should come from:")
    }
}

impl Select for ValueSource {
    fn options() -> Vec<Self> {
        vec![
            ValueSource::DefaultValue,
            ValueSource::EnvVariable,
            ValueSource::CommandLine,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "DefaultValue".to_string(),
            "EnvVariable".to_string(),
            "CommandLine".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "DefaultValue" => Some(ValueSource::DefaultValue),
            "EnvVariable" => Some(ValueSource::EnvVariable),
            "CommandLine" => Some(ValueSource::CommandLine),
            _ => None,
        }
    }
}

crate::default_style!(ValueSource => ValueSourceStyle);

impl Elicitation for ValueSource {
    type Style = ValueSourceStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ValueSource");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose value source:"),
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
                "Invalid ValueSource: {}",
                label
            )))
        })
    }
}

impl ElicitIntrospect for ValueSource {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "clap::ValueSource",
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
