//! Tests for `SqlxWorkflowPlugin` — verified workflow contract tools.
//!
//! Unit tests do not require a database.  Integration tests use
//! `sqlite::memory:` (no external DB) and are gated behind the `api` feature:
//!
//! ```bash
//! cargo test -p elicit_sqlx --features api -- workflow
//! ```

use elicit_sqlx::workflow::{WfConnectParams, WfPoolSqlParams};
use elicit_sqlx::{
    ConnectedAndExecuted, DbConnected, FullCommit, QueryExecuted, SqlxWorkflowPlugin,
    TransactionCommitted, TransactionOpen, connected_and_executed, full_commit,
};
use elicitation::contracts::{And, Established, both};
use elicitation::{ColumnValue, ElicitPlugin};

// ── Proposition combinators ───────────────────────────────────────────────────

#[test]
fn proposition_and_combinator_compiles() {
    let db: Established<DbConnected> = Established::assert();
    let qe: Established<QueryExecuted> = Established::assert();
    let _combined: Established<ConnectedAndExecuted> = connected_and_executed(db, qe);
}

#[test]
fn full_commit_combinator_compiles() {
    let db: Established<DbConnected> = Established::assert();
    let tx: Established<TransactionOpen> = Established::assert();
    let committed: Established<TransactionCommitted> = Established::assert();
    let _proof: Established<FullCommit> = full_commit(db, tx, committed);
}

#[test]
fn both_composes_nested_proofs() {
    let a: Established<DbConnected> = Established::assert();
    let b: Established<TransactionOpen> = Established::assert();
    let _ab: Established<And<DbConnected, TransactionOpen>> = both(a, b);
}

// ── Plugin structure ──────────────────────────────────────────────────────────

#[test]
fn plugin_name_is_sqlx_workflow() {
    let plugin = SqlxWorkflowPlugin::new();
    assert_eq!(plugin.name(), "sqlx_workflow");
}

#[test]
fn plugin_lists_thirteen_tools() {
    let plugin = SqlxWorkflowPlugin::new();
    let tools = plugin.list_tools();
    assert_eq!(tools.len(), 13, "expected 13 tools, got {}", tools.len());
}

#[test]
fn all_tools_have_sqlx_workflow_prefix() {
    let plugin = SqlxWorkflowPlugin::new();
    for tool in plugin.list_tools() {
        assert!(
            tool.name.starts_with("sqlx_workflow__"),
            "tool '{}' missing sqlx_workflow__ prefix",
            tool.name
        );
    }
}

#[test]
fn all_tools_have_non_empty_description() {
    let plugin = SqlxWorkflowPlugin::new();
    for tool in plugin.list_tools() {
        assert!(
            !tool.description.as_deref().unwrap_or("").is_empty(),
            "tool '{}' has empty description",
            tool.name
        );
    }
}

#[test]
fn connect_description_mentions_establishes_db_connected() {
    let plugin = SqlxWorkflowPlugin::new();
    let connect = plugin
        .list_tools()
        .into_iter()
        .find(|t| t.name == "sqlx_workflow__connect")
        .expect("connect tool not found");
    let desc = connect.description.unwrap_or_default();
    assert!(
        desc.contains("Establishes"),
        "should mention 'Establishes': {desc}"
    );
    assert!(
        desc.contains("DbConnected"),
        "should mention DbConnected: {desc}"
    );
}

#[test]
fn begin_description_mentions_transaction_open() {
    let plugin = SqlxWorkflowPlugin::new();
    let begin = plugin
        .list_tools()
        .into_iter()
        .find(|t| t.name == "sqlx_workflow__begin")
        .expect("begin tool not found");
    let desc = begin.description.unwrap_or_default();
    assert!(
        desc.contains("TransactionOpen"),
        "should mention TransactionOpen: {desc}"
    );
}

// ── Param struct deserialization ──────────────────────────────────────────────

#[test]
fn wf_connect_params_deserializes() {
    let json = r#"{"database_url":"sqlite::memory:"}"#;
    let p: WfConnectParams = serde_json::from_str(json).unwrap();
    assert_eq!(p.database_url, "sqlite::memory:");
}

#[test]
fn wf_pool_sql_params_args_defaults_to_empty() {
    use uuid::Uuid;
    let id = Uuid::new_v4();
    let json = format!(r#"{{"pool_id":"{id}","sql":"SELECT 1"}}"#);
    let p: WfPoolSqlParams = serde_json::from_str(&json).unwrap();
    assert!(p.args.is_empty(), "args should default to empty Vec");
}

#[test]
fn wf_pool_sql_params_accepts_args() {
    use uuid::Uuid;
    let id = Uuid::new_v4();
    let json = format!(r#"{{"pool_id":"{id}","sql":"SELECT ?","args":["hello",42]}}"#);
    let p: WfPoolSqlParams = serde_json::from_str(&json).unwrap();
    assert_eq!(p.args.len(), 2);
}

// ── Integration tests (sqlite::memory:) ──────────────────────────────────────

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn workflow_connect_establishes_db_connected() {
    let plugin = SqlxWorkflowPlugin::new();
    let (pool_id, _proof) = plugin
        .connect("sqlite::memory:")
        .await
        .expect("connect failed");
    // cleanup
    plugin.disconnect(pool_id).await.expect("disconnect failed");
}

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn workflow_execute_and_fetch_all() {
    let plugin = SqlxWorkflowPlugin::new();
    let (pool_id, _db) = plugin
        .connect_with("sqlite::memory:", Some(1))
        .await
        .unwrap();

    plugin
        .execute(
            pool_id,
            "CREATE TABLE wf_test (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
            &[],
        )
        .await
        .unwrap();

    let (result, _qe) = plugin
        .execute(
            pool_id,
            "INSERT INTO wf_test (id, name) VALUES (?, ?)",
            &[
                serde_json::Value::Number(1.into()),
                serde_json::Value::String("Alice".to_string()),
            ],
        )
        .await
        .unwrap();
    assert_eq!(result.rows_affected, 1);

    let (rows, _rf) = plugin
        .fetch_all_data(pool_id, "SELECT id, name FROM wf_test", &[])
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert!(matches!(rows[0].columns[0].value, ColumnValue::BigInt(1)));

    plugin.disconnect(pool_id).await.unwrap();
}

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn workflow_fetch_one_and_optional() {
    let plugin = SqlxWorkflowPlugin::new();
    let (pool_id, _db) = plugin
        .connect_with("sqlite::memory:", Some(1))
        .await
        .unwrap();

    plugin
        .execute(pool_id, "CREATE TABLE opt_test (val INTEGER)", &[])
        .await
        .unwrap();
    plugin
        .execute(pool_id, "INSERT INTO opt_test VALUES (99)", &[])
        .await
        .unwrap();

    // fetch_one succeeds
    let (row, _rf) = plugin
        .fetch_one_data(pool_id, "SELECT val FROM opt_test WHERE val = 99", &[])
        .await
        .unwrap();
    assert!(matches!(row.columns[0].value, ColumnValue::BigInt(99)));

    // fetch_optional returns Some
    let maybe = plugin
        .fetch_optional_data(pool_id, "SELECT val FROM opt_test WHERE val = 99", &[])
        .await
        .unwrap();
    assert!(maybe.is_some());

    // fetch_optional returns None for no-match
    let none = plugin
        .fetch_optional_data(pool_id, "SELECT val FROM opt_test WHERE val = 0", &[])
        .await
        .unwrap();
    assert!(none.is_none());

    plugin.disconnect(pool_id).await.unwrap();
}

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn workflow_begin_tx_execute_commit_chain() {
    let plugin = SqlxWorkflowPlugin::new();
    let (pool_id, _db_proof) = plugin
        .connect_with("sqlite::memory:", Some(1))
        .await
        .unwrap();

    plugin
        .execute(pool_id, "CREATE TABLE tx_test (val INTEGER)", &[])
        .await
        .unwrap();

    // begin → TransactionOpen
    let (tx_id, _tx_open) = plugin.begin(pool_id).await.unwrap();

    // tx_execute → QueryExecuted
    let (_, _qe) = plugin
        .tx_execute(tx_id, "INSERT INTO tx_test (val) VALUES (42)", &[])
        .await
        .unwrap();

    // commit → TransactionCommitted  (full proof chain)
    let _tx_committed = plugin.commit(tx_id).await.unwrap();

    // verify committed data visible outside transaction
    let (rows, _) = plugin
        .fetch_all_data(pool_id, "SELECT val FROM tx_test WHERE val = 42", &[])
        .await
        .unwrap();
    assert_eq!(rows.len(), 1, "committed row should be visible");

    plugin.disconnect(pool_id).await.unwrap();
}

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn workflow_rollback_undoes_changes() {
    let plugin = SqlxWorkflowPlugin::new();
    let (pool_id, _db) = plugin
        .connect_with("sqlite::memory:", Some(1))
        .await
        .unwrap();

    plugin
        .execute(pool_id, "CREATE TABLE rb_test (val INTEGER)", &[])
        .await
        .unwrap();

    let (tx_id, _tx_open) = plugin.begin(pool_id).await.unwrap();

    plugin
        .tx_execute(tx_id, "INSERT INTO rb_test (val) VALUES (99)", &[])
        .await
        .unwrap();

    // rollback → TransactionRolledBack
    let _rolled_back = plugin.rollback(tx_id).await.unwrap();

    // verify no rows visible after rollback
    let none = plugin
        .fetch_optional_data(pool_id, "SELECT val FROM rb_test WHERE val = 99", &[])
        .await
        .unwrap();
    assert!(none.is_none(), "rolled-back row should not be visible");

    plugin.disconnect(pool_id).await.unwrap();
}

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn workflow_tx_fetch_reads_uncommitted_within_tx() {
    let plugin = SqlxWorkflowPlugin::new();
    let (pool_id, _db) = plugin
        .connect_with("sqlite::memory:", Some(1))
        .await
        .unwrap();

    plugin
        .execute(pool_id, "CREATE TABLE vis_test (val INTEGER)", &[])
        .await
        .unwrap();

    let (tx_id, _tx_open) = plugin.begin(pool_id).await.unwrap();
    plugin
        .tx_execute(tx_id, "INSERT INTO vis_test (val) VALUES (7)", &[])
        .await
        .unwrap();

    // read within same transaction — should see the uncommitted insert
    let (rows, _rf) = plugin
        .tx_fetch_all_data(tx_id, "SELECT val FROM vis_test", &[])
        .await
        .unwrap();
    assert_eq!(
        rows.len(),
        1,
        "uncommitted insert should be visible within tx"
    );

    plugin.rollback(tx_id).await.unwrap();
    plugin.disconnect(pool_id).await.unwrap();
}
