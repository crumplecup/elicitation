//! [`egui::UiKind`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use egui::UiKind;

impl Prompt for UiKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose UI region kind:")
    }
}

impl Select for UiKind {
    fn options() -> Vec<Self> {
        vec![
            UiKind::Window,
            UiKind::CentralPanel,
            UiKind::LeftPanel,
            UiKind::RightPanel,
            UiKind::TopPanel,
            UiKind::BottomPanel,
            UiKind::Modal,
            UiKind::Frame,
            UiKind::ScrollArea,
            UiKind::Resize,
            UiKind::Menu,
            UiKind::Popup,
            UiKind::Tooltip,
            UiKind::Picker,
            UiKind::TableCell,
            UiKind::GenericArea,
            UiKind::Collapsible,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Window".to_string(),
            "CentralPanel".to_string(),
            "LeftPanel".to_string(),
            "RightPanel".to_string(),
            "TopPanel".to_string(),
            "BottomPanel".to_string(),
            "Modal".to_string(),
            "Frame".to_string(),
            "ScrollArea".to_string(),
            "Resize".to_string(),
            "Menu".to_string(),
            "Popup".to_string(),
            "Tooltip".to_string(),
            "Picker".to_string(),
            "TableCell".to_string(),
            "GenericArea".to_string(),
            "Collapsible".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Window" => Some(UiKind::Window),
            "CentralPanel" => Some(UiKind::CentralPanel),
            "LeftPanel" => Some(UiKind::LeftPanel),
            "RightPanel" => Some(UiKind::RightPanel),
            "TopPanel" => Some(UiKind::TopPanel),
            "BottomPanel" => Some(UiKind::BottomPanel),
            "Modal" => Some(UiKind::Modal),
            "Frame" => Some(UiKind::Frame),
            "ScrollArea" => Some(UiKind::ScrollArea),
            "Resize" => Some(UiKind::Resize),
            "Menu" => Some(UiKind::Menu),
            "Popup" => Some(UiKind::Popup),
            "Tooltip" => Some(UiKind::Tooltip),
            "Picker" => Some(UiKind::Picker),
            "TableCell" => Some(UiKind::TableCell),
            "GenericArea" => Some(UiKind::GenericArea),
            "Collapsible" => Some(UiKind::Collapsible),
            _ => None,
        }
    }
}

crate::default_style!(egui::UiKind => UiKindStyle);

impl Elicitation for UiKind {
    type Style = UiKindStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting egui::UiKind");
        let params =
            mcp::select_params(Self::prompt().unwrap_or("Choose UiKind:"), &Self::labels());
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
                "Invalid egui::UiKind: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("egui::UiKind", "window")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("egui::UiKind", "window")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("egui::UiKind", "window")
    }
}

impl ElicitIntrospect for UiKind {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "egui::UiKind",
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
