//! BTreeSet<T> implementation for ordered unique item collection.
use rmcp::service::{Peer, RoleClient};

use crate::{ElicitResult, Elicitation, Prompt};
use std::collections::BTreeSet;

impl<T> Prompt for BTreeSet<T>
where
    T: Elicitation + Ord + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Would you like to add items to this ordered set?")
    }
}

impl<T> Elicitation for BTreeSet<T>
where
    T: Elicitation + Ord + Send,
{
    #[tracing::instrument(skip(client), fields(item_type = std::any::type_name::<T>()))]
    async fn elicit(
        client: &Peer<RoleClient>,
    ) -> ElicitResult<Self> {
        let mut set = BTreeSet::new();
        tracing::debug!("Eliciting BTreeSet");

        loop {
            let add_more = if set.is_empty() {
                tracing::debug!("Prompting for first item");
                bool::elicit(client).await?
            } else {
                tracing::debug!(count = set.len(), "Prompting for additional item");
                bool::elicit(client).await?
            };

            if !add_more {
                tracing::debug!(final_count = set.len(), "Set complete");
                break;
            }

            tracing::debug!("Eliciting item");
            let item = T::elicit(client).await?;

            // Automatic duplicate handling - sets ignore duplicates
            if !set.insert(item) {
                tracing::debug!("Duplicate item ignored (already in set)");
            }
        }

        Ok(set)
    }
}
