//! Tests for terminal-specific constraints.

use accesskit::{Node, NodeId, Rect, Role};
use elicit_ui::{
    ConstraintContext, ConstraintSetBuilder, MinReadableSize, TerminalAccessible,
    TerminalNoOverflow, Viewport,
};
use std::collections::HashMap;

/// Build a simple node map with a root and optional children.
fn make_context(
    nodes: Vec<(NodeId, Node)>,
    viewport_cols: u32,
    viewport_rows: u32,
) -> (HashMap<NodeId, Node>, Viewport) {
    let map: HashMap<NodeId, Node> = nodes.into_iter().collect();
    let viewport = Viewport::new(viewport_cols, viewport_rows);
    (map, viewport)
}

fn make_node(role: Role, bounds: Option<Rect>, children: Vec<NodeId>) -> Node {
    let mut node = Node::new(role);
    if let Some(b) = bounds {
        node.set_bounds(b);
    }
    if !children.is_empty() {
        node.set_children(children);
    }
    node
}

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Rect {
    Rect { x0, y0, x1, y1 }
}

// ─── TerminalNoOverflow ───

#[test]
fn terminal_no_overflow_fits() {
    let root_id = NodeId::from(0u64);
    let node = make_node(Role::Window, Some(rect(0.0, 0.0, 80.0, 24.0)), vec![]);
    let (nodes, viewport) = make_context(vec![(root_id, node)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = ConstraintSetBuilder::default()
        .hard(TerminalNoOverflow)
        .build();
    let result = set.verify(root_id, &ctx);
    assert!(result.is_valid(), "80x24 should fit in 80x24 viewport");
}

#[test]
fn terminal_no_overflow_exceeds_cols() {
    let root_id = NodeId::from(0u64);
    // 100 cols wide in 80-col viewport
    let node = make_node(Role::Window, Some(rect(0.0, 0.0, 100.0, 24.0)), vec![]);
    let (nodes, viewport) = make_context(vec![(root_id, node)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = ConstraintSetBuilder::default()
        .hard(TerminalNoOverflow)
        .build();
    let result = set.verify(root_id, &ctx);
    assert!(
        !result.is_valid(),
        "100 cols should overflow 80-col viewport"
    );
    assert_eq!(result.hard_violations.len(), 1);
}

#[test]
fn terminal_no_overflow_exceeds_rows() {
    let root_id = NodeId::from(0u64);
    // 30 rows tall in 24-row viewport
    let node = make_node(Role::Window, Some(rect(0.0, 0.0, 80.0, 30.0)), vec![]);
    let (nodes, viewport) = make_context(vec![(root_id, node)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = ConstraintSetBuilder::default()
        .hard(TerminalNoOverflow)
        .build();
    let result = set.verify(root_id, &ctx);
    assert!(
        !result.is_valid(),
        "30 rows should overflow 24-row viewport"
    );
}

#[test]
fn terminal_no_overflow_child_overflows() {
    let root_id = NodeId::from(0u64);
    let child_id = NodeId::from(1u64);

    let root = make_node(
        Role::Window,
        Some(rect(0.0, 0.0, 80.0, 24.0)),
        vec![child_id],
    );
    // Child starts at col 70, 20 cols wide → exceeds 80-col viewport
    let child = make_node(Role::Group, Some(rect(70.0, 0.0, 90.0, 10.0)), vec![]);

    let (nodes, viewport) = make_context(vec![(root_id, root), (child_id, child)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = ConstraintSetBuilder::default()
        .hard(TerminalNoOverflow)
        .build();
    let result = set.verify(root_id, &ctx);
    assert!(
        !result.is_valid(),
        "Child at col 70+20 overflows 80-col viewport"
    );
}

#[test]
fn terminal_no_overflow_no_bounds_passes() {
    let root_id = NodeId::from(0u64);
    let node = make_node(Role::Window, None, vec![]);
    let (nodes, viewport) = make_context(vec![(root_id, node)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = ConstraintSetBuilder::default()
        .hard(TerminalNoOverflow)
        .build();
    let result = set.verify(root_id, &ctx);
    assert!(result.is_valid(), "Node without bounds should pass");
}

// ─── MinReadableSize ───

#[test]
fn min_readable_size_passes() {
    let root_id = NodeId::from(0u64);
    let node = make_node(Role::Window, Some(rect(0.0, 0.0, 20.0, 5.0)), vec![]);
    let (nodes, viewport) = make_context(vec![(root_id, node)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = ConstraintSetBuilder::default()
        .hard(MinReadableSize::default())
        .build();
    let result = set.verify(root_id, &ctx);
    assert!(result.is_valid(), "20×5 exceeds default 10×3 minimum");
}

#[test]
fn min_readable_size_too_narrow() {
    let root_id = NodeId::from(0u64);
    // 8 cols wide — below 10-col minimum
    let node = make_node(Role::Group, Some(rect(0.0, 0.0, 8.0, 5.0)), vec![]);
    let (nodes, viewport) = make_context(vec![(root_id, node)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = ConstraintSetBuilder::default()
        .hard(MinReadableSize::default())
        .build();
    let result = set.verify(root_id, &ctx);
    assert!(!result.is_valid(), "8 cols below 10-col minimum");
}

#[test]
fn min_readable_size_too_short() {
    let root_id = NodeId::from(0u64);
    // 2 rows tall — below 3-row minimum
    let node = make_node(Role::Window, Some(rect(0.0, 0.0, 20.0, 2.0)), vec![]);
    let (nodes, viewport) = make_context(vec![(root_id, node)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = ConstraintSetBuilder::default()
        .hard(MinReadableSize::default())
        .build();
    let result = set.verify(root_id, &ctx);
    assert!(!result.is_valid(), "2 rows below 3-row minimum");
}

#[test]
fn min_readable_size_skips_non_containers() {
    let root_id = NodeId::from(0u64);
    // A button (leaf widget) with tiny bounds should NOT trigger
    let node = make_node(Role::Button, Some(rect(0.0, 0.0, 3.0, 1.0)), vec![]);
    let (nodes, viewport) = make_context(vec![(root_id, node)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = ConstraintSetBuilder::default()
        .hard(MinReadableSize::default())
        .build();
    let result = set.verify(root_id, &ctx);
    assert!(result.is_valid(), "Button is not a container — skip");
}

#[test]
fn min_readable_size_custom_thresholds() {
    let root_id = NodeId::from(0u64);
    // 15×4 should fail with custom min 20×5
    let node = make_node(Role::Group, Some(rect(0.0, 0.0, 15.0, 4.0)), vec![]);
    let (nodes, viewport) = make_context(vec![(root_id, node)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = ConstraintSetBuilder::default()
        .hard(MinReadableSize {
            min_cols: 20,
            min_rows: 5,
        })
        .build();
    let result = set.verify(root_id, &ctx);
    assert!(!result.is_valid(), "15×4 below custom 20×5 minimum");
}

#[test]
fn min_readable_nested_container_violation() {
    let root_id = NodeId::from(0u64);
    let child_id = NodeId::from(1u64);

    let root = make_node(
        Role::Window,
        Some(rect(0.0, 0.0, 80.0, 24.0)),
        vec![child_id],
    );
    // Child container is too small
    let child = make_node(Role::Group, Some(rect(0.0, 0.0, 5.0, 1.0)), vec![]);

    let (nodes, viewport) = make_context(vec![(root_id, root), (child_id, child)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = ConstraintSetBuilder::default()
        .hard(MinReadableSize::default())
        .build();
    let result = set.verify(root_id, &ctx);
    assert!(!result.is_valid(), "Child group 5×1 below 10×3 minimum");
    assert_eq!(result.hard_violations.len(), 1);
}

// ─── TerminalAccessible ───

#[test]
fn terminal_accessible_default_passes_valid_tree() {
    let root_id = NodeId::from(0u64);
    let button_id = NodeId::from(1u64);

    let root = make_node(
        Role::Window,
        Some(rect(0.0, 0.0, 80.0, 24.0)),
        vec![button_id],
    );
    let mut button = Node::new(Role::Button);
    button.set_label("Submit");
    button.set_bounds(rect(5.0, 5.0, 20.0, 8.0));

    let (nodes, viewport) = make_context(vec![(root_id, root), (button_id, button)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = TerminalAccessible::default().to_constraint_set();
    let result = set.verify(root_id, &ctx);
    assert!(result.is_valid(), "Valid tree should pass all constraints");
}

#[test]
fn terminal_accessible_catches_overflow() {
    let root_id = NodeId::from(0u64);
    let node = make_node(Role::Window, Some(rect(0.0, 0.0, 120.0, 24.0)), vec![]);
    let (nodes, viewport) = make_context(vec![(root_id, node)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = TerminalAccessible::default().to_constraint_set();
    let result = set.verify(root_id, &ctx);
    assert!(
        !result.is_valid(),
        "TerminalAccessible should catch overflow"
    );
}

#[test]
fn terminal_accessible_catches_small_pane() {
    let root_id = NodeId::from(0u64);
    // Window below 10×3 min
    let node = make_node(Role::Window, Some(rect(0.0, 0.0, 5.0, 2.0)), vec![]);
    let (nodes, viewport) = make_context(vec![(root_id, node)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = TerminalAccessible::default().to_constraint_set();
    let result = set.verify(root_id, &ctx);
    assert!(
        !result.is_valid(),
        "TerminalAccessible should catch small pane"
    );
}

#[test]
fn terminal_accessible_with_custom_min() {
    let root_id = NodeId::from(0u64);
    // 30×10 pane, custom min 40×12
    let node = make_node(Role::Window, Some(rect(0.0, 0.0, 30.0, 10.0)), vec![]);
    let (nodes, viewport) = make_context(vec![(root_id, node)], 80, 24);
    let ctx = ConstraintContext {
        nodes: &nodes,
        viewport,
    };

    let set = TerminalAccessible::with_min_readable(40, 12).to_constraint_set();
    let result = set.verify(root_id, &ctx);
    assert!(!result.is_valid(), "30×10 below custom 40×12 minimum");
}
