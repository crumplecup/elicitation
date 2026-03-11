//! Snapshot-style tests for Mermaid and DOT renderers.

use elicitation::{
    DotRenderer, Elicit, GraphRenderer, MermaidDirection, MermaidRenderer, Prompt, Select,
    TypeGraph,
};

// --- Simple stable type for snapshot tests ---

#[derive(Debug, Clone, Elicit)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Elicit)]
struct Location {
    label: String,
    direction: Direction,
}

// --- Mermaid renderer ---

#[test]
fn mermaid_output_starts_with_graph_td() {
    let graph = TypeGraph::from_root("Location").unwrap();
    let output = MermaidRenderer::new().render(&graph);
    assert!(
        output.starts_with("graph TD\n"),
        "Should start with 'graph TD\\n'"
    );
}

#[test]
fn mermaid_lr_direction() {
    let graph = TypeGraph::from_root("Location").unwrap();
    let renderer = MermaidRenderer {
        direction: MermaidDirection::LeftRight,
        ..Default::default()
    };
    let output = renderer.render(&graph);
    assert!(output.starts_with("graph LR\n"));
}

#[test]
fn mermaid_contains_composite_node() {
    let graph = TypeGraph::from_root("Location").unwrap();
    let output = MermaidRenderer::new().render(&graph);
    // Location is a Survey node and should appear
    assert!(output.contains("Location"), "Location node should appear");
    assert!(output.contains("survey"), "Survey label should appear");
}

#[test]
fn mermaid_contains_edge_to_registered_type() {
    let graph = TypeGraph::from_root("Location").unwrap();
    let output = MermaidRenderer::new().render(&graph);
    // There should be an edge from Location to Direction
    assert!(
        output.contains("Location__") || output.contains("-->"),
        "Should contain edges"
    );
    assert!(output.contains("Direction"), "Direction node should appear");
}

#[test]
fn mermaid_excludes_primitives_by_default() {
    let graph = TypeGraph::from_root("Location").unwrap();
    let output = MermaidRenderer::new().render(&graph);
    // 'String' is a primitive — should not appear as a declared node
    // (it's excluded from node declarations, not edges)
    // The edge to String should also be suppressed (include_primitives=false)
    // So 'String' should not appear at all in the output
    assert!(
        !output.contains("String["),
        "String primitive node should not be declared"
    );
}

#[test]
fn mermaid_includes_primitives_when_enabled() {
    let graph = TypeGraph::from_root("Location").unwrap();
    let renderer = MermaidRenderer {
        include_primitives: true,
        ..Default::default()
    };
    let output = renderer.render(&graph);
    // With include_primitives=true, String should appear
    assert!(
        output.contains("String"),
        "String should appear with include_primitives"
    );
}

#[test]
fn mermaid_sanitizes_colons_in_variant_nodes() {
    let graph = TypeGraph::from_root("Direction").unwrap();
    let renderer = MermaidRenderer {
        include_primitives: true,
        ..Default::default()
    };
    let output = renderer.render(&graph);
    // Mermaid node *identifiers* must not contain `::` — sanitised to `__`.
    // The sanitised id (e.g. `Direction__North`) should appear.
    assert!(
        output.contains("Direction__North"),
        "Direction::North id should be sanitised to Direction__North; output:\n{output}"
    );
    // No identifier part (before `[`) should contain raw `::`.
    let bad_id = output.lines().any(|line| {
        let trimmed = line.trim();
        // Node declaration lines start with the id before `[`
        if let Some(id) = trimmed.split('[').next() {
            id.contains("::")
        } else {
            false
        }
    });
    assert!(!bad_id, "No Mermaid node id should contain raw '::'");
}

// --- DOT renderer ---

#[test]
fn dot_output_starts_with_digraph() {
    let graph = TypeGraph::from_root("Location").unwrap();
    let output = DotRenderer::new().render(&graph);
    assert!(
        output.starts_with("digraph {"),
        "Should start with 'digraph {{'"
    );
    assert!(output.ends_with('}'), "Should end with '}}'");
}

#[test]
fn dot_contains_composite_node() {
    let graph = TypeGraph::from_root("Location").unwrap();
    let output = DotRenderer::new().render(&graph);
    assert!(
        output.contains("Location"),
        "Location should appear in DOT output"
    );
    assert!(
        output.contains("lightyellow") || output.contains("lightblue"),
        "Should use colored nodes"
    );
}

#[test]
fn dot_excludes_primitives_by_default() {
    let graph = TypeGraph::from_root("Location").unwrap();
    let output = DotRenderer::new().render(&graph);
    // String primitive should not appear as a node
    assert!(
        !output.contains("String [label"),
        "String primitive should not be declared"
    );
}

#[test]
fn dot_direction_enum_is_lightblue() {
    let graph = TypeGraph::from_root("Direction").unwrap();
    let output = DotRenderer::new().render(&graph);
    assert!(
        output.contains("lightblue"),
        "Select nodes should be lightblue"
    );
}
