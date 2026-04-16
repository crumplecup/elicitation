//! AccessKit display for [`EnumDescriptor`].

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::EnumDescriptor;

use super::ArchiveDisplay;

/// Display strategies for an [`EnumDescriptor`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema)]
pub enum EnumDescriptorMode {
    /// A compact row showing name and label count.
    #[default]
    Row,
    /// An expanded group listing all enum labels.
    Detailed,
}

impl ArchiveDisplay for EnumDescriptor {
    type Mode = EnumDescriptorMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            EnumDescriptorMode::Row => Role(AkRole::Row),
            EnumDescriptorMode::Detailed => Role(AkRole::Group),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        match mode {
            EnumDescriptorMode::Row => {
                let label = format!(
                    "{}.{} ({} labels: {})",
                    self.schema,
                    self.name,
                    self.labels.len(),
                    self.labels.join(", "),
                );
                nodes.push((root_id, NodeJson::new(Role(AkRole::Row)).with_label(label)));
            }
            EnumDescriptorMode::Detailed => {
                let item_ids: Vec<NodeId> = (0..self.labels.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                for (i, lbl) in self.labels.iter().enumerate() {
                    let id = NodeId::from(id_base + 1 + i as u64);
                    nodes.push((
                        id,
                        NodeJson::new(Role(AkRole::ListItem)).with_label(format!("{}: {}", i, lbl)),
                    ));
                }
                let root = NodeJson::new(Role(AkRole::Group))
                    .with_label(format!("enum: {}.{}", self.schema, self.name))
                    .with_children(item_ids);
                nodes.insert(0, (root_id, root));
            }
        }

        (root_id, nodes)
    }
}
