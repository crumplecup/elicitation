//! Tests for cross-registration arithmetic via the quantity bus.

use elicit_uom::UomQuantityPlugin;

fn result_text(r: &rmcp::model::CallToolResult) -> String {
    r.content
        .iter()
        .filter_map(|c| c.as_text().map(|t| t.text.clone()))
        .collect()
}

async fn create_quantity(plugin: &UomQuantityPlugin, tool: &str, value: f64, unit: &str) -> String {
    let result = plugin
        .invoke_tool(tool, serde_json::json!({ "value": value, "unit": unit }))
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&result)).unwrap();
    v["id"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn test_div_length_by_time_gives_velocity() {
    let plugin = UomQuantityPlugin::new();
    let len_id = create_quantity(&plugin, "uom_length__new", 100.0, "meter").await;
    let time_id = create_quantity(&plugin, "uom_time__new", 10.0, "second").await;

    let result = plugin
        .invoke_tool(
            "uom_qty__div",
            serde_json::json!({ "lhs_id": len_id, "rhs_id": time_id }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&result)).unwrap();
    assert_eq!(v["registration"], "velocity");
    let si = v["si_value"].as_f64().unwrap();
    assert!((si - 10.0).abs() < 1e-9, "Expected 10 m/s, got {si}");
}

#[tokio::test]
async fn test_mul_length_by_length_gives_area() {
    let plugin = UomQuantityPlugin::new();
    let l1 = create_quantity(&plugin, "uom_length__new", 3.0, "meter").await;
    let l2 = create_quantity(&plugin, "uom_length__new", 4.0, "meter").await;

    let result = plugin
        .invoke_tool(
            "uom_qty__mul",
            serde_json::json!({ "lhs_id": l1, "rhs_id": l2 }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&result)).unwrap();
    assert_eq!(v["registration"], "area");
    let si = v["si_value"].as_f64().unwrap();
    assert!((si - 12.0).abs() < 1e-9, "Expected 12 m², got {si}");
}

#[tokio::test]
async fn test_add_same_registration() {
    let plugin = UomQuantityPlugin::new();
    let m1 = create_quantity(&plugin, "uom_mass__new", 2.0, "kilogram").await;
    let m2 = create_quantity(&plugin, "uom_mass__new", 3.0, "kilogram").await;

    let result = plugin
        .invoke_tool(
            "uom_qty__add",
            serde_json::json!({ "lhs_id": m1, "rhs_id": m2 }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&result)).unwrap();
    assert_eq!(v["registration"], "mass");
    let si = v["si_value"].as_f64().unwrap();
    assert!((si - 5.0).abs() < 1e-9, "Expected 5 kg, got {si}");
}

#[tokio::test]
async fn test_sqrt_area_gives_length() {
    let plugin = UomQuantityPlugin::new();
    let area_id = create_quantity(&plugin, "uom_area__new", 9.0, "square_meter").await;

    let result = plugin
        .invoke_tool("uom_qty__sqrt", serde_json::json!({ "id": area_id }))
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&result)).unwrap();
    assert_eq!(v["registration"], "length");
    let si = v["si_value"].as_f64().unwrap();
    assert!((si - 3.0).abs() < 1e-9, "Expected 3 m, got {si}");
}

#[tokio::test]
async fn test_scale_quantity() {
    let plugin = UomQuantityPlugin::new();
    let e_id = create_quantity(&plugin, "uom_energy__new", 100.0, "joule").await;

    let result = plugin
        .invoke_tool(
            "uom_qty__scale",
            serde_json::json!({ "id": e_id, "factor": 2.5 }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&result)).unwrap();
    assert_eq!(v["registration"], "energy");
    let si = v["si_value"].as_f64().unwrap();
    assert!((si - 250.0).abs() < 1e-9, "Expected 250 J, got {si}");
}
