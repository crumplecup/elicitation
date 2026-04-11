//! Compile-time and runtime proofs that [`PolarsDbBackend`] satisfies `elicit_db` traits.

use elicit_db::{DbQueryExecutor, DbSchemaManager, DbTableManager, DbTransactor};
use elicit_polars::PolarsDbBackend;

fn _assert_send_sync<T: Send + Sync>() {}
fn _assert_query_executor<T: DbQueryExecutor>() {}
fn _assert_table_manager<T: DbTableManager>() {}
fn _assert_schema_manager<T: DbSchemaManager>() {}
fn _assert_transactor<T: DbTransactor>() {}

#[test]
fn polars_backend_satisfies_trait_bounds() {
    _assert_send_sync::<PolarsDbBackend>();
    _assert_query_executor::<PolarsDbBackend>();
    _assert_table_manager::<PolarsDbBackend>();
    _assert_schema_manager::<PolarsDbBackend>();
    _assert_transactor::<PolarsDbBackend>();
}

#[tokio::test(flavor = "multi_thread")]
async fn register_and_query_frame() {
    use polars::prelude::*;

    let backend = PolarsDbBackend::new();
    let df = df! {
        "id" => [1i64, 2, 3],
        "name" => ["alice", "bob", "carol"],
    }
    .unwrap();
    backend.register_frame("users", df).await;

    let (rows, _proof) = backend
        .query_rows("SELECT * FROM users", &[])
        .await
        .unwrap();
    assert_eq!(rows.rows.len(), 3);
}

#[tokio::test(flavor = "multi_thread")]
async fn rollback_restores_snapshot() {
    use elicit_db::IsolationLevel;
    use polars::prelude::*;

    let backend = PolarsDbBackend::new();
    let original = df! { "x" => [1i64, 2, 3] }.unwrap();
    backend.register_frame("t", original).await;

    let (handle, _open) = backend.begin(IsolationLevel::ReadCommitted).await.unwrap();

    // Overwrite the table after begin
    let modified = df! { "x" => [99i64] }.unwrap();
    backend.register_frame("t", modified).await;

    // Rollback should restore original three rows
    let _rolled_back = backend.rollback(handle).await.unwrap();
    let (rows, _) = backend.query_rows("SELECT * FROM t", &[]).await.unwrap();
    assert_eq!(rows.rows.len(), 3);
}

#[tokio::test(flavor = "multi_thread")]
async fn commit_discards_snapshot() {
    use elicit_db::IsolationLevel;
    use polars::prelude::*;

    let backend = PolarsDbBackend::new();
    let df = df! { "v" => [10i64] }.unwrap();
    backend.register_frame("vals", df).await;

    let (handle, _open) = backend.begin(IsolationLevel::ReadCommitted).await.unwrap();

    let new_df = df! { "v" => [20i64, 30i64] }.unwrap();
    backend.register_frame("vals", new_df).await;

    // Commit — snapshot discarded, new data persists
    let (_committed, _tx_proof, _durable) = backend.commit(handle).await.unwrap();
    let (rows, _) = backend.query_rows("SELECT * FROM vals", &[]).await.unwrap();
    assert_eq!(rows.rows.len(), 2);
}

#[tokio::test(flavor = "multi_thread")]
async fn create_and_list_tables() {
    use elicit_db::DbColumn;

    let backend = PolarsDbBackend::new();
    let cols = vec![
        DbColumn {
            name: "id".into(),
            ty: "BIGINT".into(),
            nullable: false,
            default_value: None,
            primary_key: true,
        },
        DbColumn {
            name: "label".into(),
            ty: "TEXT".into(),
            nullable: true,
            default_value: None,
            primary_key: false,
        },
    ];
    backend
        .create_table("default", "items", cols)
        .await
        .unwrap();

    let tables = backend.list_tables("default").await.unwrap();
    assert!(tables.iter().any(|t| t.name == "items"));
}

#[tokio::test(flavor = "multi_thread")]
async fn schema_manager_returns_default() {
    use elicit_db::DbSchemaManager;

    let backend = PolarsDbBackend::new();
    let schemas = backend.list_schemas().await.unwrap();
    assert_eq!(schemas, vec!["default"]);

    let info = backend.schema_info("any").await.unwrap();
    assert_eq!(info.name, "default");
}
