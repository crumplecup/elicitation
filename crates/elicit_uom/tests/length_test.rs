//! Tests for length quantity creation, emission, and conversion.

use elicit_uom::{UomQuantityPlugin, convert_to_unit};

fn result_text(r: &rmcp::model::CallToolResult) -> String {
    r.content
        .iter()
        .filter_map(|c| c.as_text().map(|t| t.text.clone()))
        .collect()
}

#[tokio::test]
async fn test_length_new_returns_uuid() {
    let plugin = UomQuantityPlugin::new();
    let result = plugin
        .invoke_tool(
            "uom_length__new",
            serde_json::json!({ "value": 100.0, "unit": "kilometer" }),
        )
        .await
        .unwrap();
    let text = result_text(&result);
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(v["registration"], "length");
    assert!((v["si_value"].as_f64().unwrap() - 100_000.0).abs() < 1e-6);
}

#[tokio::test]
async fn test_length_emit() {
    let plugin = UomQuantityPlugin::new();
    let result = plugin
        .invoke_tool(
            "uom_length__new",
            serde_json::json!({ "value": 1.0, "unit": "meter" }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&result)).unwrap();
    let id = v["id"].as_str().unwrap().to_string();

    let result2 = plugin
        .invoke_tool("uom_length__emit", serde_json::json!({ "ids": [id] }))
        .await
        .unwrap();
    let snippet = result_text(&result2);
    assert!(snippet.contains("Length") || snippet.contains("length") || snippet.contains("let"));
}

#[tokio::test]
async fn test_qty_convert_km_to_mile() {
    let plugin = UomQuantityPlugin::new();
    let result = plugin
        .invoke_tool(
            "uom_length__new",
            serde_json::json!({ "value": 1.0, "unit": "kilometer" }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&result)).unwrap();
    let id = v["id"].as_str().unwrap().to_string();

    let result2 = plugin
        .invoke_tool(
            "uom_qty__convert",
            serde_json::json!({ "id": id, "to_unit": "mile" }),
        )
        .await
        .unwrap();
    let v2: serde_json::Value = serde_json::from_str(&result_text(&result2)).unwrap();
    // 1 km = 0.621371 miles
    let miles = v2["value"].as_f64().unwrap();
    assert!(
        (miles - 0.621_371).abs() < 1e-3,
        "Expected ~0.621371 miles, got {}",
        miles
    );
}

#[test]
fn test_convert_to_unit_direct() {
    // 1000 meters = 1 km
    let km = convert_to_unit("length", 1000.0, "kilometer").unwrap();
    assert!((km - 1.0).abs() < 1e-9);

    // 373.15 K in kelvin = 373.15
    let k = convert_to_unit("temperature", 373.15, "kelvin").unwrap();
    assert!((k - 373.15).abs() < 1e-6);
}

#[tokio::test]
async fn test_length_list_tools_includes_new() {
    use elicitation::ElicitPlugin;
    let plugin = UomQuantityPlugin::new();
    let tools = plugin.list_tools();
    let names: Vec<String> = tools.iter().map(|t| t.name.to_string()).collect();
    assert!(names.contains(&"uom_length__new".to_string()));
    assert!(names.contains(&"uom_length__emit".to_string()));
    assert!(names.contains(&"uom_qty__convert".to_string()));
    assert!(names.contains(&"uom_qty__add".to_string()));
    assert!(tools.len() >= 50);
}
