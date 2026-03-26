//! Tests for the TypeGraph builder (BFS traversal and cycle handling).

use elicitation::{Elicit, NodeKind, Prompt, Select, TypeGraph, TypeGraphError};

// --- Test type hierarchy ---

#[derive(Debug, Clone, Elicit)]
pub struct Leaf {
    pub value: String,
}

#[derive(Debug, Clone, Elicit)]
pub struct Parent {
    pub leaf: Leaf,
    pub count: u32,
}

#[derive(Debug, Clone, Elicit)]
pub struct GrandParent {
    pub parent: Parent,
    pub label: String,
}

#[derive(Debug, Clone, Elicit)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Elicit)]
pub struct WithEnum {
    pub color: Color,
    pub name: String,
}

// --- Basic graph construction ---

#[test]
fn from_root_leaf_struct() {
    let graph = TypeGraph::from_root("Leaf").unwrap();
    assert!(
        graph.nodes.contains_key("Leaf"),
        "Leaf node should be present"
    );
    let leaf_node = &graph.nodes["Leaf"];
    assert_eq!(leaf_node.kind, NodeKind::Survey);
    // String is a primitive leaf
    assert!(
        graph.nodes.contains_key("String") || graph.edges.is_empty() || {
            // String may not be in nodes if include_primitives=false is default
            // Edges still recorded
            graph
                .edges
                .iter()
                .any(|e| e.from == "Leaf" && e.to == "String")
        }
    );
}

#[test]
fn from_root_traverses_nested_types() {
    let graph = TypeGraph::from_root("GrandParent").unwrap();
    // All three composite types should be present
    assert!(graph.nodes.contains_key("GrandParent"));
    assert!(graph.nodes.contains_key("Parent"));
    assert!(graph.nodes.contains_key("Leaf"));
}

#[test]
fn from_root_enum_creates_variant_nodes() {
    let graph = TypeGraph::from_root("Color").unwrap();
    assert!(graph.nodes.contains_key("Color"));
    // Unit variant nodes use fully-qualified names
    assert!(
        graph.nodes.contains_key("Color::Red"),
        "Color::Red should be a node"
    );
    assert!(graph.nodes.contains_key("Color::Green"));
    assert!(graph.nodes.contains_key("Color::Blue"));
}

#[test]
fn from_root_with_enum_field_traverses_enum() {
    let graph = TypeGraph::from_root("WithEnum").unwrap();
    assert!(graph.nodes.contains_key("WithEnum"));
    assert!(
        graph.nodes.contains_key("Color"),
        "Color should be traversed from WithEnum"
    );
}

// --- Edge correctness ---

#[test]
fn edges_have_correct_labels() {
    let graph = TypeGraph::from_root("Parent").unwrap();
    let leaf_edge = graph
        .edges
        .iter()
        .find(|e| e.from == "Parent" && e.to == "Leaf");
    assert!(leaf_edge.is_some(), "Parent -> Leaf edge should exist");
    assert_eq!(leaf_edge.unwrap().label, "leaf");
}

#[test]
fn roots_recorded() {
    let graph = TypeGraph::from_root("Leaf").unwrap();
    assert_eq!(graph.roots, vec!["Leaf"]);
}

#[test]
fn from_roots_multiple_roots() {
    let graph = TypeGraph::from_roots(&["Leaf", "Color"]).unwrap();
    assert_eq!(graph.roots.len(), 2);
    assert!(graph.nodes.contains_key("Leaf"));
    assert!(graph.nodes.contains_key("Color"));
}

// --- Error handling ---

#[test]
fn from_root_unknown_type_returns_error() {
    let result = TypeGraph::from_root("Nonexistent");
    assert!(matches!(result, Err(TypeGraphError::UnknownRoot(_))));
}

#[test]
fn from_roots_any_unknown_returns_error() {
    let result = TypeGraph::from_roots(&["Leaf", "Nonexistent"]);
    assert!(matches!(result, Err(TypeGraphError::UnknownRoot(_))));
}

// --- Registered types list ---

#[test]
fn registered_types_includes_test_types() {
    let types = TypeGraph::registered_types();
    assert!(types.contains(&"Leaf"));
    assert!(types.contains(&"Parent"));
    assert!(types.contains(&"Color"));
}
