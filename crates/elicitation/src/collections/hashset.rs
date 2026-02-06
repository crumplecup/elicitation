//! HashSet<T> implementation for unique item collection.

use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};
use std::collections::HashSet;
use std::hash::Hash;

// Default-only style for HashSet
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum HashSetStyle {
    #[default]
    Default,
}

impl Prompt for HashSetStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for HashSetStyle {
    type Style = HashSetStyle;

    #[tracing::instrument(skip(_communicator), level = "trace")]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(Self::Default)
    }
}

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
    type Style = HashSetStyle;

    #[tracing::instrument(skip(communicator), fields(item_type = std::any::type_name::<T>()))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let mut set = HashSet::new();
        tracing::debug!("Eliciting HashSet");

        loop {
            let add_more = if set.is_empty() {
                tracing::debug!("Prompting for first item");
                bool::elicit(communicator).await?
            } else {
                tracing::debug!(count = set.len(), "Prompting for additional item");
                bool::elicit(communicator).await?
            };

            if !add_more {
                tracing::debug!(final_count = set.len(), "Set complete");
                break;
            }

            tracing::debug!("Eliciting item");
            let item = T::elicit(communicator).await?;

            // Automatic duplicate handling - sets ignore duplicates
            if !set.insert(item) {
                tracing::debug!("Duplicate item ignored (already in set)");
            }
        }

        Ok(set)
    }
}
