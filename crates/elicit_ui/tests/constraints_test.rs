//! Tests for the constraint system, spatial constraints, and constraint profiles.

use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};
use elicit_ui::{
    Constraint, ConstraintContext, ConstraintProfile, ConstraintSetBuilder, GridAlignment,
    HasLabelConstraint, Layout, MinSpacing, MinTouchTargetConstraint, NoOverflowConstraint,
    Reflow320, ResizeText200, TextSpacing, ValidRoleConstraint, Viewport,
};
use std::collections::BTreeMap;

// ─── Helpers ───────────────────────────────────────────────────────────

fn make_node(role: Role, label: &str, bounds: Option<(f64, f64, f64, f64)>) -> Node {
    let mut node = Node::new(role);
    if !label.is_empty() {
        node.set_label(label);
    }
    if let Some((x0, y0, x1, y1)) = bounds {
        node.set_bounds(accesskit::Rect { x0, y0, x1, y1 });
    }
    node
}

fn make_ctx(nodes: &BTreeMap<NodeId, Node>, viewport: Viewport) -> ConstraintContext<'_> {
    ConstraintContext { nodes, viewport }
}

// ─── Constraint trait unit tests ───────────────────────────────────────

#[test]
fn has_label_passes_for_labeled_button() {
    let id = NodeId::from(1u64);
    let node = make_node(Role::Button, "Submit", None);
    let nodes = BTreeMap::from([(id, node)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    assert!(HasLabelConstraint.check(id, &ctx).is_ok());
}

#[test]
fn has_label_fails_for_unlabeled_button() {
    let id = NodeId::from(1u64);
    let node = make_node(Role::Button, "", None);
    let nodes = BTreeMap::from([(id, node)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    assert!(HasLabelConstraint.check(id, &ctx).is_err());
}

#[test]
fn valid_role_passes_for_known_role() {
    let id = NodeId::from(1u64);
    let node = make_node(Role::Button, "OK", None);
    let nodes = BTreeMap::from([(id, node)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    assert!(ValidRoleConstraint.check(id, &ctx).is_ok());
}

#[test]
fn no_overflow_passes_when_within_viewport() {
    let id = NodeId::from(1u64);
    let node = make_node(Role::Button, "OK", Some((0.0, 0.0, 100.0, 50.0)));
    let nodes = BTreeMap::from([(id, node)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    assert!(NoOverflowConstraint.check(id, &ctx).is_ok());
}

#[test]
fn no_overflow_fails_when_beyond_viewport() {
    let id = NodeId::from(1u64);
    let node = make_node(Role::Button, "OK", Some((0.0, 0.0, 2000.0, 50.0)));
    let nodes = BTreeMap::from([(id, node)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    assert!(NoOverflowConstraint.check(id, &ctx).is_err());
}

#[test]
fn min_touch_target_passes_for_large_button() {
    let id = NodeId::from(1u64);
    let node = make_node(Role::Button, "OK", Some((0.0, 0.0, 100.0, 100.0)));
    let nodes = BTreeMap::from([(id, node)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    assert!(MinTouchTargetConstraint.check(id, &ctx).is_ok());
}

#[test]
fn min_touch_target_fails_for_tiny_button() {
    let id = NodeId::from(1u64);
    let node = make_node(Role::Button, "OK", Some((0.0, 0.0, 20.0, 20.0)));
    let nodes = BTreeMap::from([(id, node)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    assert!(MinTouchTargetConstraint.check(id, &ctx).is_err());
}

// ─── Spatial constraints ───────────────────────────────────────────────

#[test]
fn reflow320_passes_for_narrow_element() {
    let id = NodeId::from(1u64);
    let node = make_node(Role::Button, "OK", Some((0.0, 0.0, 300.0, 50.0)));
    let nodes = BTreeMap::from([(id, node)]);
    let ctx = make_ctx(&nodes, Viewport::new(320, 480));

    assert!(Reflow320.check(id, &ctx).is_ok());
}

#[test]
fn reflow320_fails_for_wide_element() {
    let id = NodeId::from(1u64);
    let node = make_node(Role::Button, "OK", Some((0.0, 0.0, 500.0, 50.0)));
    let nodes = BTreeMap::from([(id, node)]);
    let ctx = make_ctx(&nodes, Viewport::new(320, 480));

    assert!(Reflow320.check(id, &ctx).is_err());
}

#[test]
fn text_spacing_passes_for_non_overlapping_siblings() {
    let root_id = NodeId::from(0u64);
    let child1_id = NodeId::from(1u64);
    let child2_id = NodeId::from(2u64);

    let child1 = make_node(Role::Label, "Hello", Some((0.0, 0.0, 100.0, 20.0)));
    let child2 = make_node(Role::Label, "World", Some((0.0, 30.0, 100.0, 50.0)));

    let mut root = Node::new(Role::Group);
    root.set_children(vec![child1_id, child2_id]);

    let nodes = BTreeMap::from([(root_id, root), (child1_id, child1), (child2_id, child2)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    assert!(TextSpacing.check(root_id, &ctx).is_ok());
}

#[test]
fn text_spacing_fails_for_overlapping_siblings() {
    let root_id = NodeId::from(0u64);
    let child1_id = NodeId::from(1u64);
    let child2_id = NodeId::from(2u64);

    let child1 = make_node(Role::Label, "Hello", Some((0.0, 0.0, 100.0, 30.0)));
    let child2 = make_node(Role::Label, "World", Some((0.0, 20.0, 100.0, 50.0)));

    let mut root = Node::new(Role::Group);
    root.set_children(vec![child1_id, child2_id]);

    let nodes = BTreeMap::from([(root_id, root), (child1_id, child1), (child2_id, child2)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    assert!(TextSpacing.check(root_id, &ctx).is_err());
}

#[test]
fn resize_text_200_passes_for_small_element() {
    let id = NodeId::from(1u64);
    let node = make_node(Role::Button, "OK", Some((0.0, 0.0, 100.0, 50.0)));
    let nodes = BTreeMap::from([(id, node)]);
    // Element doubled: 200x100, fits in 1920x1080
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    assert!(ResizeText200.check(id, &ctx).is_ok());
}

#[test]
fn resize_text_200_fails_for_large_element() {
    let id = NodeId::from(1u64);
    // Element is 1000x600 — doubled to 2000x1200, exceeds 1920x1080
    let node = make_node(Role::Button, "OK", Some((0.0, 0.0, 1000.0, 600.0)));
    let nodes = BTreeMap::from([(id, node)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    assert!(ResizeText200.check(id, &ctx).is_err());
}

#[test]
fn grid_alignment_passes_for_aligned_element() {
    let id = NodeId::from(1u64);
    let node = make_node(Role::Button, "OK", Some((8.0, 16.0, 100.0, 50.0)));
    let nodes = BTreeMap::from([(id, node)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    assert!((GridAlignment { step: 8.0 }).check(id, &ctx).is_ok());
}

#[test]
fn grid_alignment_fails_for_misaligned_element() {
    let id = NodeId::from(1u64);
    let node = make_node(Role::Button, "OK", Some((5.0, 10.0, 100.0, 50.0)));
    let nodes = BTreeMap::from([(id, node)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    assert!((GridAlignment { step: 8.0 }).check(id, &ctx).is_err());
}

#[test]
fn min_spacing_passes_for_well_spaced_siblings() {
    let root_id = NodeId::from(0u64);
    let child1_id = NodeId::from(1u64);
    let child2_id = NodeId::from(2u64);

    let child1 = make_node(Role::Button, "A", Some((0.0, 0.0, 50.0, 30.0)));
    let child2 = make_node(Role::Button, "B", Some((0.0, 50.0, 50.0, 80.0)));

    let mut root = Node::new(Role::Group);
    root.set_children(vec![child1_id, child2_id]);

    let nodes = BTreeMap::from([(root_id, root), (child1_id, child1), (child2_id, child2)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    // 20px gap, min 8px → pass
    assert!((MinSpacing { min_gap: 8.0 }).check(root_id, &ctx).is_ok());
}

// ─── ConstraintSet builder tests ───────────────────────────────────────

#[test]
fn constraint_set_collects_violations() {
    let root_id = NodeId::from(0u64);
    let child_id = NodeId::from(1u64);

    // Unlabeled button, too small
    let child = make_node(Role::Button, "", Some((0.0, 0.0, 20.0, 20.0)));
    let mut root = Node::new(Role::Window);
    root.set_children(vec![child_id]);

    let nodes = BTreeMap::from([(root_id, root), (child_id, child)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    let constraint_set = ConstraintSetBuilder::default()
        .hard(HasLabelConstraint)
        .hard(MinTouchTargetConstraint)
        .build();

    let verification = constraint_set.verify(root_id, &ctx);

    // Child has no label + too small = 2 hard violations
    assert!(!verification.is_valid());
    assert!(verification.hard_violations.len() >= 2);
}

#[test]
fn constraint_set_advisory_does_not_fail() {
    let root_id = NodeId::from(0u64);
    let child_id = NodeId::from(1u64);

    // Misaligned but labeled and large enough
    let child = make_node(Role::Button, "OK", Some((5.0, 5.0, 105.0, 55.0)));
    let mut root = Node::new(Role::Window);
    root.set_children(vec![child_id]);

    let nodes = BTreeMap::from([(root_id, root), (child_id, child)]);
    let ctx = make_ctx(&nodes, Viewport::new(1920, 1080));

    let constraint_set = ConstraintSetBuilder::default()
        .hard(HasLabelConstraint)
        .advisory(GridAlignment { step: 8.0 })
        .build();

    let verification = constraint_set.verify(root_id, &ctx);

    // Hard constraints pass, advisory has warning
    assert!(verification.is_valid());
    assert!(verification.warning_count() >= 1);
}

// ─── ConstraintProfile tests ──────────────────────────────────────────

#[test]
fn constraint_profile_wcag_a_passes_valid_tree() {
    let update = create_valid_tree();
    let layout = Layout::from_update(update);
    let viewport = Viewport::new(1920, 1080);

    let result = layout.verify_with_profile(viewport, ConstraintProfile::WcagA);
    assert!(result.is_ok());
}

#[test]
fn constraint_profile_wcag_aa_passes_valid_tree() {
    let update = create_valid_tree();
    let layout = Layout::from_update(update);
    let viewport = Viewport::new(1920, 1080);

    let result = layout.verify_with_profile(viewport, ConstraintProfile::WcagAA);
    assert!(result.is_ok());
}

#[test]
fn constraint_profile_wcag_aaa_passes_large_button() {
    let update = create_large_button_tree();
    let layout = Layout::from_update(update);
    let viewport = Viewport::new(1920, 1080);

    let result = layout.verify_with_profile(viewport, ConstraintProfile::WcagAAA);
    assert!(result.is_ok());
}

#[test]
fn constraint_profile_wcag_aaa_fails_small_button() {
    let viewport = Viewport::new(1920, 1080);

    let update = create_small_button_tree();
    let layout = Layout::from_update(update);
    let result = layout.verify_with_profile(viewport, ConstraintProfile::WcagAAA);
    assert!(result.is_err());
}

#[test]
fn verify_custom_with_reflow() {
    let update = create_wide_element_tree();
    let layout = Layout::from_update(update);
    let viewport = Viewport::new(320, 480);

    let constraint_set = ConstraintSetBuilder::default().hard(Reflow320).build();

    let result = layout.verify_custom(viewport, &constraint_set);
    assert!(result.is_err());
}

// ─── Tree helpers ─────────────────────────────────────────────────────

fn create_valid_tree() -> TreeUpdate {
    let button_id = NodeId::from(1u64);
    let root_id = NodeId::from(0u64);

    let mut button = Node::new(Role::Button);
    button.set_label("Submit");
    button.set_bounds(accesskit::Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 100.0,
        y1: 50.0,
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

fn create_large_button_tree() -> TreeUpdate {
    let button_id = NodeId::from(1u64);
    let root_id = NodeId::from(0u64);

    let mut button = Node::new(Role::Button);
    button.set_label("Submit");
    button.set_bounds(accesskit::Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 200.0,
        y1: 100.0,
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

fn create_small_button_tree() -> TreeUpdate {
    let button_id = NodeId::from(1u64);
    let root_id = NodeId::from(0u64);

    let mut button = Node::new(Role::Button);
    button.set_label("X");
    button.set_bounds(accesskit::Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 30.0,
        y1: 30.0,
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

fn create_wide_element_tree() -> TreeUpdate {
    let elem_id = NodeId::from(1u64);
    let root_id = NodeId::from(0u64);

    let mut elem = Node::new(Role::Label);
    elem.set_label("Very wide content that does not reflow");
    elem.set_bounds(accesskit::Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 500.0,
        y1: 30.0,
    });

    let mut root = Node::new(Role::Window);
    root.set_children(vec![elem_id]);

    TreeUpdate {
        nodes: vec![(root_id, root), (elem_id, elem)],
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    }
}
