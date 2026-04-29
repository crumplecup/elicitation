//! AccessKit display for [`DatabaseDescriptor`].
use elicitation::{Elicit, KaniCompose};

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::DatabaseDescriptor;

use super::ArchiveDisplay;

/// Display strategies for a [`DatabaseDescriptor`].
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema, Elicit, KaniCompose,
)]
pub enum DatabaseDescriptorMode {
    /// Overview panel: name, version, backend badge, schema count.
    #[default]
    Overview,
    /// Compact connection card: id + backend only.
    ConnectionCard,
}

impl ArchiveDisplay for DatabaseDescriptor {
    type Mode = DatabaseDescriptorMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            DatabaseDescriptorMode::Overview => Role(AkRole::Window),
            DatabaseDescriptorMode::ConnectionCard => Role(AkRole::Article),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let root = NodeJson::new(Self::root_role(mode))
            .with_label(format!("{} ({})", self.db_name, self.backend));
        (root_id, vec![(root_id, root)])
    }
}
