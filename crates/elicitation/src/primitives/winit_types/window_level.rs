//! [`winit::window::WindowLevel`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use winit::window::WindowLevel;

impl Prompt for WindowLevel {
    fn prompt() -> Option<&'static str> {
        Some("Choose window stacking level:")
    }
}

impl Select for WindowLevel {
    fn options() -> Vec<Self> {
        vec![
            WindowLevel::AlwaysOnBottom,
            WindowLevel::Normal,
            WindowLevel::AlwaysOnTop,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "AlwaysOnBottom".to_string(),
            "Normal".to_string(),
            "AlwaysOnTop".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "AlwaysOnBottom" => Some(WindowLevel::AlwaysOnBottom),
            "Normal" => Some(WindowLevel::Normal),
            "AlwaysOnTop" => Some(WindowLevel::AlwaysOnTop),
            _ => None,
        }
    }
}

crate::default_style!(winit::window::WindowLevel => WindowLevelStyle);

impl Elicitation for WindowLevel {
    type Style = WindowLevelStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting winit::window::WindowLevel");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose window level:"),
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
                "Invalid winit::window::WindowLevel: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "winit::window::WindowLevel",
            "AlwaysOnBottom",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "winit::window::WindowLevel",
            "AlwaysOnBottom",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "winit::window::WindowLevel",
            "AlwaysOnBottom",
        )
    }
}

impl ElicitIntrospect for WindowLevel {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "winit::window::WindowLevel",
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
