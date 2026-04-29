//! AccessKit display for [`FunctionDescriptor`].
use elicitation::{Elicit, KaniCompose};

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::FunctionDescriptor;

use super::ArchiveDisplay;

/// Display strategies for a [`FunctionDescriptor`].
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
pub enum FunctionDescriptorMode {
    /// A compact row for use in a function-browser list.
    #[default]
    Row,
    /// An expanded group showing all properties including body preview.
    Detailed,
}

impl ArchiveDisplay for FunctionDescriptor {
    type Mode = FunctionDescriptorMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            FunctionDescriptorMode::Row => Role(AkRole::Row),
            FunctionDescriptorMode::Detailed => Role(AkRole::Group),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        let kind = if self.is_procedure { "proc" } else { "fn" };

        match mode {
            FunctionDescriptorMode::Row => {
                let label = format!(
                    "{}.{}({}) → {} [{kind}|{}|{}]",
                    self.schema,
                    self.name,
                    self.arguments,
                    self.return_type,
                    self.language,
                    self.volatility,
                );
                nodes.push((root_id, NodeJson::new(Role(AkRole::Row)).with_label(label)));
            }
            FunctionDescriptorMode::Detailed => {
                let props: &[(&str, String)] = &[
                    ("schema", self.schema.clone()),
                    ("name", self.name.clone()),
                    ("kind", kind.to_string()),
                    ("arguments", self.arguments.clone()),
                    ("return_type", self.return_type.clone()),
                    ("language", self.language.clone()),
                    ("volatility", format!("{}", self.volatility)),
                    ("body_preview", self.body_preview.clone()),
                ];
                let child_ids: Vec<NodeId> = (0..props.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                let root = NodeJson::new(Role(AkRole::Group))
                    .with_label(format!("{kind}: {}.{}", self.schema, self.name))
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
