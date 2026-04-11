//! Compile-time proofs for `SqlxDbBackend`.
//!
//! All tests are pure compile-time checks — no live database required.

use elicit_db::{
    DbBackend, DbBackupManager, DbDatabaseManager, DbIndexManager, DbMonitor, DbQueryExecutor,
    DbRoleManager, DbSchemaManager, DbServerAdmin, DbSessionManager, DbTableManager, DbTransactor,
};
use elicit_sqlx::SqlxDbBackend;

/// `SqlxDbBackend` is `Send + Sync`.
const _: () = {
    fn assert_send_sync<T: Send + Sync>() {}
    fn check() {
        assert_send_sync::<SqlxDbBackend>();
    }
    let _ = check;
};

/// `SqlxDbBackend` implements all 11 sub-traits.
const _: () = {
    fn assert_session<T: DbSessionManager>() {}
    fn assert_server<T: DbServerAdmin>() {}
    fn assert_database<T: DbDatabaseManager>() {}
    fn assert_schema<T: DbSchemaManager>() {}
    fn assert_table<T: DbTableManager>() {}
    fn assert_query<T: DbQueryExecutor>() {}
    fn assert_transactor<T: DbTransactor>() {}
    fn assert_index<T: DbIndexManager>() {}
    fn assert_role<T: DbRoleManager>() {}
    fn assert_monitor<T: DbMonitor>() {}
    fn assert_backup<T: DbBackupManager>() {}

    fn check() {
        assert_session::<SqlxDbBackend>();
        assert_server::<SqlxDbBackend>();
        assert_database::<SqlxDbBackend>();
        assert_schema::<SqlxDbBackend>();
        assert_table::<SqlxDbBackend>();
        assert_query::<SqlxDbBackend>();
        assert_transactor::<SqlxDbBackend>();
        assert_index::<SqlxDbBackend>();
        assert_role::<SqlxDbBackend>();
        assert_monitor::<SqlxDbBackend>();
        assert_backup::<SqlxDbBackend>();
    }
    let _ = check;
};

/// `SqlxDbBackend` satisfies the `DbBackend` supertrait.
const _: () = {
    fn assert_backend<T: DbBackend>() {}
    fn check() {
        assert_backend::<SqlxDbBackend>();
    }
    let _ = check;
};

/// `SqlxDbBackend` can be used as `dyn DbSessionManager`.
const _: () = {
    fn assert_object_safe(_: &dyn DbSessionManager) {}
    let _ = assert_object_safe;
};

#[test]
fn sqlx_db_backend_send_sync() {
    // Compile-time: if SqlxDbBackend is not Send+Sync, this won't compile.
    fn require_send_sync<T: Send + Sync>(_: &T) {}
    // We don't construct one (needs async + AnyPool), but the type-check is enough.
    let _ = require_send_sync::<SqlxDbBackend>;
}

#[test]
fn db_value_variants_exist() {
    use elicit_db::DbValue;
    // Ensure DbValue variants used in bind_params exist and are reachable.
    let _null = DbValue::Null;
    let _bool = DbValue::Bool(true);
    let _int = DbValue::Int(42);
    let _float = DbValue::Float(3.14);
    let _text = DbValue::Text("hello".into());
    let _bytes = DbValue::Bytes(vec![1, 2, 3]);
    let _json = DbValue::Json(serde_json::json!({"key": "value"}));
}

#[test]
fn db_column_construction() {
    use elicit_db::DbColumn;
    let col = DbColumn {
        name: "id".into(),
        ty: "bigint".into(),
        nullable: false,
        default_value: None,
        primary_key: true,
    };
    assert_eq!(col.name, "id");
    assert!(!col.nullable);
    assert!(col.primary_key);
}

#[test]
fn isolation_level_display() {
    use elicit_db::IsolationLevel;
    assert_eq!(IsolationLevel::ReadCommitted.to_string(), "READ COMMITTED");
    assert_eq!(IsolationLevel::Serializable.to_string(), "SERIALIZABLE");
    assert_eq!(
        IsolationLevel::RepeatableRead.to_string(),
        "REPEATABLE READ"
    );
    assert_eq!(
        IsolationLevel::ReadUncommitted.to_string(),
        "READ UNCOMMITTED"
    );
}

#[test]
fn tx_marker_state_transitions() {
    use elicit_db::{IsolationLevel, Open, TxMarker};
    let open: TxMarker<Open> = TxMarker::open(IsolationLevel::ReadCommitted);
    assert_eq!(open.isolation, IsolationLevel::ReadCommitted);
    let committed = open.commit();
    assert_eq!(committed.isolation, IsolationLevel::ReadCommitted);

    let open2: TxMarker<Open> = TxMarker::open(IsolationLevel::Serializable);
    let rolled_back = open2.rollback();
    assert_eq!(rolled_back.isolation, IsolationLevel::Serializable);
}

#[test]
fn db_row_get_by_name() {
    use elicit_db::{DbRow, DbValue};
    let row = DbRow(vec![
        ("id".into(), DbValue::Int(1)),
        ("name".into(), DbValue::Text("alice".into())),
    ]);
    assert_eq!(row.get("id"), Some(&DbValue::Int(1)));
    assert_eq!(row.get("name"), Some(&DbValue::Text("alice".into())));
    assert_eq!(row.get("missing"), None);
}
