//! Complex layout verification tests for Phase 2 constraint coverage.
//!
//! Covers multi-element layouts, nested containers, mixed pass/fail
//! scenarios, edge cases, and all error variants.

use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};
use elicit_ui::{Layout, VerificationErrorKind, Viewport};

// ============================================================================
// Helpers
// ============================================================================

/// Create a form layout with multiple labeled inputs.
fn create_form_tree(fields: &[(&str, Role, f64, f64)]) -> TreeUpdate {
    let root_id = NodeId::from(0u64);
    let mut root = Node::new(Role::Form);

    let mut nodes = vec![];
    let mut child_ids = vec![];

    for (i, (label, role, width, height)) in fields.iter().enumerate() {
        let id = NodeId::from((i + 1) as u64);
        let mut node = Node::new(*role);
        node.set_label(*label);
        node.set_bounds(accesskit::Rect {
            x0: 10.0,
            y0: (i as f64) * 60.0,
            x1: 10.0 + width,
            y1: (i as f64) * 60.0 + height,
        });
        child_ids.push(id);
        nodes.push((id, node));
    }

    root.set_children(child_ids);
    nodes.insert(0, (root_id, root));

    TreeUpdate {
        nodes,
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    }
}

/// Create a nested toolbar layout: toolbar → buttons.
fn create_toolbar_tree(buttons: &[(&str, f64, f64)]) -> TreeUpdate {
    let root_id = NodeId::from(0u64);
    let toolbar_id = NodeId::from(100u64);
    let mut root = Node::new(Role::Window);

    let mut toolbar = Node::new(Role::Toolbar);
    let mut nodes = vec![];
    let mut button_ids = vec![];

    for (i, (label, width, height)) in buttons.iter().enumerate() {
        let id = NodeId::from((i + 1) as u64);
        let mut btn = Node::new(Role::Button);
        btn.set_label(*label);
        btn.set_bounds(accesskit::Rect {
            x0: (i as f64) * (*width + 10.0),
            y0: 0.0,
            x1: (i as f64) * (*width + 10.0) + width,
            y1: *height,
        });
        button_ids.push(id);
        nodes.push((id, btn));
    }

    toolbar.set_children(button_ids);
    root.set_children(vec![toolbar_id]);

    nodes.insert(0, (root_id, root));
    nodes.insert(1, (toolbar_id, toolbar));

    TreeUpdate {
        nodes,
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    }
}

// ============================================================================
// Multi-element tests
// ============================================================================

#[test]
fn test_form_all_fields_valid() {
    let update = create_form_tree(&[
        ("Username", Role::TextInput, 200.0, 50.0),
        ("Password", Role::TextInput, 200.0, 50.0),
        ("Remember me", Role::CheckBox, 50.0, 50.0),
        ("Submit", Role::Button, 100.0, 50.0),
    ]);
    let layout = Layout::from_update(update);
    let vp = Viewport::new(1920, 1080);

    let result = layout.verify_aa(vp);
    assert!(result.is_ok(), "All-valid form should pass AA: {result:?}");
}

#[test]
fn test_form_one_missing_label() {
    let update = create_form_tree(&[
        ("Username", Role::TextInput, 200.0, 50.0),
        ("", Role::TextInput, 200.0, 50.0), // Missing label
        ("Submit", Role::Button, 100.0, 50.0),
    ]);
    let layout = Layout::from_update(update);
    let vp = Viewport::new(1920, 1080);

    let result = layout.verify_a(vp);
    assert!(result.is_err(), "Form with one missing label should fail");

    let report = result.unwrap_err();
    assert_eq!(report.error_count(), 1, "Exactly one error expected");
    assert!(matches!(
        &report.errors[0].kind,
        VerificationErrorKind::EmptyLabel(_)
    ));
}

#[test]
fn test_form_multiple_errors() {
    let update = create_form_tree(&[
        ("", Role::TextInput, 200.0, 50.0),   // Missing label
        ("", Role::Button, 200.0, 50.0),       // Missing label
        ("OK", Role::CheckBox, 200.0, 50.0),   // Valid
    ]);
    let layout = Layout::from_update(update);
    let vp = Viewport::new(1920, 1080);

    let result = layout.verify_a(vp);
    assert!(result.is_err(), "Multiple missing labels should fail");

    let report = result.unwrap_err();
    assert_eq!(report.error_count(), 2, "Should have exactly 2 errors");
}

// ============================================================================
// Nested container tests
// ============================================================================

#[test]
fn test_toolbar_all_buttons_valid() {
    let update = create_toolbar_tree(&[
        ("New", 80.0, 50.0),
        ("Open", 80.0, 50.0),
        ("Save", 80.0, 50.0),
    ]);
    let layout = Layout::from_update(update);
    let vp = Viewport::new(1920, 1080);

    let result = layout.verify_aaa(vp);
    assert!(result.is_ok(), "Toolbar with valid buttons should pass AAA");
}

#[test]
fn test_toolbar_nested_unlabeled_button() {
    let update = create_toolbar_tree(&[
        ("New", 80.0, 50.0),
        ("", 80.0, 50.0), // Unlabeled icon button
        ("Save", 80.0, 50.0),
    ]);
    let layout = Layout::from_update(update);
    let vp = Viewport::new(1920, 1080);

    let result = layout.verify_a(vp);
    assert!(
        result.is_err(),
        "Toolbar with unlabeled button should fail"
    );
}

#[test]
fn test_toolbar_small_buttons_aaa() {
    let update = create_toolbar_tree(&[
        ("New", 30.0, 30.0),  // Too small for AAA
        ("Open", 30.0, 30.0), // Too small for AAA
        ("Save", 50.0, 50.0), // OK
    ]);
    let layout = Layout::from_update(update);
    let vp = Viewport::new(1920, 1080);

    // Should pass AA (no target size check)
    let layout_clone = layout.clone();
    let result_aa = layout_clone.verify_aa(vp);
    assert!(result_aa.is_ok(), "Small buttons should pass AA");

    // Should fail AAA (target size check)
    let result_aaa = layout.verify_aaa(vp);
    assert!(result_aaa.is_err(), "Small buttons should fail AAA");

    let report = result_aaa.unwrap_err();
    let size_errors = report
        .errors
        .iter()
        .filter(|e| matches!(e.kind, VerificationErrorKind::BelowMinTargetSize(_, _, _)))
        .count();
    assert_eq!(size_errors, 2, "Two buttons are below minimum target size");
}

// ============================================================================
// Non-interactive elements
// ============================================================================

#[test]
fn test_decorative_elements_pass() {
    let root_id = NodeId::from(0u64);
    let image_id = NodeId::from(1u64);
    let text_id = NodeId::from(2u64);

    let mut root = Node::new(Role::Window);
    root.set_children(vec![image_id, text_id]);

    // Decorative image without label — should pass (not interactive)
    let image = Node::new(Role::Image);

    // Static text without explicit label — should pass (not interactive)
    let text = Node::new(Role::Label);

    let update = TreeUpdate {
        nodes: vec![(root_id, root), (image_id, image), (text_id, text)],
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    };

    let layout = Layout::from_update(update);
    let vp = Viewport::new(1920, 1080);

    let result = layout.verify_aaa(vp);
    assert!(
        result.is_ok(),
        "Decorative elements should pass all levels: {result:?}"
    );
}

// ============================================================================
// Viewport edge cases
// ============================================================================

#[test]
fn test_element_exactly_fills_viewport() {
    let root_id = NodeId::from(0u64);
    let btn_id = NodeId::from(1u64);

    let mut root = Node::new(Role::Window);
    root.set_children(vec![btn_id]);

    let mut btn = Node::new(Role::Button);
    btn.set_label("Full Screen");
    btn.set_bounds(accesskit::Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 800.0,
        y1: 600.0,
    });

    let update = TreeUpdate {
        nodes: vec![(root_id, root), (btn_id, btn)],
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    };

    let layout = Layout::from_update(update);
    let vp = Viewport::new(800, 600);

    let result = layout.verify_aa(vp);
    assert!(
        result.is_ok(),
        "Element exactly filling viewport should pass"
    );
}

#[test]
fn test_element_one_pixel_overflow() {
    let root_id = NodeId::from(0u64);
    let btn_id = NodeId::from(1u64);

    let mut root = Node::new(Role::Window);
    root.set_children(vec![btn_id]);

    let mut btn = Node::new(Role::Button);
    btn.set_label("Overflow");
    btn.set_bounds(accesskit::Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 801.0, // One pixel over
        y1: 600.0,
    });

    let update = TreeUpdate {
        nodes: vec![(root_id, root), (btn_id, btn)],
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    };

    let layout = Layout::from_update(update);
    let vp = Viewport::new(800, 600);

    let result = layout.verify_aa(vp);
    assert!(result.is_err(), "One pixel overflow should be detected");

    let report = result.unwrap_err();
    assert!(report.errors.iter().any(|e| matches!(
        e.kind,
        VerificationErrorKind::OverflowsViewport(_, _, _, _, _, _, _)
    )));
}

#[test]
fn test_small_viewport() {
    let update = create_form_tree(&[
        ("Name", Role::TextInput, 200.0, 50.0),
        ("Submit", Role::Button, 100.0, 50.0),
    ]);
    let layout = Layout::from_update(update);

    // Viewport smaller than form elements
    let vp = Viewport::new(100, 100);

    let result = layout.verify_aa(vp);
    assert!(
        result.is_err(),
        "Elements overflowing small viewport should fail"
    );
}

// ============================================================================
// Mixed error types in single validation
// ============================================================================

#[test]
fn test_mixed_errors_label_and_overflow() {
    let root_id = NodeId::from(0u64);
    let btn1_id = NodeId::from(1u64);
    let btn2_id = NodeId::from(2u64);

    let mut root = Node::new(Role::Window);
    root.set_children(vec![btn1_id, btn2_id]);

    // Button 1: missing label
    let mut btn1 = Node::new(Role::Button);
    btn1.set_label("");
    btn1.set_bounds(accesskit::Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 100.0,
        y1: 50.0,
    });

    // Button 2: overflows viewport
    let mut btn2 = Node::new(Role::Button);
    btn2.set_label("Overflow");
    btn2.set_bounds(accesskit::Rect {
        x0: 900.0,
        y0: 0.0,
        x1: 1100.0,
        y1: 50.0,
    });

    let update = TreeUpdate {
        nodes: vec![(root_id, root), (btn1_id, btn1), (btn2_id, btn2)],
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    };

    let layout = Layout::from_update(update);
    let vp = Viewport::new(1000, 600);

    let result = layout.verify_aa(vp);
    assert!(result.is_err(), "Mixed errors should fail");

    let report = result.unwrap_err();

    let has_label_error = report.errors.iter().any(|e| {
        matches!(
            e.kind,
            VerificationErrorKind::EmptyLabel(_) | VerificationErrorKind::MissingLabel(_)
        )
    });
    let has_overflow_error = report.errors.iter().any(|e| {
        matches!(
            e.kind,
            VerificationErrorKind::OverflowsViewport(_, _, _, _, _, _, _)
        )
    });

    assert!(has_label_error, "Should have label error");
    assert!(has_overflow_error, "Should have overflow error");
    assert!(
        report.error_count() >= 2,
        "Should have at least 2 different errors"
    );
}

#[test]
fn test_mixed_errors_label_size_overflow() {
    let root_id = NodeId::from(0u64);
    let btn1_id = NodeId::from(1u64);
    let btn2_id = NodeId::from(2u64);
    let btn3_id = NodeId::from(3u64);

    let mut root = Node::new(Role::Window);
    root.set_children(vec![btn1_id, btn2_id, btn3_id]);

    // Button 1: missing label
    let mut btn1 = Node::new(Role::Button);
    btn1.set_label("");
    btn1.set_bounds(accesskit::Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 100.0,
        y1: 50.0,
    });

    // Button 2: too small for AAA
    let mut btn2 = Node::new(Role::Button);
    btn2.set_label("Tiny");
    btn2.set_bounds(accesskit::Rect {
        x0: 0.0,
        y0: 60.0,
        x1: 30.0,
        y1: 90.0,
    });

    // Button 3: overflows viewport
    let mut btn3 = Node::new(Role::Button);
    btn3.set_label("Far Away");
    btn3.set_bounds(accesskit::Rect {
        x0: 1800.0,
        y0: 0.0,
        x1: 2000.0,
        y1: 50.0,
    });

    let update = TreeUpdate {
        nodes: vec![
            (root_id, root),
            (btn1_id, btn1),
            (btn2_id, btn2),
            (btn3_id, btn3),
        ],
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    };

    let layout = Layout::from_update(update);
    let vp = Viewport::new(1920, 1080);

    // AAA should catch all three error types
    let result = layout.verify_aaa(vp);
    assert!(result.is_err(), "Mixed AAA errors should fail");

    let report = result.unwrap_err();

    let label_errors = report
        .errors
        .iter()
        .filter(|e| matches!(e.kind, VerificationErrorKind::EmptyLabel(_)))
        .count();
    let size_errors = report
        .errors
        .iter()
        .filter(|e| matches!(e.kind, VerificationErrorKind::BelowMinTargetSize(_, _, _)))
        .count();
    let overflow_errors = report
        .errors
        .iter()
        .filter(|e| {
            matches!(
                e.kind,
                VerificationErrorKind::OverflowsViewport(_, _, _, _, _, _, _)
            )
        })
        .count();

    assert_eq!(label_errors, 1, "One label error");
    assert_eq!(size_errors, 1, "One size error");
    assert_eq!(overflow_errors, 1, "One overflow error");
}

// ============================================================================
// Elements without bounds
// ============================================================================

#[test]
fn test_element_without_bounds_passes() {
    let root_id = NodeId::from(0u64);
    let btn_id = NodeId::from(1u64);

    let mut root = Node::new(Role::Window);
    root.set_children(vec![btn_id]);

    // Button with label but NO bounds set
    let mut btn = Node::new(Role::Button);
    btn.set_label("No Bounds");
    // Intentionally not setting bounds

    let update = TreeUpdate {
        nodes: vec![(root_id, root), (btn_id, btn)],
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    };

    let layout = Layout::from_update(update);
    let vp = Viewport::new(1920, 1080);

    // All levels should pass (no bounds = not rendered yet = skip checks)
    let result = layout.verify_aaa(vp);
    assert!(
        result.is_ok(),
        "Element without bounds should pass all levels"
    );
}

// ============================================================================
// Verification level progression
// ============================================================================

#[test]
fn test_level_progression_a_aa_aaa() {
    // A button that passes A and AA but fails AAA (too small)
    let update = create_form_tree(&[("Click", Role::Button, 30.0, 30.0)]);

    // Level A: passes (no size check)
    let layout_a = Layout::from_update(update.clone());
    let result_a = layout_a.verify_a(Viewport::new(1920, 1080));
    assert!(result_a.is_ok(), "Small button should pass Level A");

    // Level AA: passes (no size check)
    let layout_aa = Layout::from_update(update.clone());
    let result_aa = layout_aa.verify_aa(Viewport::new(1920, 1080));
    assert!(result_aa.is_ok(), "Small button should pass Level AA");

    // Level AAA: fails (size check)
    let layout_aaa = Layout::from_update(update);
    let result_aaa = layout_aaa.verify_aaa(Viewport::new(1920, 1080));
    assert!(result_aaa.is_err(), "Small button should fail Level AAA");
}
