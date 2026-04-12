//! AccessKit display for [`QueryResult`].

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::QueryResult;

use super::ArchiveDisplay;

/// Display strategies for a [`QueryResult`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema)]
pub enum QueryResultMode {
    /// Full data grid: column headers + one row per result row.
    #[default]
    DataGrid,
    /// Summary statistics: row count, column count, spatial flag.
    StatsSummary,
    /// Spatial map placeholder — emits a Document root for geo-viewer embedding.
    /// Only meaningful when `has_spatial()` is `true`.
    SpatialMap,
}

impl ArchiveDisplay for QueryResult {
    type Mode = QueryResultMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            QueryResultMode::DataGrid => Role(AkRole::Grid),
            QueryResultMode::StatsSummary => Role(AkRole::Article),
            QueryResultMode::SpatialMap => Role(AkRole::Document),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        match mode {
            QueryResultMode::DataGrid => {
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
                    let cell = NodeJson::new(Role(AkRole::ColumnHeader))
                        .with_label(format!("{}: {}", col.name, col.sql_type));
                    nodes.push((cell_id, cell));
                }

                let row_offset = id_base + 2 + self.columns.len() as u64;
                let row_ids: Vec<NodeId> = self
                    .rows
                    .rows
                    .iter()
                    .enumerate()
                    .map(|(i, _)| NodeId::from(row_offset + i as u64))
                    .collect();

                for (i, db_row) in self.rows.rows.iter().enumerate() {
                    let row_id = NodeId::from(row_offset + i as u64);
                    let label = db_row
                        .0
                        .iter()
                        .map(|(k, v)| format!("{}: {:?}", k, v))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let row_node = NodeJson::new(Role(AkRole::Row)).with_label(label);
                    nodes.push((row_id, row_node));
                }

                let mut all_children = vec![header_id];
                all_children.extend_from_slice(&row_ids);
                let root = NodeJson::new(Role(AkRole::Grid))
                    .with_label(format!(
                        "{} rows × {} cols",
                        self.row_count,
                        self.columns.len()
                    ))
                    .with_children(all_children);

                nodes.insert(0, (header_id, header_row));
                nodes.insert(0, (root_id, root));
            }
            QueryResultMode::StatsSummary => {
                let label = format!(
                    "{} rows, {} columns{}",
                    self.row_count,
                    self.columns.len(),
                    if self.has_spatial() {
                        format!(", spatial: {}", self.spatial_column_names.join(", "))
                    } else {
                        String::new()
                    },
                );
                let root = NodeJson::new(Role(AkRole::Article)).with_label(label);
                nodes.push((root_id, root));
            }
            QueryResultMode::SpatialMap => {
                let cols = if self.has_spatial() {
                    self.spatial_column_names.join(", ")
                } else {
                    "(none)".to_string()
                };
                let root = NodeJson::new(Role(AkRole::Document))
                    .with_label(format!("spatial result — geo columns: {}", cols));
                nodes.push((root_id, root));
            }
        }

        (root_id, nodes)
    }
}
