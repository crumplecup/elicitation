//! AccessKit display for [`SequenceDescriptor`].
use elicitation::Elicit;

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::SequenceDescriptor;

use super::ArchiveDisplay;

/// Display strategies for a [`SequenceDescriptor`].
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema, Elicit,
)]
pub enum SequenceDescriptorMode {
    /// A compact row for the sequence browser list.
    #[default]
    Row,
    /// An expanded group showing all properties.
    Detailed,
}

impl ArchiveDisplay for SequenceDescriptor {
    type Mode = SequenceDescriptorMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            SequenceDescriptorMode::Row => Role(AkRole::Row),
            SequenceDescriptorMode::Detailed => Role(AkRole::Group),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        let current = self
            .current_value
            .map_or("not started".to_string(), |v| v.to_string());
        let cycle_flag = if self.cycle { " CYCLE" } else { "" };

        match mode {
            SequenceDescriptorMode::Row => {
                let label = format!(
                    "{}.{} start={} inc={} [{min}..{max}]{cycle_flag} current={current}",
                    self.schema,
                    self.name,
                    self.start_value,
                    self.increment_by,
                    min = self.min_value,
                    max = self.max_value,
                );
                nodes.push((root_id, NodeJson::new(Role(AkRole::Row)).with_label(label)));
            }
            SequenceDescriptorMode::Detailed => {
                let owned = self.owned_by.clone().unwrap_or_default();
                let props: &[(&str, String)] = &[
                    ("schema", self.schema.clone()),
                    ("name", self.name.clone()),
                    ("current_value", current),
                    ("start_value", self.start_value.to_string()),
                    ("increment_by", self.increment_by.to_string()),
                    ("min_value", self.min_value.to_string()),
                    ("max_value", self.max_value.to_string()),
                    ("cycle", self.cycle.to_string()),
                    ("owned_by", owned),
                ];
                let child_ids: Vec<NodeId> = (0..props.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                let root = NodeJson::new(Role(AkRole::Group))
                    .with_label(format!("sequence: {}.{}", self.schema, self.name))
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
