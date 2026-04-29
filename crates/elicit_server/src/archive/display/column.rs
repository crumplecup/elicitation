//! AccessKit display for [`ColumnDescriptor`].
use elicitation::{Elicit, KaniCompose};

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::ColumnDescriptor;

use super::ArchiveDisplay;

/// Display strategies for a [`ColumnDescriptor`].
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
pub enum ColumnDescriptorMode {
    /// A single row node suitable for embedding in a parent grid or list.
    #[default]
    Inline,
    /// An expanded group with individual property cells.
    Detailed,
}

impl ArchiveDisplay for ColumnDescriptor {
    type Mode = ColumnDescriptorMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            ColumnDescriptorMode::Inline => Role(AkRole::Row),
            ColumnDescriptorMode::Detailed => Role(AkRole::Group),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        match mode {
            ColumnDescriptorMode::Inline => {
                let mut flags = Vec::new();
                if self.is_primary_key {
                    flags.push("PK");
                }
                if self.is_foreign_key {
                    flags.push("FK");
                }
                if !self.nullable {
                    flags.push("NOT NULL");
                }
                if self.is_spatial {
                    flags.push("GEO");
                }
                let label = if flags.is_empty() {
                    format!("{}: {}", self.name, self.sql_type)
                } else {
                    format!("{}: {} [{}]", self.name, self.sql_type, flags.join(", "))
                };
                let root = NodeJson::new(Role(AkRole::Row)).with_label(label);
                nodes.push((root_id, root));
            }
            ColumnDescriptorMode::Detailed => {
                let properties: &[(&str, String)] = &[
                    ("name", self.name.clone()),
                    ("type", self.sql_type.clone()),
                    ("nullable", self.nullable.to_string()),
                    ("pk", self.is_primary_key.to_string()),
                    ("fk", self.is_foreign_key.to_string()),
                    ("spatial", self.is_spatial.to_string()),
                    ("default", self.default_value.clone().unwrap_or_default()),
                ];

                let child_ids: Vec<NodeId> = (0..properties.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();

                let root = NodeJson::new(Role(AkRole::Group))
                    .with_label(format!("column: {}", self.name))
                    .with_children(child_ids);

                for (i, (key, val)) in properties.iter().enumerate() {
                    let prop_id = NodeId::from(id_base + 1 + i as u64);
                    let prop =
                        NodeJson::new(Role(AkRole::Cell)).with_label(format!("{}: {}", key, val));
                    nodes.push((prop_id, prop));
                }

                nodes.insert(0, (root_id, root));
            }
        }

        (root_id, nodes)
    }
}
