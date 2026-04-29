//! AccessKit display for [`DomainDescriptor`].
use elicitation::{Elicit, KaniCompose};

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::DomainDescriptor;

use super::ArchiveDisplay;

/// Display strategies for a [`DomainDescriptor`].
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
pub enum DomainDescriptorMode {
    /// A compact row showing name and base type.
    #[default]
    Row,
    /// An expanded group with all constraint details.
    Detailed,
}

impl ArchiveDisplay for DomainDescriptor {
    type Mode = DomainDescriptorMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            DomainDescriptorMode::Row => Role(AkRole::Row),
            DomainDescriptorMode::Detailed => Role(AkRole::Group),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        let not_null = if self.not_null { " NOT NULL" } else { "" };

        match mode {
            DomainDescriptorMode::Row => {
                let checks = if self.check_constraints.is_empty() {
                    String::new()
                } else {
                    format!(" CHECK({})", self.check_constraints.join(", "))
                };
                let label = format!(
                    "{}.{} :: {}{}{checks}",
                    self.schema, self.name, self.base_type, not_null,
                );
                nodes.push((root_id, NodeJson::new(Role(AkRole::Row)).with_label(label)));
            }
            DomainDescriptorMode::Detailed => {
                let default = self.default_expr.clone().unwrap_or_default();
                let checks = self.check_constraints.join("; ");
                let props: &[(&str, String)] = &[
                    ("schema", self.schema.clone()),
                    ("name", self.name.clone()),
                    ("base_type", self.base_type.clone()),
                    ("not_null", self.not_null.to_string()),
                    ("default", default),
                    ("checks", checks),
                ];
                let child_ids: Vec<NodeId> = (0..props.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                let root = NodeJson::new(Role(AkRole::Group))
                    .with_label(format!("domain: {}.{}", self.schema, self.name))
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
