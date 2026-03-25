//! Result<T, E> implementation for success/error elicitation.

use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};

// Default-only style for Result
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ResultStyle {
    #[default]
    Default,
}

impl Prompt for ResultStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for ResultStyle {
    type Style = ResultStyle;

    #[tracing::instrument(skip(_communicator), level = "trace")]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(Self::Default)
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

impl<T, E> Prompt for Result<T, E>
where
    T: Elicitation + Send,
    E: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Is this a success (Ok) or failure (Err)?")
    }
}

impl<T, E> Elicitation for Result<T, E>
where
    T: Elicitation + Send,
    E: Elicitation + Send,
{
    type Style = ResultStyle;

    #[tracing::instrument(skip(communicator), fields(
        ok_type = std::any::type_name::<T>(),
        err_type = std::any::type_name::<E>()
    ))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Result");

        // First, ask if it's Ok or Err
        // We use a bool here - true for Ok, false for Err
        tracing::debug!("Asking if result is Ok or Err");
        let is_ok = bool::elicit(communicator).await?;

        if is_ok {
            tracing::debug!("Eliciting Ok variant");
            let value = T::elicit(communicator).await?;
            tracing::debug!("Result::Ok created");
            Ok(Ok(value))
        } else {
            tracing::debug!("Eliciting Err variant");
            let error = E::elicit(communicator).await?;
            tracing::debug!("Result::Err created");
            Ok(Err(error))
        }
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        {
            let mut ts = <T as Elicitation>::kani_proof();
            ts.extend(<E as Elicitation>::kani_proof());
            ts
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        {
            let mut ts = <T as Elicitation>::verus_proof();
            ts.extend(<E as Elicitation>::verus_proof());
            ts
        }
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        {
            let mut ts = <T as Elicitation>::creusot_proof();
            ts.extend(<E as Elicitation>::creusot_proof());
            ts
        }
    }
}
