//! Integration tests for `elicit_surrealdb` workflow plugins.

use elicit_surrealdb::{
    AuthDatabase, AuthNamespace, AuthRecord, AuthRoot, Datetime, Duration, ExperimentalFeature,
    Geometry, Kind, Number, PlannerStrategy, RecordId, SurrealCapabilities, SurrealConfig,
    SurrealConnectionPlugin, SurrealConstructorsPlugin, SurrealCrudPlugin, SurrealSchemaPlugin,
    SurrealSelectPlugin, SurrealTransactionPlugin, Table, Value,
};
use elicitation::{ElicitComplete, ElicitPlugin};

#[track_caller]
fn assert_proofs<T: ElicitComplete>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn plugins_create_successfully() {
    assert_eq!(SurrealConstructorsPlugin.name(), "surreal_constructors");
    assert_eq!(SurrealSchemaPlugin.name(), "surreal_schema");
    assert_eq!(SurrealCrudPlugin.name(), "surreal_crud");
    assert_eq!(SurrealConnectionPlugin.name(), "surreal_connection");
    assert_eq!(SurrealSelectPlugin::new().name(), "surreal_select");
    assert_eq!(SurrealTransactionPlugin::new().name(), "surreal_txn");
}

#[test]
fn constructors_plugin_lists_expected_tools() {
    let names: Vec<String> = SurrealConstructorsPlugin
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "duration__new",
        "duration__from_secs",
        "datetime__now",
        "datetime__from_timestamp",
        "table__new",
        "record_id__from_table_key",
        "record_id__parse_simple",
        "number__from_int",
        "number__from_float",
        "number__from_decimal_str",
        "kind__either",
        "kind__option",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn schema_plugin_lists_expected_tools() {
    let names: Vec<String> = SurrealSchemaPlugin
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "define_namespace",
        "define_database",
        "define_table",
        "define_field",
        "define_index",
        "define_event",
        "define_function",
        "define_param",
        "define_analyzer",
        "define_access_jwt",
        "define_access_record",
        "define_user",
        "remove_table",
        "remove_field",
        "remove_index",
        "info_for_db",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn crud_plugin_lists_expected_tools() {
    let names: Vec<String> = SurrealCrudPlugin
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "select_raw",
        "create_raw",
        "insert_raw",
        "update_raw",
        "upsert_raw",
        "delete_raw",
        "merge_raw",
        "patch_raw",
        "relate_raw",
        "select_rust",
        "create_rust",
        "insert_rust",
        "update_rust",
        "delete_rust",
        "query_rust",
        "live_rust",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn connection_plugin_lists_expected_tools() {
    let names: Vec<String> = SurrealConnectionPlugin
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "ws_client",
        "http_client",
        "memory_client",
        "surrealkv_client",
        "signin_root",
        "signin_ns",
        "signin_db",
        "signin_record",
        "use_ns_db",
        "full_setup",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn select_plugin_lists_expected_tools() {
    let names: Vec<String> = SurrealSelectPlugin::new()
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "surreal_select__start",
        "surreal_select__set_projections",
        "surreal_select__set_from",
        "surreal_select__add_where",
        "surreal_select__add_fetch",
        "surreal_select__set_order_by",
        "surreal_select__set_group_by",
        "surreal_select__set_limit",
        "surreal_select__set_start",
        "surreal_select__set_split",
        "surreal_select__set_version",
        "surreal_select__inspect",
        "surreal_select__emit_surreal",
        "surreal_select__emit_rust",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn txn_plugin_lists_expected_tools() {
    let names: Vec<String> = SurrealTransactionPlugin::new()
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "surreal_txn__start",
        "surreal_txn__add_statement",
        "surreal_txn__inspect",
        "surreal_txn__emit_commit",
        "surreal_txn__emit_cancel",
        "surreal_txn__emit_rust",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn shadow_type_proofs_non_empty() {
    assert_proofs::<Value>("Value");
    assert_proofs::<Kind>("Kind");
    assert_proofs::<Number>("Number");
    assert_proofs::<Datetime>("Datetime");
    assert_proofs::<Duration>("Duration");
    assert_proofs::<Table>("Table");
    assert_proofs::<RecordId>("RecordId");
    assert_proofs::<Geometry>("Geometry");
    assert_proofs::<AuthRoot>("AuthRoot");
    assert_proofs::<AuthNamespace>("AuthNamespace");
    assert_proofs::<AuthDatabase>("AuthDatabase");
    assert_proofs::<AuthRecord>("AuthRecord");
    assert_proofs::<PlannerStrategy>("PlannerStrategy");
    assert_proofs::<ExperimentalFeature>("ExperimentalFeature");
    assert_proofs::<SurrealCapabilities>("SurrealCapabilities");
    assert_proofs::<SurrealConfig>("SurrealConfig");
}
