//! AccessKit display for [`QueryHistoryEntry`].
use elicitation::Elicit;

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::QueryHistoryEntry;

use super::ArchiveDisplay;

/// Display strategies for a [`QueryHistoryEntry`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema, Elicit,
)]
pub enum QueryHistoryEntryMode {
    /// A compact row showing timestamp, duration, and a SQL snippet.
    #[default]
    Row,
    /// An expanded group with all fields.
    Detailed,
}

impl ArchiveDisplay for QueryHistoryEntry {
    type Mode = QueryHistoryEntryMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            QueryHistoryEntryMode::Row => Role(AkRole::Row),
            QueryHistoryEntryMode::Detailed => Role(AkRole::Article),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        let sql_snippet: String = self.sql.chars().take(80).collect();
        let status = if self.error.is_some() { "ERR" } else { "OK" };

        match mode {
            QueryHistoryEntryMode::Row => {
                let label = format!(
                    "[{}] {}ms {} — {}",
                    status,
                    self.duration_ms,
                    self.executed_at.format("%H:%M:%S"),
                    sql_snippet,
                );
                nodes.push((root_id, NodeJson::new(Role(AkRole::Row)).with_label(label)));
            }
            QueryHistoryEntryMode::Detailed => {
                let error = self.error.clone().unwrap_or_default();
                let row_count = self.row_count.map_or("n/a".to_string(), |n| n.to_string());
                let props: &[(&str, String)] = &[
                    ("id", self.id.to_string()),
                    ("executed_at", self.executed_at.to_rfc3339()),
                    ("duration_ms", self.duration_ms.to_string()),
                    ("row_count", row_count),
                    ("status", status.to_string()),
                    ("error", error),
                    ("sql", self.sql.clone()),
                ];
                let child_ids: Vec<NodeId> = (0..props.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                let root = NodeJson::new(Role(AkRole::Article))
                    .with_label(format!("query #{}", self.id))
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
