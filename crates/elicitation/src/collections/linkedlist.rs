//! LinkedList<T> implementation for linked list elicitation.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};
use std::collections::LinkedList;

// Default-only style for LinkedList
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LinkedListStyle {
    #[default]
    Default,
}

impl Prompt for LinkedListStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for LinkedListStyle {
    type Style = LinkedListStyle;

    async fn elicit(_client: &ElicitClient<'_>) -> ElicitResult<Self> {
        Ok(Self::Default)
    }
}

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
    type Style = LinkedListStyle;

    #[tracing::instrument(skip(client), fields(item_type = std::any::type_name::<T>()))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
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
