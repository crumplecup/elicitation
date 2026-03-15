//! [`clap::ColorChoice`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use clap::ColorChoice;

impl Prompt for ColorChoice {
    fn prompt() -> Option<&'static str> {
        Some("Choose terminal color output mode:")
    }
}

impl Select for ColorChoice {
    fn options() -> Vec<Self> {
        vec![ColorChoice::Auto, ColorChoice::Always, ColorChoice::Never]
    }

    fn labels() -> Vec<String> {
        vec![
            "Auto (detect terminal)".to_string(),
            "Always".to_string(),
            "Never".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Auto (detect terminal)" => Some(ColorChoice::Auto),
            "Always" => Some(ColorChoice::Always),
            "Never" => Some(ColorChoice::Never),
            _ => None,
        }
    }
}

crate::default_style!(ColorChoice => ColorChoiceStyle);

impl Elicitation for ColorChoice {
    type Style = ColorChoiceStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ColorChoice");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose color mode:"),
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
                "Invalid ColorChoice: {}",
                label
            )))
        })
    }
}

impl ElicitIntrospect for ColorChoice {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "clap::ColorChoice",
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
