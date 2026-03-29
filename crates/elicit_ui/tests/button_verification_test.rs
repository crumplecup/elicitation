/// Test button verification and rendering pipeline.

use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};
use elicit_ui::{Layout, Viewport};

/// Create a simple button in an AccessKit tree.
fn create_button_tree(label: &str, width: f64, height: f64) -> TreeUpdate {
    let button_id = NodeId::from(1u64);
    let root_id = NodeId::from(0u64);

    let mut button = Node::new(Role::Button);
    button.set_label(label);
    button.set_bounds(accesskit::Rect {
        x0: 0.0,
        y0: 0.0,
        x1: width,
        y1: height,
    });

    let mut root = Node::new(Role::Window);
    root.set_children(vec![button_id]);

    TreeUpdate {
        nodes: vec![(root_id, root), (button_id, button)],
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    }
}

#[test]
fn test_button_valid_label() {
    let update = create_button_tree("Submit", 100.0, 50.0);
    let layout = Layout::from_update(update);

    let viewport = Viewport::new(1920, 1080);
    let result = layout.verify_a(viewport);

    assert!(result.is_ok(), "Button with valid label should pass Level A");
}

#[test]
fn test_button_missing_label() {
    let update = create_button_tree("", 100.0, 50.0);
    let layout = Layout::from_update(update);

    let viewport = Viewport::new(1920, 1080);
    let result = layout.verify_a(viewport);

    assert!(result.is_err(), "Button with empty label should fail");

    if let Err(report) = result {
        assert!(report.has_errors());
        assert_eq!(report.error_count(), 1);
    }
}

#[test]
fn test_button_below_min_target_size() {
    let update = create_button_tree("Click Me", 30.0, 30.0);
    let layout = Layout::from_update(update);

    let viewport = Viewport::new(1920, 1080);

    // Level A and AA don't check target size
    let result_a = layout.clone().verify_a(viewport);
    assert!(result_a.is_ok(), "Small button should pass Level A");

    // Level AAA checks target size
    let result_aaa = layout.verify_aaa(viewport);
    assert!(
        result_aaa.is_err(),
        "Button below 44x44 should fail Level AAA"
    );

    if let Err(report) = result_aaa {
        assert!(report.has_errors());
        // Should have error about target size
        let has_size_error = report.errors.iter().any(|e| {
            matches!(
                e.kind,
                elicit_ui::VerificationErrorKind::BelowMinTargetSize(_, _, _)
            )
        });
        assert!(has_size_error, "Should have target size error");
    }
}

#[test]
fn test_button_meets_min_target_size() {
    let update = create_button_tree("Large Button", 50.0, 50.0);
    let layout = Layout::from_update(update);

    let viewport = Viewport::new(1920, 1080);

    // Should pass all levels
    let result = layout.verify_aaa(viewport);
    assert!(
        result.is_ok(),
        "Button ≥44x44 should pass all verification levels"
    );
}

#[test]
fn test_button_overflow_viewport() {
    // Create button at position that would overflow
    let button_id = NodeId::from(1u64);
    let root_id = NodeId::from(0u64);

    let mut button = Node::new(Role::Button);
    button.set_label("Overflow Button");
    button.set_bounds(accesskit::Rect {
        x0: 1900.0, // Near edge
        y0: 0.0,
        x1: 2000.0, // Extends beyond 1920
        y1: 50.0,
    });

    let mut root = Node::new(Role::Window);
    root.set_children(vec![button_id]);

    let update = TreeUpdate {
        nodes: vec![(root_id, root), (button_id, button)],
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    };

    let layout = Layout::from_update(update);
    let viewport = Viewport::new(1920, 1080);

    let result = layout.verify_aa(viewport);
    assert!(result.is_err(), "Button overflowing viewport should fail");

    if let Err(report) = result {
        assert!(report.has_errors());
        let has_overflow_error = report.errors.iter().any(|e| {
            matches!(
                e.kind,
                elicit_ui::VerificationErrorKind::OverflowsViewport(_, _, _, _, _, _, _)
            )
        });
        assert!(has_overflow_error, "Should have overflow error");
    }
}

#[test]
fn test_state_transitions() {
    let update = create_button_tree("State Test", 100.0, 50.0);
    let pending = Layout::from_update(update);

    let viewport = Viewport::new(1920, 1080);

    // Pending → Verified
    let verified = pending.verify_aa(viewport).expect("Should verify");

    // Check we can access verified state
    assert_eq!(verified.viewport(), viewport);
    assert_eq!(verified.report().error_count(), 0);

    // Verified → Rendered (egui backend not enabled in test, would need feature flag)
    // This would be: let rendered = verified.render_egui(&ctx);
}
