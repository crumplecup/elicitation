//! AccessKit display for [`MonitorSnapshot`].
use elicitation::Elicit;

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::MonitorSnapshot;

use super::ArchiveDisplay;

/// Display strategies for a [`MonitorSnapshot`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema, Elicit,
)]
pub enum MonitorSnapshotMode {
    /// Summary dashboard: one group per data category.
    #[default]
    Dashboard,
    /// A flat list of active session rows.
    SessionList,
}

impl ArchiveDisplay for MonitorSnapshot {
    type Mode = MonitorSnapshotMode;

    fn root_role(_mode: &Self::Mode) -> Role {
        Role(AkRole::Group)
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();
        let mut next = id_base + 1;

        match mode {
            MonitorSnapshotMode::Dashboard => {
                let cache_str = self
                    .cache_hit
                    .map_or("n/a".to_string(), |r| format!("{:.1}%", r * 100.0));

                // summary items
                let summary_items: &[String] = &[
                    format!("sessions: {}", self.sessions.len()),
                    format!("roles: {}", self.roles.len()),
                    format!("cache_hit: {}", cache_str),
                    format!("backups: {}", self.backups.len()),
                ];
                let item_ids: Vec<NodeId> = (0..summary_items.len())
                    .map(|i| NodeId::from(next + i as u64))
                    .collect();
                for (i, s) in summary_items.iter().enumerate() {
                    nodes.push((
                        NodeId::from(next + i as u64),
                        NodeJson::new(Role(AkRole::ListItem)).with_label(s.clone()),
                    ));
                }
                next += summary_items.len() as u64;
                let root = NodeJson::new(Role(AkRole::Group))
                    .with_label("monitor snapshot".to_string())
                    .with_children(item_ids);
                nodes.insert(0, (root_id, root));
                let _ = next;
            }
            MonitorSnapshotMode::SessionList => {
                let item_ids: Vec<NodeId> = (0..self.sessions.len())
                    .map(|i| NodeId::from(next + i as u64))
                    .collect();
                for (i, s) in self.sessions.iter().enumerate() {
                    let id = NodeId::from(next + i as u64);
                    let label = format!(
                        "[{}] {} ({}) — {}",
                        s.pid,
                        s.database.as_deref().unwrap_or("?"),
                        s.state,
                        s.query
                            .as_deref()
                            .unwrap_or("")
                            .chars()
                            .take(60)
                            .collect::<String>(),
                    );
                    nodes.push((id, NodeJson::new(Role(AkRole::ListItem)).with_label(label)));
                }
                let root = NodeJson::new(Role(AkRole::List))
                    .with_label(format!("sessions ({})", self.sessions.len()))
                    .with_children(item_ids);
                nodes.insert(0, (root_id, root));
            }
        }

        (root_id, nodes)
    }
}
