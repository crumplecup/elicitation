//! LinkedList<T> implementation for linked list elicitation.

use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};
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

    #[tracing::instrument(skip(_communicator), level = "trace")]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
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

    #[tracing::instrument(skip(communicator), fields(item_type = std::any::type_name::<T>()))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let mut list = LinkedList::new();
        tracing::debug!("Eliciting LinkedList");

        loop {
            let add_more = if list.is_empty() {
                tracing::debug!("Prompting for first item");
                bool::elicit(communicator).await?
            } else {
                tracing::debug!(count = list.len(), "Prompting for additional item");
                bool::elicit(communicator).await?
            };

            if !add_more {
                tracing::debug!(final_count = list.len(), "LinkedList complete");
                break;
            }

            tracing::debug!("Eliciting item");
            let item = T::elicit(communicator).await?;
            list.push_back(item);
        }

        Ok(list)
    }
}
