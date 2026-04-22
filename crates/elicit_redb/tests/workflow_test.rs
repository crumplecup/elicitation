//! Integration tests for `elicit_redb` stateful plugin.

use elicit_redb::RedbPlugin;
use elicitation::StatefulPlugin;

#[test]
fn plugin_name_is_redb() {
    let plugin = RedbPlugin::new();
    assert_eq!(plugin.name(), "redb");
}

#[test]
fn plugin_lists_tools() {
    let plugin = RedbPlugin::new();
    let tools = plugin.list_tools();

    // Verify core DB tools are present
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    for expected in &[
        "database_create",
        "database_open",
        "database_open_read_only",
        "database_builder_create",
        "database_builder_open",
        "database_begin_write",
        "database_begin_read",
        "database_compact",
        "database_check_integrity",
        "database_close",
    ] {
        assert!(
            names.iter().any(|n| n == expected),
            "missing tool: {expected}"
        );
    }
}

#[test]
fn plugin_lists_transaction_tools() {
    let plugin = RedbPlugin::new();
    let tools = plugin.list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    for expected in &[
        "write_txn_commit",
        "write_txn_abort",
        "write_txn_set_durability",
        "write_txn_list_tables",
        "write_txn_delete_table",
        "write_txn_rename_table",
        "write_txn_savepoint_persistent",
        "write_txn_savepoint_ephemeral",
        "write_txn_savepoint_restore",
        "read_txn_list_tables",
        "read_txn_close",
    ] {
        assert!(
            names.iter().any(|n| n == expected),
            "missing tool: {expected}"
        );
    }
}

#[test]
fn plugin_lists_table_tools() {
    let plugin = RedbPlugin::new();
    let tools = plugin.list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    for expected in &[
        "table_u64_u64__insert",
        "table_u64_u64__get",
        "table_u64_u64__remove",
        "table_str_str__insert",
        "table_str_str__get",
        "table_bytes_bytes__insert",
    ] {
        assert!(
            names.iter().any(|n| n == expected),
            "missing tool: {expected}"
        );
    }
}

#[test]
fn plugin_tool_count_reasonable() {
    let plugin = RedbPlugin::new();
    let count = plugin.list_tools().len();
    // 10 db tools + 11 txn tools + 24 table tools = 45 minimum
    assert!(count >= 45, "expected at least 45 tools, got {count}");
}
