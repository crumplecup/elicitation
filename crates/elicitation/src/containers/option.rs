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
}
