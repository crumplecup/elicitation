//! `ArchiveDbBackend` — a `DbBackend`-implementing wrapper for the archive module.
//!
//! Wraps [`SqlxDbBackend`] and delegates all 20 sub-traits unchanged.
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
    AccessAuthorized, AnonymousBlockExecuted, AuditLogRetentionMet, AuditLogTamperEvident,
    AuditLogged, BackupConsistent, CheckConstraintDefined, ColumnExists, ConnectionEstablished,
    ConnectionId, ConstraintSatisfied, DatabaseCreated, DbBackupManager, DbColumn, DbCommitResult,
    DbConstraintFactory, DbConstraintMeta, DbDatabaseManager, DbExecuteResult, DbExplain,
    DbIndexInfo, DbIndexManager, DbIsolationFactory, DbMonitor, DbPublicationDescriptor,
    DbQueryExecutor, DbQueryRowsResult, DbReplicationFactory, DbReplicationMeta,
    DbReplicationSlotDescriptor, DbResult, DbRoleInfo, DbRoleManager, DbRoutineDescriptor,
    DbRoutineFactory, DbRoutineMeta, DbSchema, DbSchemaManager, DbSecurityFactory, DbSecurityMeta,
    DbServerAdmin, DbSessionInfo, DbSessionManager, DbStatActivity, DbSubscriptionDescriptor,
    DbTableInfo, DbTableManager, DbTransactor, DbValue, EncryptedAtRest, EncryptedInTransit,
    ForeignKeyDefined, FunctionAltered, FunctionCreated, FunctionDropped,
    FunctionParallelRestricted, FunctionParallelSafe, FunctionParallelUnsafe,
    FunctionSecurityDefiner, FunctionSecurityInvoker, IndexExists, IsolationLevel,
    LeastPrivilegeEnforced, LogicalReplicationConfigured, LogicalReplicationSlotCreated,
    MultiFactorAuthEnforced, NotNullConstraintDefined, Open, PasswordPolicyEnforced,
    PhysicalReplicationSlotCreated, PrimaryKeyDefined, ProcedureCreated, ProcedureDropped,
    PublicationCreated, ReadCommittedIsolation, ReadUncommittedIsolation, RepeatableReadIsolation,
    ReplicationSlotDropped, RolledBack, RowLevelSecurityEnabled, RowLevelSecurityPolicyDefined,
    SchemaCreated, SerializableIsolation, SessionIsolationLevelSet, SessionTimeoutEnforced,
    SqlInjectionPrevented, SslModeRequired, StreamingReplicationConfigured, SubscriptionCreated,
    TableCreated, TableExists, TransactionHandle, TransactionIsolationLevelSet,
    TransactionReadOnly, TransactionReadWrite, TriggerFunctionCreated, TriggerWhenConditionDefined,
    TxMarker, UniqueConstraintDefined, WALReplayable, WalLevelLogical, WalLevelReplica,
};
use elicit_redb::RedbBackend;
use elicit_sqlx::SqlxDbBackend;
use elicitation::Established;
use futures::future::BoxFuture;
use tracing::instrument;

use crate::archive::errors::{ArchiveError, ArchiveErrorKind, ArchiveResult};

// ── ArchiveDbBackend ──────────────────────────────────────────────────────────

/// A fully-capable database backend for the archive module.
///
/// Wraps [`SqlxDbBackend`] and implements all 20 `elicit_db` sub-traits by
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

// ── DbRoutineFactory ──────────────────────────────────────────────────────────

impl DbRoutineFactory for ArchiveDbBackend {
    fn create_function(
        &self,
        descriptor: DbRoutineDescriptor,
    ) -> BoxFuture<
        '_,
        DbResult<(
            DbRoutineDescriptor,
            Established<FunctionCreated>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.create_function(descriptor)
    }

    fn drop_function(
        &self,
        schema: &str,
        name: &str,
        arg_types: &[String],
    ) -> BoxFuture<'_, DbResult<(Established<FunctionDropped>, Established<AuditLogged>)>> {
        self.0.drop_function(schema, name, arg_types)
    }

    fn alter_function(
        &self,
        descriptor: DbRoutineDescriptor,
    ) -> BoxFuture<
        '_,
        DbResult<(
            DbRoutineDescriptor,
            Established<FunctionAltered>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.alter_function(descriptor)
    }

    fn create_procedure(
        &self,
        descriptor: DbRoutineDescriptor,
    ) -> BoxFuture<
        '_,
        DbResult<(
            DbRoutineDescriptor,
            Established<ProcedureCreated>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.create_procedure(descriptor)
    }

    fn drop_procedure(
        &self,
        schema: &str,
        name: &str,
        arg_types: &[String],
    ) -> BoxFuture<'_, DbResult<(Established<ProcedureDropped>, Established<AuditLogged>)>> {
        self.0.drop_procedure(schema, name, arg_types)
    }

    fn declare_parallel_safe(
        &self,
        schema: &str,
        name: &str,
    ) -> BoxFuture<'_, DbResult<(Established<FunctionParallelSafe>, Established<AuditLogged>)>>
    {
        self.0.declare_parallel_safe(schema, name)
    }

    fn declare_parallel_restricted(
        &self,
        schema: &str,
        name: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<FunctionParallelRestricted>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.declare_parallel_restricted(schema, name)
    }

    fn declare_parallel_unsafe(
        &self,
        schema: &str,
        name: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<FunctionParallelUnsafe>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.declare_parallel_unsafe(schema, name)
    }

    fn set_security_definer(
        &self,
        schema: &str,
        name: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<FunctionSecurityDefiner>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.set_security_definer(schema, name)
    }

    fn set_security_invoker(
        &self,
        schema: &str,
        name: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<FunctionSecurityInvoker>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.set_security_invoker(schema, name)
    }

    fn execute_anonymous_block(
        &self,
        body: &str,
        language: &str,
    ) -> BoxFuture<'_, DbResult<Established<AnonymousBlockExecuted>>> {
        self.0.execute_anonymous_block(body, language)
    }

    fn create_trigger_function(
        &self,
        descriptor: DbRoutineDescriptor,
    ) -> BoxFuture<
        '_,
        DbResult<(
            DbRoutineDescriptor,
            Established<TriggerFunctionCreated>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.create_trigger_function(descriptor)
    }

    fn define_trigger_when(
        &self,
        schema: &str,
        table: &str,
        trigger_name: &str,
        when_expr: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<TriggerWhenConditionDefined>,
            Established<AuditLogged>,
        )>,
    > {
        self.0
            .define_trigger_when(schema, table, trigger_name, when_expr)
    }
}

// ── DbRoutineMeta ─────────────────────────────────────────────────────────────

impl DbRoutineMeta for ArchiveDbBackend {
    fn list_functions(&self, schema: &str) -> BoxFuture<'_, DbResult<Vec<DbRoutineDescriptor>>> {
        self.0.list_functions(schema)
    }

    fn list_procedures(&self, schema: &str) -> BoxFuture<'_, DbResult<Vec<DbRoutineDescriptor>>> {
        self.0.list_procedures(schema)
    }

    fn routine_info(
        &self,
        schema: &str,
        name: &str,
        arg_types: &[String],
    ) -> BoxFuture<'_, DbResult<DbRoutineDescriptor>> {
        self.0.routine_info(schema, name, arg_types)
    }
}

// ── DbConstraintFactory ───────────────────────────────────────────────────────

impl DbConstraintFactory for ArchiveDbBackend {
    fn add_check_constraint(
        &self,
        schema: &str,
        table: &str,
        name: &str,
        expression: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<CheckConstraintDefined>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.add_check_constraint(schema, table, name, expression)
    }

    fn add_primary_key(
        &self,
        schema: &str,
        table: &str,
        columns: &[String],
    ) -> BoxFuture<'_, DbResult<(Established<PrimaryKeyDefined>, Established<AuditLogged>)>> {
        self.0.add_primary_key(schema, table, columns)
    }

    fn add_unique_constraint(
        &self,
        schema: &str,
        table: &str,
        name: &str,
        columns: &[String],
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<UniqueConstraintDefined>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.add_unique_constraint(schema, table, name, columns)
    }

    fn add_foreign_key(
        &self,
        schema: &str,
        table: &str,
        name: &str,
        columns: &[String],
        referenced_table: &str,
        referenced_columns: &[String],
    ) -> BoxFuture<'_, DbResult<(Established<ForeignKeyDefined>, Established<AuditLogged>)>> {
        self.0.add_foreign_key(
            schema,
            table,
            name,
            columns,
            referenced_table,
            referenced_columns,
        )
    }

    fn add_not_null(
        &self,
        schema: &str,
        table: &str,
        column: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<NotNullConstraintDefined>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.add_not_null(schema, table, column)
    }

    fn drop_constraint(
        &self,
        schema: &str,
        table: &str,
        name: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        self.0.drop_constraint(schema, table, name)
    }
}

// ── DbConstraintMeta ──────────────────────────────────────────────────────────

impl DbConstraintMeta for ArchiveDbBackend {
    fn list_constraints(
        &self,
        schema: &str,
        table: &str,
    ) -> BoxFuture<'_, DbResult<Vec<(String, String)>>> {
        self.0.list_constraints(schema, table)
    }

    fn verify_constraints(
        &self,
        schema: &str,
        table: &str,
    ) -> BoxFuture<'_, DbResult<Established<ConstraintSatisfied>>> {
        self.0.verify_constraints(schema, table)
    }
}

// ── DbIsolationFactory ────────────────────────────────────────────────────────

impl DbIsolationFactory for ArchiveDbBackend {
    fn begin_read_committed(
        &self,
    ) -> BoxFuture<
        '_,
        DbResult<(
            TransactionHandle,
            TxMarker<Open>,
            Established<ReadCommittedIsolation>,
        )>,
    > {
        self.0.begin_read_committed()
    }

    fn begin_repeatable_read(
        &self,
    ) -> BoxFuture<
        '_,
        DbResult<(
            TransactionHandle,
            TxMarker<Open>,
            Established<RepeatableReadIsolation>,
        )>,
    > {
        self.0.begin_repeatable_read()
    }

    fn begin_serializable(
        &self,
    ) -> BoxFuture<
        '_,
        DbResult<(
            TransactionHandle,
            TxMarker<Open>,
            Established<SerializableIsolation>,
        )>,
    > {
        self.0.begin_serializable()
    }

    fn begin_read_uncommitted(
        &self,
    ) -> BoxFuture<
        '_,
        DbResult<(
            TransactionHandle,
            TxMarker<Open>,
            Established<ReadUncommittedIsolation>,
        )>,
    > {
        self.0.begin_read_uncommitted()
    }

    fn begin_read_only(
        &self,
        isolation: IsolationLevel,
    ) -> BoxFuture<
        '_,
        DbResult<(
            TransactionHandle,
            TxMarker<Open>,
            Established<TransactionReadOnly>,
        )>,
    > {
        self.0.begin_read_only(isolation)
    }

    fn begin_read_write(
        &self,
        isolation: IsolationLevel,
    ) -> BoxFuture<
        '_,
        DbResult<(
            TransactionHandle,
            TxMarker<Open>,
            Established<TransactionReadWrite>,
        )>,
    > {
        self.0.begin_read_write(isolation)
    }

    fn set_session_isolation(
        &self,
        level: IsolationLevel,
    ) -> BoxFuture<'_, DbResult<Established<SessionIsolationLevelSet>>> {
        self.0.set_session_isolation(level)
    }

    fn set_transaction_isolation(
        &self,
        handle: &TransactionHandle,
        level: IsolationLevel,
    ) -> BoxFuture<'_, DbResult<Established<TransactionIsolationLevelSet>>> {
        self.0.set_transaction_isolation(handle, level)
    }
}

// ── DbSecurityFactory ─────────────────────────────────────────────────────────

impl DbSecurityFactory for ArchiveDbBackend {
    fn enforce_tls(
        &self,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<SslModeRequired>,
            Established<EncryptedInTransit>,
        )>,
    > {
        self.0.enforce_tls()
    }

    fn configure_encryption_at_rest(
        &self,
    ) -> BoxFuture<'_, DbResult<Established<EncryptedAtRest>>> {
        self.0.configure_encryption_at_rest()
    }

    fn enable_row_level_security(
        &self,
        schema: &str,
        table: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<RowLevelSecurityEnabled>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.enable_row_level_security(schema, table)
    }

    fn define_rls_policy(
        &self,
        schema: &str,
        table: &str,
        policy_name: &str,
        using_expr: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<RowLevelSecurityPolicyDefined>,
            Established<AuditLogged>,
        )>,
    > {
        self.0
            .define_rls_policy(schema, table, policy_name, using_expr)
    }

    fn enforce_mfa(&self) -> BoxFuture<'_, DbResult<Established<MultiFactorAuthEnforced>>> {
        self.0.enforce_mfa()
    }

    fn enforce_password_policy(
        &self,
    ) -> BoxFuture<'_, DbResult<Established<PasswordPolicyEnforced>>> {
        self.0.enforce_password_policy()
    }

    fn enforce_session_timeout(
        &self,
        timeout_ms: u64,
    ) -> BoxFuture<'_, DbResult<Established<SessionTimeoutEnforced>>> {
        self.0.enforce_session_timeout(timeout_ms)
    }

    fn enforce_parameterized_queries(
        &self,
    ) -> BoxFuture<'_, DbResult<Established<SqlInjectionPrevented>>> {
        self.0.enforce_parameterized_queries()
    }

    fn verify_audit_log_integrity(
        &self,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<AuditLogTamperEvident>,
            Established<AuditLogRetentionMet>,
        )>,
    > {
        self.0.verify_audit_log_integrity()
    }

    fn apply_least_privilege(
        &self,
    ) -> BoxFuture<'_, DbResult<Established<LeastPrivilegeEnforced>>> {
        self.0.apply_least_privilege()
    }
}

// ── DbSecurityMeta ────────────────────────────────────────────────────────────

impl DbSecurityMeta for ArchiveDbBackend {
    fn tls_status(&self) -> BoxFuture<'_, DbResult<bool>> {
        self.0.tls_status()
    }

    fn hba_rules(&self) -> BoxFuture<'_, DbResult<Vec<(String, String, String, String)>>> {
        self.0.hba_rules()
    }

    fn idle_transaction_sessions(&self, threshold_ms: u64) -> BoxFuture<'_, DbResult<Vec<i32>>> {
        self.0.idle_transaction_sessions(threshold_ms)
    }

    fn security_settings(&self) -> BoxFuture<'_, DbResult<Vec<(String, String)>>> {
        self.0.security_settings()
    }
}

// ── DbReplicationFactory ──────────────────────────────────────────────────────

impl DbReplicationFactory for ArchiveDbBackend {
    fn create_publication(
        &self,
        descriptor: DbPublicationDescriptor,
    ) -> BoxFuture<
        '_,
        DbResult<(
            DbPublicationDescriptor,
            Established<PublicationCreated>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.create_publication(descriptor)
    }

    fn create_subscription(
        &self,
        descriptor: DbSubscriptionDescriptor,
    ) -> BoxFuture<
        '_,
        DbResult<(
            DbSubscriptionDescriptor,
            Established<SubscriptionCreated>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.create_subscription(descriptor)
    }

    fn create_physical_slot(
        &self,
        name: &str,
        immediately_reserve: bool,
    ) -> BoxFuture<
        '_,
        DbResult<(
            DbReplicationSlotDescriptor,
            Established<PhysicalReplicationSlotCreated>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.create_physical_slot(name, immediately_reserve)
    }

    fn create_logical_slot(
        &self,
        name: &str,
        plugin: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            DbReplicationSlotDescriptor,
            Established<LogicalReplicationSlotCreated>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.create_logical_slot(name, plugin)
    }

    fn drop_slot(
        &self,
        name: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<ReplicationSlotDropped>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.drop_slot(name)
    }

    fn configure_streaming_replication(
        &self,
        max_wal_senders: u32,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<WalLevelReplica>,
            Established<StreamingReplicationConfigured>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.configure_streaming_replication(max_wal_senders)
    }

    fn configure_logical_replication(
        &self,
        max_replication_slots: u32,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<WalLevelLogical>,
            Established<LogicalReplicationConfigured>,
            Established<AuditLogged>,
        )>,
    > {
        self.0.configure_logical_replication(max_replication_slots)
    }
}

// ── DbReplicationMeta ─────────────────────────────────────────────────────────

impl DbReplicationMeta for ArchiveDbBackend {
    fn replication_slot_lag(
        &self,
    ) -> BoxFuture<'_, DbResult<Vec<(DbReplicationSlotDescriptor, u64)>>> {
        self.0.replication_slot_lag()
    }

    fn list_publications(&self) -> BoxFuture<'_, DbResult<Vec<DbPublicationDescriptor>>> {
        self.0.list_publications()
    }

    fn list_subscriptions(&self) -> BoxFuture<'_, DbResult<Vec<DbSubscriptionDescriptor>>> {
        self.0.list_subscriptions()
    }

    fn streaming_replication_status(&self) -> BoxFuture<'_, DbResult<Vec<(String, String)>>> {
        self.0.streaming_replication_status()
    }
}

// ── ArchiveKvBackend ──────────────────────────────────────────────────────────

/// An embedded key-value backend for the archive module.
///
/// Wraps [`RedbBackend`] and exposes the `DbEmbeddedBackend` constituent traits.
/// Obtain via [`ArchiveKvBackend::open`] (file-backed) or
/// [`ArchiveKvBackend::in_memory`] (transient, useful for tests).
pub struct ArchiveKvBackend(RedbBackend);

impl ArchiveKvBackend {
    /// Open or create a redb file at `path`.
    #[instrument(skip_all, fields(%path))]
    pub fn open(path: &str) -> ArchiveResult<Self> {
        RedbBackend::open(path)
            .map(Self)
            .map_err(|e| ArchiveError::new(ArchiveErrorKind::Backend(e.to_string())))
    }

    /// Create a transient in-memory redb instance (for tests / ephemeral sessions).
    pub fn in_memory() -> ArchiveResult<Self> {
        RedbBackend::in_memory()
            .map(Self)
            .map_err(|e| ArchiveError::new(ArchiveErrorKind::Backend(e.to_string())))
    }

    /// Access the underlying [`RedbBackend`] for direct KV operations.
    pub fn backend(&self) -> &RedbBackend {
        &self.0
    }
}
