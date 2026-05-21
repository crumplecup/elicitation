//! JSON-serializable representation of [`accesskit::TreeUpdate`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{NodeId, NodeJson, Tree, TreeId};

/// A serializable node entry in a [`TreeUpdateJson`].
///
/// Pairs a [`NodeId`] with its [`NodeJson`] for inclusion in a tree update.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NodeEntry {
    /// The stable ID of the node.
    pub id: NodeId,
    /// The full node data.
    pub node: NodeJson,
}

/// JSON-serializable representation of an [`accesskit::TreeUpdate`].
///
/// A `TreeUpdate` is an atomic change set that transitions a tree from one
/// known state to the next. Every update must include the current focus node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TreeUpdateJson {
    /// New or updated nodes. Each entry overwrites any existing node with
    /// the same ID. Order does not matter.
    pub nodes: Vec<NodeEntry>,

    /// The tree this update applies to.
    ///
    /// Use `TreeId::ROOT` for the main tree. Required on the first update
    /// for a subtree.
    pub tree_id: TreeId,

    /// Updated tree metadata (root node ID, toolkit info).
    ///
    /// May be omitted if unchanged, but must be provided when initialising a
    /// new tree.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree: Option<Tree>,

    /// The node that currently holds keyboard focus.
    ///
    /// Must always be provided, even when focus has not changed.
    pub focus: NodeId,
}

impl TreeUpdateJson {
    /// Creates an empty update targeting the root tree.
    pub fn new(focus: NodeId) -> Self {
        Self {
            nodes: Vec::new(),
            tree_id: TreeId(accesskit::TreeId::ROOT),
            tree: None,
            focus,
        }
    }

    /// Adds a node entry to the update.
    pub fn push_node(mut self, id: NodeId, node: NodeJson) -> Self {
        self.nodes.push(NodeEntry { id, node });
        self
    }

    /// Sets the tree metadata.
    pub fn with_tree(mut self, tree: Tree) -> Self {
        self.tree = Some(tree);
        self
    }

    /// Sets the tree ID (for subtrees).
    pub fn with_tree_id(mut self, id: TreeId) -> Self {
        self.tree_id = id;
        self
    }
}

impl From<TreeUpdateJson> for accesskit::TreeUpdate {
    fn from(j: TreeUpdateJson) -> Self {
        accesskit::TreeUpdate {
            nodes: j
                .nodes
                .into_iter()
                .map(|entry| (entry.id.0, accesskit::Node::from(entry.node)))
                .collect(),
            tree: j.tree.map(accesskit::Tree::from),
            tree_id: j.tree_id.0,
            focus: j.focus.0,
        }
    }
}

impl From<accesskit::TreeUpdate> for TreeUpdateJson {
    fn from(u: accesskit::TreeUpdate) -> Self {
        TreeUpdateJson {
            nodes: u
                .nodes
                .into_iter()
                .map(|(id, node)| NodeEntry {
                    id: NodeId(id),
                    node: NodeJson::from(&node),
                })
                .collect(),
            tree_id: TreeId(u.tree_id),
            tree: u.tree.map(Tree::from),
            focus: NodeId(u.focus),
        }
    }
}
