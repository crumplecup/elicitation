//! Tests for the TypeSpecPlugin MCP tools (describe_type, explore_type).

use elicitation::{ElicitPlugin, PluginRegistry, TypeSpecPlugin};

// ── describe_type ─────────────────────────────────────────────────────────────

#[test]
fn plugin_lists_two_tools() {
    let plugin = TypeSpecPlugin::new();
    let tools = plugin.list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    assert!(
        names.contains(&"describe_type"),
        "should have describe_type"
    );
    assert!(names.contains(&"explore_type"), "should have explore_type");
}

#[test]
fn plugin_name_is_type_spec() {
    assert_eq!(TypeSpecPlugin::new().name(), "type_spec");
}

#[test]
fn describe_type_tool_has_type_name_param() {
    let plugin = TypeSpecPlugin::new();
    let tools = plugin.list_tools();
    let describe = tools.iter().find(|t| t.name == "describe_type").unwrap();
    let schema = describe.schema_as_json_value();
    let props = &schema["properties"]["type_name"];
    assert_eq!(props["type"], "string");
}

#[test]
fn explore_type_tool_has_two_params() {
    let plugin = TypeSpecPlugin::new();
    let tools = plugin.list_tools();
    let explore = tools.iter().find(|t| t.name == "explore_type").unwrap();
    let schema = explore.schema_as_json_value();
    assert!(schema["properties"]["type_name"].is_object());
    assert!(schema["properties"]["category"].is_object());
}

// ── Registration in PluginRegistry ────────────────────────────────────────────

#[test]
fn plugin_registers_in_registry() {
    let registry = PluginRegistry::new().register("type_spec", TypeSpecPlugin::new());
    drop(registry);
}
