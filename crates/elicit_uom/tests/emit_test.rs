//! Tests for UomCodePlugin emit tools.

use elicit_uom::{UomCodePlugin, UomQuantityPlugin};
use elicitation::ElicitPlugin;

fn result_text(r: &rmcp::model::CallToolResult) -> String {
    r.content
        .iter()
        .filter_map(|c| c.as_text().map(|t| t.text.clone()))
        .collect()
}

#[tokio::test]
async fn test_emit_main_with_length() {
    let qty = UomQuantityPlugin::new();
    let code_plugin = UomCodePlugin::with_bus(qty.bus());

    let create_result = qty
        .invoke_tool(
            "uom_length__new",
            serde_json::json!({ "value": 100.0, "unit": "kilometer" }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&create_result)).unwrap();
    let id = v["id"].as_str().unwrap().to_string();

    let result = code_plugin
        .invoke_tool("uom_code__emit_main", serde_json::json!({ "ids": [id] }))
        .await
        .unwrap();
    let code = result_text(&result);
    assert!(code.contains("fn main()"), "Expected fn main() in: {code}");
    assert!(
        code.contains("Length") || code.contains("meter"),
        "Expected Length usage: {code}"
    );
}

#[tokio::test]
async fn test_emit_conversion_code() {
    let qty = UomQuantityPlugin::new();
    let code_plugin = UomCodePlugin::with_bus(qty.bus());

    let result = code_plugin
        .invoke_tool(
            "uom_code__emit_conversion",
            serde_json::json!({
                "registration": "length",
                "value": 1.0,
                "from_unit": "kilometer",
                "to_unit": "mile",
                "var_name": "dist"
            }),
        )
        .await
        .unwrap();
    let code = result_text(&result);
    assert!(code.contains("Length"), "Expected Length in: {code}");
    assert!(code.contains("kilometer"), "Expected kilometer in: {code}");
    assert!(code.contains("mile"), "Expected mile in: {code}");
}

#[tokio::test]
async fn test_emit_physics_formula_kinetic_energy() {
    let qty = UomQuantityPlugin::new();
    let code_plugin = UomCodePlugin::with_bus(qty.bus());

    let result = code_plugin
        .invoke_tool(
            "uom_code__emit_physics_formula",
            serde_json::json!({ "formula": "KineticEnergy" }),
        )
        .await
        .unwrap();
    let code = result_text(&result);
    assert!(code.contains("0.5") || code.contains("½"));
    assert!(code.contains("Mass") || code.contains("mass"));
    assert!(code.contains("Velocity") || code.contains("velocity"));
    assert!(code.contains("Energy") || code.contains("energy"));
}

#[tokio::test]
async fn test_emit_snippet() {
    let qty = UomQuantityPlugin::new();
    let code_plugin = UomCodePlugin::with_bus(qty.bus());

    let create_result = qty
        .invoke_tool(
            "uom_velocity__new",
            serde_json::json!({ "value": 10.0, "unit": "meter_per_second" }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&create_result)).unwrap();
    let id = v["id"].as_str().unwrap().to_string();

    let result = code_plugin
        .invoke_tool("uom_code__emit_snippet", serde_json::json!({ "ids": [id] }))
        .await
        .unwrap();
    let snippet = result_text(&result);
    assert!(!snippet.is_empty());
}

#[test]
fn test_code_plugin_list_tools() {
    let qty = UomQuantityPlugin::new();
    let plugin = UomCodePlugin::with_bus(qty.bus());
    let tools = plugin.list_tools();
    let names: Vec<String> = tools.iter().map(|t| t.name.to_string()).collect();
    assert!(names.contains(&"uom_code__emit_main".to_string()));
    assert!(names.contains(&"uom_code__catalog_physics_constants".to_string()));
    assert!(names.contains(&"uom_code__catalog_quantities".to_string()));
    assert_eq!(tools.len(), 15);
}
