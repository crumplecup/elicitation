//! `reqwest::Body` elicitation support.
//!
//! Direct elicitation models the in-memory body subset, which is also the
//! subset recoverable from `&self` during code emission.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};

crate::default_style!(Body => BodyStyle);

#[tracing::instrument(skip(body))]
fn body_bytes(body: &reqwest::Body) -> Option<&[u8]> {
    body.as_bytes()
}

impl Prompt for reqwest::Body {
    fn prompt() -> Option<&'static str> {
        Some("Construct an in-memory HTTP body from its raw bytes.")
    }
}

impl Elicitation for reqwest::Body {
    type Style = BodyStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting reqwest::Body");

        let bytes = Vec::<u8>::elicit(communicator).await?;
        tracing::debug!(byte_len = bytes.len(), "Body bytes selected");

        Ok(reqwest::Body::from(bytes))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("body")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("body")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("body")
    }
}

impl ElicitIntrospect for reqwest::Body {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "reqwest::Body",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![crate::FieldInfo {
                    name: "bytes",
                    type_name: "Vec<u8>",
                    prompt: Some("Raw HTTP body bytes"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for reqwest::Body {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "reqwest::Body".to_string(),
            fields: vec![(
                "bytes".to_string(),
                Box::new(
                    <Vec<u8> as crate::ElicitPromptTree>::prompt_tree()
                        .with_prompt(Some("Raw HTTP body bytes".to_string())),
                ),
            )],
        }
    }
}

impl ToCodeLiteral for reqwest::Body {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let Some(bytes) = body_bytes(self) else {
            let message = "reqwest::Body code recovery requires an in-memory body; streamed bodies cannot be reconstructed from &self";
            return quote::quote! {{
                compile_error!(#message);
            }};
        };
        let bytes = bytes.iter();

        quote::quote! {
            reqwest::Body::from(::std::vec![#(#bytes),*])
        }
    }
}
