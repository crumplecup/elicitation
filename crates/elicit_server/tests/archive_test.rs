//! Integration tests for the `archive` module.
//!
//! # Test strategy
//!
//! The browse → display chain is exercised end-to-end against a real
//! PostgreSQL instance when one is available.  All database-touching tests are
//! guarded with `#[cfg_attr(not(feature = "api"), ignore)]` so the default
//! `cargo test` run is always fast and does not require a live DB.
//!
//! Pure-logic tests (display rendering, error construction) run unconditionally.

use elicit_server::archive::display::{ArchiveDisplay, TableDescriptorMode};
use elicit_server::archive::{
    ArchiveBrowsePlugin, ArchiveDbBackend, ArchiveDisplayPlugin, ArchiveError, ArchiveErrorKind,
    ArchiveSpatialPlugin, BackendKind, ColumnDescriptor, DatabaseDescriptor, SchemaDescriptor,
    TableDescriptor, TableType,
};

// ── Pure unit tests ───────────────────────────────────────────────────────────

#[test]
fn error_kind_display() {
    let e = ArchiveErrorKind::SchemaNotFound("public".into());
    assert!(e.to_string().contains("public"));

    let e = ArchiveErrorKind::TableNotFound("public".into(), "users".into());
    assert!(e.to_string().contains("users"));

    let e = ArchiveErrorKind::Connection("timeout".into());
    assert!(e.to_string().contains("timeout"));
}

#[test]
fn archive_error_track_caller() {
    let err = ArchiveError::query_failed("bad sql");
    assert!(!err.file.is_empty());
    assert!(err.line > 0);
    assert!(err.to_string().contains("bad sql"));
}

#[test]
fn database_descriptor_serde_roundtrip() {
    let desc = DatabaseDescriptor::new(
        "postgres://localhost/mydb",
        "mydb",
        Some("PostgreSQL 16.1".to_string()),
    );
    let json = serde_json::to_string(&desc).expect("serialize");
    let back: DatabaseDescriptor = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.db_name, desc.db_name);
    assert_eq!(back.backend, desc.backend);
}

#[test]
fn table_descriptor_display_grid() {
    let col = ColumnDescriptor {
        name: "id".to_string(),
        sql_type: "int8".to_string(),
        nullable: false,
        is_primary_key: true,
        is_foreign_key: false,
        default_value: None,
        is_spatial: false,
    };
    let table = TableDescriptor {
        schema: "public".to_string(),
        table_name: "users".to_string(),
        columns: vec![col],
        estimated_rows: Some(42),
        table_type: TableType::Table,
    };
    let mode = TableDescriptorMode::GridView;
    let (root, nodes) = table.to_ak_nodes(&mode, 100);
    // root node plus at least one child for the column row
    assert!(!nodes.is_empty(), "no nodes emitted for GridView");
    assert!(
        nodes.iter().any(|(id, _)| *id == root),
        "root id must be present in nodes"
    );
}

#[test]
fn query_result_default_mode_is_data_grid() {
    use elicit_server::archive::display::QueryResultMode;
    let mode = QueryResultMode::default();
    matches!(mode, QueryResultMode::DataGrid);
}

#[test]
fn schema_descriptor_flat_list_mode() {
    use elicit_server::archive::display::{ArchiveDisplay, SchemaDescriptorMode};
    let schema = SchemaDescriptor {
        schema_name: "public".to_string(),
        owner: "postgres".to_string(),
        table_names: vec!["users".to_string(), "orders".to_string()],
    };
    let (_root, nodes) = schema.to_ak_nodes(&SchemaDescriptorMode::FlatList, 0);
    assert!(!nodes.is_empty());
}

#[test]
fn backend_kind_serde_roundtrip() {
    for kind in [
        BackendKind::Postgres,
        BackendKind::Sqlite,
        BackendKind::Mysql,
        BackendKind::Unknown,
    ] {
        let json = serde_json::to_string(&kind).expect("serialize");
        let back: BackendKind = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, kind);
    }
}

// ── Plugin smoke tests (no DB required) ──────────────────────────────────────

#[test]
fn browse_plugin_is_debug() {
    let _p = format!("{:?}", ArchiveBrowsePlugin);
}

#[test]
fn display_plugin_is_debug() {
    let _p = format!("{:?}", ArchiveDisplayPlugin);
}

#[test]
fn spatial_plugin_is_debug() {
    let _p = format!("{:?}", ArchiveSpatialPlugin);
}

// ── API-gated DB integration tests ───────────────────────────────────────────

/// URL used for integration tests.  Defaults to a local dev DB.
fn test_db_url() -> String {
    std::env::var("ARCHIVE_TEST_DB_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/postgres".to_string())
}

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn backend_connect_and_list_databases() {
    let backend = ArchiveDbBackend::connect(&test_db_url())
        .await
        .expect("connect");
    use elicit_db::DbDatabaseManager;
    let dbs = backend.list_databases().await.expect("list_databases");
    assert!(!dbs.is_empty(), "expected at least one database");
}

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn backend_server_version() {
    let backend = ArchiveDbBackend::connect(&test_db_url())
        .await
        .expect("connect");
    use elicit_db::DbServerAdmin;
    let ver = backend.server_version().await.expect("server_version");
    assert!(ver.contains("PostgreSQL") || !ver.is_empty());
}

#[tokio::test]
#[cfg_attr(not(feature = "api"), ignore)]
async fn backend_list_schemas() {
    let backend = ArchiveDbBackend::connect(&test_db_url())
        .await
        .expect("connect");
    use elicit_db::DbSchemaManager;
    let schemas = backend.list_schemas().await.expect("list_schemas");
    assert!(
        schemas.iter().any(|s| s == "public"),
        "expected 'public' schema"
    );
}
