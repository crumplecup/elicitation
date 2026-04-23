//! AccessKit display for [`SavedQuery`].
use elicitation::Elicit;

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::SavedQuery;

use super::ArchiveDisplay;

/// Display strategies for a [`SavedQuery`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema, Elicit,
)]
pub enum SavedQueryMode {
    /// A compact row showing name and a SQL snippet.
    #[default]
    Row,
    /// An expanded article with all fields.
    Detailed,
}

impl ArchiveDisplay for SavedQuery {
    type Mode = SavedQueryMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            SavedQueryMode::Row => Role(AkRole::Row),
            SavedQueryMode::Detailed => Role(AkRole::Article),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        let sql_snippet: String = self.sql.chars().take(60).collect();

        match mode {
            SavedQueryMode::Row => {
                let label = format!("{} — {}", self.name, sql_snippet);
                nodes.push((root_id, NodeJson::new(Role(AkRole::Row)).with_label(label)));
            }
            SavedQueryMode::Detailed => {
                let props: &[(&str, String)] = &[
                    ("id", self.id.to_string()),
                    ("name", self.name.clone()),
                    ("created_at", self.created_at.to_rfc3339()),
                    ("updated_at", self.updated_at.to_rfc3339()),
                    ("sql", self.sql.clone()),
                ];
                let child_ids: Vec<NodeId> = (0..props.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                let root = NodeJson::new(Role(AkRole::Article))
                    .with_label(format!("snippet: {}", self.name))
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
