//! Integration tests for all `elicit_axum` plugins.

use elicit_axum::{
    AxumExtractorAdded, AxumHandlerDefined, AxumHandlerPlugin, AxumResponseDefined,
    AxumResponsePlugin, AxumRouteAdded, AxumRouterCreated, AxumRouterPlugin, AxumServePlugin,
    AxumServerConfigured,
};
use elicitation::{ElicitPlugin, VerifiedWorkflow};

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

// ── Plugin smoke tests ────────────────────────────────────────────────────────

#[test]
fn router_plugin_creates_successfully() {
    let p = AxumRouterPlugin::new();
    assert_eq!(p.name(), "axum_router");
}

#[test]
fn handler_plugin_creates_successfully() {
    let p = AxumHandlerPlugin::new();
    assert_eq!(p.name(), "axum_handler");
}

#[test]
fn response_plugin_creates_successfully() {
    let p = AxumResponsePlugin::new();
    assert_eq!(p.name(), "axum_response");
}

#[test]
fn serve_plugin_creates_successfully() {
    let p = AxumServePlugin::new();
    assert_eq!(p.name(), "axum_serve");
}

// ── Tool registration ─────────────────────────────────────────────────────────

#[test]
fn router_plugin_list_tools_non_empty() {
    let tools = AxumRouterPlugin::new().list_tools();
    assert!(
        !tools.is_empty(),
        "AxumRouterPlugin must expose at least one tool"
    );
}

#[test]
fn router_plugin_list_tools_expected_names() {
    let tools = AxumRouterPlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    for name in &[
        "axum_router__new",
        "axum_router__add_route",
        "axum_router__add_layer",
        "axum_router__set_fallback",
        "axum_router__describe",
        "axum_router__emit",
    ] {
        assert!(names.contains(name), "missing tool: {name}");
    }
}

#[test]
fn handler_plugin_list_tools_expected_names() {
    let tools = AxumHandlerPlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    for name in &[
        "axum_handler__new",
        "axum_handler__add_extractor",
        "axum_handler__set_body",
        "axum_handler__describe",
        "axum_handler__emit",
    ] {
        assert!(names.contains(name), "missing tool: {name}");
    }
}

#[test]
fn response_plugin_list_tools_expected_names() {
    let tools = AxumResponsePlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    for name in &[
        "axum_response__json",
        "axum_response__html",
        "axum_response__redirect_permanent",
        "axum_response__redirect_temporary",
        "axum_response__no_content",
        "axum_response__status",
        "axum_response__describe",
        "axum_response__emit",
    ] {
        assert!(names.contains(name), "missing tool: {name}");
    }
}

#[test]
fn serve_plugin_list_tools_expected_names() {
    let tools = AxumServePlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    for name in &[
        "axum_serve__new",
        "axum_serve__with_graceful_shutdown",
        "axum_serve__describe",
        "axum_serve__emit_main",
    ] {
        assert!(names.contains(name), "missing tool: {name}");
    }
}

// ── Proposition proofs ────────────────────────────────────────────────────────

#[test]
fn axum_router_propositions_non_empty() {
    assert_verified::<AxumRouterCreated>("AxumRouterCreated");
    assert_verified::<AxumRouteAdded>("AxumRouteAdded");
}

#[test]
fn axum_handler_propositions_non_empty() {
    assert_verified::<AxumHandlerDefined>("AxumHandlerDefined");
    assert_verified::<AxumExtractorAdded>("AxumExtractorAdded");
}

#[test]
fn axum_response_propositions_non_empty() {
    assert_verified::<AxumResponseDefined>("AxumResponseDefined");
}

#[test]
fn axum_serve_propositions_non_empty() {
    assert_verified::<AxumServerConfigured>("AxumServerConfigured");
}
