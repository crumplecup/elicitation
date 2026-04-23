//! AccessKit display for [`ConstraintDescriptor`].
use elicitation::Elicit;

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::ConstraintDescriptor;

use super::ArchiveDisplay;

/// Display strategies for a [`ConstraintDescriptor`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema, Elicit,
)]
pub enum ConstraintDescriptorMode {
    /// A single row suitable for embedding in a parent list.
    #[default]
    Inline,
    /// An expanded group showing all properties.
    Detailed,
}

impl ArchiveDisplay for ConstraintDescriptor {
    type Mode = ConstraintDescriptorMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            ConstraintDescriptorMode::Inline => Role(AkRole::Row),
            ConstraintDescriptorMode::Detailed => Role(AkRole::Group),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        let cols = self.columns.join(", ");

        match mode {
            ConstraintDescriptorMode::Inline => {
                let label = if let Some(def) = &self.definition {
                    format!("{} [{}] ({}) — {}", self.name, self.kind, cols, def)
                } else {
                    format!("{} [{}] ({})", self.name, self.kind, cols)
                };
                nodes.push((root_id, NodeJson::new(Role(AkRole::Row)).with_label(label)));
            }
            ConstraintDescriptorMode::Detailed => {
                let def = self.definition.clone().unwrap_or_default();
                let props: &[(&str, String)] = &[
                    ("name", self.name.clone()),
                    ("kind", format!("{}", self.kind)),
                    ("columns", cols),
                    ("definition", def),
                ];
                let child_ids: Vec<NodeId> = (0..props.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                let root = NodeJson::new(Role(AkRole::Group))
                    .with_label(format!("constraint: {}", self.name))
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
