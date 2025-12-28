//! Option<T> implementation for optional value elicitation.

use crate::{Elicit, ElicitResult, Prompt};

impl<T: Elicit + Send> Prompt for Option<T> {
    fn prompt() -> Option<&'static str> {
        Some("Would you like to provide a value for this field?")
    }
}

impl<T: Elicit + Send> Elicit for Option<T> {
    #[tracing::instrument(skip(client), fields(inner_type = std::any::type_name::<T>()))]
    async fn elicit<U: pmcp::shared::transport::Transport>(
        client: &pmcp::Client<U>,
    ) -> ElicitResult<Self> {
        tracing::debug!("Eliciting optional value");

        // First ask if they want to provide a value
        let provide = bool::elicit(client).await?;

        if provide {
            tracing::debug!("User chose to provide value");
            T::elicit(client).await.map(Some)
        } else {
            tracing::debug!("User chose to skip");
            Ok(None)
        }
    }
}
