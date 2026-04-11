//! Ratatui render backend for verified AccessKit trees.
//!
//! Implements [`UiRenderer`] to convert AccessKit node trees
//! into ratatui `TuiNode` structures for terminal rendering.

use accesskit::{Node, NodeId, Role};
use elicit_ui::{RenderComplete, RenderStats, UiRenderer, UiResult, VerifiedTree, WidgetId};
use elicitation::Established;
use std::collections::HashMap;
use std::sync::Mutex;

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
    last_tree: Mutex<Option<TuiNode>>,
}

impl RatatuiBackend {
    /// Create a new ratatui render backend.
    pub fn new() -> Self {
        Self {
            last_tree: Mutex::new(None),
        }
    }

    /// Get the last rendered `TuiNode` tree.
    ///
    /// Returns `None` if `render` hasn't been called yet.
    pub fn last_tui_tree(&self) -> Option<TuiNode> {
        self.last_tree.lock().unwrap().clone()
    }
}

impl Default for RatatuiBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl UiRenderer for RatatuiBackend {
    #[tracing::instrument(skip(self, tree))]
    fn render(&self, tree: &VerifiedTree) -> UiResult<(RenderStats, Established<RenderComplete>)> {
        let mut stats = RenderStats::default();

        let tree_update = accesskit::TreeUpdate {
            nodes: tree
                .nodes()
                .iter()
                .map(|(id, n)| (*id, n.clone()))
                .collect(),
            tree: Some(accesskit::Tree::new(tree.root())),
            tree_id: accesskit::TreeId::ROOT,
            focus: tree.root(),
        };

        let tui_tree = tui_accesskit_convert::tree_update_to_tui_node(&tree_update);
        count_nodes(tree.nodes(), tree.root(), &mut stats);

        *self.last_tree.lock().unwrap() = tui_tree;

        tracing::debug!(
            visited = stats.nodes_visited,
            widgets = stats.widgets_rendered,
            containers = stats.containers_rendered,
            skipped = stats.nodes_skipped,
            "Ratatui render pass complete"
        );

        Ok((stats, Established::assert()))
    }

    fn render_partial(&self, _node_id: WidgetId, tree: &VerifiedTree) -> UiResult<RenderStats> {
        let mut stats = RenderStats::default();
        count_nodes(tree.nodes(), tree.root(), &mut stats);
        Ok(stats)
    }

    fn supports_role(&self, _role: Role) -> bool {
        true
    }

    fn backend_name(&self) -> &str {
        "ratatui"
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
    let is_container = matches!(
        node.role(),
        Role::Window
            | Role::Pane
            | Role::Form
            | Role::Group
            | Role::Section
            | Role::Region
            | Role::Main
            | Role::GenericContainer
            | Role::Document
            | Role::Toolbar
            | Role::List
            | Role::ListBox
            | Role::Table
            | Role::Grid
            | Role::TabList
            | Role::Dialog
            | Role::AlertDialog
            | Role::Menu
            | Role::MenuBar
            | Role::Navigation
            | Role::ScrollView
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
