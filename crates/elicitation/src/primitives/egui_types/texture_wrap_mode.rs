//! [`egui::TextureWrapMode`](egui::epaint::textures::TextureWrapMode) elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use egui::epaint::textures::TextureWrapMode;

impl Prompt for TextureWrapMode {
    fn prompt() -> Option<&'static str> {
        Some("Choose texture wrap mode:")
    }
}

impl Select for TextureWrapMode {
    fn options() -> Vec<Self> {
        vec![
            TextureWrapMode::ClampToEdge,
            TextureWrapMode::Repeat,
            TextureWrapMode::MirroredRepeat,
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

crate::default_style!(egui::epaint::textures::TextureWrapMode => TextureWrapModeStyle);

impl Elicitation for TextureWrapMode {
    type Style = TextureWrapModeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting egui::TextureWrapMode");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose TextureWrapMode:"),
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
                "Invalid egui::TextureWrapMode: {label}"
            )))
        })
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "egui::TextureWrapMode",
            "clamptoedge",
        )
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "egui::TextureWrapMode",
            "clamptoedge",
        )
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "egui::TextureWrapMode",
            "clamptoedge",
        )
    }
}

impl ElicitIntrospect for TextureWrapMode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "egui::TextureWrapMode",
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
