//! AccessKit display for [`TriggerDescriptor`].
use elicitation::Elicit;

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::TriggerDescriptor;

use super::ArchiveDisplay;

/// Display strategies for a [`TriggerDescriptor`].
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema, Elicit,
)]
pub enum TriggerDescriptorMode {
    /// A compact row for the trigger browser list.
    #[default]
    Row,
    /// An expanded group showing all properties.
    Detailed,
}

impl ArchiveDisplay for TriggerDescriptor {
    type Mode = TriggerDescriptorMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            TriggerDescriptorMode::Row => Role(AkRole::Row),
            TriggerDescriptorMode::Detailed => Role(AkRole::Group),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        let mut events = Vec::new();
        if self.events.on_insert {
            events.push("INSERT");
        }
        if self.events.on_update {
            events.push("UPDATE");
        }
        if self.events.on_delete {
            events.push("DELETE");
        }
        if self.events.on_truncate {
            events.push("TRUNCATE");
        }
        let events_str = events.join("|");
        let granularity = if self.row_level { "ROW" } else { "STMT" };
        let enabled = if self.enabled { "enabled" } else { "disabled" };

        match mode {
            TriggerDescriptorMode::Row => {
                let label = format!(
                    "{} {} {} ON {}.{} → {} [{enabled}]",
                    self.name, self.timing, events_str, self.schema, self.table, self.function,
                );
                nodes.push((root_id, NodeJson::new(Role(AkRole::Row)).with_label(label)));
            }
            TriggerDescriptorMode::Detailed => {
                let props: &[(&str, String)] = &[
                    ("schema", self.schema.clone()),
                    ("table", self.table.clone()),
                    ("name", self.name.clone()),
                    ("timing", self.timing.clone()),
                    ("events", events_str),
                    ("granularity", granularity.to_string()),
                    ("function", self.function.clone()),
                    ("enabled", enabled.to_string()),
                ];
                let child_ids: Vec<NodeId> = (0..props.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                let root = NodeJson::new(Role(AkRole::Group))
                    .with_label(format!("trigger: {}.{}", self.table, self.name))
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
