//! Integration tests for all `elicit_tower` plugins.

use elicit_tower::{
    TowerBackoffCreated, TowerBalanceCreated, TowerBalancePlugin, TowerBoxServiceCreated,
    TowerBudgetCreated, TowerBuilderPlugin, TowerHttpLayerCreated, TowerLayerCreated,
    TowerLimitPlugin, TowerLoadCreated, TowerRateCreated, TowerRetryLayerCreated, TowerRetryPlugin,
    TowerServiceBuilderCreated, TowerServiceBuilderDone, TowerServiceBuilderLayerAdded,
    TowerSteerCreated, TowerSteerPlugin, TowerUtilLayerCreated, TowerUtilPlugin,
};
use elicitation::{ElicitPlugin, VerifiedWorkflow};

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

// ── Plugin smoke tests ────────────────────────────────────────────────────────

#[test]
fn limit_plugin_creates_successfully() {
    let p = TowerLimitPlugin::new();
    assert_eq!(p.name(), "tower_limit");
}

#[test]
fn retry_plugin_creates_successfully() {
    let p = TowerRetryPlugin::new();
    assert_eq!(p.name(), "tower_retry");
}

// ── Tool registration ─────────────────────────────────────────────────────────

#[test]
fn limit_plugin_list_tools_non_empty() {
    let tools = TowerLimitPlugin::new().list_tools();
    assert!(
        !tools.is_empty(),
        "TowerLimitPlugin must expose at least one tool"
    );
}

#[test]
fn limit_plugin_list_tools_expected_names() {
    let tools = TowerLimitPlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    let expected = [
        "tower_limit__concurrency_limit_layer_new",
        "tower_limit__rate_limit_layer_new",
        "tower_limit__rate_new",
        "tower_limit__timeout_layer_new",
        "tower_limit__buffer_layer_new",
        "tower_limit__load_shed_layer_new",
        "tower_limit__spawn_ready_layer_new",
        "tower_limit__layer_describe",
    ];
    for name in &expected {
        assert!(names.contains(name), "missing tool: {name}");
    }
}

#[test]
fn retry_plugin_list_tools_expected_names() {
    let tools = TowerRetryPlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    let expected = [
        "tower_retry__backoff_new",
        "tower_retry__budget_new",
        "tower_retry__retry_layer_new",
        "tower_retry__filter_layer_new",
        "tower_retry__backoff_describe",
        "tower_retry__budget_describe",
    ];
    for name in &expected {
        assert!(names.contains(name), "missing tool: {name}");
    }
}

// ── Proposition proofs ────────────────────────────────────────────────────────

#[test]
fn tower_limit_propositions_non_empty() {
    assert_verified::<TowerLayerCreated>("TowerLayerCreated");
    assert_verified::<TowerRateCreated>("TowerRateCreated");
}

#[test]
fn tower_retry_propositions_non_empty() {
    assert_verified::<TowerBackoffCreated>("TowerBackoffCreated");
    assert_verified::<TowerBudgetCreated>("TowerBudgetCreated");
    assert_verified::<TowerRetryLayerCreated>("TowerRetryLayerCreated");
}

#[test]
fn tower_http_propositions_non_empty() {
    assert_verified::<TowerHttpLayerCreated>("TowerHttpLayerCreated");
}

// ── New plugin smoke tests ────────────────────────────────────────────────────

#[test]
fn util_plugin_creates_successfully() {
    let p = TowerUtilPlugin::new();
    assert_eq!(p.name(), "tower_util");
}

#[test]
fn builder_plugin_creates_successfully() {
    let p = TowerBuilderPlugin::new();
    assert_eq!(p.name(), "tower_builder");
}

#[test]
fn balance_plugin_creates_successfully() {
    let p = TowerBalancePlugin::new();
    assert_eq!(p.name(), "tower_balance");
}

#[test]
fn steer_plugin_creates_successfully() {
    let p = TowerSteerPlugin::new();
    assert_eq!(p.name(), "tower_steer");
}

#[test]
fn util_plugin_list_tools_expected_names() {
    let tools = TowerUtilPlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    for name in &[
        "tower_util__map_err_layer_new",
        "tower_util__map_request_layer_new",
        "tower_util__map_response_layer_new",
        "tower_util__map_result_layer_new",
        "tower_util__and_then_layer_new",
        "tower_util__then_layer_new",
        "tower_util__box_service_new",
        "tower_util__box_clone_service_new",
        "tower_util__layer_describe",
        "tower_util__box_service_describe",
    ] {
        assert!(names.contains(name), "missing tool: {name}");
    }
}

#[test]
fn builder_plugin_list_tools_expected_names() {
    let tools = TowerBuilderPlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    for name in &[
        "tower_builder__new",
        "tower_builder__add_layer",
        "tower_builder__build",
        "tower_builder__describe",
    ] {
        assert!(names.contains(name), "missing tool: {name}");
    }
}

#[test]
fn balance_plugin_list_tools_expected_names() {
    let tools = TowerBalancePlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    for name in &[
        "tower_balance__p2c_new",
        "tower_balance__peak_ewma_new",
        "tower_balance__pending_requests_new",
        "tower_balance__describe",
    ] {
        assert!(names.contains(name), "missing tool: {name}");
    }
}

#[test]
fn steer_plugin_list_tools_expected_names() {
    let tools = TowerSteerPlugin::new().list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    for name in &["tower_steer__new", "tower_steer__describe"] {
        assert!(names.contains(name), "missing tool: {name}");
    }
}

#[test]
fn tower_util_propositions_non_empty() {
    assert_verified::<TowerUtilLayerCreated>("TowerUtilLayerCreated");
    assert_verified::<TowerBoxServiceCreated>("TowerBoxServiceCreated");
}

#[test]
fn tower_builder_propositions_non_empty() {
    assert_verified::<TowerServiceBuilderCreated>("TowerServiceBuilderCreated");
    assert_verified::<TowerServiceBuilderLayerAdded>("TowerServiceBuilderLayerAdded");
    assert_verified::<TowerServiceBuilderDone>("TowerServiceBuilderDone");
}

#[test]
fn tower_balance_propositions_non_empty() {
    assert_verified::<TowerBalanceCreated>("TowerBalanceCreated");
    assert_verified::<TowerLoadCreated>("TowerLoadCreated");
}

#[test]
fn tower_steer_propositions_non_empty() {
    assert_verified::<TowerSteerCreated>("TowerSteerCreated");
}
