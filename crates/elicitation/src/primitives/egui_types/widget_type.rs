//! [`egui::WidgetType`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use egui::WidgetType;

impl Prompt for WidgetType {
    fn prompt() -> Option<&'static str> {
        Some("Choose widget type:")
    }
}

impl Select for WidgetType {
    fn options() -> Vec<Self> {
        vec![
            WidgetType::Label,
            WidgetType::Link,
            WidgetType::TextEdit,
            WidgetType::Button,
            WidgetType::Checkbox,
            WidgetType::RadioButton,
            WidgetType::RadioGroup,
            WidgetType::SelectableLabel,
            WidgetType::ComboBox,
            WidgetType::Slider,
            WidgetType::DragValue,
            WidgetType::ColorButton,
            WidgetType::Image,
            WidgetType::CollapsingHeader,
            WidgetType::Panel,
            WidgetType::ProgressIndicator,
            WidgetType::Window,
            WidgetType::Other,
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

crate::default_style!(egui::WidgetType => WidgetTypeStyle);

impl Elicitation for WidgetType {
    type Style = WidgetTypeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting egui::WidgetType");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose WidgetType:"),
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
                "Invalid egui::WidgetType: {label}"
            )))
        })
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("egui::WidgetType", "label")
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("egui::WidgetType", "label")
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("egui::WidgetType", "label")
    }
}

impl ElicitIntrospect for WidgetType {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "egui::WidgetType",
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
