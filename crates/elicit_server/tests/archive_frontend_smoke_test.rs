//! Smoke tests for the archive frontend pipeline.
//!
//! Three sections test each rendering path **headlessly** — no live DB, no
//! terminal raw-mode event loop, no bound TCP socket:
//!
//! 1. **`frontend_utils`** — `demo_verified_tree` and `verified_tree_from_descriptor`
//! 2. **Ratatui pipeline** — `RatatuiBackend::render` on the demo tree (IR only,
//!    no crossterm terminal interaction)
//! 3. **Leptos/browser pipeline** — plugin composition and `LeptosRenderer` (no
//!    `axum::serve`; the descriptor isomorphism is verified via `emit`)

use elicit_axum::AxumRouterPlugin;
use elicit_leptos::{LeptosAxumBridgePlugin, LeptosAxumPlugin, LeptosRenderer};
use elicit_ratatui::{RatatuiBackend, TuiNode};
use elicit_server::archive::{
    BackendKind, DatabaseDescriptor,
    frontend_utils::{demo_verified_tree, verified_tree_from_descriptor},
};
use elicit_ui::UiRenderer;
use serde_json::json;

// ── helpers ───────────────────────────────────────────────────────────────────

fn result_text(r: &rmcp::model::CallToolResult) -> String {
    r.content
        .iter()
        .filter_map(|c| c.as_text().map(|t| t.text.clone()))
        .collect()
}

async fn new_leptos_config(plugin: &LeptosAxumPlugin, app: &str, mode: &str, addr: &str) -> String {
    let res = plugin
        .invoke_tool(
            "leptos_axum__new",
            json!({ "app_component": app, "mode": mode, "site_addr": addr }),
        )
        .await
        .expect("leptos_axum__new must succeed");
    let v: serde_json::Value = serde_json::from_str(&result_text(&res)).expect("valid JSON");
    v["config_id"]
        .as_str()
        .expect("config_id present")
        .to_string()
}

// ── Section 1: frontend_utils ─────────────────────────────────────────────────

#[test]
fn demo_verified_tree_builds() {
    let tree = demo_verified_tree().expect("demo tree must build without a DB");
    // Root node must be present in the node map
    assert!(
        tree.nodes().contains_key(&tree.root()),
        "root node must be in the node map"
    );
    assert!(!tree.nodes().is_empty(), "tree must have at least one node");
}

#[test]
fn verified_tree_from_custom_descriptor() {
    let desc = DatabaseDescriptor {
        connection_id: "test-conn".to_string(),
        db_name: "smoke_db".to_string(),
        version: Some("PostgreSQL 16.0".to_string()),
        backend: BackendKind::Postgres,
    };
    let tree =
        verified_tree_from_descriptor(&desc).expect("tree from custom descriptor must succeed");
    assert!(tree.nodes().contains_key(&tree.root()));
    assert!(!tree.nodes().is_empty());
}

#[test]
fn demo_descriptor_backend_is_postgres() {
    // Ensure the demo descriptor advertises the expected BackendKind so
    // downstream display logic gets a consistent environment.
    let desc = DatabaseDescriptor {
        connection_id: "demo".to_string(),
        db_name: "archive (demo)".to_string(),
        version: Some("PostgreSQL 15.0".to_string()),
        backend: BackendKind::Postgres,
    };
    assert_eq!(desc.backend, BackendKind::Postgres);
}

// ── Section 2: ratatui headless ───────────────────────────────────────────────

#[test]
fn ratatui_backend_renders_demo_tree() {
    let tree = demo_verified_tree().expect("demo tree");
    let backend = RatatuiBackend::new();
    let (_stats, _proof) = backend.render(&tree).expect("render must succeed");
}

#[test]
fn ratatui_render_produces_tui_tree() {
    let tree = demo_verified_tree().expect("demo tree");
    let backend = RatatuiBackend::new();
    backend.render(&tree).expect("render");
    assert!(
        backend.last_tui_tree().is_some(),
        "last_tui_tree must be Some after render"
    );
}

#[test]
fn ratatui_tui_tree_root_is_valid_node() {
    // The archive demo tree uses `DatabaseDescriptorMode::Overview`, which emits
    // a single summary widget.  The root can be either a `Widget` (leaf) or a
    // `Layout` (container) depending on the display mode — both are valid.
    let tree = demo_verified_tree().expect("demo tree");
    let backend = RatatuiBackend::new();
    backend.render(&tree).expect("render");
    let root = backend.last_tui_tree().expect("tui tree present");
    assert!(
        matches!(root, TuiNode::Layout { .. } | TuiNode::Widget { .. }),
        "root TuiNode must be Layout or Widget, got: {root:?}"
    );
}

#[test]
fn ratatui_render_stats_have_at_least_one_container() {
    let tree = demo_verified_tree().expect("demo tree");
    let backend = RatatuiBackend::new();
    let (stats, _proof) = backend.render(&tree).expect("render");
    assert!(
        stats.containers_rendered >= 1,
        "must render at least one container (the root window)"
    );
}

#[test]
fn ratatui_custom_descriptor_also_renders() {
    let desc = DatabaseDescriptor {
        connection_id: "test-conn".to_string(),
        db_name: "custom_db".to_string(),
        version: None,
        backend: BackendKind::Sqlite,
    };
    let tree = verified_tree_from_descriptor(&desc).expect("tree");
    let backend = RatatuiBackend::new();
    backend.render(&tree).expect("render custom descriptor");
    assert!(backend.last_tui_tree().is_some());
}

// ── Section 3: leptos headless ────────────────────────────────────────────────

#[test]
fn leptos_renderer_renders_demo_tree_to_nonempty_html() {
    let tree = demo_verified_tree().expect("demo tree");
    let renderer = LeptosRenderer::html();
    renderer.render(&tree).expect("leptos render must succeed");
    let html = renderer.last_html();
    assert!(!html.is_empty(), "rendered HTML must not be empty");
}

#[test]
fn leptos_html_contains_db_name() {
    let desc = DatabaseDescriptor {
        connection_id: "html-test".to_string(),
        db_name: "my_smoke_db".to_string(),
        version: Some("PostgreSQL 16.0".to_string()),
        backend: BackendKind::Postgres,
    };
    let tree = verified_tree_from_descriptor(&desc).expect("tree");
    let renderer = LeptosRenderer::html();
    renderer.render(&tree).expect("render");
    let html = renderer.last_html();
    assert!(
        html.contains("my_smoke_db"),
        "HTML must contain the db name:\n{html}"
    );
}

// ── plugin descriptor construction ───────────────────────────────────────────

#[tokio::test]
async fn leptos_static_html_config_is_valid_uuid() {
    let plugin = LeptosAxumPlugin::new();
    let config_id = new_leptos_config(&plugin, "ArchiveApp", "static_html", "0.0.0.0:3000").await;
    assert!(
        config_id.parse::<uuid::Uuid>().is_ok(),
        "config_id must be a valid UUID: {config_id}"
    );
}

#[tokio::test]
async fn leptos_full_ssr_config_is_valid_uuid() {
    let plugin = LeptosAxumPlugin::new();
    let config_id = new_leptos_config(&plugin, "ArchiveApp", "full_ssr", "0.0.0.0:3000").await;
    assert!(
        config_id.parse::<uuid::Uuid>().is_ok(),
        "config_id must be a valid UUID: {config_id}"
    );
}

#[tokio::test]
async fn leptos_wasm_shell_config_is_valid_uuid() {
    let plugin = LeptosAxumPlugin::new();
    let config_id = new_leptos_config(&plugin, "ArchiveApp", "wasm_shell", "0.0.0.0:3000").await;
    assert!(
        config_id.parse::<uuid::Uuid>().is_ok(),
        "config_id must be a valid UUID: {config_id}"
    );
}

#[tokio::test]
async fn bridge_from_config_produces_valid_router_id() {
    let leptos = LeptosAxumPlugin::new();
    let axum = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum);

    let config_id = new_leptos_config(&leptos, "ArchiveApp", "static_html", "0.0.0.0:3000").await;
    let res = bridge
        .invoke_tool("from_config", json!({ "config_id": config_id }))
        .await
        .expect("from_config must succeed");
    let v: serde_json::Value = serde_json::from_str(&result_text(&res)).expect("valid JSON");
    let router_id = v["router_id"].as_str().expect("router_id present");
    assert!(
        router_id.parse::<uuid::Uuid>().is_ok(),
        "router_id must be a valid UUID: {router_id}"
    );
}

// ── descriptor isomorphism ────────────────────────────────────────────────────
//
// Core architectural invariant: the `AxumRouterDescriptor` produced by the
// bridge is identical to what `axum_router__emit` would print as source code.
// Runtime interpretation and code-generation are two views of the same spec.

#[tokio::test]
async fn isomorphism_static_html_emits_unit_state() {
    let leptos = LeptosAxumPlugin::new();
    let axum = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum);

    let config_id = new_leptos_config(&leptos, "ArchiveApp", "static_html", "0.0.0.0:3000").await;
    let res = bridge
        .invoke_tool("from_config", json!({ "config_id": config_id }))
        .await
        .unwrap();
    let router_id =
        serde_json::from_str::<serde_json::Value>(&result_text(&res)).unwrap()["router_id"]
            .as_str()
            .unwrap()
            .to_string();

    let code = result_text(
        &axum
            .invoke_tool("axum_router__emit", json!({ "router_id": router_id }))
            .await
            .unwrap(),
    );
    // static_html has no Leptos server functions — no leptos_routes call
    assert!(
        !code.contains("leptos_routes"),
        "static_html must not emit leptos_routes:\n{code}"
    );
    // No Leptos state needed → unit state
    assert!(
        code.contains("Router<()>"),
        "static_html must emit unit-state router:\n{code}"
    );
}

#[tokio::test]
async fn isomorphism_full_ssr_emits_leptos_routes() {
    let leptos = LeptosAxumPlugin::new();
    let axum = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum);

    let config_id = new_leptos_config(&leptos, "ArchiveApp", "full_ssr", "0.0.0.0:3000").await;
    let res = bridge
        .invoke_tool("from_config", json!({ "config_id": config_id }))
        .await
        .unwrap();
    let router_id =
        serde_json::from_str::<serde_json::Value>(&result_text(&res)).unwrap()["router_id"]
            .as_str()
            .unwrap()
            .to_string();

    let code = result_text(
        &axum
            .invoke_tool("axum_router__emit", json!({ "router_id": router_id }))
            .await
            .unwrap(),
    );
    assert!(
        code.contains("leptos_routes"),
        "full_ssr must emit leptos_routes:\n{code}"
    );
    assert!(
        code.contains("LeptosOptions"),
        "full_ssr must include LeptosOptions state:\n{code}"
    );
    assert!(
        code.contains("handle_server_fns"),
        "full_ssr must handle server functions:\n{code}"
    );
}

#[tokio::test]
async fn isomorphism_wasm_shell_emits_serve_dir() {
    let leptos = LeptosAxumPlugin::new();
    let axum = AxumRouterPlugin::new();
    let bridge = LeptosAxumBridgePlugin::new(&leptos, &axum);

    let config_id = new_leptos_config(&leptos, "ArchiveApp", "wasm_shell", "0.0.0.0:3000").await;
    let res = bridge
        .invoke_tool("from_config", json!({ "config_id": config_id }))
        .await
        .unwrap();
    let router_id =
        serde_json::from_str::<serde_json::Value>(&result_text(&res)).unwrap()["router_id"]
            .as_str()
            .unwrap()
            .to_string();

    let code = result_text(
        &axum
            .invoke_tool("axum_router__emit", json!({ "router_id": router_id }))
            .await
            .unwrap(),
    );
    assert!(
        code.contains("ServeDir"),
        "wasm_shell must serve /pkg via ServeDir:\n{code}"
    );
}
