//! Tests for UomCodePlugin catalog tools.

use elicit_uom::{UomCodePlugin, UomQuantityPlugin};
use elicitation::ElicitPlugin;

fn result_text(r: &rmcp::model::CallToolResult) -> String {
    r.content
        .iter()
        .filter_map(|c| c.as_text().map(|t| t.text.clone()))
        .collect()
}

#[tokio::test]
async fn test_catalog_physics_constants() {
    let qty = UomQuantityPlugin::new();
    let plugin = UomCodePlugin::with_bus(qty.bus());

    let result = plugin
        .invoke_tool("uom_code__catalog_physics_constants", serde_json::json!({}))
        .await
        .unwrap();
    let text = result_text(&result);
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    let arr = v.as_array().unwrap();
    assert!(arr.len() >= 7);

    let c = arr.iter().find(|e| e["symbol"] == "c").unwrap();
    assert!((c["value"].as_f64().unwrap() - 299_792_458.0).abs() < 1.0);
}

#[tokio::test]
async fn test_catalog_quantities_all() {
    let qty = UomQuantityPlugin::new();
    let plugin = UomCodePlugin::with_bus(qty.bus());

    let result = plugin
        .invoke_tool(
            "uom_code__catalog_quantities",
            serde_json::json!({ "filter": "all" }),
        )
        .await
        .unwrap();
    let text = result_text(&result);
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    let arr = v.as_array().unwrap();
    assert_eq!(arr.len(), 18, "Expected 18 registrations");
}

#[tokio::test]
async fn test_catalog_quantities_base() {
    let qty = UomQuantityPlugin::new();
    let plugin = UomCodePlugin::with_bus(qty.bus());

    let result = plugin
        .invoke_tool(
            "uom_code__catalog_quantities",
            serde_json::json!({ "filter": "base" }),
        )
        .await
        .unwrap();
    let text = result_text(&result);
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    let arr = v.as_array().unwrap();
    assert_eq!(arr.len(), 7, "Expected 7 base quantities");
}

#[tokio::test]
async fn test_catalog_units_length() {
    let qty = UomQuantityPlugin::new();
    let plugin = UomCodePlugin::with_bus(qty.bus());

    let result = plugin
        .invoke_tool(
            "uom_code__catalog_units",
            serde_json::json!({ "registration": "length" }),
        )
        .await
        .unwrap();
    let text = result_text(&result);
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    let units = v["units"].as_array().unwrap();
    assert!(units.iter().any(|u| u == "meter"));
    assert!(units.iter().any(|u| u == "foot"));
    assert!(units.iter().any(|u| u == "kilometer"));
}

#[tokio::test]
async fn test_catalog_dimension_length() {
    let qty = UomQuantityPlugin::new();
    let plugin = UomCodePlugin::with_bus(qty.bus());

    let result = plugin
        .invoke_tool(
            "uom_code__catalog_dimension",
            serde_json::json!({ "registration": "length" }),
        )
        .await
        .unwrap();
    let text = result_text(&result);
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(v["dimension"], "L");
}

#[tokio::test]
async fn test_catalog_formula_list() {
    let qty = UomQuantityPlugin::new();
    let plugin = UomCodePlugin::with_bus(qty.bus());

    let result = plugin
        .invoke_tool("uom_code__catalog_formula_list", serde_json::json!({}))
        .await
        .unwrap();
    let text = result_text(&result);
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    let arr = v.as_array().unwrap();
    assert_eq!(arr.len(), 5);
    assert!(arr.iter().any(|f| f["name"] == "KineticEnergy"));
}

#[tokio::test]
async fn test_catalog_base_quantities() {
    let qty = UomQuantityPlugin::new();
    let plugin = UomCodePlugin::with_bus(qty.bus());

    let result = plugin
        .invoke_tool("uom_code__catalog_base_quantities", serde_json::json!({}))
        .await
        .unwrap();
    let text = result_text(&result);
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    let arr = v.as_array().unwrap();
    assert_eq!(arr.len(), 7);
    assert!(arr.iter().any(|e| e["registration"] == "length"));
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
