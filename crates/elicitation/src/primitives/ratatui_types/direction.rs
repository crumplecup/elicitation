//! [`ratatui::layout::Direction`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use ratatui::layout::Direction;

impl Prompt for Direction {
    fn prompt() -> Option<&'static str> {
        Some("Choose layout direction:")
    }
}

impl Select for Direction {
    fn options() -> Vec<Self> {
        vec![Direction::Horizontal, Direction::Vertical]
    }

    fn labels() -> Vec<String> {
        vec!["Horizontal".to_string(), "Vertical".to_string()]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Horizontal" => Some(Direction::Horizontal),
            "Vertical" => Some(Direction::Vertical),
            _ => None,
        }
    }
}

crate::default_style!(ratatui::layout::Direction => RatatuiDirectionStyle);

impl Elicitation for Direction {
    type Style = RatatuiDirectionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ratatui::layout::Direction");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose direction:"),
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
                "Invalid ratatui::layout::Direction: {label}"
            )))
        })
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "ratatui::layout::Direction",
            "Horizontal",
        )
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "ratatui::layout::Direction",
            "Horizontal",
        )
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "ratatui::layout::Direction",
            "Horizontal",
        )
    }
}

impl ElicitIntrospect for Direction {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "ratatui::layout::Direction",
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
