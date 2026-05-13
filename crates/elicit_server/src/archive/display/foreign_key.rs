//! AccessKit display for [`ForeignKeyDescriptor`].
use elicitation::{Elicit, KaniCompose};

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::ForeignKeyDescriptor;

use super::ArchiveDisplay;

/// Display strategies for a [`ForeignKeyDescriptor`].
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniCompose,
)]
pub enum ForeignKeyDescriptorMode {
    /// A single row suitable for embedding in a parent list or grid.
    #[default]
    Inline,
    /// An expanded group showing all properties.
    Detailed,
}

impl ArchiveDisplay for ForeignKeyDescriptor {
    type Mode = ForeignKeyDescriptorMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            ForeignKeyDescriptorMode::Inline => Role(AkRole::Row),
            ForeignKeyDescriptorMode::Detailed => Role(AkRole::Group),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        match mode {
            ForeignKeyDescriptorMode::Inline => {
                let label = format!(
                    "{}: {}.{} → {}.{}.{} (DELETE {}, UPDATE {})",
                    self.constraint_name,
                    self.from_column,
                    self.from_column,
                    self.to_schema,
                    self.to_table,
                    self.to_column,
                    self.on_delete,
                    self.on_update,
                );
                nodes.push((root_id, NodeJson::new(Role(AkRole::Row)).with_label(label)));
            }
            ForeignKeyDescriptorMode::Detailed => {
                let props: &[(&str, String)] = &[
                    ("constraint", self.constraint_name.clone()),
                    ("from_column", self.from_column.clone()),
                    ("to_schema", self.to_schema.clone()),
                    ("to_table", self.to_table.clone()),
                    ("to_column", self.to_column.clone()),
                    ("on_delete", format!("{}", self.on_delete)),
                    ("on_update", format!("{}", self.on_update)),
                ];
                let child_ids: Vec<NodeId> = (0..props.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                let root = NodeJson::new(Role(AkRole::Group))
                    .with_label(format!("fk: {}", self.constraint_name))
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
