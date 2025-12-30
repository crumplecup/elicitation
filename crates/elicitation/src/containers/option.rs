//! Option<T> implementation for optional value elicitation.
use rmcp::service::{Peer, RoleClient};

use crate::{ElicitResult, Elicitation, Prompt};

impl<T: Elicitation + Send> Prompt for Option<T> {
    fn prompt() -> Option<&'static str> {
        Some("Would you like to provide a value for this field?")
    }
}

impl<T: Elicitation + Send> Elicitation for Option<T> {
    #[tracing::instrument(skip(client), fields(inner_type = std::any::type_name::<T>()))]
    async fn elicit(
        client: &Peer<RoleClient>,
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
