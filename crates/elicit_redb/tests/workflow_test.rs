//! Integration tests for `elicit_redb` stateful plugin.

use elicit_redb::RedbPlugin;
use elicitation::StatefulPlugin;
use redb::ReadableDatabase as _;
use std::sync::Arc;
use tempfile::NamedTempFile;

// ── helpers ───────────────────────────────────────────────────────────────────

/// Create a temp file path that doesn't exist yet (redb needs the path, not an open file).
fn temp_db_path() -> (NamedTempFile, String) {
    let f = NamedTempFile::new().expect("tempfile");
    let path = f.path().to_str().expect("utf8 path").to_owned();
    (f, path)
}

fn ctx(plugin: &RedbPlugin) -> Arc<elicit_redb::RedbCtx> {
    plugin.ctx()
}

// ── tool listing tests ────────────────────────────────────────────────────────

#[test]
fn plugin_name_is_redb() {
    let plugin = RedbPlugin::new();
    assert_eq!(plugin.name(), "redb");
}

#[test]
fn plugin_lists_tools() {
    let plugin = RedbPlugin::new();
    let tools = plugin.list_tools();

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
fn plugin_lists_multimap_tools() {
    let plugin = RedbPlugin::new();
    let tools = plugin.list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    for expected in &[
        "multimap_u64_u64__insert",
        "multimap_u64_u64__get",
        "multimap_u64_u64__remove",
        "multimap_u64_u64__remove_all",
        "multimap_u64_u64__len",
        "multimap_u64_u64__iter",
        "multimap_str_str__insert",
        "multimap_str_str__get",
        "multimap_bytes_bytes__insert",
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
    // 10 db tools + 11 txn tools + 24 table tools + 24 multimap tools = 69 minimum
    assert!(count >= 69, "expected at least 69 tools, got {count}");
}

// ── functional tests ──────────────────────────────────────────────────────────

/// Creates a temp DB, inserts a (u64, u64) key, reads it back, verifies.
#[test]
fn u64_table_roundtrip() {
    let (_f, path) = temp_db_path();
    let plugin = RedbPlugin::new();
    let c = ctx(&plugin);

    // Create DB
    let db = redb::Database::create(&path).expect("create db");
    let db_id = uuid::Uuid::new_v4();
    c.lock_databases().unwrap().insert(db_id, db);

    // Begin write txn
    let write_txn = {
        let dbs = c.lock_databases().unwrap();
        let db = dbs.get(&db_id).unwrap();
        db.begin_write().expect("begin_write")
    };
    let txn_id = uuid::Uuid::new_v4();
    c.lock_write_txns().unwrap().insert(txn_id, write_txn);

    // Insert entry using the table definition directly (mirroring what the tool does)
    {
        let mut txns = c.lock_write_txns().unwrap();
        let txn = txns.get_mut(&txn_id).unwrap();
        let def = redb::TableDefinition::<u64, u64>::new("counters");
        let mut table = txn.open_table(def).expect("open_table");
        table.insert(42u64, 100u64).expect("insert");
    }

    // Commit
    let txn = c.lock_write_txns().unwrap().remove(&txn_id).unwrap();
    txn.commit().expect("commit");

    // Begin read txn
    let read_txn = {
        let dbs = c.lock_databases().unwrap();
        let db = dbs.get(&db_id).unwrap();
        db.begin_read().expect("begin_read")
    };
    let rtxn_id = uuid::Uuid::new_v4();
    c.lock_read_txns().unwrap().insert(rtxn_id, read_txn);

    // Read back
    let value = {
        let txns = c.lock_read_txns().unwrap();
        let txn = txns.get(&rtxn_id).unwrap();
        let def = redb::TableDefinition::<u64, u64>::new("counters");
        let table = txn.open_table(def).expect("open_table");
        table.get(42u64).expect("get").map(|g| g.value())
    };

    assert_eq!(value, Some(100u64));
}

/// Verifies that a multimap can hold multiple values per key.
#[test]
fn str_multimap_roundtrip() {
    let (_f, path) = temp_db_path();
    let plugin = RedbPlugin::new();
    let c = ctx(&plugin);

    let db = redb::Database::create(&path).expect("create db");
    let db_id = uuid::Uuid::new_v4();
    c.lock_databases().unwrap().insert(db_id, db);

    let write_txn = {
        let dbs = c.lock_databases().unwrap();
        dbs.get(&db_id).unwrap().begin_write().expect("begin_write")
    };
    let txn_id = uuid::Uuid::new_v4();
    c.lock_write_txns().unwrap().insert(txn_id, write_txn);

    // Insert three values under the same key
    {
        let mut txns = c.lock_write_txns().unwrap();
        let txn = txns.get_mut(&txn_id).unwrap();
        let def = redb::MultimapTableDefinition::<&str, &str>::new("tags");
        let mut table = txn.open_multimap_table(def).expect("open_multimap_table");
        table.insert("item:1", "tag:rust").expect("insert");
        table.insert("item:1", "tag:fast").expect("insert");
        table.insert("item:1", "tag:embedded").expect("insert");
    }

    let txn = c.lock_write_txns().unwrap().remove(&txn_id).unwrap();
    txn.commit().expect("commit");

    // Read back via read txn
    let read_txn = {
        let dbs = c.lock_databases().unwrap();
        dbs.get(&db_id).unwrap().begin_read().expect("begin_read")
    };
    let rtxn_id = uuid::Uuid::new_v4();
    c.lock_read_txns().unwrap().insert(rtxn_id, read_txn);

    let values: Vec<String> = {
        let txns = c.lock_read_txns().unwrap();
        let txn = txns.get(&rtxn_id).unwrap();
        let def = redb::MultimapTableDefinition::<&str, &str>::new("tags");
        let table = txn.open_multimap_table(def).expect("open_multimap_table");
        table
            .get("item:1")
            .expect("get")
            .map(|r| r.map(|g| g.value().to_owned()))
            .collect::<Result<Vec<_>, _>>()
            .expect("collect values")
    };

    assert_eq!(values.len(), 3);
    assert!(values.contains(&"tag:rust".to_owned()));
    assert!(values.contains(&"tag:fast".to_owned()));
    assert!(values.contains(&"tag:embedded".to_owned()));
}

/// Verifies compact and check_integrity succeed on a live database.
#[test]
fn database_maintenance_ops() {
    let (_f, path) = temp_db_path();
    // maintenance ops work on a raw redb::Database, no plugin context needed
    let mut db = redb::Database::create(&path).expect("create db");

    let ok = db.check_integrity().expect("check_integrity");
    assert!(ok, "fresh db should pass integrity check");

    let _ = db.compact().expect("compact");
    // compact returns true/false depending on whether pages were freed; both are valid
}
