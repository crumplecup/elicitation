//! HashSet<T> implementation for unique item collection.

use crate::{ElicitResult, Elicitation, Prompt};
use std::collections::HashSet;
use std::hash::Hash;

impl<T> Prompt for HashSet<T>
where
    T: Elicitation + Hash + Eq + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Would you like to add items to this set?")
    }
}

impl<T> Elicitation for HashSet<T>
where
    T: Elicitation + Hash + Eq + Send,
{
    #[tracing::instrument(skip(client), fields(item_type = std::any::type_name::<T>()))]
    async fn elicit<U: pmcp::shared::transport::Transport>(
        client: &pmcp::Client<U>,
    ) -> ElicitResult<Self> {
        let mut set = HashSet::new();
        tracing::debug!("Eliciting HashSet");

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
