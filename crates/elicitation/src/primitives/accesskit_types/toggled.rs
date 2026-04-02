//! [`accesskit::Toggled`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use accesskit::Toggled;

impl Prompt for Toggled {
    fn prompt() -> Option<&'static str> {
        Some("Choose the toggled state:")
    }
}

impl Select for Toggled {
    fn options() -> Vec<Self> {
        vec![Toggled::False, Toggled::True, Toggled::Mixed]
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

crate::default_style!(accesskit::Toggled => ToggledStyle);

impl Elicitation for Toggled {
    type Style = ToggledStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::Toggled");
        let params =
            mcp::select_params(Self::prompt().unwrap_or("Choose Toggled:"), &Self::labels());
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
                "Invalid accesskit::Toggled: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("accesskit::Toggled", "false")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("accesskit::Toggled", "false")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("accesskit::Toggled", "false")
    }
}

impl ElicitIntrospect for Toggled {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::Toggled",
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
