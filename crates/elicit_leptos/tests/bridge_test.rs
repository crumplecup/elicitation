//! Integration tests for [`LeptosAxumBridgePlugin`].

use elicit_axum::AxumRouterPlugin;
use elicit_leptos::{LeptosAxumBridgePlugin, LeptosAxumPlugin};
use serde_json::json;

// ── helpers ───────────────────────────────────────────────────────────────────

fn result_text(r: &rmcp::model::CallToolResult) -> String {
    r.content
        .iter()
        .filter_map(|c| c.as_text())
        .map(|t| t.text.as_ref())
        .collect::<Vec<_>>()
        .join("")
}

async fn new_leptos_config(plugin: &LeptosAxumPlugin, app: &str, mode: &str, addr: &str) -> String {
    let res = plugin
        .invoke_tool(
            "leptos_axum__new",
            json!({ "app_component": app, "mode": mode, "site_addr": addr }),
        )
        .await
        .unwrap();
    let text = result_text(&res);
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    v["config_id"].as_str().unwrap().to_string()
}

// ── from_config: basic ────────────────────────────────────────────────────────

#[tokio::test]
async fn from_config_returns_valid_uuid() {
    let leptos = LeptosAxumPlugin::new();
    let axum = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum);

    let leptos_id = new_leptos_config(&leptos, "App", "full_ssr", "0.0.0.0:3000").await;

    let res = bridge
        .invoke_tool("from_config", json!({ "config_id": leptos_id }))
        .await
        .unwrap();
    let text = result_text(&res);
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    let router_id = v["router_id"].as_str().expect("router_id must be present");
    assert!(
        router_id.parse::<uuid::Uuid>().is_ok(),
        "router_id must be a valid UUID: {router_id}"
    );
}

#[tokio::test]
async fn unknown_config_id_returns_error() {
    let leptos = LeptosAxumPlugin::new();
    let axum = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum);

    let res = bridge
        .invoke_tool(
            "from_config",
            json!({ "config_id": "00000000-0000-0000-0000-000000000000" }),
        )
        .await;
    assert!(res.is_err(), "unknown config_id should return Err");
}

// ── returned UUID composes with axum_router tools ─────────────────────────────

#[tokio::test]
async fn returned_uuid_composable_with_add_layer() {
    let leptos = LeptosAxumPlugin::new();
    let axum_plugin = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum_plugin);

    let leptos_id = new_leptos_config(&leptos, "App", "full_ssr", "0.0.0.0:3000").await;
    let res = bridge
        .invoke_tool("from_config", json!({ "config_id": leptos_id }))
        .await
        .unwrap();
    let text = result_text(&res);
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    let router_id = v["router_id"].as_str().unwrap().to_string();

    // The returned UUID must work directly with the axum router plugin
    let layer_res = axum_plugin
        .invoke_tool(
            "axum_router__add_layer",
            json!({ "router_id": router_id, "layer_expr": "tower_http::trace::TraceLayer::new_for_http()" }),
        )
        .await
        .unwrap();
    assert!(
        !layer_res.is_error.unwrap_or(false),
        "add_layer should succeed"
    );
}

#[tokio::test]
async fn returned_uuid_composable_with_emit() {
    let leptos = LeptosAxumPlugin::new();
    let axum_plugin = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum_plugin);

    let leptos_id = new_leptos_config(&leptos, "MyApp", "full_ssr", "0.0.0.0:8080").await;
    let res = bridge
        .invoke_tool("from_config", json!({ "config_id": leptos_id }))
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    let router_id = v["router_id"].as_str().unwrap().to_string();

    let emit_res = axum_plugin
        .invoke_tool("axum_router__emit", json!({ "router_id": router_id }))
        .await
        .unwrap();
    let code = result_text(&emit_res);
    assert!(
        code.contains("leptos_routes"),
        "emitted code must contain leptos_routes:\n{code}"
    );
    assert!(
        code.contains("LeptosOptions"),
        "emitted code must have LeptosOptions state:\n{code}"
    );
}

// ── display modes ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn bare_display_mode_no_shell_wrapper() {
    let leptos = LeptosAxumPlugin::new();
    let axum_plugin = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum_plugin);

    let leptos_id = new_leptos_config(&leptos, "App", "full_ssr", "0.0.0.0:3000").await;
    let res = bridge
        .invoke_tool(
            "from_config",
            json!({ "config_id": leptos_id, "display_mode": "bare" }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    let router_id = v["router_id"].as_str().unwrap().to_string();
    let code = result_text(
        &axum_plugin
            .invoke_tool("axum_router__emit", json!({ "router_id": router_id }))
            .await
            .unwrap(),
    );
    assert!(
        !code.contains("StandardShell") && !code.contains("DashboardShell"),
        "bare mode must not add shell wrapper:\n{code}"
    );
}

#[tokio::test]
async fn standard_display_mode_adds_shell_import() {
    let leptos = LeptosAxumPlugin::new();
    let axum_plugin = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum_plugin);

    let leptos_id = new_leptos_config(&leptos, "App", "full_ssr", "0.0.0.0:3000").await;
    let res = bridge
        .invoke_tool(
            "from_config",
            json!({ "config_id": leptos_id, "display_mode": "standard" }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    let router_id = v["router_id"].as_str().unwrap().to_string();
    let code = result_text(
        &axum_plugin
            .invoke_tool("axum_router__emit", json!({ "router_id": router_id }))
            .await
            .unwrap(),
    );
    assert!(
        code.contains("StandardShell"),
        "standard mode must reference StandardShell:\n{code}"
    );
}

#[tokio::test]
async fn dashboard_display_mode_adds_shell_import() {
    let leptos = LeptosAxumPlugin::new();
    let axum_plugin = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum_plugin);

    let leptos_id = new_leptos_config(&leptos, "App", "full_ssr", "0.0.0.0:3000").await;
    let res = bridge
        .invoke_tool(
            "from_config",
            json!({ "config_id": leptos_id, "display_mode": "dashboard" }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    let router_id = v["router_id"].as_str().unwrap().to_string();
    let code = result_text(
        &axum_plugin
            .invoke_tool("axum_router__emit", json!({ "router_id": router_id }))
            .await
            .unwrap(),
    );
    assert!(
        code.contains("DashboardShell"),
        "dashboard mode must reference DashboardShell:\n{code}"
    );
}

// ── serving modes ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn full_ssr_has_leptos_routes_and_state() {
    let leptos = LeptosAxumPlugin::new();
    let axum_plugin = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum_plugin);

    let leptos_id = new_leptos_config(&leptos, "App", "full_ssr", "0.0.0.0:3000").await;
    let res = bridge
        .invoke_tool(
            "from_config",
            json!({ "config_id": leptos_id, "display_mode": "bare" }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    let router_id = v["router_id"].as_str().unwrap().to_string();
    let code = result_text(
        &axum_plugin
            .invoke_tool("axum_router__emit", json!({ "router_id": router_id }))
            .await
            .unwrap(),
    );
    assert!(code.contains("LeptosOptions"), "full_ssr state:\n{code}");
    assert!(
        code.contains("leptos_routes"),
        "leptos_routes call:\n{code}"
    );
    assert!(
        code.contains("handle_server_fns"),
        "server-fn route:\n{code}"
    );
    assert!(
        code.contains("file_and_error_handler"),
        "static fallback:\n{code}"
    );
}

#[tokio::test]
async fn static_html_has_unit_state() {
    let leptos = LeptosAxumPlugin::new();
    let axum_plugin = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum_plugin);

    let leptos_id = new_leptos_config(&leptos, "App", "static_html", "0.0.0.0:3000").await;
    let res = bridge
        .invoke_tool(
            "from_config",
            json!({ "config_id": leptos_id, "display_mode": "bare" }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    let router_id = v["router_id"].as_str().unwrap().to_string();
    let code = result_text(
        &axum_plugin
            .invoke_tool("axum_router__emit", json!({ "router_id": router_id }))
            .await
            .unwrap(),
    );
    assert!(
        code.contains("Router<()>"),
        "static_html should use unit state:\n{code}"
    );
    assert!(
        !code.contains("leptos_routes"),
        "static_html must not use leptos_routes:\n{code}"
    );
}

#[tokio::test]
async fn wasm_shell_has_serve_dir_layer() {
    let leptos = LeptosAxumPlugin::new();
    let axum_plugin = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum_plugin);

    let leptos_id = new_leptos_config(&leptos, "App", "wasm_shell", "0.0.0.0:3000").await;
    let res = bridge
        .invoke_tool(
            "from_config",
            json!({ "config_id": leptos_id, "display_mode": "bare" }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    let router_id = v["router_id"].as_str().unwrap().to_string();
    let code = result_text(
        &axum_plugin
            .invoke_tool("axum_router__emit", json!({ "router_id": router_id }))
            .await
            .unwrap(),
    );
    assert!(
        code.contains("ServeDir"),
        "wasm_shell must serve /pkg via ServeDir:\n{code}"
    );
}

// ── list_tools ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn bridge_plugin_lists_one_tool() {
    use elicitation::ElicitPlugin;
    let leptos = LeptosAxumPlugin::new();
    let axum_plugin = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum_plugin);

    let tools = bridge.list_tools();
    assert_eq!(tools.len(), 1, "bridge plugin has exactly one tool");
    assert_eq!(tools[0].name.as_ref(), "leptos_axum_bridge__from_config");
}

// ── db slot tests ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn from_config_with_db_slot_emits_with_state() {
    let leptos = LeptosAxumPlugin::new();
    let axum_plugin = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum_plugin);

    let leptos_id = new_leptos_config(&leptos, "App", "full_ssr", "0.0.0.0:3000").await;
    let res = bridge
        .invoke_tool(
            "from_config",
            json!({
                "config_id": leptos_id,
                "db_pool_type": "sqlx::AnyPool",
                "db_var_name": "pool",
                "provide_leptos_context": false
            }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    let router_id = v["router_id"].as_str().unwrap().to_string();
    let code = result_text(
        &axum_plugin
            .invoke_tool("axum_router__emit", json!({ "router_id": router_id }))
            .await
            .unwrap(),
    );

    assert!(
        code.contains("Router<sqlx::AnyPool>"),
        "state type must be pool type:\n{code}"
    );
    assert!(
        code.contains(".with_state(pool)"),
        "must emit .with_state(pool):\n{code}"
    );
    assert!(
        !code.contains("leptos_routes_with_context"),
        "provide_leptos_context=false must not use _with_context:\n{code}"
    );
}

#[tokio::test]
async fn from_config_with_db_slot_and_leptos_context() {
    let leptos = LeptosAxumPlugin::new();
    let axum_plugin = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum_plugin);

    let leptos_id = new_leptos_config(&leptos, "App", "full_ssr", "0.0.0.0:3000").await;
    let res = bridge
        .invoke_tool(
            "from_config",
            json!({
                "config_id": leptos_id,
                "db_pool_type": "sqlx::AnyPool",
                "db_var_name": "pool",
                "provide_leptos_context": true
            }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    let router_id = v["router_id"].as_str().unwrap().to_string();
    let code = result_text(
        &axum_plugin
            .invoke_tool("axum_router__emit", json!({ "router_id": router_id }))
            .await
            .unwrap(),
    );

    assert!(
        code.contains("leptos_routes_with_context"),
        "provide_leptos_context=true must use _with_context:\n{code}"
    );
    assert!(
        code.contains("provide_context(pool.clone())"),
        "must inject provide_context:\n{code}"
    );
    assert!(
        code.contains(".with_state(pool)"),
        "must emit .with_state(pool):\n{code}"
    );
}

#[tokio::test]
async fn from_config_db_slot_requires_both_fields() {
    let leptos = LeptosAxumPlugin::new();
    let axum_plugin = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum_plugin);

    let leptos_id = new_leptos_config(&leptos, "App", "full_ssr", "0.0.0.0:3000").await;
    // Only pool_type, no var_name → should error
    let res = bridge
        .invoke_tool(
            "from_config",
            json!({ "config_id": leptos_id, "db_pool_type": "sqlx::AnyPool" }),
        )
        .await;
    assert!(res.is_err(), "partial db slot params must return an error");
}

#[tokio::test]
async fn axum_router_set_db_slot_tool() {
    let axum_plugin = AxumRouterPlugin::new();

    // Create a bare router
    let new_res = axum_plugin
        .invoke_tool("axum_router__new", json!({ "state_type": "()" }))
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&new_res)).unwrap();
    let router_id = v["router_id"].as_str().unwrap().to_string();

    // Set the db slot
    axum_plugin
        .invoke_tool(
            "axum_router__set_db_slot",
            json!({
                "router_id": router_id,
                "pool_type": "sqlx::AnyPool",
                "var_name": "pool",
                "provide_leptos_context": false
            }),
        )
        .await
        .unwrap();

    // Emit and verify
    let code = result_text(
        &axum_plugin
            .invoke_tool("axum_router__emit", json!({ "router_id": router_id }))
            .await
            .unwrap(),
    );
    assert!(
        code.contains("Router<sqlx::AnyPool>"),
        "db_slot overrides state_type in emit:\n{code}"
    );
    assert!(
        code.contains(".with_state(pool)"),
        "db_slot emits with_state:\n{code}"
    );
}

#[tokio::test]
async fn axum_router_set_custom_state_tool() {
    let axum_plugin = AxumRouterPlugin::new();

    let new_res = axum_plugin
        .invoke_tool("axum_router__new", json!({ "state_type": "()" }))
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&new_res)).unwrap();
    let router_id = v["router_id"].as_str().unwrap().to_string();

    axum_plugin
        .invoke_tool(
            "axum_router__set_custom_state",
            json!({
                "router_id": router_id,
                "state_type": "AppState",
                "state_expr": "AppState::new(pool, config)"
            }),
        )
        .await
        .unwrap();

    let code = result_text(
        &axum_plugin
            .invoke_tool("axum_router__emit", json!({ "router_id": router_id }))
            .await
            .unwrap(),
    );
    assert!(
        code.contains("Router<AppState>"),
        "custom state type in emit:\n{code}"
    );
    assert!(
        code.contains(".with_state(AppState::new(pool, config))"),
        "custom state expr in emit:\n{code}"
    );
}
