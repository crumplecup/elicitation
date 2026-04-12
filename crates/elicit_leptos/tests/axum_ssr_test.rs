//! Integration tests for `LeptosAxumPlugin`.

use elicit_leptos::{LeptosAxumPlugin, axum_ssr::LeptosAxumServerConfigured};
use elicitation::{ElicitPlugin, VerifiedWorkflow};

// ── Helpers ────────────────────────────────────────────────────────────────────

fn result_text(r: &rmcp::model::CallToolResult) -> String {
    r.content
        .iter()
        .filter_map(|c| c.as_text().map(|t| t.text.clone()))
        .collect()
}

async fn new_config(
    plugin: &LeptosAxumPlugin,
    app_component: &str,
    mode: &str,
    site_addr: &str,
) -> String {
    let res = plugin
        .invoke_tool(
            "leptos_axum__new",
            serde_json::json!({
                "app_component": app_component,
                "mode": mode,
                "site_addr": site_addr
            }),
        )
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    v["config_id"].as_str().unwrap().to_string()
}

// ── Plugin creation ────────────────────────────────────────────────────────────

#[test]
fn plugin_creates_successfully() {
    let p = LeptosAxumPlugin::new();
    assert_eq!(p.name(), "leptos_axum");
}

#[test]
fn plugin_lists_expected_tools() {
    let tools = LeptosAxumPlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    for expected in &[
        "leptos_axum__new",
        "leptos_axum__add_route",
        "leptos_axum__add_response_header",
        "leptos_axum__set_server_fn_route",
        "leptos_axum__set_static_handler",
        "leptos_axum__set_pkg_dir",
        "leptos_axum__describe",
        "leptos_axum__emit",
    ] {
        assert!(names.contains(expected), "missing tool: {expected}");
    }
}

#[test]
fn plugin_tools_are_non_empty() {
    let tools = LeptosAxumPlugin::new().list_tools();
    assert!(!tools.is_empty());
}

// ── VerifiedWorkflow ───────────────────────────────────────────────────────────

#[test]
fn leptos_axum_server_configured_has_proofs() {
    assert!(
        LeptosAxumServerConfigured::validate_proofs_non_empty(),
        "LeptosAxumServerConfigured must have at least one proof"
    );
}

// ── new tool ──────────────────────────────────────────────────────────────────

#[tokio::test]
async fn new_static_html_returns_config_id() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "App", "static_html", "0.0.0.0:3000").await;
    assert!(!id.is_empty());
    // Must be a valid UUID
    assert!(id.parse::<uuid::Uuid>().is_ok(), "not a UUID: {id}");
}

#[tokio::test]
async fn new_full_ssr_returns_config_id() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "App", "full_ssr", "127.0.0.1:8080").await;
    assert!(id.parse::<uuid::Uuid>().is_ok());
}

#[tokio::test]
async fn new_wasm_shell_returns_config_id() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "ArchiveApp", "wasm_shell", "0.0.0.0:4000").await;
    assert!(id.parse::<uuid::Uuid>().is_ok());
}

// ── describe tool ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn describe_static_html_has_expected_fields() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "App", "static_html", "0.0.0.0:3000").await;

    let res = plugin
        .invoke_tool(
            "leptos_axum__describe",
            serde_json::json!({ "config_id": id }),
        )
        .await
        .unwrap();
    let desc: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();

    assert_eq!(desc["app_component"], "App");
    assert_eq!(desc["mode"], "static_html");
    assert_eq!(desc["site_addr"], "0.0.0.0:3000");
    assert_eq!(desc["pkg_dir"], "pkg");
    assert_eq!(desc["static_file_handler"], false);
}

#[tokio::test]
async fn describe_full_ssr_defaults_static_handler_true() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "App", "full_ssr", "0.0.0.0:3000").await;

    let res = plugin
        .invoke_tool(
            "leptos_axum__describe",
            serde_json::json!({ "config_id": id }),
        )
        .await
        .unwrap();
    let desc: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();

    assert_eq!(desc["static_file_handler"], true);
}

// ── add_route tool ────────────────────────────────────────────────────────────

#[tokio::test]
async fn add_route_appears_in_descriptor() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "App", "static_html", "0.0.0.0:3000").await;

    plugin
        .invoke_tool(
            "leptos_axum__add_route",
            serde_json::json!({
                "config_id": id,
                "method": "get",
                "path": "/api/health",
                "handler": "health_handler"
            }),
        )
        .await
        .unwrap();

    let res = plugin
        .invoke_tool(
            "leptos_axum__describe",
            serde_json::json!({ "config_id": id }),
        )
        .await
        .unwrap();
    let desc: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    let routes = desc["custom_routes"].as_array().unwrap();
    assert_eq!(routes.len(), 1);
    assert_eq!(routes[0]["path"], "/api/health");
    assert_eq!(routes[0]["method"], "get");
    assert_eq!(routes[0]["handler"], "health_handler");
}

// ── add_response_header tool ──────────────────────────────────────────────────

#[tokio::test]
async fn add_response_header_appears_in_descriptor() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "App", "static_html", "0.0.0.0:3000").await;

    plugin
        .invoke_tool(
            "leptos_axum__add_response_header",
            serde_json::json!({
                "config_id": id,
                "name": "Cache-Control",
                "value": "no-store"
            }),
        )
        .await
        .unwrap();

    let res = plugin
        .invoke_tool(
            "leptos_axum__describe",
            serde_json::json!({ "config_id": id }),
        )
        .await
        .unwrap();
    let desc: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    let headers = desc["response_headers"].as_array().unwrap();
    assert_eq!(headers[0]["name"], "Cache-Control");
    assert_eq!(headers[0]["value"], "no-store");
}

// ── set_server_fn_route tool ──────────────────────────────────────────────────

#[tokio::test]
async fn set_server_fn_route_updates_descriptor() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "App", "wasm_shell", "0.0.0.0:3000").await;

    plugin
        .invoke_tool(
            "leptos_axum__set_server_fn_route",
            serde_json::json!({ "config_id": id, "prefix": "/api/v2/leptos" }),
        )
        .await
        .unwrap();

    let res = plugin
        .invoke_tool(
            "leptos_axum__describe",
            serde_json::json!({ "config_id": id }),
        )
        .await
        .unwrap();
    let desc: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    assert_eq!(desc["server_fn_route"], "/api/v2/leptos");
}

// ── set_pkg_dir tool ──────────────────────────────────────────────────────────

#[tokio::test]
async fn set_pkg_dir_updates_descriptor() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "App", "wasm_shell", "0.0.0.0:3000").await;

    plugin
        .invoke_tool(
            "leptos_axum__set_pkg_dir",
            serde_json::json!({ "config_id": id, "dir": "dist/pkg" }),
        )
        .await
        .unwrap();

    let res = plugin
        .invoke_tool(
            "leptos_axum__describe",
            serde_json::json!({ "config_id": id }),
        )
        .await
        .unwrap();
    let desc: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    assert_eq!(desc["pkg_dir"], "dist/pkg");
}

// ── set_static_handler tool ───────────────────────────────────────────────────

#[tokio::test]
async fn set_static_handler_disabled_for_static_html() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "App", "static_html", "0.0.0.0:3000").await;

    plugin
        .invoke_tool(
            "leptos_axum__set_static_handler",
            serde_json::json!({ "config_id": id, "enabled": false }),
        )
        .await
        .unwrap();

    let res = plugin
        .invoke_tool(
            "leptos_axum__describe",
            serde_json::json!({ "config_id": id }),
        )
        .await
        .unwrap();
    let desc: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
    assert_eq!(desc["static_file_handler"], false);
}

// ── emit tool — static_html ───────────────────────────────────────────────────

#[tokio::test]
async fn emit_static_html_contains_leptos_renderer() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "App", "static_html", "0.0.0.0:3000").await;

    let res = plugin
        .invoke_tool("leptos_axum__emit", serde_json::json!({ "config_id": id }))
        .await
        .unwrap();
    let code = result_text(&res);

    assert!(
        code.contains("LeptosRenderer"),
        "missing LeptosRenderer:\n{code}"
    );
    assert!(
        code.contains("axum::response::Html"),
        "missing Html:\n{code}"
    );
    assert!(code.contains("tokio::main"), "missing tokio::main:\n{code}");
    assert!(code.contains("0.0.0.0:3000"), "missing addr:\n{code}");
    assert!(
        !code.contains("leptos_axum"),
        "static_html must not use leptos_axum:\n{code}"
    );
}

#[tokio::test]
async fn emit_static_html_includes_custom_route() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "App", "static_html", "0.0.0.0:3000").await;

    plugin
        .invoke_tool(
            "leptos_axum__add_route",
            serde_json::json!({
                "config_id": id,
                "method": "get",
                "path": "/api/ping",
                "handler": "ping_handler"
            }),
        )
        .await
        .unwrap();

    let res = plugin
        .invoke_tool("leptos_axum__emit", serde_json::json!({ "config_id": id }))
        .await
        .unwrap();
    let code = result_text(&res);

    assert!(code.contains("/api/ping"), "missing custom route:\n{code}");
    assert!(code.contains("ping_handler"), "missing handler:\n{code}");
}

// ── emit tool — full_ssr ──────────────────────────────────────────────────────

#[tokio::test]
async fn emit_full_ssr_contains_generate_route_list() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "ArchiveApp", "full_ssr", "0.0.0.0:8080").await;

    let res = plugin
        .invoke_tool("leptos_axum__emit", serde_json::json!({ "config_id": id }))
        .await
        .unwrap();
    let code = result_text(&res);

    assert!(
        code.contains("generate_route_list"),
        "missing generate_route_list:\n{code}"
    );
    assert!(
        code.contains("LeptosRoutes"),
        "missing LeptosRoutes:\n{code}"
    );
    assert!(
        code.contains("ArchiveApp"),
        "missing app component:\n{code}"
    );
    assert!(
        code.contains("file_and_error_handler"),
        "missing fallback:\n{code}"
    );
    assert!(code.contains("0.0.0.0:8080"), "missing addr:\n{code}");
}

#[tokio::test]
async fn emit_full_ssr_without_static_handler_omits_fallback() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "App", "full_ssr", "0.0.0.0:3000").await;

    plugin
        .invoke_tool(
            "leptos_axum__set_static_handler",
            serde_json::json!({ "config_id": id, "enabled": false }),
        )
        .await
        .unwrap();

    let res = plugin
        .invoke_tool("leptos_axum__emit", serde_json::json!({ "config_id": id }))
        .await
        .unwrap();
    let code = result_text(&res);

    assert!(
        !code.contains("file_and_error_handler"),
        "fallback should be absent:\n{code}"
    );
}

// ── emit tool — wasm_shell ────────────────────────────────────────────────────

#[tokio::test]
async fn emit_wasm_shell_contains_handle_server_fns() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "App", "wasm_shell", "0.0.0.0:4000").await;

    let res = plugin
        .invoke_tool("leptos_axum__emit", serde_json::json!({ "config_id": id }))
        .await
        .unwrap();
    let code = result_text(&res);

    assert!(
        code.contains("handle_server_fns"),
        "missing handle_server_fns:\n{code}"
    );
    assert!(code.contains("ServeDir"), "missing ServeDir:\n{code}");
    assert!(
        code.contains("/api/leptos"),
        "missing default server_fn_route:\n{code}"
    );
    assert!(code.contains("0.0.0.0:4000"), "missing addr:\n{code}");
}

#[tokio::test]
async fn emit_wasm_shell_uses_custom_server_fn_route() {
    let plugin = LeptosAxumPlugin::new();
    let id = new_config(&plugin, "App", "wasm_shell", "0.0.0.0:4000").await;

    plugin
        .invoke_tool(
            "leptos_axum__set_server_fn_route",
            serde_json::json!({ "config_id": id, "prefix": "/leptos/fn" }),
        )
        .await
        .unwrap();

    let res = plugin
        .invoke_tool("leptos_axum__emit", serde_json::json!({ "config_id": id }))
        .await
        .unwrap();
    let code = result_text(&res);

    assert!(
        code.contains("/leptos/fn"),
        "custom prefix missing:\n{code}"
    );
}

// ── error cases ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn describe_invalid_id_returns_error() {
    let plugin = LeptosAxumPlugin::new();
    let res = plugin
        .invoke_tool(
            "leptos_axum__describe",
            serde_json::json!({ "config_id": "not-a-uuid" }),
        )
        .await;
    assert!(res.is_err(), "expected error for bad UUID");
}

#[tokio::test]
async fn describe_unknown_uuid_returns_error() {
    let plugin = LeptosAxumPlugin::new();
    let res = plugin
        .invoke_tool(
            "leptos_axum__describe",
            serde_json::json!({ "config_id": "00000000-0000-0000-0000-000000000000" }),
        )
        .await;
    assert!(res.is_err(), "expected error for unknown UUID");
}
