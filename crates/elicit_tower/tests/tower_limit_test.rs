//! Integration tests for `TowerLimitPlugin` and `TowerRetryPlugin`.

use elicit_tower::{
    TowerBackoffCreated, TowerBudgetCreated, TowerHttpLayerCreated, TowerLayerCreated,
    TowerLimitPlugin, TowerRateCreated, TowerRetryLayerCreated, TowerRetryPlugin,
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
