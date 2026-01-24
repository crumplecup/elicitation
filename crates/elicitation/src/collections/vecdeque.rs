//! VecDeque<T> implementation for double-ended queue elicitation.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};
use std::collections::VecDeque;

// Default-only style for VecDeque
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum VecDequeStyle {
    #[default]
    Default,
}

impl Prompt for VecDequeStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for VecDequeStyle {
    type Style = VecDequeStyle;

    #[tracing::instrument(skip(_client), level = "trace")]
    async fn elicit(_client: &ElicitClient<'_>) -> ElicitResult<Self> {
        Ok(Self::Default)
    }
}

impl<T> Prompt for VecDeque<T>
where
    T: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Would you like to add items to this deque?")
    }
}

impl<T> Elicitation for VecDeque<T>
where
    T: Elicitation + Send,
{
    type Style = VecDequeStyle;

    #[tracing::instrument(skip(client), fields(item_type = std::any::type_name::<T>()))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        let mut deque = VecDeque::new();
        tracing::debug!("Eliciting VecDeque");

        loop {
            let add_more = if deque.is_empty() {
                tracing::debug!("Prompting for first item");
                bool::elicit(client).await?
            } else {
                tracing::debug!(count = deque.len(), "Prompting for additional item");
                bool::elicit(client).await?
            };

            if !add_more {
                tracing::debug!(final_count = deque.len(), "VecDeque complete");
                break;
            }

            tracing::debug!("Eliciting item");
            let item = T::elicit(client).await?;
            deque.push_back(item);
        }

        Ok(deque)
    }
}
