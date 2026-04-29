//! AccessKit display for [`ExplainNode`].
//!
//! The tree is recursive; ID allocation uses a `&mut u64` counter so that each
//! node in an arbitrarily deep subtree gets a unique `NodeId`.
use elicitation::{Elicit, KaniCompose};

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::ExplainNode;

use super::ArchiveDisplay;

/// Display strategies for an [`ExplainNode`].
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema, Elicit, KaniCompose,
)]
pub enum ExplainNodeMode {
    /// A recursive tree item — the only meaningful display for plan nodes.
    #[default]
    TreeNode,
}

impl ArchiveDisplay for ExplainNode {
    type Mode = ExplainNodeMode;

    fn root_role(_mode: &Self::Mode) -> Role {
        Role(AkRole::TreeItem)
    }

    fn to_ak_nodes(&self, _mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let mut counter = id_base;
        let mut nodes = Vec::new();
        let root_id = build_explain_node(self, &mut counter, &mut nodes);
        (root_id, nodes)
    }
}

/// Recursively build AccessKit nodes for `node` and all its children.
///
/// `counter` is advanced past every ID allocated in this subtree so that
/// sibling subtrees start at a non-overlapping base.
fn build_explain_node(
    node: &ExplainNode,
    counter: &mut u64,
    nodes: &mut Vec<(NodeId, NodeJson)>,
) -> NodeId {
    let my_id = NodeId::from(*counter);
    *counter += 1;

    let relation = node.relation_name.as_deref().unwrap_or("").to_string();
    let actual = match (node.actual_rows, node.actual_total_time) {
        (Some(rows), Some(ms)) => format!(" actual_rows={rows} time={ms:.3}ms"),
        (Some(rows), None) => format!(" actual_rows={rows}"),
        _ => String::new(),
    };
    let label = if relation.is_empty() {
        format!(
            "{} cost={:.2}..{:.2} rows={}{}",
            node.node_type, node.startup_cost, node.total_cost, node.plan_rows, actual,
        )
    } else {
        format!(
            "{} on {} cost={:.2}..{:.2} rows={}{}",
            node.node_type, relation, node.startup_cost, node.total_cost, node.plan_rows, actual,
        )
    };

    let child_root_ids: Vec<NodeId> = node
        .children
        .iter()
        .map(|child| build_explain_node(child, counter, nodes))
        .collect();

    let tree_item = NodeJson::new(Role(AkRole::TreeItem))
        .with_label(label)
        .with_children(child_root_ids);

    nodes.push((my_id, tree_item));
    my_id
}
