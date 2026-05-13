//! Tests for LeptosCodePlugin.

use elicit_leptos::LeptosCodePlugin;
use elicitation::ElicitPlugin;

fn result_text(r: &rmcp::model::CallToolResult) -> String {
    r.content
        .iter()
        .filter_map(|c| c.as_text().map(|t| t.text.clone()))
        .collect()
}

#[tokio::test]
async fn test_view_emit() {
    let plugin = LeptosCodePlugin::new();
    let r = plugin
        .invoke_tool(
            "leptos_code__view_emit",
            serde_json::json!({"content": "<p>\"Hello\"</p>"}),
        )
        .await
        .unwrap();
    let text = result_text(&r);
    assert!(text.contains("view!"));
}

#[tokio::test]
async fn test_component_new_and_emit() {
    let plugin = LeptosCodePlugin::new();
    let r = plugin
        .invoke_tool(
            "leptos_code__component_new",
            serde_json::json!({"name": "Counter"}),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&r)).unwrap();
    let id = v["id"].as_str().unwrap().to_string();

    plugin
        .invoke_tool(
            "leptos_code__component_set_body",
            serde_json::json!({"id": id, "body": "view! { <p>\"Count\"</p> }"}),
        )
        .await
        .unwrap();

    let r2 = plugin
        .invoke_tool("leptos_code__component_emit", serde_json::json!({"id": id}))
        .await
        .unwrap();
    let code = result_text(&r2);
    assert!(code.contains("Counter"));
    assert!(code.contains("#[component]"));
}

#[tokio::test]
async fn test_catalog_template_counter() {
    let plugin = LeptosCodePlugin::new();
    let r = plugin
        .invoke_tool(
            "leptos_code__catalog_template",
            serde_json::json!({"name": "counter"}),
        )
        .await
        .unwrap();
    let code = result_text(&r);
    assert!(code.contains("RwSignal") || code.contains("count"));
}

#[tokio::test]
async fn test_list_tools_count() {
    let plugin = LeptosCodePlugin::new();
    let tools = plugin.list_tools();
    assert!(
        tools.len() >= 40,
        "Expected at least 40 code tools, got {}",
        tools.len()
    );
}

#[tokio::test]
async fn test_show_emit() {
    let plugin = LeptosCodePlugin::new();
    let r = plugin
        .invoke_tool(
            "leptos_code__show_emit",
            serde_json::json!({
                "when_expr": "logged_in.get()",
                "children": "<p>\"Welcome\"</p>",
                "fallback": "<p>\"Please log in\"</p>"
            }),
        )
        .await
        .unwrap();
    let code = result_text(&r);
    assert!(code.contains("Show"));
    assert!(code.contains("logged_in.get()"));
}

#[tokio::test]
async fn test_router_link_emit() {
    let plugin = LeptosCodePlugin::new();
    let r = plugin
        .invoke_tool(
            "leptos_code__router_link_emit",
            serde_json::json!({
                "href": "/about",
                "children": "\"About\""
            }),
        )
        .await
        .unwrap();
    let code = result_text(&r);
    assert!(code.contains("<A"));
    assert!(code.contains("/about"));
}
