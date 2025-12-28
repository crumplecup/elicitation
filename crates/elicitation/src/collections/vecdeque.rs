//! VecDeque<T> implementation for double-ended queue elicitation.

use crate::{ElicitResult, Elicitation, Prompt};
use std::collections::VecDeque;

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
    #[tracing::instrument(skip(client), fields(item_type = std::any::type_name::<T>()))]
    async fn elicit<U: pmcp::shared::transport::Transport>(
        client: &pmcp::Client<U>,
    ) -> ElicitResult<Self> {
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
