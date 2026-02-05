//! BTreeMap<K, V> implementation for ordered key-value elicitation.

use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};
use std::collections::BTreeMap;

// Default-only style for BTreeMap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BTreeMapStyle {
    #[default]
    Default,
}

impl Prompt for BTreeMapStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for BTreeMapStyle {
    type Style = BTreeMapStyle;

    #[tracing::instrument(skip(_communicator), level = "trace")]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(Self::Default)
    }
}

impl<K, V> Prompt for BTreeMap<K, V>
where
    K: Elicitation + Ord + Send,
    V: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Would you like to add entries to this ordered map?")
    }
}

impl<K, V> Elicitation for BTreeMap<K, V>
where
    K: Elicitation + Ord + Send,
    V: Elicitation + Send,
{
    type Style = BTreeMapStyle;

    #[tracing::instrument(skip(communicator), fields(
        key_type = std::any::type_name::<K>(),
        value_type = std::any::type_name::<V>()
    ))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let mut map = BTreeMap::new();
        tracing::debug!("Eliciting BTreeMap");

        loop {
            let add_more = if map.is_empty() {
                tracing::debug!("Prompting for first entry");
                bool::elicit(communicator).await?
            } else {
                tracing::debug!(count = map.len(), "Prompting for additional entry");
                bool::elicit(communicator).await?
            };

            if !add_more {
                tracing::debug!(final_count = map.len(), "Map complete");
                break;
            }

            // Elicit key
            tracing::debug!("Eliciting key");
            let key = K::elicit(communicator).await?;

            // Check for duplicate keys
            if map.contains_key(&key) {
                tracing::warn!("Key already exists in map");
                let replace = bool::elicit(communicator).await?;
                if !replace {
                    tracing::debug!("Skipping duplicate key");
                    continue;
                }
                tracing::debug!("Replacing existing value");
            }

            // Elicit value
            tracing::debug!("Eliciting value for key");
            let value = V::elicit(communicator).await?;

            map.insert(key, value);
        }

        Ok(map)
    }
}
