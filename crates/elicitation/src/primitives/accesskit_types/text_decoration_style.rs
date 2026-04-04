//! [`accesskit::TextDecorationStyle`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use accesskit::TextDecorationStyle;

impl Prompt for TextDecorationStyle {
    fn prompt() -> Option<&'static str> {
        Some("Choose the text decoration style:")
    }
}

impl Select for TextDecorationStyle {
    fn options() -> Vec<Self> {
        vec![
            TextDecorationStyle::Solid,
            TextDecorationStyle::Dotted,
            TextDecorationStyle::Dashed,
            TextDecorationStyle::Double,
            TextDecorationStyle::Wavy,
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

crate::default_style!(accesskit::TextDecorationStyle => TextDecorationStyleStyle);

impl Elicitation for TextDecorationStyle {
    type Style = TextDecorationStyleStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::TextDecorationStyle");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose TextDecorationStyle:"),
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
                "Invalid accesskit::TextDecorationStyle: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "accesskit::TextDecorationStyle",
            "solid",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "accesskit::TextDecorationStyle",
            "solid",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "accesskit::TextDecorationStyle",
            "solid",
        )
    }
}

impl ElicitIntrospect for TextDecorationStyle {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::TextDecorationStyle",
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
