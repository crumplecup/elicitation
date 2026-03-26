//! Integration tests for `elicit_sqlx` runtime types.
//!
//! Requires a live PostgreSQL database. Set `DATABASE_URL` in `.env` or
//! environment before running. Ignored unless the `api` feature is enabled:
//!
//! ```bash
//! cargo test -p elicit_sqlx --features api
//! ```

use elicit_sqlx::{AnyColumn, AnyRow, AnyTypeInfo};
use elicitation::{ColumnValue, SqlTypeKind};
use sqlx::AnyPool;

/// Load DATABASE_URL from `.env` (workspace root) or environment.
fn database_url() -> String {
    let _ = dotenvy::from_path("../../.env");
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for runtime tests")
}

async fn connect() -> AnyPool {
    sqlx::any::install_default_drivers();
    AnyPool::connect(&database_url())
        .await
        .expect("failed to connect to test database")
}

// ── Connection ────────────────────────────────────────────────────────────────

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn test_connect_and_ping() {
    let pool = connect().await;
    let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, 1);
}

// ── DDL + DML ─────────────────────────────────────────────────────────────────

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn test_execute_create_and_drop() {
    let pool = connect().await;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS elicit_test_execute (
            id   SERIAL PRIMARY KEY,
            name TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("CREATE TABLE failed");

    let result = sqlx::query("INSERT INTO elicit_test_execute (name) VALUES ('hello')")
        .execute(&pool)
        .await
        .expect("INSERT failed");

    assert_eq!(result.rows_affected(), 1);

    sqlx::query("DROP TABLE elicit_test_execute")
        .execute(&pool)
        .await
        .expect("DROP TABLE failed");
}

// ── AnyRow + to_row_data ──────────────────────────────────────────────────────

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn test_any_row_to_row_data_scalar() {
    let pool = connect().await;

    let raw = sqlx::query("SELECT 42::BIGINT AS answer")
        .fetch_one(&pool)
        .await
        .expect("fetch failed");

    let row = AnyRow::from(raw);
    let data = row.to_row_data();

    assert_eq!(data.columns.len(), 1);
    let entry = &data.columns[0];
    assert_eq!(entry.name, "answer");
    assert!(
        matches!(entry.value, ColumnValue::BigInt(42)),
        "expected BigInt(42), got {:?}",
        entry.value
    );
}

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn test_any_row_to_row_data_multi_type() {
    let pool = connect().await;

    let raw = sqlx::query(
        "SELECT
            true::BOOLEAN           AS b,
            1::SMALLINT             AS s,
            2::INTEGER              AS i,
            3::BIGINT               AS g,
            1.5::REAL               AS r,
            2.5::DOUBLE PRECISION   AS d,
            'hello'::TEXT           AS t",
    )
    .fetch_one(&pool)
    .await
    .expect("fetch failed");

    let row = AnyRow::from(raw);
    let data = row.to_row_data();

    assert_eq!(data.columns.len(), 7);

    let by_name = |name: &str| -> ColumnValue {
        data.columns
            .iter()
            .find(|e| e.name == name)
            .map(|e| e.value.clone())
            .unwrap_or_else(|| panic!("column {name} not found"))
    };

    assert!(matches!(by_name("b"), ColumnValue::Bool(true)));
    assert!(matches!(by_name("s"), ColumnValue::SmallInt(1)));
    assert!(matches!(by_name("i"), ColumnValue::Integer(2)));
    assert!(matches!(by_name("g"), ColumnValue::BigInt(3)));
    assert!(matches!(by_name("t"), ColumnValue::Text(ref s) if s == "hello"));

    if let ColumnValue::Real(f) = by_name("r") {
        assert!((f - 1.5).abs() < 0.001, "expected ~1.5, got {f}");
    } else {
        panic!("expected Real for column r, got {:?}", by_name("r"));
    }
    if let ColumnValue::Double(d) = by_name("d") {
        assert!((d - 2.5).abs() < 0.001, "expected ~2.5, got {d}");
    } else {
        panic!("expected Double for column d, got {:?}", by_name("d"));
    }
}

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn test_any_row_null_column() {
    let pool = connect().await;

    let raw = sqlx::query("SELECT NULL::TEXT AS nullable")
        .fetch_one(&pool)
        .await
        .expect("fetch failed");

    let row = AnyRow::from(raw);
    let data = row.to_row_data();
    let entry = &data.columns[0];
    assert_eq!(entry.name, "nullable");
    assert!(matches!(entry.value, ColumnValue::Null));
}

// ── AnyRow column metadata ────────────────────────────────────────────────────

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn test_any_row_len_and_column_names() {
    let pool = connect().await;

    let raw = sqlx::query("SELECT 1::INT AS a, 2::INT AS b, 3::INT AS c")
        .fetch_one(&pool)
        .await
        .expect("fetch failed");

    let row = AnyRow::from(raw);
    assert_eq!(row.len(), 3);
    assert!(!row.is_empty());

    let names = row.column_names();
    assert_eq!(names, vec!["a", "b", "c"]);
}

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn test_any_row_columns_as_descriptors() {
    let pool = connect().await;

    let raw = sqlx::query("SELECT 42::BIGINT AS id, 'name'::TEXT AS label")
        .fetch_one(&pool)
        .await
        .expect("fetch failed");

    let row = AnyRow::from(raw);
    let descs = row.columns_as_descriptors();

    assert_eq!(descs.len(), 2);
    assert_eq!(descs[0].name, "id");
    assert_eq!(descs[0].type_kind, SqlTypeKind::BigInt);
    assert_eq!(descs[1].name, "label");
    assert_eq!(descs[1].type_kind, SqlTypeKind::Text);
}

// ── AnyColumn wrapper ─────────────────────────────────────────────────────────

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn test_any_column_wrapper_methods() {
    let pool = connect().await;

    let raw = sqlx::query("SELECT 1::BIGINT AS first_col, 'x'::TEXT AS second_col")
        .fetch_one(&pool)
        .await
        .expect("fetch failed");

    let row = AnyRow::from(raw);
    let cols: Vec<AnyColumn> = row.columns();

    assert_eq!(cols.len(), 2);

    assert_eq!(cols[0].ordinal(), 0);
    assert_eq!(cols[0].name(), "first_col");
    assert_eq!(cols[0].type_kind(), SqlTypeKind::BigInt);
    assert_eq!(cols[0].type_name(), "BIGINT");

    assert_eq!(cols[1].ordinal(), 1);
    assert_eq!(cols[1].name(), "second_col");
    assert_eq!(cols[1].type_kind(), SqlTypeKind::Text);
    assert_eq!(cols[1].type_name(), "TEXT");
}

// ── AnyTypeInfo wrapper ───────────────────────────────────────────────────────

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn test_any_type_info_wrapper() {
    let pool = connect().await;

    let raw = sqlx::query("SELECT true::BOOLEAN AS flag")
        .fetch_one(&pool)
        .await
        .expect("fetch failed");

    let row = AnyRow::from(raw);
    let cols = row.columns();

    let type_info = AnyTypeInfo::from(cols[0].0.type_info.clone());
    assert_eq!(type_info.kind(), SqlTypeKind::Bool);
    assert!(!type_info.is_null());
}

// ── fetch_optional ────────────────────────────────────────────────────────────

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn test_fetch_optional_returns_none_on_no_match() {
    let pool = connect().await;

    let maybe = sqlx::query("SELECT 1::INT AS n WHERE false")
        .fetch_optional(&pool)
        .await
        .expect("fetch_optional failed");

    assert!(maybe.is_none());
}

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn test_fetch_optional_returns_some_on_match() {
    let pool = connect().await;

    let maybe = sqlx::query("SELECT 99::INT AS n")
        .fetch_optional(&pool)
        .await
        .expect("fetch_optional failed");

    let raw = maybe.expect("expected Some row");
    let row = AnyRow::from(raw);
    let data = row.to_row_data();
    assert!(matches!(data.columns[0].value, ColumnValue::Integer(99)));
}

// ── fetch_all ─────────────────────────────────────────────────────────────────

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn test_fetch_all_multiple_rows() {
    let pool = connect().await;

    let rows = sqlx::query("SELECT n::BIGINT AS n FROM (VALUES (1), (2), (3)) AS t(n)")
        .fetch_all(&pool)
        .await
        .expect("fetch_all failed");

    assert_eq!(rows.len(), 3);
    let values: Vec<ColumnValue> = rows
        .into_iter()
        .map(|r| AnyRow::from(r).to_row_data().columns[0].value.clone())
        .collect();
    assert!(matches!(values[0], ColumnValue::BigInt(1)));
    assert!(matches!(values[1], ColumnValue::BigInt(2)));
    assert!(matches!(values[2], ColumnValue::BigInt(3)));
}
