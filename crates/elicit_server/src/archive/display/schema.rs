//! AccessKit display for [`SchemaDescriptor`].
use elicitation::{Elicit, KaniCompose};

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::SchemaDescriptor;

use super::ArchiveDisplay;

/// Display strategies for a [`SchemaDescriptor`].
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
pub enum SchemaDescriptorMode {
    /// Tree node with child items for each table name.
    #[default]
    TreeView,
    /// Flat vertical list of table names.
    FlatList,
}

impl ArchiveDisplay for SchemaDescriptor {
    type Mode = SchemaDescriptorMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            SchemaDescriptorMode::TreeView => Role(AkRole::TreeItem),
            SchemaDescriptorMode::FlatList => Role(AkRole::List),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        let child_role = match mode {
            SchemaDescriptorMode::TreeView => Role(AkRole::TreeItem),
            SchemaDescriptorMode::FlatList => Role(AkRole::ListItem),
        };

        let child_ids: Vec<NodeId> = self
            .table_names
            .iter()
            .enumerate()
            .map(|(i, _)| NodeId::from(id_base + 1 + i as u64))
            .collect();

        let root = NodeJson::new(Self::root_role(mode))
            .with_label(format!("{} ({})", self.schema_name, self.owner))
            .with_children(child_ids.clone());

        for (i, table_name) in self.table_names.iter().enumerate() {
            let child_id = NodeId::from(id_base + 1 + i as u64);
            let child = NodeJson::new(child_role).with_label(table_name.clone());
            nodes.push((child_id, child));
        }

        nodes.insert(0, (root_id, root));
        (root_id, nodes)
    }
}
