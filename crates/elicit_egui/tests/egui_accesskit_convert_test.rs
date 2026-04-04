//! Tests for bidirectional egui ↔ AccessKit conversion.

use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};
use elicit_egui::egui_accesskit_convert::{tree_update_to_ui_node, ui_node_to_tree_update};
use elicit_egui::serde_types::{ContainerJson, LayoutJson, RangeJson, UiNode, WidgetJson};

// ── Forward: UiNode → AccessKit ────────────────────────────

#[test]
fn button_converts_to_accesskit() {
    let node = UiNode::Widget {
        widget: WidgetJson::Button {
            text: "Submit".into(),
            wrap: false,
            fill: None,
            stroke: None,
            selected: false,
            frame: true,
            min_size: None,
        },
    };

    let update = ui_node_to_tree_update(&node);
    assert_eq!(update.nodes.len(), 1);

    let (_, ak_node) = &update.nodes[0];
    assert_eq!(ak_node.role(), Role::Button);
    assert_eq!(ak_node.label(), Some("Submit"));
}

#[test]
fn checkbox_preserves_toggle_state() {
    let node = UiNode::Widget {
        widget: WidgetJson::Checkbox {
            text: "Agree".into(),
            checked: true,
        },
    };

    let update = ui_node_to_tree_update(&node);
    let (_, ak_node) = &update.nodes[0];
    assert_eq!(ak_node.role(), Role::CheckBox);
    assert!(matches!(ak_node.toggled(), Some(accesskit::Toggled::True)));
}

#[test]
fn slider_preserves_numeric_range() {
    let node = UiNode::Widget {
        widget: WidgetJson::Slider {
            value: 42.0,
            range: RangeJson {
                min: 0.0,
                max: 100.0,
            },
            step: None,
            text: Some("Volume".into()),
            prefix: None,
            suffix: None,
            logarithmic: false,
            clamping: true,
            show_value: true,
        },
    };

    let update = ui_node_to_tree_update(&node);
    let (_, ak_node) = &update.nodes[0];
    assert_eq!(ak_node.role(), Role::Slider);
    assert_eq!(ak_node.numeric_value(), Some(42.0));
    assert_eq!(ak_node.min_numeric_value(), Some(0.0));
    assert_eq!(ak_node.max_numeric_value(), Some(100.0));
}

#[test]
fn container_with_children_creates_tree() {
    let node = UiNode::Container {
        container: ContainerJson::Window {
            title: "Main".into(),
            default_pos: None,
            default_size: None,
            resizable: true,
            collapsible: true,
            scroll: false,
            title_bar: true,
        },
        children: vec![
            UiNode::Widget {
                widget: WidgetJson::Label {
                    text: "Hello".into(),
                    wrap: false,
                    color: None,
                },
            },
            UiNode::Widget {
                widget: WidgetJson::Button {
                    text: "OK".into(),
                    wrap: false,
                    fill: None,
                    stroke: None,
                    selected: false,
                    frame: true,
                    min_size: None,
                },
            },
        ],
    };

    let update = ui_node_to_tree_update(&node);
    // 3 nodes: 2 children + 1 container (container pushed last in DFS)
    assert_eq!(update.nodes.len(), 3);

    // Root should be the window (first NodeId)
    let root_id = update.tree.as_ref().unwrap().root;
    let window_node = update
        .nodes
        .iter()
        .find(|(id, _)| *id == root_id)
        .map(|(_, n)| n)
        .unwrap();
    assert_eq!(window_node.role(), Role::Window);
    assert_eq!(window_node.children().len(), 2);
}

#[test]
fn hyperlink_preserves_url() {
    let node = UiNode::Widget {
        widget: WidgetJson::Hyperlink {
            text: "Docs".into(),
            url: "https://example.com".into(),
        },
    };

    let update = ui_node_to_tree_update(&node);
    let (_, ak_node) = &update.nodes[0];
    assert_eq!(ak_node.role(), Role::Link);
    assert_eq!(ak_node.url(), Some("https://example.com"));
}

#[test]
fn text_input_preserves_placeholder() {
    let node = UiNode::Widget {
        widget: WidgetJson::TextEditSingleline {
            text: "".into(),
            hint: Some("Enter email".into()),
            interactive: true,
        },
    };

    let update = ui_node_to_tree_update(&node);
    let (_, ak_node) = &update.nodes[0];
    assert_eq!(ak_node.role(), Role::TextInput);
    assert_eq!(ak_node.placeholder(), Some("Enter email"));
}

// ── Reverse: AccessKit → UiNode ────────────────────────────

#[test]
fn accesskit_button_to_ui_node() {
    let root_id = NodeId::from(0u64);
    let mut btn = Node::new(Role::Button);
    btn.set_label("Click");

    let update = TreeUpdate {
        nodes: vec![(root_id, btn)],
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    };

    let ui_node = tree_update_to_ui_node(&update).unwrap();
    match ui_node {
        UiNode::Widget {
            widget: WidgetJson::Button { text, .. },
        } => assert_eq!(text, "Click"),
        other => panic!("Expected Button widget, got: {other:?}"),
    }
}

#[test]
fn accesskit_slider_roundtrip() {
    let node = UiNode::Widget {
        widget: WidgetJson::Slider {
            value: 75.0,
            range: RangeJson {
                min: 10.0,
                max: 200.0,
            },
            step: None,
            text: Some("Brightness".into()),
            prefix: None,
            suffix: None,
            logarithmic: false,
            clamping: true,
            show_value: true,
        },
    };

    // Forward
    let update = ui_node_to_tree_update(&node);

    // Reverse
    let roundtripped = tree_update_to_ui_node(&update).unwrap();

    match roundtripped {
        UiNode::Widget {
            widget: WidgetJson::Slider {
                value, range, text, ..
            },
        } => {
            assert!((value - 75.0).abs() < f64::EPSILON);
            assert!((range.min - 10.0).abs() < f64::EPSILON);
            assert!((range.max - 200.0).abs() < f64::EPSILON);
            assert_eq!(text, Some("Brightness".into()));
        }
        other => panic!("Expected Slider, got: {other:?}"),
    }
}

#[test]
fn container_roundtrip_preserves_structure() {
    let node = UiNode::Container {
        container: ContainerJson::Group,
        children: vec![
            UiNode::Widget {
                widget: WidgetJson::Label {
                    text: "A".into(),
                    wrap: false,
                    color: None,
                },
            },
            UiNode::Widget {
                widget: WidgetJson::Label {
                    text: "B".into(),
                    wrap: false,
                    color: None,
                },
            },
        ],
    };

    let update = ui_node_to_tree_update(&node);
    let roundtripped = tree_update_to_ui_node(&update).unwrap();

    // Should be a container with 2 children
    match roundtripped {
        UiNode::Container { children, .. } => {
            assert_eq!(children.len(), 2);
        }
        other => panic!("Expected Container, got: {other:?}"),
    }
}

#[test]
fn layout_grid_roundtrip() {
    let node = UiNode::Layout {
        layout: LayoutJson::Grid {
            id: "test-grid".into(),
            num_columns: Some(3),
            striped: true,
            min_col_width: None,
            max_col_width: None,
            spacing: None,
        },
        children: vec![UiNode::Widget {
            widget: WidgetJson::Label {
                text: "Cell 1".into(),
                wrap: false,
                color: None,
            },
        }],
    };

    let update = ui_node_to_tree_update(&node);

    // Grid should become Role::Grid
    let grid_node = update
        .nodes
        .iter()
        .find(|(_, n)| n.role() == Role::Grid)
        .map(|(_, n)| n);
    assert!(grid_node.is_some(), "Should have a Grid role node");
}

#[test]
fn empty_tree_returns_none() {
    let update = TreeUpdate {
        nodes: vec![],
        tree: None,
        tree_id: TreeId::ROOT,
        focus: NodeId::from(0u64),
    };

    assert!(tree_update_to_ui_node(&update).is_none());
}
