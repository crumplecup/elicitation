//! `reqwest::Client` elicitation (Primitive pattern).
//!
//! A `Client` is always default-constructed — there is no user input to elicit.
//! The `Elicitation` impl gives agents the full power-trait suite: introspection,
//! kani proofs, and style negotiation.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata,
};

crate::default_style!(reqwest::Client => ClientStyle);

impl Prompt for reqwest::Client {
    fn prompt() -> Option<&'static str> {
        Some("HTTP client (auto-constructed with default settings)")
    }
}

impl Elicitation for reqwest::Client {
    type Style = ClientStyle;

    #[tracing::instrument(skip(_communicator))]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Constructing reqwest::Client");
        Ok(reqwest::Client::new())
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("client")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("client")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("client")
    }
}

impl ElicitIntrospect for reqwest::Client {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "reqwest::Client",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl crate::ElicitPromptTree for reqwest::Client {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Leaf {
            prompt: "HTTP client (auto-constructed with default settings)".to_string(),
            type_name: "reqwest::Client".to_string(),
        }
    }
}

impl crate::emit_code::ToCodeLiteral for reqwest::Client {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        quote::quote! { reqwest::Client::new() }
    }
}
