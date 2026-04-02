//! [`accesskit::TextAlign`] elicitation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use accesskit::TextAlign;

impl Prompt for TextAlign {
    fn prompt() -> Option<&'static str> {
        Some("Choose the text alignment:")
    }
}

impl Select for TextAlign {
    fn options() -> Vec<Self> {
        vec![
            TextAlign::Left,
            TextAlign::Right,
            TextAlign::Center,
            TextAlign::Justify,
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

crate::default_style!(accesskit::TextAlign => TextAlignStyle);

impl Elicitation for TextAlign {
    type Style = TextAlignStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::TextAlign");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose TextAlign:"),
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
                "Invalid accesskit::TextAlign: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("accesskit::TextAlign", "left")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("accesskit::TextAlign", "left")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("accesskit::TextAlign", "left")
    }
}

impl ElicitIntrospect for TextAlign {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::TextAlign",
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
