//! AccessKit display for [`DdlDescriptor`].

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::DdlDescriptor;

use super::ArchiveDisplay;

/// Display strategies for a [`DdlDescriptor`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema)]
pub enum DdlDescriptorMode {
    /// A verbatim DDL text block.
    #[default]
    Block,
}

impl ArchiveDisplay for DdlDescriptor {
    type Mode = DdlDescriptorMode;

    fn root_role(_mode: &Self::Mode) -> Role {
        Role(AkRole::GenericContainer)
    }

    fn to_ak_nodes(&self, _mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let label = format!("-- {}.{}\n{}", self.schema, self.object_name, self.ddl);
        let root = NodeJson::new(Role(AkRole::GenericContainer)).with_label(label);
        (root_id, vec![(root_id, root)])
    }
}
