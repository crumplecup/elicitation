//! [`accesskit::AriaCurrent`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use accesskit::AriaCurrent;

impl Prompt for AriaCurrent {
    fn prompt() -> Option<&'static str> {
        Some("Choose the aria-current value:")
    }
}

impl Select for AriaCurrent {
    fn options() -> Vec<Self> {
        vec![
            AriaCurrent::False,
            AriaCurrent::True,
            AriaCurrent::Page,
            AriaCurrent::Step,
            AriaCurrent::Location,
            AriaCurrent::Date,
            AriaCurrent::Time,
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

crate::default_style!(accesskit::AriaCurrent => AriaCurrentStyle);

impl Elicitation for AriaCurrent {
    type Style = AriaCurrentStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::AriaCurrent");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose AriaCurrent:"),
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
                "Invalid accesskit::AriaCurrent: {label}"
            )))
        })
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("accesskit::AriaCurrent", "false")
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("accesskit::AriaCurrent", "false")
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "accesskit::AriaCurrent",
            "false",
        )
    }
}

impl ElicitIntrospect for AriaCurrent {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::AriaCurrent",
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
