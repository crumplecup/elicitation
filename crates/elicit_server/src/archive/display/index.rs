//! AccessKit display for [`IndexDescriptor`].
use elicitation::{Elicit, KaniCompose};

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::IndexDescriptor;

use super::ArchiveDisplay;

/// Display strategies for an [`IndexDescriptor`].
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema, Elicit, KaniCompose,
)]
pub enum IndexDescriptorMode {
    /// A single row suitable for embedding in a parent list.
    #[default]
    Row,
    /// An expanded group showing all properties.
    Detailed,
}

impl ArchiveDisplay for IndexDescriptor {
    type Mode = IndexDescriptorMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            IndexDescriptorMode::Row => Role(AkRole::Row),
            IndexDescriptorMode::Detailed => Role(AkRole::Group),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        let cols = self.column_names.join(", ");
        let unique_flag = if self.is_unique { " UNIQUE" } else { "" };

        match mode {
            IndexDescriptorMode::Row => {
                let label = format!(
                    "{}{} [{}] ({})",
                    self.index_name, unique_flag, self.index_method, cols,
                );
                nodes.push((root_id, NodeJson::new(Role(AkRole::Row)).with_label(label)));
            }
            IndexDescriptorMode::Detailed => {
                let props: &[(&str, String)] = &[
                    ("name", self.index_name.clone()),
                    ("table", format!("{}.{}", self.schema, self.table_name)),
                    ("columns", cols),
                    ("method", self.index_method.clone()),
                    ("unique", self.is_unique.to_string()),
                ];
                let child_ids: Vec<NodeId> = (0..props.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                let root = NodeJson::new(Role(AkRole::Group))
                    .with_label(format!("index: {}", self.index_name))
                    .with_children(child_ids);
                for (i, (k, v)) in props.iter().enumerate() {
                    let id = NodeId::from(id_base + 1 + i as u64);
                    nodes.push((
                        id,
                        NodeJson::new(Role(AkRole::Cell)).with_label(format!("{k}: {v}")),
                    ));
                }
                nodes.insert(0, (root_id, root));
            }
        }

        (root_id, nodes)
    }
}
