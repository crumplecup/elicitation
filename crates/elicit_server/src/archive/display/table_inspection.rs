//! AccessKit display for [`TableInspection`].
use elicitation::Elicit;

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::TableInspection;

use super::{
    ArchiveDisplay, ConstraintDescriptorMode, ForeignKeyDescriptorMode, IndexDescriptorMode,
};

/// Display strategies for a [`TableInspection`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema, Elicit,
)]
pub enum TableInspectionMode {
    /// List of foreign key rows.
    #[default]
    FkList,
    /// List of constraint rows.
    ConstraintList,
    /// List of index rows.
    IndexList,
}

impl ArchiveDisplay for TableInspection {
    type Mode = TableInspectionMode;

    fn root_role(_mode: &Self::Mode) -> Role {
        Role(AkRole::List)
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();
        let mut next_id = id_base + 1;

        match mode {
            TableInspectionMode::FkList => {
                let item_ids: Vec<NodeId> = self
                    .foreign_keys
                    .iter()
                    .enumerate()
                    .map(|(i, _)| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                for fk in &self.foreign_keys {
                    let (_, fk_nodes) = fk.to_ak_nodes(&ForeignKeyDescriptorMode::Inline, next_id);
                    nodes.extend(fk_nodes);
                    next_id += 1;
                }
                let root = NodeJson::new(Role(AkRole::List))
                    .with_label(format!("foreign keys ({})", self.foreign_keys.len()))
                    .with_children(item_ids);
                nodes.insert(0, (root_id, root));
            }
            TableInspectionMode::ConstraintList => {
                let item_ids: Vec<NodeId> = self
                    .constraints
                    .iter()
                    .enumerate()
                    .map(|(i, _)| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                for c in &self.constraints {
                    let (_, c_nodes) = c.to_ak_nodes(&ConstraintDescriptorMode::Inline, next_id);
                    nodes.extend(c_nodes);
                    next_id += 1;
                }
                let root = NodeJson::new(Role(AkRole::List))
                    .with_label(format!("constraints ({})", self.constraints.len()))
                    .with_children(item_ids);
                nodes.insert(0, (root_id, root));
            }
            TableInspectionMode::IndexList => {
                let item_ids: Vec<NodeId> = self
                    .indexes
                    .iter()
                    .enumerate()
                    .map(|(i, _)| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                for idx in &self.indexes {
                    let (_, idx_nodes) = idx.to_ak_nodes(&IndexDescriptorMode::Row, next_id);
                    nodes.extend(idx_nodes);
                    next_id += 1;
                }
                let root = NodeJson::new(Role(AkRole::List))
                    .with_label(format!("indexes ({})", self.indexes.len()))
                    .with_children(item_ids);
                nodes.insert(0, (root_id, root));
            }
        }

        (root_id, nodes)
    }
}
