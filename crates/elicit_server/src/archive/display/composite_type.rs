//! AccessKit display for [`CompositeTypeDescriptor`].

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::CompositeTypeDescriptor;

use super::ArchiveDisplay;

/// Display strategies for a [`CompositeTypeDescriptor`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema)]
pub enum CompositeTypeDescriptorMode {
    /// A compact row showing name and attribute count.
    #[default]
    Row,
    /// An expanded group listing all attributes.
    Detailed,
}

impl ArchiveDisplay for CompositeTypeDescriptor {
    type Mode = CompositeTypeDescriptorMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            CompositeTypeDescriptorMode::Row => Role(AkRole::Row),
            CompositeTypeDescriptorMode::Detailed => Role(AkRole::Group),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        match mode {
            CompositeTypeDescriptorMode::Row => {
                let attrs: Vec<String> = self
                    .attributes
                    .iter()
                    .map(|a| format!("{} {}", a.name, a.type_name))
                    .collect();
                let label = format!("{}.{} ({{{}}}) ", self.schema, self.name, attrs.join(", "),);
                nodes.push((root_id, NodeJson::new(Role(AkRole::Row)).with_label(label)));
            }
            CompositeTypeDescriptorMode::Detailed => {
                let item_ids: Vec<NodeId> = (0..self.attributes.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                for (i, attr) in self.attributes.iter().enumerate() {
                    let id = NodeId::from(id_base + 1 + i as u64);
                    nodes.push((
                        id,
                        NodeJson::new(Role(AkRole::ListItem))
                            .with_label(format!("{}: {}", attr.name, attr.type_name)),
                    ));
                }
                let root = NodeJson::new(Role(AkRole::Group))
                    .with_label(format!("composite: {}.{}", self.schema, self.name))
                    .with_children(item_ids);
                nodes.insert(0, (root_id, root));
            }
        }

        (root_id, nodes)
    }
}
