//! Tests for bidirectional TuiNode ↔ AccessKit conversion.

use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};
use elicit_ratatui::tui_accesskit_convert::{tree_update_to_tui_node, tui_node_to_tree_update};
use elicit_ratatui::{
    BlockJson, BordersJson, DirectionJson, ScrollbarOrientationJson, TuiNode, WidgetJson,
};

// ── Forward: TuiNode → AccessKit ────────────────────────────

#[test]
fn paragraph_converts_to_label() {
    let node = TuiNode::Widget {
        widget: Box::new(WidgetJson::Paragraph {
            text: "Hello, world".into(),
            style: None,
            wrap: true,
            scroll: None,
            alignment: None,
            block: None,
        }),
    };

    let update = tui_node_to_tree_update(&node);
    assert_eq!(update.nodes.len(), 1);

    let (_, ak_node) = &update.nodes[0];
    assert_eq!(ak_node.role(), Role::Label);
    assert_eq!(ak_node.value(), Some("Hello, world"));
}

#[test]
fn list_converts_to_accesskit_list() {
    let node = TuiNode::Widget {
        widget: Box::new(WidgetJson::List {
            items: vec!["Item A".into(), "Item B".into(), "Item C".into()],
            block: Some(BlockJson {
                title: Some("My List".into()),
                borders: BordersJson::All,
                border_type: None,
                border_style: None,
                style: None,
                padding: None,
            }),
            style: None,
            highlight_style: None,
            highlight_symbol: None,
            state: None,
        }),
    };

    let update = tui_node_to_tree_update(&node);
    let (_, ak_node) = &update.nodes[0];
    assert_eq!(ak_node.role(), Role::List);
    assert_eq!(ak_node.label(), Some("My List"));
    // Items encoded as newline-separated value
    assert!(ak_node.value().unwrap().contains("Item A"));
}

#[test]
fn gauge_preserves_progress() {
    let node = TuiNode::Widget {
        widget: Box::new(WidgetJson::Gauge {
            ratio: 0.75,
            label: Some("Loading...".into()),
            block: None,
            style: None,
            gauge_style: None,
        }),
    };

    let update = tui_node_to_tree_update(&node);
    let (_, ak_node) = &update.nodes[0];
    assert_eq!(ak_node.role(), Role::ProgressIndicator);
    assert!((ak_node.numeric_value().unwrap() - 75.0).abs() < f64::EPSILON);
    assert_eq!(ak_node.label(), Some("Loading..."));
}

#[test]
fn tabs_preserves_titles() {
    let node = TuiNode::Widget {
        widget: Box::new(WidgetJson::Tabs {
            titles: vec!["Tab 1".into(), "Tab 2".into(), "Tab 3".into()],
            selected: Some(1),
            block: None,
            style: None,
            highlight_style: None,
            divider: None,
        }),
    };

    let update = tui_node_to_tree_update(&node);
    let (_, ak_node) = &update.nodes[0];
    assert_eq!(ak_node.role(), Role::TabList);
    assert_eq!(ak_node.numeric_value(), Some(1.0));
}

#[test]
fn scrollbar_preserves_orientation() {
    let node = TuiNode::Widget {
        widget: Box::new(WidgetJson::Scrollbar {
            orientation: ScrollbarOrientationJson::HorizontalBottom,
            thumb_symbol: None,
            track_symbol: None,
            begin_symbol: None,
            end_symbol: None,
            style: None,
            thumb_style: None,
            track_style: None,
            state: None,
        }),
    };

    let update = tui_node_to_tree_update(&node);
    let (_, ak_node) = &update.nodes[0];
    assert_eq!(ak_node.role(), Role::ScrollBar);
    assert_eq!(
        ak_node.orientation(),
        Some(accesskit::Orientation::Horizontal)
    );
}

#[test]
fn layout_creates_container_with_children() {
    let node = TuiNode::Layout {
        direction: DirectionJson::Vertical,
        constraints: vec![],
        children: vec![
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: "Top".into(),
                    style: None,
                    wrap: true,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            },
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: "Bottom".into(),
                    style: None,
                    wrap: true,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            },
        ],
        margin: None,
    };

    let update = tui_node_to_tree_update(&node);
    assert_eq!(update.nodes.len(), 3);

    // Root should be a container with 2 children
    let root_id = update.tree.as_ref().unwrap().root;
    let container = update
        .nodes
        .iter()
        .find(|(id, _)| *id == root_id)
        .map(|(_, n)| n)
        .unwrap();
    assert_eq!(container.role(), Role::GenericContainer);
    assert_eq!(container.children().len(), 2);
}

// ── Reverse: AccessKit → TuiNode ────────────────────────────

#[test]
fn accesskit_label_to_paragraph() {
    let root_id = NodeId::from(0u64);
    let mut label = Node::new(Role::Label);
    label.set_value("Hello");

    let update = TreeUpdate {
        nodes: vec![(root_id, label)],
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    };

    let tui_node = tree_update_to_tui_node(&update).unwrap();
    match tui_node {
        TuiNode::Widget { widget } => match *widget {
            WidgetJson::Paragraph { text, .. } => assert_eq!(text, "Hello"),
            other => panic!("Expected Paragraph, got: {other:?}"),
        },
        other => panic!("Expected Widget, got: {other:?}"),
    }
}

#[test]
fn accesskit_progress_to_gauge() {
    let root_id = NodeId::from(0u64);
    let mut progress = Node::new(Role::ProgressIndicator);
    progress.set_numeric_value(60.0);
    progress.set_max_numeric_value(100.0);
    progress.set_label("Downloading");

    let update = TreeUpdate {
        nodes: vec![(root_id, progress)],
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    };

    let tui_node = tree_update_to_tui_node(&update).unwrap();
    match tui_node {
        TuiNode::Widget { widget } => match *widget {
            WidgetJson::Gauge { ratio, label, .. } => {
                assert!((ratio - 0.6).abs() < f64::EPSILON);
                assert_eq!(label, Some("Downloading".into()));
            }
            other => panic!("Expected Gauge, got: {other:?}"),
        },
        other => panic!("Expected Widget, got: {other:?}"),
    }
}

#[test]
fn paragraph_roundtrip() {
    let original = TuiNode::Widget {
        widget: Box::new(WidgetJson::Paragraph {
            text: "Round trip".into(),
            style: None,
            wrap: true,
            scroll: None,
            alignment: None,
            block: None,
        }),
    };

    let update = tui_node_to_tree_update(&original);
    let roundtripped = tree_update_to_tui_node(&update).unwrap();

    match roundtripped {
        TuiNode::Widget { widget } => match *widget {
            WidgetJson::Paragraph { text, .. } => assert_eq!(text, "Round trip"),
            other => panic!("Expected Paragraph, got: {other:?}"),
        },
        other => panic!("Expected Widget, got: {other:?}"),
    }
}

#[test]
fn gauge_roundtrip() {
    let original = TuiNode::Widget {
        widget: Box::new(WidgetJson::Gauge {
            ratio: 0.5,
            label: Some("Half".into()),
            block: None,
            style: None,
            gauge_style: None,
        }),
    };

    let update = tui_node_to_tree_update(&original);
    let roundtripped = tree_update_to_tui_node(&update).unwrap();

    match roundtripped {
        TuiNode::Widget { widget } => match *widget {
            WidgetJson::Gauge { ratio, label, .. } => {
                assert!((ratio - 0.5).abs() < 0.01);
                assert_eq!(label, Some("Half".into()));
            }
            other => panic!("Expected Gauge, got: {other:?}"),
        },
        other => panic!("Expected Widget, got: {other:?}"),
    }
}

#[test]
fn layout_roundtrip_preserves_children_count() {
    let original = TuiNode::Layout {
        direction: DirectionJson::Horizontal,
        constraints: vec![],
        children: vec![
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Clear),
            },
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Clear),
            },
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Clear),
            },
        ],
        margin: None,
    };

    let update = tui_node_to_tree_update(&original);
    let roundtripped = tree_update_to_tui_node(&update).unwrap();

    match roundtripped {
        TuiNode::Layout { children, .. } => {
            assert_eq!(children.len(), 3);
        }
        other => panic!("Expected Layout, got: {other:?}"),
    }
}

#[test]
fn empty_tree_returns_none() {
    let update = TreeUpdate {
        nodes: vec![],
        tree: None,
        tree_id: TreeId::ROOT,
        focus: NodeId::from(0u64),
    };

    assert!(tree_update_to_tui_node(&update).is_none());
}
