//! Boolean type implementation using the Affirm pattern.

use crate::{
    Affirm, ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata,
};

// Generate default-only style enum
crate::default_style!(bool => BoolStyle);

impl Prompt for bool {
    fn prompt() -> Option<&'static str> {
        Some("Please answer yes or no:")
    }
}

impl Affirm for bool {}

impl Elicitation for bool {
    type Style = BoolStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        use crate::verification::types::BoolDefault;

        tracing::debug!("Eliciting bool via BoolDefault wrapper");

        // Use verification wrapper internally
        let wrapper = BoolDefault::elicit(communicator).await?;

        // Unwrap to primitive
        Ok(wrapper.into_inner())
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        use crate::verification::types::BoolDefault;
        <BoolDefault as crate::Elicitation>::kani_proof()
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

impl ElicitIntrospect for bool {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Affirm
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bool",
            description: Self::prompt(),
            details: PatternDetails::Affirm,
        }
    }
}
