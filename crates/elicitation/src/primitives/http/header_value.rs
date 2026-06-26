//! `http::HeaderValue` elicitation (Primitive pattern).

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitPromptTree, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, PromptTree, TypeMetadata, mcp,
};
use http::HeaderValue;

crate::default_style!(HeaderValue => HeaderValueStyle);

impl Prompt for HeaderValue {
    fn prompt() -> Option<&'static str> {
        Some("Enter HTTP header value:")
    }
}

impl Elicitation for HeaderValue {
    type Style = HeaderValueStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting http::HeaderValue");

        let params = mcp::text_params(Self::prompt().unwrap_or("Enter HTTP header value:"));

        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(params),
            )
            .await?;

        let value = mcp::extract_value(result)?;
        let raw = mcp::parse_string(value)?;

        parse_header_value(&raw)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("header_value")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("header_value")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("header_value")
    }
}

impl ElicitIntrospect for HeaderValue {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "http::HeaderValue",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for HeaderValue {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt()
                .unwrap_or("Enter HTTP header value:")
                .to_string(),
            type_name: "http::HeaderValue".to_string(),
        }
    }
}

impl crate::emit_code::ToCodeLiteral for HeaderValue {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let bytes = self.as_bytes().iter();

        quote::quote! {
            http::header::HeaderValue::from_bytes(&[#(#bytes),*])
                .map_err(elicitation::ElicitError::from)?
        }
    }
}

#[tracing::instrument]
fn parse_header_value(raw: &str) -> ElicitResult<HeaderValue> {
    HeaderValue::from_str(raw).map_err(crate::ElicitError::from)
}
