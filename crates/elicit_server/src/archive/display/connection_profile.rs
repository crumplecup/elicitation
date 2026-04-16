//! AccessKit display for [`ConnectionProfile`].

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::ConnectionProfile;

use super::ArchiveDisplay;

/// Display strategies for a [`ConnectionProfile`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema)]
pub enum ConnectionProfileMode {
    /// A summary card showing name, backend, and colour badge.
    #[default]
    Card,
    /// A compact row for use in a connection-picker list.
    Row,
}

impl ArchiveDisplay for ConnectionProfile {
    type Mode = ConnectionProfileMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            ConnectionProfileMode::Card => Role(AkRole::Article),
            ConnectionProfileMode::Row => Role(AkRole::Row),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        let color_tag = self
            .color
            .as_deref()
            .map_or(String::new(), |c| format!(" [{c}]"));

        match mode {
            ConnectionProfileMode::Row => {
                let label = format!("{}{} ({})", self.name, color_tag, self.backend);
                nodes.push((root_id, NodeJson::new(Role(AkRole::Row)).with_label(label)));
            }
            ConnectionProfileMode::Card => {
                let props: &[(&str, String)] = &[
                    ("name", self.name.clone()),
                    ("backend", format!("{}", self.backend)),
                    ("url_env", self.url_env_key.clone()),
                    ("color", self.color.clone().unwrap_or_default()),
                ];
                let child_ids: Vec<NodeId> = (0..props.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                let root = NodeJson::new(Role(AkRole::Article))
                    .with_label(format!("connection: {}{}", self.name, color_tag))
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
