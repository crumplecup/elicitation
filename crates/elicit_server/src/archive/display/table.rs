//! AccessKit display for [`TableDescriptor`].
use elicitation::Elicit;

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::TableDescriptor;

use super::ArchiveDisplay;

/// Display strategies for a [`TableDescriptor`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema, Elicit,
)]
pub enum TableDescriptorMode {
    /// Column headers in a grid — the standard tabular view.
    #[default]
    GridView,
    /// Vertical list of column descriptors.
    ColumnList,
    /// Compact summary card: name, type, row count, spatial flag.
    SummaryCard,
}

impl ArchiveDisplay for TableDescriptor {
    type Mode = TableDescriptorMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            TableDescriptorMode::GridView => Role(AkRole::Grid),
            TableDescriptorMode::ColumnList => Role(AkRole::List),
            TableDescriptorMode::SummaryCard => Role(AkRole::Article),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        let label = format!(
            "{}.{} [{}]{}",
            self.schema,
            self.table_name,
            self.table_type,
            if self.has_spatial() { " ⊕geo" } else { "" },
        );

        match mode {
            TableDescriptorMode::GridView => {
                let header_id = NodeId::from(id_base + 1);
                let col_ids: Vec<NodeId> = self
                    .columns
                    .iter()
                    .enumerate()
                    .map(|(i, _)| NodeId::from(id_base + 2 + i as u64))
                    .collect();

                let header_row = NodeJson::new(Role(AkRole::Row))
                    .with_label("column headers".to_string())
                    .with_children(col_ids.clone());

                for (i, col) in self.columns.iter().enumerate() {
                    let cell_id = NodeId::from(id_base + 2 + i as u64);
                    let cell_label = format!(
                        "{}: {}{}",
                        col.name,
                        col.sql_type,
                        if col.nullable { "" } else { " NOT NULL" },
                    );
                    let cell = NodeJson::new(Role(AkRole::ColumnHeader)).with_label(cell_label);
                    nodes.push((cell_id, cell));
                }

                let root = NodeJson::new(Role(AkRole::Grid))
                    .with_label(label)
                    .with_children(vec![header_id]);

                nodes.insert(0, (header_id, header_row));
                nodes.insert(0, (root_id, root));
            }
            TableDescriptorMode::ColumnList => {
                let item_ids: Vec<NodeId> = self
                    .columns
                    .iter()
                    .enumerate()
                    .map(|(i, _)| NodeId::from(id_base + 1 + i as u64))
                    .collect();

                for (i, col) in self.columns.iter().enumerate() {
                    let item_id = NodeId::from(id_base + 1 + i as u64);
                    let mut flags = Vec::new();
                    if col.is_primary_key {
                        flags.push("PK");
                    }
                    if col.is_foreign_key {
                        flags.push("FK");
                    }
                    if col.is_spatial {
                        flags.push("GEO");
                    }
                    let suffix = if flags.is_empty() {
                        String::new()
                    } else {
                        format!(" [{}]", flags.join(", "))
                    };
                    let item = NodeJson::new(Role(AkRole::ListItem))
                        .with_label(format!("{}: {}{}", col.name, col.sql_type, suffix));
                    nodes.push((item_id, item));
                }

                let root = NodeJson::new(Role(AkRole::List))
                    .with_label(label)
                    .with_children(item_ids);
                nodes.insert(0, (root_id, root));
            }
            TableDescriptorMode::SummaryCard => {
                let detail = format!(
                    "{}.{} | {} | {} cols | rows≈{}",
                    self.schema,
                    self.table_name,
                    self.table_type,
                    self.columns.len(),
                    self.estimated_rows
                        .map_or("?".to_string(), |n| n.to_string()),
                );
                let root = NodeJson::new(Role(AkRole::Article)).with_label(detail);
                nodes.push((root_id, root));
            }
        }

        (root_id, nodes)
    }
}
