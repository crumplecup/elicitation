//! Integration tests for `elicit_redb` workflow plugins.

use elicit_redb::{
    RedbBackendPlugin, RedbDatabasePlugin, RedbMultimapPlugin, RedbSavepointPlugin,
    RedbTablePlugin, RedbTransactionPlugin, RedbTypesPlugin,
};
use elicitation::ElicitPlugin;

#[test]
fn plugins_create_successfully() {
    assert_eq!(RedbDatabasePlugin.name(), "redb_database");
    assert_eq!(RedbTablePlugin.name(), "redb_table");
    assert_eq!(RedbSavepointPlugin.name(), "redb_savepoint");
    assert_eq!(RedbTransactionPlugin::new().name(), "redb_txn");
    assert_eq!(RedbMultimapPlugin.name(), "redb_multimap");
    assert_eq!(RedbTypesPlugin.name(), "redb_types");
    assert_eq!(RedbBackendPlugin.name(), "redb_backend");
}

#[test]
fn database_plugin_lists_expected_tools() {
    let names: Vec<String> = RedbDatabasePlugin
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "redb_database__create",
        "redb_database__open",
        "redb_database__open_read_only",
        "redb_database__builder_new",
        "redb_database__begin_write",
        "redb_database__begin_read",
        "redb_database__compact",
        "redb_database__check_integrity",
        "redb_database__stats",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn table_plugin_lists_expected_tools() {
    let names: Vec<String> = RedbTablePlugin
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "redb_table__define",
        "redb_table__open_write",
        "redb_table__open_read",
        "redb_table__insert",
        "redb_table__get",
        "redb_table__remove",
        "redb_table__iter",
        "redb_table__range",
        "redb_table__pop",
        "redb_table__retain",
        "redb_table__len",
        "redb_table__rename",
        "redb_table__delete",
        "redb_table__list",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn savepoint_plugin_lists_expected_tools() {
    let names: Vec<String> = RedbSavepointPlugin
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "redb_savepoint__create_persistent",
        "redb_savepoint__create_ephemeral",
        "redb_savepoint__restore",
        "redb_savepoint__list",
        "redb_savepoint__delete",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn txn_plugin_lists_expected_tools() {
    let names: Vec<String> = RedbTransactionPlugin::new()
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "redb_txn__start",
        "redb_txn__add_op",
        "redb_txn__set_durability",
        "redb_txn__set_two_phase",
        "redb_txn__inspect",
        "redb_txn__emit",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn multimap_plugin_lists_expected_tools() {
    let names: Vec<String> = RedbMultimapPlugin
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "redb_multimap__define",
        "redb_multimap__open_write",
        "redb_multimap__open_read",
        "redb_multimap__insert",
        "redb_multimap__get",
        "redb_multimap__remove",
        "redb_multimap__remove_all",
        "redb_multimap__iter",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn types_plugin_lists_expected_tools() {
    let names: Vec<String> = RedbTypesPlugin
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "redb_types__impl_key",
        "redb_types__impl_value",
        "redb_types__impl_mut_in_place",
        "redb_types__derive_key_bincode",
        "redb_types__derive_value_json",
        "redb_types__type_name",
        "redb_types__fixed_width_key",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn backend_plugin_lists_expected_tools() {
    let names: Vec<String> = RedbBackendPlugin
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "redb_backend__impl_storage",
        "redb_backend__read_impl",
        "redb_backend__write_impl",
        "redb_backend__in_memory_struct",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}
