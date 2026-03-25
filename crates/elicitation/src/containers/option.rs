//! Option<T> implementation for optional value elicitation.

use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};

// For generic types, we create default-only style that ignores the type parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OptionStyle {
    #[default]
    Default,
}

impl Prompt for OptionStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for OptionStyle {
    type Style = OptionStyle;

    #[tracing::instrument(skip(_communicator), level = "trace")]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(Self::Default)
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_single_variant_enum("OptionStyle")
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_single_variant_enum("OptionStyle")
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_single_variant_enum("OptionStyle")
    }
}

impl<T: Elicitation + Send> Prompt for Option<T> {
    fn prompt() -> Option<&'static str> {
        Some("Would you like to provide a value for this field?")
    }
}

impl<T: Elicitation + Send> Elicitation for Option<T> {
    type Style = OptionStyle;

    #[tracing::instrument(skip(communicator), fields(inner_type = std::any::type_name::<T>()))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting optional value");

        // First ask if they want to provide a value
        let provide = bool::elicit(communicator).await?;

        if provide {
            tracing::debug!("User chose to provide value");
            T::elicit(communicator).await.map(Some)
        } else {
            tracing::debug!("User chose to skip");
            Ok(None)
        }
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::kani_proof()
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::verus_proof()
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::creusot_proof()
    }
}
