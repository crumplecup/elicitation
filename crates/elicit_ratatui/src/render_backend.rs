//! Ratatui render backend for verified AccessKit trees.
//!
//! Implements [`RenderBackend`] to convert AccessKit node trees
//! into ratatui `TuiNode` structures for terminal rendering.

use accesskit::{Node, NodeId};
use elicit_ui::{RenderBackend, RenderStats};
use std::collections::HashMap;

use crate::serde_types::TuiNode;
use crate::tui_accesskit_convert;

/// Ratatui render backend for verified AccessKit trees.
///
/// Converts the AccessKit tree to a [`TuiNode`] tree on each render call.
/// The resulting `TuiNode` can then be rendered to a ratatui `Frame`.
///
/// # Example
///
/// ```rust,no_run
/// use elicit_ratatui::RatatuiBackend;
///
/// let backend = RatatuiBackend::new();
/// let tui_tree = backend.last_tui_tree();
/// ```
pub struct RatatuiBackend {
    last_tree: std::cell::RefCell<Option<TuiNode>>,
}

impl RatatuiBackend {
    /// Create a new ratatui render backend.
    pub fn new() -> Self {
        Self {
            last_tree: std::cell::RefCell::new(None),
        }
    }

    /// Get the last rendered `TuiNode` tree.
    ///
    /// Returns `None` if `render_tree` hasn't been called yet.
    pub fn last_tui_tree(&self) -> Option<TuiNode> {
        self.last_tree.borrow().clone()
    }
}

impl Default for RatatuiBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderBackend for RatatuiBackend {
    fn render_tree(&self, nodes: &HashMap<NodeId, Node>, root: NodeId) -> RenderStats {
        let mut stats = RenderStats::default();

        // Build a TreeUpdate from the node map for the converter
        let tree_update = accesskit::TreeUpdate {
            nodes: nodes.iter().map(|(id, n)| (*id, n.clone())).collect(),
            tree: Some(accesskit::Tree::new(root)),
            tree_id: accesskit::TreeId::ROOT,
            focus: root,
        };

        // Convert to TuiNode tree
        let tui_tree = tui_accesskit_convert::tree_update_to_tui_node(&tree_update);

        // Count stats by walking the original AccessKit tree
        count_nodes(nodes, root, &mut stats);

        *self.last_tree.borrow_mut() = tui_tree;

        tracing::debug!(
            visited = stats.nodes_visited,
            widgets = stats.widgets_rendered,
            containers = stats.containers_rendered,
            skipped = stats.nodes_skipped,
            "Ratatui render pass complete"
        );

        stats
    }
}

fn count_nodes(nodes: &HashMap<NodeId, Node>, node_id: NodeId, stats: &mut RenderStats) {
    let Some(node) = nodes.get(&node_id) else {
        stats.nodes_skipped += 1;
        return;
    };
    stats.nodes_visited += 1;

    if node.is_hidden() {
        stats.nodes_skipped += 1;
        return;
    }

    let children = node.children();
    // Container roles are always containers, even without children
    let is_container = matches!(
        node.role(),
        accesskit::Role::Window
            | accesskit::Role::Pane
            | accesskit::Role::Form
            | accesskit::Role::Group
            | accesskit::Role::Section
            | accesskit::Role::Region
            | accesskit::Role::Main
            | accesskit::Role::GenericContainer
            | accesskit::Role::Document
            | accesskit::Role::Toolbar
            | accesskit::Role::List
            | accesskit::Role::ListBox
            | accesskit::Role::Table
            | accesskit::Role::Grid
            | accesskit::Role::TabList
            | accesskit::Role::Dialog
            | accesskit::Role::AlertDialog
            | accesskit::Role::Menu
            | accesskit::Role::MenuBar
            | accesskit::Role::Navigation
            | accesskit::Role::ScrollView
    ) || !children.is_empty();

    if is_container {
        stats.containers_rendered += 1;
        for child_id in children {
            count_nodes(nodes, *child_id, stats);
        }
    } else {
        stats.widgets_rendered += 1;
    }
}
