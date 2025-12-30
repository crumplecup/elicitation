//! LinkedList<T> implementation for linked list elicitation.
use rmcp::service::{Peer, RoleClient};

use crate::{ElicitResult, Elicitation, Prompt};
use std::collections::LinkedList;

impl<T> Prompt for LinkedList<T>
where
    T: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Would you like to add items to this list?")
    }
}

impl<T> Elicitation for LinkedList<T>
where
    T: Elicitation + Send,
{
    #[tracing::instrument(skip(client), fields(item_type = std::any::type_name::<T>()))]
    async fn elicit(
        client: &Peer<RoleClient>,
    ) -> ElicitResult<Self> {
        let mut list = LinkedList::new();
        tracing::debug!("Eliciting LinkedList");

        loop {
            let add_more = if list.is_empty() {
                tracing::debug!("Prompting for first item");
                bool::elicit(client).await?
            } else {
                tracing::debug!(count = list.len(), "Prompting for additional item");
                bool::elicit(client).await?
            };

            if !add_more {
                tracing::debug!(final_count = list.len(), "LinkedList complete");
                break;
            }

            tracing::debug!("Eliciting item");
            let item = T::elicit(client).await?;
            list.push_back(item);
        }

        Ok(list)
    }
}
