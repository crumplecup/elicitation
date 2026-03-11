//! Tests for TypeGraphPlugin MCP tool definitions and schema.

use elicitation::plugin::ElicitPlugin;
use elicitation::{Elicit, Prompt, Select, TypeGraphPlugin};

// --- Test types ---

#[derive(Debug, Clone, Elicit)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Elicit)]
struct Card {
    suit: Suit,
    value: u8,
}

fn plugin() -> TypeGraphPlugin {
    TypeGraphPlugin::new()
}

// --- Plugin metadata ---

#[test]
fn plugin_name_is_type_graph() {
    assert_eq!(plugin().name(), "type_graph");
}

#[test]
fn plugin_lists_three_tools() {
    let tools = plugin().list_tools();
    assert_eq!(tools.len(), 3);
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    assert!(names.contains(&"list_types"));
    assert!(names.contains(&"graph_type"));
    assert!(names.contains(&"describe_edges"));
}

// --- Tool schema validation ---

#[test]
fn list_types_tool_has_empty_schema() {
    let tools = plugin().list_tools();
    let tool = tools.iter().find(|t| t.name == "list_types").unwrap();
    let schema = tool.schema_as_json_value();
    // No required params
    assert!(!schema["required"].is_array() || schema["required"].as_array().unwrap().is_empty());
}

#[test]
fn graph_type_tool_requires_root() {
    let tools = plugin().list_tools();
    let tool = tools.iter().find(|t| t.name == "graph_type").unwrap();
    let schema = tool.schema_as_json_value();
    assert!(
        schema["properties"]["root"].is_object(),
        "root param should exist"
    );
    assert!(
        schema["properties"]["format"].is_object(),
        "format param should exist"
    );
    assert!(
        schema["properties"]["include_primitives"].is_object(),
        "include_primitives should exist"
    );
    let required = schema["required"].as_array().unwrap();
    assert!(
        required.iter().any(|v| v == "root"),
        "root should be required"
    );
    assert!(
        !required.iter().any(|v| v == "format"),
        "format should be optional"
    );
}

#[test]
fn describe_edges_tool_requires_type_name() {
    let tools = plugin().list_tools();
    let tool = tools.iter().find(|t| t.name == "describe_edges").unwrap();
    let schema = tool.schema_as_json_value();
    assert!(schema["properties"]["type_name"].is_object());
    let required = schema["required"].as_array().unwrap();
    assert!(required.iter().any(|v| v == "type_name"));
}

// --- into_arc ---

#[test]
fn into_arc_produces_arc_plugin() {
    let arc = TypeGraphPlugin::new().into_arc();
    assert_eq!(arc.name(), "type_graph");
    assert_eq!(arc.list_tools().len(), 3);
}

// --- Registry types (verifying test types are registered) ---

#[test]
fn test_types_are_registered() {
    use elicitation::all_graphable_types;
    let types = all_graphable_types();
    assert!(
        types.contains(&"Card"),
        "Card should be registered via #[derive(Elicit)]"
    );
    assert!(
        types.contains(&"Suit"),
        "Suit should be registered via #[derive(Elicit)]"
    );
}

#[test]
fn type_graph_builds_from_card() {
    use elicitation::TypeGraph;
    let graph = TypeGraph::from_root("Card").unwrap();
    assert!(graph.nodes.contains_key("Card"));
    assert!(graph.nodes.contains_key("Suit"));
}
