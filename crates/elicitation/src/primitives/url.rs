//! URL type implementation.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata,
};

// Generate default-only style enum
crate::default_style!(url::Url => UrlStyle);

impl Prompt for url::Url {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a URL:")
    }
}

impl Elicitation for url::Url {
    type Style = UrlStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        use crate::verification::types::UrlValid;

        tracing::debug!("Eliciting Url via UrlValid wrapper");

        // Use verification wrapper internally
        let wrapper = UrlValid::elicit(communicator).await?;

        // Unwrap to primitive
        Ok(wrapper.into_inner())
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

impl ElicitIntrospect for url::Url {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "url::Url",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}
