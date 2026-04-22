//! [`PolarsDbBackend`] — an in-memory SQL backend backed by Polars `SQLContext`.
//!
//! Implements a subset of [`elicit_db`] traits:
//! - [`DbQueryExecutor`] — executes SQL via `SQLContext`
//! - [`DbTableManager`] — register/unregister DataFrames, inspect schemas
//! - [`DbSchemaManager`] — stub; always returns a single `"default"` schema
//! - [`DbTransactor`] — snapshot-based transactions; `begin` captures frames,
//!   `rollback` restores them, `commit` discards the snapshot
//!
//! Polars SQL does not support positional bind parameters; any `params` argument
//! is accepted for trait compatibility but silently ignored.

use std::collections::HashMap;
use std::sync::Arc;

use elicit_db::{
    AuditLogged, ColumnExists, DbColumn, DbError, DbErrorKind, DbExplain, DbResult, DbRow, DbRows,
    DbSchema, DbTableInfo, DbTransactor, DbValue, IsolationLevel, RowVisible, SchemaCreated,
    TableCreated, TableExists, TransactionCommitted, TransactionHandle, TxMarker,
};
use elicit_db::{Committed, Durable, Open, RolledBack};
use elicit_db::{DbQueryExecutor, DbSchemaManager, DbTableManager};
use elicitation::Established;
use futures::future::BoxFuture;
use polars::prelude::*;
use polars::sql::SQLContext;
use tokio::sync::Mutex;
use tracing::instrument;
use uuid::Uuid;

// ── Types ──────────────────────────────────────────────────────────────────────

type FrameSnapshot = HashMap<String, DataFrame>;
type SnapshotMap = HashMap<String, (TxMarker<Open>, FrameSnapshot)>;

// ── Struct ─────────────────────────────────────────────────────────────────────

/// In-memory SQL backend powered by Polars [`SQLContext`].
///
/// Frames are registered by name and queried with standard SQL. Transactions
/// use a copy-on-snapshot model: `begin` materialises all registered frames
/// into memory; `rollback` re-registers them, restoring prior state.
///
/// This backend is intentionally limited to the four traits that Polars can
/// meaningfully implement. The remaining seven `elicit_db` traits
/// (`DbSessionManager`, `DbServerAdmin`, `DbDatabaseManager`,
/// `DbIndexManager`, `DbRoleManager`, `DbMonitor`, `DbBackupManager`) are
/// not implemented.
pub struct PolarsDbBackend {
    ctx: Arc<Mutex<SQLContext>>,
    snapshots: Arc<Mutex<SnapshotMap>>,
}

impl PolarsDbBackend {
    /// Create a new, empty backend with no registered tables.
    pub fn new() -> Self {
        Self {
            ctx: Arc::new(Mutex::new(SQLContext::new())),
            snapshots: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a [`DataFrame`] as a named table visible to SQL queries.
    #[instrument(skip(self, df))]
    pub async fn register_frame(&self, name: &str, df: DataFrame) {
        self.ctx.lock().await.register(name, df.lazy());
    }

    /// Materialise a registered table back into a [`DataFrame`].
    ///
    /// Returns `None` if the table does not exist or cannot be collected.
    #[instrument(skip(self))]
    pub async fn collect_table(&self, name: &str) -> Option<DataFrame> {
        let lf = {
            let mut ctx = self.ctx.lock().await;
            ctx.execute(&format!("SELECT * FROM \"{name}\"")).ok()?
        };
        lf.collect().ok()
    }
}

impl Default for PolarsDbBackend {
    fn default() -> Self {
        Self::new()
    }
}

// ── Polars → DbValue conversion ────────────────────────────────────────────────

fn df_to_db_rows(df: &DataFrame) -> DbRows {
    let n = df.height();
    let mut rows = Vec::with_capacity(n);
    for i in 0..n {
        let cols: Vec<(String, DbValue)> = df
            .columns()
            .iter()
            .map(|s: &Column| {
                let name = s.name().to_string();
                let val = match s.get(i) {
                    Ok(AnyValue::Null) => DbValue::Null,
                    Ok(AnyValue::Boolean(b)) => DbValue::Bool(b),
                    Ok(AnyValue::Int8(v)) => DbValue::Int(v as i64),
                    Ok(AnyValue::Int16(v)) => DbValue::Int(v as i64),
                    Ok(AnyValue::Int32(v)) => DbValue::Int(v as i64),
                    Ok(AnyValue::Int64(v)) => DbValue::Int(v),
                    Ok(AnyValue::UInt8(v)) => DbValue::Int(v as i64),
                    Ok(AnyValue::UInt16(v)) => DbValue::Int(v as i64),
                    Ok(AnyValue::UInt32(v)) => DbValue::Int(v as i64),
                    Ok(AnyValue::UInt64(v)) => DbValue::Int(v as i64),
                    Ok(AnyValue::Float32(v)) => DbValue::Float(v as f64),
                    Ok(AnyValue::Float64(v)) => DbValue::Float(v),
                    Ok(AnyValue::String(s)) => DbValue::Text(s.to_string()),
                    Ok(AnyValue::StringOwned(s)) => DbValue::Text(s.to_string()),
                    Ok(AnyValue::Binary(b)) => DbValue::Bytes(b.to_vec()),
                    Ok(AnyValue::BinaryOwned(b)) => DbValue::Bytes(b),
                    Ok(other) => DbValue::Text(format!("{other}")),
                    Err(_) => DbValue::Null,
                };
                (name, val)
            })
            .collect();
        rows.push(DbRow(cols));
    }
    DbRows {
        rows,
        affected: n as u64,
    }
}

// ── DbColumn → Polars DataType ─────────────────────────────────────────────────

fn db_col_to_polars_dtype(ty: &str) -> DataType {
    match ty.to_uppercase().as_str() {
        "BIGINT" | "INT8" | "INTEGER" | "INT4" | "INT" => DataType::Int64,
        "SMALLINT" | "INT2" => DataType::Int32,
        "BOOLEAN" | "BOOL" => DataType::Boolean,
        "REAL" | "FLOAT4" => DataType::Float32,
        "FLOAT" | "FLOAT8" | "DOUBLE PRECISION" => DataType::Float64,
        "BYTEA" | "BLOB" | "BYTES" => DataType::Binary,
        "JSON" | "JSONB" => DataType::String,
        _ => DataType::String,
    }
}

// ── DbQueryExecutor ────────────────────────────────────────────────────────────

impl DbQueryExecutor for PolarsDbBackend {
    /// Execute a DML/DDL statement.
    ///
    /// Polars SQL is read-oriented; bind `params` are ignored because
    /// `SQLContext` only accepts literal SQL strings with no positional placeholders.
    #[instrument(skip_all)]
    fn execute(
        &self,
        sql: &str,
        _params: &[DbValue],
    ) -> BoxFuture<'_, DbResult<(u64, Established<AuditLogged>)>> {
        let sql = sql.to_owned();
        Box::pin(async move {
            let _lf = {
                let mut ctx = self.ctx.lock().await;
                ctx.execute(&sql)
                    .map_err(|e| DbError::new(DbErrorKind::QueryFailed(e.to_string())))?
            };
            Ok((0u64, Established::assert()))
        })
    }

    /// Execute a query and return the result rows.
    ///
    /// Bind `params` are ignored; embed values directly in `sql` before calling.
    #[instrument(skip_all)]
    fn query_rows(
        &self,
        sql: &str,
        _params: &[DbValue],
    ) -> BoxFuture<'_, DbResult<(DbRows, Established<RowVisible>)>> {
        let sql = sql.to_owned();
        Box::pin(async move {
            let lf = {
                let mut ctx = self.ctx.lock().await;
                ctx.execute(&sql)
                    .map_err(|e| DbError::new(DbErrorKind::QueryFailed(e.to_string())))?
            };
            let df = lf
                .collect()
                .map_err(|e| DbError::new(DbErrorKind::QueryFailed(e.to_string())))?;
            Ok((df_to_db_rows(&df), Established::assert()))
        })
    }

    /// Return a synthetic explain plan; Polars has no native `EXPLAIN` command.
    #[instrument(skip_all)]
    fn explain(&self, sql: &str, _analyze: bool) -> BoxFuture<'_, DbResult<DbExplain>> {
        let sql = sql.to_owned();
        Box::pin(async move {
            Ok(DbExplain {
                plan: format!("Polars query plan for: {sql}"),
                startup_cost: None,
                total_cost: None,
                actual_rows: None,
                actual_time_ms: None,
            })
        })
    }

    /// Execute inside an auto-managed transaction.
    ///
    /// Bind `params` are ignored (see [`execute`](Self::execute)).
    #[instrument(skip_all)]
    fn execute_in_transaction(
        &self,
        sql: &str,
        _params: &[DbValue],
        _isolation: IsolationLevel,
    ) -> BoxFuture<
        '_,
        DbResult<(
            u64,
            Established<TransactionCommitted>,
            Established<AuditLogged>,
        )>,
    > {
        let sql = sql.to_owned();
        Box::pin(async move {
            let _lf = {
                let mut ctx = self.ctx.lock().await;
                ctx.execute(&sql)
                    .map_err(|e| DbError::new(DbErrorKind::QueryFailed(e.to_string())))?
            };
            Ok((0u64, Established::assert(), Established::assert()))
        })
    }
}

// ── DbTableManager ─────────────────────────────────────────────────────────────

impl DbTableManager for PolarsDbBackend {
    /// Register an empty DataFrame with the given column schema as a new table.
    #[instrument(skip_all)]
    fn create_table(
        &self,
        _schema: &str,
        name: &str,
        columns: Vec<DbColumn>,
    ) -> BoxFuture<'_, DbResult<(Established<TableCreated>, Established<AuditLogged>)>> {
        let name = name.to_owned();
        Box::pin(async move {
            let schema = Schema::from_iter(
                columns
                    .iter()
                    .map(|c| Field::new(c.name.as_str().into(), db_col_to_polars_dtype(&c.ty))),
            );
            let df = DataFrame::empty_with_schema(&schema);
            self.ctx.lock().await.register(&name, df.lazy());
            Ok((Established::assert(), Established::assert()))
        })
    }

    /// Unregister a table from the SQL context.
    #[instrument(skip_all)]
    fn drop_table(
        &self,
        _schema: &str,
        name: &str,
        _cascade: bool,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let name = name.to_owned();
        Box::pin(async move {
            self.ctx.lock().await.unregister(&name);
            Ok(Established::assert())
        })
    }

    /// List all tables registered in this context.
    #[instrument(skip_all)]
    fn list_tables(&self, _schema: &str) -> BoxFuture<'_, DbResult<Vec<DbTableInfo>>> {
        Box::pin(async move {
            let tables = self.ctx.lock().await.get_tables();
            let infos = tables
                .into_iter()
                .map(|name| DbTableInfo {
                    schema: "default".into(),
                    name,
                    columns: vec![],
                    row_count_estimate: None,
                    size_bytes: None,
                })
                .collect();
            Ok(infos)
        })
    }

    /// Retrieve column metadata for a registered table.
    #[instrument(skip_all)]
    fn inspect_table(
        &self,
        _schema: &str,
        name: &str,
    ) -> BoxFuture<'_, DbResult<(DbTableInfo, Established<TableExists>)>> {
        let name = name.to_owned();
        Box::pin(async move {
            let lf = {
                let mut ctx = self.ctx.lock().await;
                ctx.execute(&format!("SELECT * FROM \"{name}\" LIMIT 0"))
                    .map_err(|e| DbError::new(DbErrorKind::NotFound(e.to_string())))?
            };
            let df = lf
                .collect()
                .map_err(|e| DbError::new(DbErrorKind::NotFound(e.to_string())))?;
            let columns = df
                .columns()
                .iter()
                .map(|s: &Column| DbColumn {
                    name: s.name().to_string(),
                    ty: format!("{:?}", s.dtype()),
                    nullable: true,
                    default_value: None,
                    primary_key: false,
                })
                .collect();
            let info = DbTableInfo {
                schema: "default".into(),
                name,
                columns,
                row_count_estimate: Some(df.height() as i64),
                size_bytes: None,
            };
            Ok((info, Established::assert()))
        })
    }

    /// Add a column by materialising the table, extending it, and re-registering.
    #[instrument(skip_all)]
    fn add_column(
        &self,
        _schema: &str,
        table: &str,
        column: DbColumn,
    ) -> BoxFuture<'_, DbResult<(Established<ColumnExists>, Established<AuditLogged>)>> {
        let table = table.to_owned();
        Box::pin(async move {
            let lf = {
                let mut ctx = self.ctx.lock().await;
                ctx.execute(&format!("SELECT * FROM \"{table}\""))
                    .map_err(|e| DbError::new(DbErrorKind::NotFound(e.to_string())))?
            };
            let mut df = lf
                .collect()
                .map_err(|e| DbError::new(DbErrorKind::QueryFailed(e.to_string())))?;
            let dtype = db_col_to_polars_dtype(&column.ty);
            let new_col = Column::new_empty(column.name.as_str().into(), &dtype)
                .new_from_index(0, df.height());
            df.with_column(new_col)
                .map_err(|e| DbError::new(DbErrorKind::SchemaError(e.to_string())))?;
            self.ctx.lock().await.register(&table, df.lazy());
            Ok((Established::assert(), Established::assert()))
        })
    }

    /// Remove a column by materialising the table, dropping the column, and re-registering.
    #[instrument(skip_all)]
    fn drop_column(
        &self,
        _schema: &str,
        table: &str,
        column: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let table = table.to_owned();
        let column = column.to_owned();
        Box::pin(async move {
            let lf = {
                let mut ctx = self.ctx.lock().await;
                ctx.execute(&format!("SELECT * FROM \"{table}\""))
                    .map_err(|e| DbError::new(DbErrorKind::NotFound(e.to_string())))?
            };
            let df = lf
                .collect()
                .map_err(|e| DbError::new(DbErrorKind::QueryFailed(e.to_string())))?;
            let df = df
                .drop(&column)
                .map_err(|e| DbError::new(DbErrorKind::SchemaError(e.to_string())))?;
            self.ctx.lock().await.register(&table, df.lazy());
            Ok(Established::assert())
        })
    }

    /// Rename a table by re-registering it under a new name and unregistering the old.
    #[instrument(skip_all)]
    fn rename_table(
        &self,
        _schema: &str,
        from: &str,
        to: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let from = from.to_owned();
        let to = to.to_owned();
        Box::pin(async move {
            let lf = {
                let mut ctx = self.ctx.lock().await;
                ctx.execute(&format!("SELECT * FROM \"{from}\""))
                    .map_err(|e| DbError::new(DbErrorKind::NotFound(e.to_string())))?
            };
            let df = lf
                .collect()
                .map_err(|e| DbError::new(DbErrorKind::QueryFailed(e.to_string())))?;
            let ctx = self.ctx.lock().await;
            ctx.register(&to, df.lazy());
            ctx.unregister(&from);
            Ok(Established::assert())
        })
    }

    /// Truncate a table by re-registering an empty frame with the same schema.
    #[instrument(skip_all)]
    fn truncate_table(
        &self,
        _schema: &str,
        name: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let name = name.to_owned();
        Box::pin(async move {
            let lf = {
                let mut ctx = self.ctx.lock().await;
                ctx.execute(&format!("SELECT * FROM \"{name}\" LIMIT 0"))
                    .map_err(|e| DbError::new(DbErrorKind::NotFound(e.to_string())))?
            };
            let df = lf
                .collect()
                .map_err(|e| DbError::new(DbErrorKind::QueryFailed(e.to_string())))?;
            self.ctx.lock().await.register(
                &name,
                DataFrame::empty_with_schema(df.schema().as_ref()).lazy(),
            );
            Ok(Established::assert())
        })
    }
}

// ── DbSchemaManager — stub ─────────────────────────────────────────────────────

impl DbSchemaManager for PolarsDbBackend {
    /// No-op; Polars has no schema concept — returns success immediately.
    #[instrument(skip_all)]
    fn create_schema(
        &self,
        _name: &str,
    ) -> BoxFuture<'_, DbResult<(Established<SchemaCreated>, Established<AuditLogged>)>> {
        Box::pin(async move { Ok((Established::assert(), Established::assert())) })
    }

    /// No-op; Polars has no schema concept — returns success immediately.
    #[instrument(skip_all)]
    fn drop_schema(
        &self,
        _name: &str,
        _cascade: bool,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        Box::pin(async move { Ok(Established::assert()) })
    }

    /// Always returns `["default"]`; Polars uses a single flat namespace.
    #[instrument(skip_all)]
    fn list_schemas(&self) -> BoxFuture<'_, DbResult<Vec<String>>> {
        Box::pin(async move { Ok(vec!["default".to_string()]) })
    }

    /// Returns a stub schema descriptor for `"default"`.
    #[instrument(skip_all)]
    fn schema_info(&self, _name: &str) -> BoxFuture<'_, DbResult<DbSchema>> {
        Box::pin(async move {
            Ok(DbSchema {
                name: "default".into(),
                owner: "polars".into(),
                tables: vec![],
            })
        })
    }
}

// ── DbTransactor — snapshot-based ─────────────────────────────────────────────

impl DbTransactor for PolarsDbBackend {
    /// Begin a transaction by snapshotting all currently registered frames.
    #[instrument(skip_all)]
    fn begin(
        &self,
        isolation: IsolationLevel,
    ) -> BoxFuture<'_, DbResult<(TransactionHandle, TxMarker<Open>)>> {
        Box::pin(async move {
            let tx_id = Uuid::new_v4().to_string();
            let table_names = self.ctx.lock().await.get_tables();
            let mut snapshot = FrameSnapshot::new();
            for name in &table_names {
                let lf = {
                    let mut ctx = self.ctx.lock().await;
                    ctx.execute(&format!("SELECT * FROM \"{name}\"")).ok()
                };
                if let Some(Ok(df)) = lf.map(|l| l.collect()) {
                    snapshot.insert(name.clone(), df);
                }
            }
            let marker = TxMarker::open(isolation);
            self.snapshots
                .lock()
                .await
                .insert(tx_id.clone(), (marker, snapshot));
            Ok((TransactionHandle(tx_id), marker))
        })
    }

    /// Commit by discarding the snapshot — registered frames remain as-is.
    #[instrument(skip_all)]
    fn commit(
        &self,
        handle: TransactionHandle,
    ) -> BoxFuture<
        '_,
        DbResult<(
            TxMarker<Committed>,
            Established<TransactionCommitted>,
            Established<Durable>,
        )>,
    > {
        Box::pin(async move {
            let entry = self.snapshots.lock().await.remove(&handle.0);
            let committed = match entry {
                Some((marker, _)) => marker.commit(),
                None => {
                    return Err(DbError::new(DbErrorKind::TransactionError(format!(
                        "no active transaction: {}",
                        handle.0
                    ))));
                }
            };
            Ok((committed, Established::assert(), Established::assert()))
        })
    }

    /// Rollback by restoring all snapshotted frames to the SQL context.
    #[instrument(skip_all)]
    fn rollback(&self, handle: TransactionHandle) -> BoxFuture<'_, DbResult<TxMarker<RolledBack>>> {
        Box::pin(async move {
            let entry = self.snapshots.lock().await.remove(&handle.0);
            let rolled_back = match entry {
                Some((marker, snapshot)) => {
                    let ctx = self.ctx.lock().await;
                    for (name, df) in snapshot {
                        ctx.register(&name, df.lazy());
                    }
                    marker.rollback()
                }
                None => {
                    return Err(DbError::new(DbErrorKind::TransactionError(format!(
                        "no active transaction: {}",
                        handle.0
                    ))));
                }
            };
            Ok(rolled_back)
        })
    }

    /// No-op savepoint; Polars has no sub-transaction concept.
    #[instrument(skip_all)]
    fn savepoint(&self, _handle: &TransactionHandle, _name: &str) -> BoxFuture<'_, DbResult<()>> {
        Box::pin(async move { Ok(()) })
    }

    /// No-op rollback-to-savepoint; Polars has no sub-transaction concept.
    #[instrument(skip_all)]
    fn rollback_to_savepoint(
        &self,
        _handle: &TransactionHandle,
        _name: &str,
    ) -> BoxFuture<'_, DbResult<()>> {
        Box::pin(async move { Ok(()) })
    }
}
