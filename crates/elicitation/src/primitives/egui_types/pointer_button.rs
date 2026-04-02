//! [`egui::PointerButton`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use egui::PointerButton;

impl Prompt for PointerButton {
    fn prompt() -> Option<&'static str> {
        Some("Choose pointer button:")
    }
}

impl Select for PointerButton {
    fn options() -> Vec<Self> {
        vec![
            PointerButton::Primary,
            PointerButton::Secondary,
            PointerButton::Middle,
            PointerButton::Extra1,
            PointerButton::Extra2,
        ]
    }

    fn labels() -> Vec<String> {
        Self::options()
            .iter()
            .map(|v| {
                serde_json::to_string(v)
                    .unwrap()
                    .trim_matches('"')
                    .to_string()
            })
            .collect()
    }

    fn from_label(label: &str) -> Option<Self> {
        serde_json::from_str(&format!("\"{}\"", label)).ok()
    }
}

crate::default_style!(egui::PointerButton => PointerButtonStyle);

impl Elicitation for PointerButton {
    type Style = PointerButtonStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting egui::PointerButton");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose PointerButton:"),
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
                "Invalid egui::PointerButton: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("egui::PointerButton", "primary")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("egui::PointerButton", "primary")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("egui::PointerButton", "primary")
    }
}

impl ElicitIntrospect for PointerButton {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "egui::PointerButton",
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
