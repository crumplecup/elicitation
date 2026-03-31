//! [`ratatui::layout::Alignment`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use ratatui::layout::Alignment;

impl Prompt for Alignment {
    fn prompt() -> Option<&'static str> {
        Some("Choose text alignment:")
    }
}

impl Select for Alignment {
    fn options() -> Vec<Self> {
        vec![Alignment::Left, Alignment::Center, Alignment::Right]
    }

    fn labels() -> Vec<String> {
        vec![
            "Left".to_string(),
            "Center".to_string(),
            "Right".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Left" => Some(Alignment::Left),
            "Center" => Some(Alignment::Center),
            "Right" => Some(Alignment::Right),
            _ => None,
        }
    }
}

crate::default_style!(ratatui::layout::Alignment => AlignmentStyle);

impl Elicitation for Alignment {
    type Style = AlignmentStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ratatui::layout::Alignment");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose alignment:"),
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
                "Invalid ratatui::layout::Alignment: {label}"
            )))
        })
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "ratatui::layout::Alignment",
            "Left",
        )
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "ratatui::layout::Alignment",
            "Left",
        )
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "ratatui::layout::Alignment",
            "Left",
        )
    }
}

impl ElicitIntrospect for Alignment {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "ratatui::layout::Alignment",
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
