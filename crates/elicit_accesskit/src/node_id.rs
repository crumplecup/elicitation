//! Wrappers for `accesskit::NodeId` and `accesskit::TreeId`.

use elicitation_derive::reflect_methods;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Wrapper around [`accesskit::NodeId`] with `JsonSchema` support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NodeId(pub accesskit::NodeId);

impl JsonSchema for NodeId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "NodeId".into()
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "integer",
            "minimum": 1,
            "description": "Non-zero u128 identifier for an accessibility node"
        })
    }
}

impl From<accesskit::NodeId> for NodeId {
    fn from(id: accesskit::NodeId) -> Self {
        Self(id)
    }
}

impl From<NodeId> for accesskit::NodeId {
    fn from(id: NodeId) -> Self {
        id.0
    }
}

impl std::ops::Deref for NodeId {
    type Target = accesskit::NodeId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[reflect_methods]
impl NodeId {
    /// Returns the raw `u128` value of this node ID.
    #[tracing::instrument(skip(self))]
    pub fn as_u128(&self) -> u128 {
        u128::from(self.0.0)
    }
}

/// Wrapper around [`accesskit::TreeId`] with `JsonSchema` support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TreeId(pub accesskit::TreeId);

impl JsonSchema for TreeId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "TreeId".into()
    }

    fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        accesskit::TreeId::json_schema(schema_gen)
    }

    fn inline_schema() -> bool {
        accesskit::TreeId::inline_schema()
    }
}

impl From<accesskit::TreeId> for TreeId {
    fn from(id: accesskit::TreeId) -> Self {
        Self(id)
    }
}

impl From<TreeId> for accesskit::TreeId {
    fn from(id: TreeId) -> Self {
        id.0
    }
}

impl std::ops::Deref for TreeId {
    type Target = accesskit::TreeId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[reflect_methods]
impl TreeId {
    /// Returns `true` if this is the root tree ID (nil UUID).
    #[tracing::instrument(skip(self))]
    pub fn is_root(&self) -> bool {
        self.0 == accesskit::TreeId::ROOT
    }
}
