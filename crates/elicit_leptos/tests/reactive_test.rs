//! Tests for LeptosReactivePlugin.

use elicit_leptos::LeptosReactivePlugin;
use elicitation::ElicitPlugin;

fn result_text(r: &rmcp::model::CallToolResult) -> String {
    r.content
        .iter()
        .filter_map(|c| c.as_text().map(|t| t.text.clone()))
        .collect()
}

#[tokio::test]
async fn test_signal_new_returns_uuid() {
    let plugin = LeptosReactivePlugin::new();
    let result = plugin
        .invoke_tool(
            "leptos_reactive__signal_new",
            serde_json::json!({"name": "count", "value": 0}),
        )
        .await
        .unwrap();
    let text = result_text(&result);
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert!(!v["id"].as_str().unwrap().is_empty());
    assert_eq!(v["name"], "count");
}

#[tokio::test]
async fn test_signal_set_and_get() {
    let plugin = LeptosReactivePlugin::new();
    let r = plugin
        .invoke_tool(
            "leptos_reactive__signal_new",
            serde_json::json!({"name": "x", "value": 10}),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&r)).unwrap();
    let id = v["id"].as_str().unwrap().to_string();

    plugin
        .invoke_tool(
            "leptos_reactive__signal_set",
            serde_json::json!({"id": id, "value": 42}),
        )
        .await
        .unwrap();

    let r2 = plugin
        .invoke_tool("leptos_reactive__signal_get", serde_json::json!({"id": id}))
        .await
        .unwrap();
    let v2: serde_json::Value = serde_json::from_str(&result_text(&r2)).unwrap();
    assert_eq!(v2["value"], 42);
}

#[tokio::test]
async fn test_memo_upper() {
    let plugin = LeptosReactivePlugin::new();
    let r = plugin
        .invoke_tool(
            "leptos_reactive__signal_new",
            serde_json::json!({"name": "greeting", "value": "hello"}),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&r)).unwrap();
    let sig_id = v["id"].as_str().unwrap().to_string();

    let r2 = plugin
        .invoke_tool(
            "leptos_reactive__memo_new",
            serde_json::json!({"source_id": sig_id, "op": "upper"}),
        )
        .await
        .unwrap();
    let v2: serde_json::Value = serde_json::from_str(&result_text(&r2)).unwrap();
    let memo_id = v2["id"].as_str().unwrap().to_string();

    let r3 = plugin
        .invoke_tool(
            "leptos_reactive__memo_get",
            serde_json::json!({"id": memo_id}),
        )
        .await
        .unwrap();
    let v3: serde_json::Value = serde_json::from_str(&result_text(&r3)).unwrap();
    assert_eq!(v3["value"], "HELLO");
}

#[tokio::test]
async fn test_signal_increment() {
    let plugin = LeptosReactivePlugin::new();
    let r = plugin
        .invoke_tool(
            "leptos_reactive__signal_new",
            serde_json::json!({"name": "n", "value": 5}),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&r)).unwrap();
    let id = v["id"].as_str().unwrap().to_string();

    plugin
        .invoke_tool(
            "leptos_reactive__signal_update",
            serde_json::json!({"id": id, "op": "increment"}),
        )
        .await
        .unwrap();

    let r2 = plugin
        .invoke_tool("leptos_reactive__signal_get", serde_json::json!({"id": id}))
        .await
        .unwrap();
    let v2: serde_json::Value = serde_json::from_str(&result_text(&r2)).unwrap();
    assert_eq!(v2["value"].as_f64().unwrap(), 6.0);
}

#[tokio::test]
async fn test_list_tools_count() {
    let plugin = LeptosReactivePlugin::new();
    let tools = plugin.list_tools();
    assert_eq!(
        tools.len(),
        22,
        "Expected 22 reactive tools, got {}",
        tools.len()
    );
}

#[tokio::test]
async fn test_owner_status() {
    let plugin = LeptosReactivePlugin::new();
    plugin
        .invoke_tool(
            "leptos_reactive__signal_new",
            serde_json::json!({"name": "a", "value": 1}),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "leptos_reactive__signal_new",
            serde_json::json!({"name": "b", "value": 2}),
        )
        .await
        .unwrap();
    let r = plugin
        .invoke_tool("leptos_reactive__owner_status", serde_json::json!({}))
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&r)).unwrap();
    assert_eq!(v["signals"].as_u64().unwrap(), 2);
}
