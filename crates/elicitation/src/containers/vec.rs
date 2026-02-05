//! Vec<T> implementation for collection elicitation.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};

// Default-only style for Vec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum VecStyle {
    #[default]
    Default,
}

impl Prompt for VecStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for VecStyle {
    type Style = VecStyle;

    #[tracing::instrument(skip(_client), level = "trace")]
    async fn elicit(_client: &ElicitClient) -> ElicitResult<Self> {
        Ok(Self::Default)
    }
}

impl<T: Elicitation + Send> Prompt for Vec<T> {
    fn prompt() -> Option<&'static str> {
        Some("Would you like to add items to this collection?")
    }
}

impl<T: Elicitation + Send> Elicitation for Vec<T> {
    type Style = VecStyle;

    #[tracing::instrument(skip(client), fields(item_type = std::any::type_name::<T>()))]
    async fn elicit(client: &ElicitClient) -> ElicitResult<Self> {
        let mut items = Vec::new();
        tracing::debug!("Eliciting vector");

        loop {
            let add_more = if items.is_empty() {
                // First item - different prompt
                tracing::debug!("Prompting for first item");
                // TODO: In future, could customize prompt: "Add first item?"
                bool::elicit(client).await?
            } else {
                // Subsequent items
                tracing::debug!(count = items.len(), "Prompting for additional item");
                // TODO: In future, could customize prompt: "Add another item?"
                bool::elicit(client).await?
            };

            if !add_more {
                tracing::debug!(final_count = items.len(), "Collection complete");
                break;
            }

            tracing::debug!("Eliciting item");
            let item = T::elicit(client).await?;
            items.push(item);
        }

        Ok(items)
    }
}
