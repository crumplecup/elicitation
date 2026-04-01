//! Tests for terminal breakpoint verification.

use accesskit::{Node, NodeId, Rect, Role};
use elicit_ui::{
    BreakpointOutcome, BreakpointTier, ConstraintSetBuilder, MinReadableSize, TerminalBreakpoint,
    TerminalBreakpointSet, TerminalNoOverflow,
};
use std::collections::HashMap;

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

/// Build a simple tree: root window (full viewport) + labeled button.
fn make_simple_tree(cols: f64, rows: f64) -> (NodeId, HashMap<NodeId, Node>) {
    let root_id = NodeId::from(0u64);
    let button_id = NodeId::from(1u64);

    let root = make_node(
        Role::Window,
        Some(rect(0.0, 0.0, cols, rows)),
        vec![button_id],
    );
    let mut button = Node::new(Role::Button);
    button.set_label("OK");
    button.set_bounds(rect(2.0, 2.0, 12.0, 4.0));

    let mut nodes = HashMap::new();
    nodes.insert(root_id, root);
    nodes.insert(button_id, button);
    (root_id, nodes)
}

// ─── TerminalBreakpointSet construction ───

#[test]
fn standard_set_has_seven_breakpoints() {
    let set = TerminalBreakpointSet::standard();
    assert_eq!(set.breakpoints().len(), 7);
}

#[test]
fn standard_set_required_count() {
    let set = TerminalBreakpointSet::standard();
    assert_eq!(set.required().len(), 5, "VT100, small, medium, large, ultrawide");
}

#[test]
fn standard_set_advisory_count() {
    let set = TerminalBreakpointSet::standard();
    assert_eq!(set.advisory().len(), 1, "tiny only");
}

#[test]
fn breakpoint_display() {
    let bp = TerminalBreakpoint::new("VT100", 80, 24, BreakpointTier::Required);
    assert_eq!(format!("{bp}"), "VT100 (80×24, required)");
}

#[test]
fn breakpoint_to_viewport() {
    let bp = TerminalBreakpoint::new("test", 120, 40, BreakpointTier::Required);
    let vp = bp.to_viewport();
    assert_eq!(vp.width, 120);
    assert_eq!(vp.height, 40);
}

#[test]
fn custom_breakpoint_added() {
    let set = TerminalBreakpointSet::standard()
        .with_breakpoint(TerminalBreakpoint::new("custom", 256, 80, BreakpointTier::Advisory));
    assert_eq!(set.breakpoints().len(), 8);
}

// ─── verify_all: small tree that fits everywhere ───

#[test]
fn small_tree_passes_all_required() {
    let (root_id, nodes) = make_simple_tree(40.0, 12.0);
    let constraint_set = ConstraintSetBuilder::default()
        .hard(TerminalNoOverflow)
        .build();
    let set = TerminalBreakpointSet::standard();
    let report = set.verify_all(root_id, &nodes, &constraint_set);

    assert!(report.is_valid(), "40×12 tree fits all viewports");
    assert_eq!(report.count(BreakpointOutcome::Pass), 7);
    assert_eq!(report.failures().len(), 0);
}

// ─── verify_all: tree that overflows small viewports ───

#[test]
fn large_tree_fails_small_viewports() {
    // Tree is 75×20 — overflows micro (40×12) and tiny (60×20) but fits VT100+
    let (root_id, nodes) = make_simple_tree(75.0, 20.0);
    let constraint_set = ConstraintSetBuilder::default()
        .hard(TerminalNoOverflow)
        .build();
    let set = TerminalBreakpointSet::standard();
    let report = set.verify_all(root_id, &nodes, &constraint_set);

    // micro is expected-fail, tiny is advisory → both should not block validity
    assert!(report.is_valid(), "Failures only at expected-fail and advisory tiers");

    // micro (40×12): tree 90×25 overflows → expected-fail
    let micro = &report.results[0];
    assert_eq!(micro.outcome, BreakpointOutcome::ExpectedFailure);

    // tiny (60×20): tree 90×25 overflows → warning
    let tiny = &report.results[1];
    assert_eq!(tiny.outcome, BreakpointOutcome::Warning);

    // VT100+ should all pass
    for r in &report.results[2..] {
        assert_eq!(r.outcome, BreakpointOutcome::Pass, "{} should pass", r.breakpoint.name);
    }
}

#[test]
fn tree_too_large_for_vt100_fails() {
    // 100×30 tree overflows VT100 (80×24)
    let (root_id, nodes) = make_simple_tree(100.0, 30.0);
    let constraint_set = ConstraintSetBuilder::default()
        .hard(TerminalNoOverflow)
        .build();
    let set = TerminalBreakpointSet::standard();
    let report = set.verify_all(root_id, &nodes, &constraint_set);

    assert!(!report.is_valid(), "100×30 overflows VT100 (required)");
    assert!(report.failures().len() >= 1);

    let vt100 = &report.results[2];
    assert_eq!(vt100.breakpoint.name, "VT100");
    assert_eq!(vt100.outcome, BreakpointOutcome::Fail);
}

// ─── verify_all with min readable size ───

#[test]
fn min_readable_at_breakpoints() {
    let root_id = NodeId::from(0u64);
    let child_id = NodeId::from(1u64);

    // Root fills viewport, child pane is 8×2 (below 10×3 min)
    let root = make_node(Role::Window, Some(rect(0.0, 0.0, 80.0, 24.0)), vec![child_id]);
    let child = make_node(Role::Group, Some(rect(0.0, 0.0, 8.0, 2.0)), vec![]);

    let mut nodes = HashMap::new();
    nodes.insert(root_id, root);
    nodes.insert(child_id, child);

    let constraint_set = ConstraintSetBuilder::default()
        .hard(MinReadableSize::default())
        .build();
    let set = TerminalBreakpointSet::standard();
    let report = set.verify_all(root_id, &nodes, &constraint_set);

    // MinReadableSize is viewport-independent (checks absolute bounds)
    // so it fails at every breakpoint
    assert!(!report.is_valid());
    assert!(report.failures().len() >= 1);
}

// ─── BreakpointReport summary ───

#[test]
fn report_summary_includes_all_breakpoints() {
    let (root_id, nodes) = make_simple_tree(40.0, 12.0);
    let constraint_set = ConstraintSetBuilder::default()
        .hard(TerminalNoOverflow)
        .build();
    let set = TerminalBreakpointSet::standard();
    let report = set.verify_all(root_id, &nodes, &constraint_set);

    let summary = report.summary();
    assert!(summary.contains("Terminal Breakpoint Report"));
    assert!(summary.contains("VT100"));
    assert!(summary.contains("ultrawide"));
    assert!(summary.contains("PASS"));
}

#[test]
fn report_summary_shows_fail() {
    let (root_id, nodes) = make_simple_tree(100.0, 30.0);
    let constraint_set = ConstraintSetBuilder::default()
        .hard(TerminalNoOverflow)
        .build();
    let set = TerminalBreakpointSet::standard();
    let report = set.verify_all(root_id, &nodes, &constraint_set);

    let summary = report.summary();
    assert!(summary.contains("FAIL"));
}

// ─── Outcome display ───

#[test]
fn outcome_display() {
    assert_eq!(format!("{}", BreakpointOutcome::Pass), "✅ pass");
    assert_eq!(format!("{}", BreakpointOutcome::Fail), "❌ fail");
    assert_eq!(format!("{}", BreakpointOutcome::Warning), "⚠️ warning");
    assert_eq!(
        format!("{}", BreakpointOutcome::ExpectedFailure),
        "📝 expected-fail"
    );
}

// ─── Tier display ───

#[test]
fn tier_display() {
    assert_eq!(format!("{}", BreakpointTier::Required), "required");
    assert_eq!(format!("{}", BreakpointTier::Advisory), "advisory");
    assert_eq!(format!("{}", BreakpointTier::ExpectedFail), "expected-fail");
}
