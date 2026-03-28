//! [`egui::CursorIcon`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use egui::CursorIcon;

impl Prompt for CursorIcon {
    fn prompt() -> Option<&'static str> {
        Some("Choose cursor icon:")
    }
}

impl Select for CursorIcon {
    fn options() -> Vec<Self> {
        vec![
            CursorIcon::Default,
            CursorIcon::None,
            CursorIcon::ContextMenu,
            CursorIcon::Help,
            CursorIcon::PointingHand,
            CursorIcon::Progress,
            CursorIcon::Wait,
            CursorIcon::Cell,
            CursorIcon::Crosshair,
            CursorIcon::Text,
            CursorIcon::VerticalText,
            CursorIcon::Alias,
            CursorIcon::Copy,
            CursorIcon::Move,
            CursorIcon::NoDrop,
            CursorIcon::NotAllowed,
            CursorIcon::Grab,
            CursorIcon::Grabbing,
            CursorIcon::AllScroll,
            CursorIcon::ResizeHorizontal,
            CursorIcon::ResizeNeSw,
            CursorIcon::ResizeNwSe,
            CursorIcon::ResizeVertical,
            CursorIcon::ResizeEast,
            CursorIcon::ResizeNorth,
            CursorIcon::ResizeNorthEast,
            CursorIcon::ResizeNorthWest,
            CursorIcon::ResizeSouth,
            CursorIcon::ResizeSouthEast,
            CursorIcon::ResizeSouthWest,
            CursorIcon::ResizeWest,
            CursorIcon::ResizeColumn,
            CursorIcon::ResizeRow,
            CursorIcon::ZoomIn,
            CursorIcon::ZoomOut,
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

crate::default_style!(egui::CursorIcon => CursorIconStyle);

impl Elicitation for CursorIcon {
    type Style = CursorIconStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting egui::CursorIcon");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose CursorIcon:"),
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
                "Invalid egui::CursorIcon: {label}"
            )))
        })
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("egui::CursorIcon", "default")
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("egui::CursorIcon", "default")
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("egui::CursorIcon", "default")
    }
}

impl ElicitIntrospect for CursorIcon {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "egui::CursorIcon",
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
