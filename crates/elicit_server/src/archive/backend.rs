//! `ArchiveDbBackend` — a `DbBackend`-implementing wrapper for the archive module.
//!
//! Wraps [`SqlxDbBackend`] and delegates all 11 sub-traits unchanged.
//! The archive plugins use this as the verified-workflow entry point:
//! callers obtain a fully capable `&dyn DbTableManager` (etc.) without
//! coupling to `elicit_sqlx` internals.
//!
//! # Why a wrapper
//!
//! [`SqlxDbBackend`] lives in `elicit_sqlx`; re-exporting it from `elicit_server`
//! would violate the workspace no-re-export rule. This newtype satisfies that
//! constraint while providing `ArchiveResult`-typed construction.

use elicit_db::{
    AccessAuthorized, AuditLogged, BackupConsistent, ColumnExists, ConnectionEstablished,
    ConnectionId, DatabaseCreated, DbColumn, DbCommitResult, DbError, DbExecuteResult, DbExplain,
    DbIndexInfo, DbMonitor, DbQueryExecutor, DbQueryRowsResult, DbResult, DbRoleInfo, DbSchema,
    DbSchemaManager, DbServerAdmin, DbSessionInfo, DbSessionManager, DbStatActivity, DbTableInfo,
    DbTableManager, DbTransactor, DbValue, IndexExists, IsolationLevel, LeastPrivilegeEnforced,
    Open, RolledBack, SchemaCreated, TableCreated, TableExists, TransactionHandle, TxMarker,
    WALReplayable,
};
use elicit_db::{DbBackupManager, DbDatabaseManager, DbIndexManager, DbRoleManager};
use elicit_sqlx::SqlxDbBackend;
use elicitation::Established;
use futures::future::BoxFuture;
use tracing::instrument;

use crate::archive::errors::{ArchiveError, ArchiveErrorKind, ArchiveResult};

// ── ArchiveDbBackend ──────────────────────────────────────────────────────────

/// A fully-capable database backend for the archive module.
///
/// Wraps [`SqlxDbBackend`] and implements all 11 `elicit_db` sub-traits by
/// delegation.  Obtain via [`ArchiveDbBackend::connect`].
pub struct ArchiveDbBackend(SqlxDbBackend);

impl ArchiveDbBackend {
    /// Connect to the given database URL and return a ready backend.
    #[instrument(skip_all, fields(url))]
    pub async fn connect(url: &str) -> ArchiveResult<Self> {
        SqlxDbBackend::connect(url)
            .await
            .map(Self)
            .map_err(|e| ArchiveError::new(ArchiveErrorKind::Connection(e.to_string())))
    }
}

// ── DbSessionManager ──────────────────────────────────────────────────────────

impl DbSessionManager for ArchiveDbBackend {
    fn connect(
        &self,
        url: &str,
    ) -> BoxFuture<'_, DbResult<(ConnectionId, Established<ConnectionEstablished>)>> {
        self.0.connect(url)
    }

    fn disconnect(&self, id: ConnectionId) -> BoxFuture<'_, DbResult<()>> {
        self.0.disconnect(id)
    }

    fn list_sessions(&self) -> BoxFuture<'_, DbResult<DbStatActivity>> {
        self.0.list_sessions()
    }

    fn terminate_session(&self, pid: i32) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        self.0.terminate_session(pid)
    }
}

// ── DbServerAdmin ─────────────────────────────────────────────────────────────

impl DbServerAdmin for ArchiveDbBackend {
    fn server_version(&self) -> BoxFuture<'_, DbResult<String>> {
        self.0.server_version()
    }

    fn list_settings(&self) -> BoxFuture<'_, DbResult<Vec<(String, String)>>> {
        self.0.list_settings()
    }

    fn list_extensions(&self) -> BoxFuture<'_, DbResult<Vec<String>>> {
        self.0.list_extensions()
    }

    fn install_extension(&self, name: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        self.0.install_extension(name)
    }

    fn reload_config(&self) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        self.0.reload_config()
    }
}

// ── DbDatabaseManager ─────────────────────────────────────────────────────────

impl DbDatabaseManager for ArchiveDbBackend {
    fn create_database(
        &self,
        name: &str,
    ) -> BoxFuture<'_, DbResult<(Established<DatabaseCreated>, Established<AuditLogged>)>> {
        self.0.create_database(name)
    }

    fn drop_database(&self, name: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        self.0.drop_database(name)
    }

    fn list_databases(&self) -> BoxFuture<'_, DbResult<Vec<String>>> {
        self.0.list_databases()
    }

    fn rename_database(
        &self,
        from: &str,
        to: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        self.0.rename_database(from, to)
    }

    fn database_size(&self, name: &str) -> BoxFuture<'_, DbResult<u64>> {
        self.0.database_size(name)
    }
}

// ── DbSchemaManager ───────────────────────────────────────────────────────────

impl DbSchemaManager for ArchiveDbBackend {
    fn create_schema(
        &self,
        name: &str,
    ) -> BoxFuture<'_, DbResult<(Established<SchemaCreated>, Established<AuditLogged>)>> {
        self.0.create_schema(name)
    }

    fn drop_schema(
        &self,
        name: &str,
        cascade: bool,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        self.0.drop_schema(name, cascade)
    }

    fn list_schemas(&self) -> BoxFuture<'_, DbResult<Vec<String>>> {
        self.0.list_schemas()
    }

    fn schema_info(&self, name: &str) -> BoxFuture<'_, DbResult<DbSchema>> {
        self.0.schema_info(name)
    }
}

// ── DbTableManager ────────────────────────────────────────────────────────────

impl DbTableManager for ArchiveDbBackend {
    fn create_table(
        &self,
        schema: &str,
        name: &str,
        columns: Vec<DbColumn>,
    ) -> BoxFuture<'_, DbResult<(Established<TableCreated>, Established<AuditLogged>)>> {
        self.0.create_table(schema, name, columns)
    }

    fn drop_table(
        &self,
        schema: &str,
        name: &str,
        cascade: bool,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        self.0.drop_table(schema, name, cascade)
    }

    fn list_tables(&self, schema: &str) -> BoxFuture<'_, DbResult<Vec<DbTableInfo>>> {
        self.0.list_tables(schema)
    }

    fn inspect_table(
        &self,
        schema: &str,
        name: &str,
    ) -> BoxFuture<'_, DbResult<(DbTableInfo, Established<TableExists>)>> {
        self.0.inspect_table(schema, name)
    }

    fn add_column(
        &self,
        schema: &str,
        table: &str,
        column: DbColumn,
    ) -> BoxFuture<'_, DbResult<(Established<ColumnExists>, Established<AuditLogged>)>> {
        self.0.add_column(schema, table, column)
    }

    fn drop_column(
        &self,
        schema: &str,
        table: &str,
        column: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        self.0.drop_column(schema, table, column)
    }

    fn rename_table(
        &self,
        schema: &str,
        from: &str,
        to: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        self.0.rename_table(schema, from, to)
    }

    fn truncate_table(
        &self,
        schema: &str,
        name: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        self.0.truncate_table(schema, name)
    }
}

// ── DbQueryExecutor ───────────────────────────────────────────────────────────

impl DbQueryExecutor for ArchiveDbBackend {
    fn execute(&self, sql: &str, params: &[DbValue]) -> BoxFuture<'_, DbExecuteResult> {
        self.0.execute(sql, params)
    }

    fn query_rows(&self, sql: &str, params: &[DbValue]) -> BoxFuture<'_, DbQueryRowsResult> {
        self.0.query_rows(sql, params)
    }

    fn explain(&self, sql: &str, analyze: bool) -> BoxFuture<'_, DbResult<DbExplain>> {
        self.0.explain(sql, analyze)
    }

    fn execute_in_transaction(
        &self,
        sql: &str,
        params: &[DbValue],
        isolation: IsolationLevel,
    ) -> BoxFuture<'_, elicit_db::DbTransactionalExecuteResult> {
        self.0.execute_in_transaction(sql, params, isolation)
    }
}

// ── DbTransactor ──────────────────────────────────────────────────────────────

impl DbTransactor for ArchiveDbBackend {
    fn begin(
        &self,
        isolation: IsolationLevel,
    ) -> BoxFuture<'_, DbResult<(TransactionHandle, TxMarker<Open>)>> {
        self.0.begin(isolation)
    }

    fn commit(&self, handle: TransactionHandle) -> BoxFuture<'_, DbCommitResult> {
        self.0.commit(handle)
    }

    fn rollback(&self, handle: TransactionHandle) -> BoxFuture<'_, DbResult<TxMarker<RolledBack>>> {
        self.0.rollback(handle)
    }

    fn savepoint(&self, handle: &TransactionHandle, name: &str) -> BoxFuture<'_, DbResult<()>> {
        self.0.savepoint(handle, name)
    }

    fn rollback_to_savepoint(
        &self,
        handle: &TransactionHandle,
        name: &str,
    ) -> BoxFuture<'_, DbResult<()>> {
        self.0.rollback_to_savepoint(handle, name)
    }
}

// ── DbIndexManager ────────────────────────────────────────────────────────────

impl DbIndexManager for ArchiveDbBackend {
    fn create_index(
        &self,
        table: &str,
        columns: &[String],
        unique: bool,
    ) -> BoxFuture<'_, DbResult<(Established<IndexExists>, Established<AuditLogged>)>> {
        self.0.create_index(table, columns, unique)
    }

    fn drop_index(&self, name: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        self.0.drop_index(name)
    }

    fn list_indexes(&self, table: &str) -> BoxFuture<'_, DbResult<Vec<DbIndexInfo>>> {
        self.0.list_indexes(table)
    }

    fn reindex(&self, table: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        self.0.reindex(table)
    }
}

// ── DbRoleManager ─────────────────────────────────────────────────────────────

impl DbRoleManager for ArchiveDbBackend {
    fn create_role(
        &self,
        name: &str,
        can_login: bool,
        superuser: bool,
    ) -> BoxFuture<'_, DbResult<(Established<AccessAuthorized>, Established<AuditLogged>)>> {
        self.0.create_role(name, can_login, superuser)
    }

    fn drop_role(&self, name: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        self.0.drop_role(name)
    }

    fn list_roles(&self) -> BoxFuture<'_, DbResult<Vec<DbRoleInfo>>> {
        self.0.list_roles()
    }

    fn grant(
        &self,
        privilege: &str,
        on: &str,
        to: &str,
    ) -> BoxFuture<'_, DbResult<(Established<AccessAuthorized>, Established<AuditLogged>)>> {
        self.0.grant(privilege, on, to)
    }

    fn revoke(
        &self,
        privilege: &str,
        on: &str,
        from: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<LeastPrivilegeEnforced>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.revoke(privilege, on, from)
    }
}

// ── DbMonitor ─────────────────────────────────────────────────────────────────

impl DbMonitor for ArchiveDbBackend {
    fn active_sessions(&self) -> BoxFuture<'_, DbResult<DbStatActivity>> {
        self.0.active_sessions()
    }

    fn slow_queries(&self, threshold_ms: u64) -> BoxFuture<'_, DbResult<Vec<DbSessionInfo>>> {
        self.0.slow_queries(threshold_ms)
    }

    fn table_bloat(&self, schema: &str) -> BoxFuture<'_, DbResult<Vec<(String, f64)>>> {
        self.0.table_bloat(schema)
    }

    fn index_usage(&self, schema: &str) -> BoxFuture<'_, DbResult<Vec<(String, u64)>>> {
        self.0.index_usage(schema)
    }

    fn lock_waits(&self) -> BoxFuture<'_, DbResult<Vec<(i32, i32)>>> {
        self.0.lock_waits()
    }

    fn cache_hit_ratio(&self) -> BoxFuture<'_, DbResult<f64>> {
        self.0.cache_hit_ratio()
    }
}

// ── DbBackupManager ───────────────────────────────────────────────────────────

impl DbBackupManager for ArchiveDbBackend {
    fn initiate_backup(
        &self,
        label: &str,
    ) -> BoxFuture<'_, DbResult<(Established<BackupConsistent>, Established<AuditLogged>)>> {
        self.0.initiate_backup(label)
    }

    fn list_backups(&self) -> BoxFuture<'_, DbResult<Vec<String>>> {
        self.0.list_backups()
    }

    fn verify_backup(&self, label: &str) -> BoxFuture<'_, DbResult<Established<BackupConsistent>>> {
        self.0.verify_backup(label)
    }

    fn wal_status(&self) -> BoxFuture<'_, DbResult<Established<WALReplayable>>> {
        self.0.wal_status()
    }
}
