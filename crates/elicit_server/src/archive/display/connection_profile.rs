//! AccessKit display for [`ConnectionProfile`].
use elicitation::{Elicit, KaniCompose};

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::{ConnectionProfile, SslMode};

use super::ArchiveDisplay;

/// Display strategies for a [`ConnectionProfile`].
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
pub enum ConnectionProfileMode {
    /// A summary card showing name, backend, and colour badge.
    #[default]
    Card,
    /// A compact row for use in a connection-picker list.
    Row,
    /// An editable form showing all fields (basic + SSH tunnel + SSL).
    Editor,
}

impl ArchiveDisplay for ConnectionProfile {
    type Mode = ConnectionProfileMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            ConnectionProfileMode::Card => Role(AkRole::Article),
            ConnectionProfileMode::Row => Role(AkRole::Row),
            ConnectionProfileMode::Editor => Role(AkRole::Form),
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
                let ssh_label = match &self.ssh_host {
                    Some(h) => format!(
                        "{}@{}:{}",
                        self.ssh_user.as_deref().unwrap_or("?"),
                        h,
                        self.ssh_port.unwrap_or(22)
                    ),
                    None => "none".to_string(),
                };
                let props: &[(&str, String)] = &[
                    ("name", self.name.clone()),
                    ("backend", format!("{}", self.backend)),
                    ("url_env", self.url_env_key.clone()),
                    ("color", self.color.clone().unwrap_or_default()),
                    ("ssh", ssh_label),
                    ("ssl_mode", format!("{}", self.ssl_mode)),
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
            ConnectionProfileMode::Editor => {
                // Each field becomes a labelled TextInput child of the Form root.
                let fields: &[(&str, String)] = &[
                    ("Name", self.name.clone()),
                    ("URL env var", self.url_env_key.clone()),
                    ("Color", self.color.clone().unwrap_or_default()),
                    // SSH tunnel
                    ("SSH host", self.ssh_host.clone().unwrap_or_default()),
                    (
                        "SSH port",
                        self.ssh_port.map_or(String::new(), |p| p.to_string()),
                    ),
                    ("SSH user", self.ssh_user.clone().unwrap_or_default()),
                    (
                        "SSH key env var",
                        self.ssh_key_env.clone().unwrap_or_default(),
                    ),
                    // SSL
                    ("SSL mode", ssl_mode_label(self.ssl_mode)),
                    (
                        "SSL cert env var",
                        self.ssl_cert_env.clone().unwrap_or_default(),
                    ),
                    (
                        "SSL key env var",
                        self.ssl_key_env.clone().unwrap_or_default(),
                    ),
                    (
                        "SSL CA env var",
                        self.ssl_ca_env.clone().unwrap_or_default(),
                    ),
                ];
                let child_ids: Vec<NodeId> = (0..fields.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                let root = NodeJson::new(Role(AkRole::Form))
                    .with_label(format!("Edit connection: {}", self.name))
                    .with_children(child_ids);
                for (i, (label, value)) in fields.iter().enumerate() {
                    let id = NodeId::from(id_base + 1 + i as u64);
                    let mut field =
                        NodeJson::new(Role(AkRole::TextInput)).with_label(label.to_string());
                    if !value.is_empty() {
                        field = field.with_value(value.clone());
                    }
                    nodes.push((id, field));
                }
                nodes.insert(0, (root_id, root));
            }
        }

        (root_id, nodes)
    }
}

fn ssl_mode_label(mode: SslMode) -> String {
    format!("{mode}")
}
