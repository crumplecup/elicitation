//! Tests for `RatatuiBackend` — verified AccessKit tree → TuiNode rendering.

use accesskit::{Node, NodeId, Rect, Role, Tree, TreeId, TreeUpdate};
use elicit_ratatui::{RatatuiBackend, TuiNode};
use elicit_ui::traits::UiTreeRenderer;
use elicit_ui::{Layout, UiRenderBackend, Viewport};

fn node_id(n: u64) -> NodeId {
    NodeId::from(n)
}

fn viewport() -> Viewport {
    Viewport::new(1920, 1080)
}

fn make_update(root_id: NodeId, nodes: Vec<(NodeId, Node)>, focus: NodeId) -> TreeUpdate {
    TreeUpdate {
        nodes,
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus,
    }
}

fn window_root(children: Vec<NodeId>) -> Node {
    let mut root = Node::new(Role::Window);
    root.set_children(children);
    root
}

fn make_button(label: &str) -> Node {
    let mut node = Node::new(Role::Button);
    node.set_label(label);
    node.set_bounds(Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 100.0,
        y1: 50.0,
    });
    node
}

fn make_label(text: &str) -> Node {
    let mut node = Node::new(Role::Label);
    node.set_value(text);
    node
}

#[test]
fn render_single_button_via_backend() {
    let root_id = node_id(0);
    let btn_id = node_id(1);

    let update = make_update(
        root_id,
        vec![
            (root_id, window_root(vec![btn_id])),
            (btn_id, make_button("Click me")),
        ],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let backend = RatatuiBackend::new();
    let (rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(rendered.root(), root_id);
    assert_eq!(stats.widgets_rendered, 1, "1 button");
    assert_eq!(stats.containers_rendered, 1, "root window");
    assert_eq!(stats.nodes_visited, 2);
}

#[test]
fn render_produces_tui_node_tree() {
    let root_id = node_id(0);
    let btn_id = node_id(1);

    let update = make_update(
        root_id,
        vec![
            (root_id, window_root(vec![btn_id])),
            (btn_id, make_button("Click me")),
        ],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let backend = RatatuiBackend::new();
    let tree = verified.into_verified_tree();
    let (tui_tree, _stats, _proof) = backend.render(&tree).unwrap();

    match tui_tree {
        TuiNode::Layout { children, .. } => {
            assert_eq!(children.len(), 1);
        }
        TuiNode::Widget { .. } => panic!("Root should be Layout, not Widget"),
        TuiNode::StatusBar { .. } => panic!("Root should be Layout, not StatusBar"),
    }
}

#[test]
fn render_form_with_multiple_children() {
    let root_id = node_id(0);
    let form_id = node_id(1);
    let label_id = node_id(2);
    let btn_id = node_id(3);

    let mut form = Node::new(Role::Form);
    form.set_children(vec![label_id, btn_id]);

    let update = make_update(
        root_id,
        vec![
            (root_id, window_root(vec![form_id])),
            (form_id, form),
            (label_id, make_label("Name")),
            (btn_id, make_button("Submit")),
        ],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let backend = RatatuiBackend::new();
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 2, "label + button");
    assert_eq!(stats.containers_rendered, 2, "window + form");
    assert_eq!(stats.nodes_visited, 4);
}

#[test]
fn render_hidden_nodes_skipped() {
    let root_id = node_id(0);
    let visible_id = node_id(1);
    let hidden_id = node_id(2);

    let mut hidden = make_button("Hidden");
    hidden.set_hidden();

    let update = make_update(
        root_id,
        vec![
            (root_id, window_root(vec![visible_id, hidden_id])),
            (visible_id, make_button("Visible")),
            (hidden_id, hidden),
        ],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let backend = RatatuiBackend::new();
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 1, "Only visible button");
    assert_eq!(stats.nodes_skipped, 1, "Hidden node skipped");
}

#[test]
fn render_empty_tree() {
    let root_id = node_id(0);

    let update = make_update(root_id, vec![(root_id, window_root(vec![]))], root_id);

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let backend = RatatuiBackend::new();
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 0);
    assert_eq!(stats.containers_rendered, 1, "Root window is a container");
}

#[test]
fn default_backend_works() {
    let backend = RatatuiBackend::default();
    assert_eq!(backend.backend_name(), "ratatui");
}
