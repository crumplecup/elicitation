//! `http::HeaderName` elicitation (Primitive pattern).

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitPromptTree, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, PromptTree, TypeMetadata, mcp,
};
use http::HeaderName;

crate::default_style!(HeaderName => HeaderNameStyle);

impl Prompt for HeaderName {
    fn prompt() -> Option<&'static str> {
        Some("Enter HTTP header name:")
    }
}

impl Elicitation for HeaderName {
    type Style = HeaderNameStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting http::HeaderName");

        let params = mcp::text_params(Self::prompt().unwrap_or("Enter HTTP header name:"));

        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(params),
            )
            .await?;

        let value = mcp::extract_value(result)?;
        let raw = mcp::parse_string(value)?;

        parse_header_name(&raw)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("header_name")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("header_name")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("header_name")
    }
}

impl ElicitIntrospect for HeaderName {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "http::HeaderName",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for HeaderName {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt()
                .unwrap_or("Enter HTTP header name:")
                .to_string(),
            type_name: "http::HeaderName".to_string(),
        }
    }
}

impl crate::emit_code::ToCodeLiteral for HeaderName {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let name = self.as_str();

        quote::quote! {
            http::header::HeaderName::from_bytes(#name.as_bytes())
                .map_err(elicitation::ElicitError::from)?
        }
    }
}

#[tracing::instrument]
fn parse_header_name(raw: &str) -> ElicitResult<HeaderName> {
    HeaderName::from_bytes(raw.trim().as_bytes()).map_err(crate::ElicitError::from)
}
