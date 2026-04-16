//! AccessKit display for [`AdminSnapshot`].

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::AdminSnapshot;

use super::ArchiveDisplay;

/// Display strategies for an [`AdminSnapshot`], mirroring [`AdminTab`].
///
/// [`AdminTab`]: crate::archive::AdminTab
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema)]
pub enum AdminSnapshotMode {
    /// Role list.
    #[default]
    RoleList,
    /// Backup inventory.
    BackupList,
    /// WAL archiving status + server version.
    WalStatus,
    /// Installed extensions.
    ExtList,
    /// Server GUC settings.
    Settings,
}

impl ArchiveDisplay for AdminSnapshot {
    type Mode = AdminSnapshotMode;

    fn root_role(_mode: &Self::Mode) -> Role {
        Role(AkRole::List)
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();
        let base_child = id_base + 1;

        match mode {
            AdminSnapshotMode::RoleList => {
                let ids: Vec<NodeId> = (0..self.roles.len())
                    .map(|i| NodeId::from(base_child + i as u64))
                    .collect();
                for (i, r) in self.roles.iter().enumerate() {
                    let flags: Vec<&str> = [
                        r.can_login.then_some("LOGIN"),
                        r.superuser.then_some("SUPER"),
                        r.can_create_db.then_some("CREATEDB"),
                        r.can_create_role.then_some("CREATEROLE"),
                    ]
                    .into_iter()
                    .flatten()
                    .collect();
                    let label = format!("{} [{}]", r.name, flags.join("|"));
                    nodes.push((
                        NodeId::from(base_child + i as u64),
                        NodeJson::new(Role(AkRole::ListItem)).with_label(label),
                    ));
                }
                let root = NodeJson::new(Role(AkRole::List))
                    .with_label(format!("roles ({})", self.roles.len()))
                    .with_children(ids);
                nodes.insert(0, (root_id, root));
            }
            AdminSnapshotMode::BackupList => {
                let ids: Vec<NodeId> = (0..self.backups.len())
                    .map(|i| NodeId::from(base_child + i as u64))
                    .collect();
                for (i, b) in self.backups.iter().enumerate() {
                    nodes.push((
                        NodeId::from(base_child + i as u64),
                        NodeJson::new(Role(AkRole::ListItem)).with_label(b.clone()),
                    ));
                }
                let root = NodeJson::new(Role(AkRole::List))
                    .with_label(format!("backups ({})", self.backups.len()))
                    .with_children(ids);
                nodes.insert(0, (root_id, root));
            }
            AdminSnapshotMode::WalStatus => {
                let wal_label = if self.wal_ready {
                    "WAL: healthy"
                } else {
                    "WAL: not ready"
                };
                let ver_id = NodeId::from(base_child);
                let wal_id = NodeId::from(base_child + 1);
                let ver = NodeJson::new(Role(AkRole::ListItem))
                    .with_label(format!("server_version: {}", self.server_version));
                let wal = NodeJson::new(Role(AkRole::ListItem)).with_label(wal_label.to_string());
                let root = NodeJson::new(Role(AkRole::List))
                    .with_label("WAL status".to_string())
                    .with_children(vec![ver_id, wal_id]);
                nodes.push((ver_id, ver));
                nodes.push((wal_id, wal));
                nodes.insert(0, (root_id, root));
            }
            AdminSnapshotMode::ExtList => {
                let ids: Vec<NodeId> = (0..self.extensions.len())
                    .map(|i| NodeId::from(base_child + i as u64))
                    .collect();
                for (i, ext) in self.extensions.iter().enumerate() {
                    nodes.push((
                        NodeId::from(base_child + i as u64),
                        NodeJson::new(Role(AkRole::ListItem)).with_label(ext.clone()),
                    ));
                }
                let root = NodeJson::new(Role(AkRole::List))
                    .with_label(format!("extensions ({})", self.extensions.len()))
                    .with_children(ids);
                nodes.insert(0, (root_id, root));
            }
            AdminSnapshotMode::Settings => {
                let ids: Vec<NodeId> = (0..self.settings.len())
                    .map(|i| NodeId::from(base_child + i as u64))
                    .collect();
                for (i, (k, v)) in self.settings.iter().enumerate() {
                    nodes.push((
                        NodeId::from(base_child + i as u64),
                        NodeJson::new(Role(AkRole::ListItem)).with_label(format!("{k} = {v}")),
                    ));
                }
                let root = NodeJson::new(Role(AkRole::List))
                    .with_label(format!("settings ({})", self.settings.len()))
                    .with_children(ids);
                nodes.insert(0, (root_id, root));
            }
        }

        (root_id, nodes)
    }
}
