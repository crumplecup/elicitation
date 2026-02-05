//! URL type implementation.

use crate::{ElicitClient, ElicitCommunicator, ElicitResult, Elicitation, Prompt};

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
}
