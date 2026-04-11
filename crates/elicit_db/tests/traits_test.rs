use elicit_db::{
    DbBackend, DbBackupManager, DbDatabaseManager, DbIndexManager, DbMonitor, DbQueryExecutor,
    DbRoleManager, DbSchemaManager, DbServerAdmin, DbSessionManager, DbTableManager, DbTransactor,
};

// If these functions compile, each trait is object-safe.
fn _assert_session(_: &dyn DbSessionManager) {}
fn _assert_server(_: &dyn DbServerAdmin) {}
fn _assert_database(_: &dyn DbDatabaseManager) {}
fn _assert_schema(_: &dyn DbSchemaManager) {}
fn _assert_table(_: &dyn DbTableManager) {}
fn _assert_query(_: &dyn DbQueryExecutor) {}
fn _assert_transactor(_: &dyn DbTransactor) {}
fn _assert_index(_: &dyn DbIndexManager) {}
fn _assert_role(_: &dyn DbRoleManager) {}
fn _assert_monitor(_: &dyn DbMonitor) {}
fn _assert_backup(_: &dyn DbBackupManager) {}
fn _assert_backend(_: &dyn DbBackend) {}

#[test]
fn object_safety_compiles() {
    // Compilation of the functions above is the test.
    // All 12 traits (11 sub-traits + DbBackend) are object-safe.
}
