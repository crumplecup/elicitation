//! [`ratatui::widgets::BorderType`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use ratatui::widgets::BorderType;

impl Prompt for BorderType {
    fn prompt() -> Option<&'static str> {
        Some("Choose border style:")
    }
}

impl Select for BorderType {
    fn options() -> Vec<Self> {
        vec![
            BorderType::Plain,
            BorderType::Rounded,
            BorderType::Double,
            BorderType::Thick,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Plain".to_string(),
            "Rounded".to_string(),
            "Double".to_string(),
            "Thick".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Plain" => Some(BorderType::Plain),
            "Rounded" => Some(BorderType::Rounded),
            "Double" => Some(BorderType::Double),
            "Thick" => Some(BorderType::Thick),
            _ => None,
        }
    }
}

crate::default_style!(ratatui::widgets::BorderType => BorderTypeStyle);

impl Elicitation for BorderType {
    type Style = BorderTypeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ratatui::widgets::BorderType");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose border type:"),
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
                "Invalid ratatui::widgets::BorderType: {label}"
            )))
        })
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "ratatui::widgets::BorderType",
            "Plain",
        )
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "ratatui::widgets::BorderType",
            "Plain",
        )
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "ratatui::widgets::BorderType",
            "Plain",
        )
    }
}

impl ElicitIntrospect for BorderType {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "ratatui::widgets::BorderType",
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
