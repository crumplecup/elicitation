//! `bytes::Bytes` elicitation support.
//!
//! Direct elicitation and code recovery both preserve the exact visible byte
//! sequence.

use bytes::Bytes;

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};

crate::default_style!(Bytes => BytesStyle);

#[tracing::instrument(skip(bytes))]
fn visible_bytes(bytes: &Bytes) -> &[u8] {
    bytes.as_ref()
}

impl Prompt for Bytes {
    fn prompt() -> Option<&'static str> {
        Some("Construct bytes::Bytes from its exact byte contents.")
    }
}

impl Elicitation for Bytes {
    type Style = BytesStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting bytes::Bytes");

        let bytes = Vec::<u8>::elicit(communicator).await?;
        tracing::debug!(byte_len = bytes.len(), "Bytes payload selected");

        Ok(Bytes::from(bytes))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("bytes")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("bytes")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("bytes")
    }
}

impl ElicitIntrospect for Bytes {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bytes::Bytes",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![crate::FieldInfo {
                    name: "bytes",
                    type_name: "Vec<u8>",
                    prompt: Some("Exact byte contents"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for Bytes {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bytes::Bytes".to_string(),
            fields: vec![(
                "bytes".to_string(),
                Box::new(
                    <Vec<u8> as crate::ElicitPromptTree>::prompt_tree()
                        .with_prompt(Some("Exact byte contents".to_string())),
                ),
            )],
        }
    }
}

impl ToCodeLiteral for Bytes {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let bytes = visible_bytes(self).iter();

        quote::quote! {
            ::bytes::Bytes::from(::std::vec![#(#bytes),*])
        }
    }
}
