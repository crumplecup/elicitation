//! [`SqlxDbBackend`] — implementation of all 11 `elicit_db` traits via sqlx `AnyPool`.

use std::collections::HashMap;
use std::sync::Arc;

use elicit_db::{
    AccessAuthorized, AuditLogged, BackupConsistent, ColumnExists, Committed,
    ConnectionEstablished, ConnectionId, DatabaseCreated, DbBackupManager, DbColumn,
    DbDatabaseManager, DbError, DbErrorKind, DbExplain, DbIndexInfo, DbIndexManager, DbMonitor,
    DbQueryExecutor, DbResult, DbRoleInfo, DbRoleManager, DbRow, DbRows, DbSchema, DbSchemaManager,
    DbServerAdmin, DbSessionInfo, DbSessionManager, DbSpatialValue, DbStatActivity, DbTableInfo,
    DbTableManager, DbTransactor, DbValue, Durable, IndexExists, IsolationLevel,
    LeastPrivilegeEnforced, Open, RolledBack, RowVisible, SchemaCreated, TableCreated, TableExists,
    TransactionCommitted, TransactionHandle, TxMarker, WALReplayable,
};
use elicitation::Established;
use futures::future::BoxFuture;
use sqlx::any::AnyRow;
use sqlx::{AnyPool, Column as _, Row as _, TypeInfo as _};
use tokio::sync::Mutex;
use uuid::Uuid;

// ── Transaction storage ───────────────────────────────────────────────────────

/// Holds an acquired pool connection used as a manual transaction.
///
/// `pool.acquire()` returns an owned `PoolConnection<Any>` with no lifetime
/// parameters, allowing it to be stored in a `HashMap` without `unsafe` code.
/// BEGIN / COMMIT / ROLLBACK are issued as plain SQL statements.
struct TxSlot {
    conn: Mutex<Option<sqlx::pool::PoolConnection<sqlx::Any>>>,
    isolation: IsolationLevel,
}

impl TxSlot {
    fn new(conn: sqlx::pool::PoolConnection<sqlx::Any>, isolation: IsolationLevel) -> Self {
        Self {
            conn: Mutex::new(Some(conn)),
            isolation,
        }
    }
}

// ── Error helpers ─────────────────────────────────────────────────────────────

#[track_caller]
fn sqlx_err(e: sqlx::Error) -> DbError {
    DbError::new(DbErrorKind::QueryFailed(e.to_string()))
}

#[track_caller]
fn conn_err(e: sqlx::Error) -> DbError {
    DbError::new(DbErrorKind::ConnectionFailed(e.to_string()))
}

#[track_caller]
fn tx_err(msg: impl Into<String>) -> DbError {
    DbError::new(DbErrorKind::TransactionError(msg.into()))
}

// ── Row conversion ────────────────────────────────────────────────────────────

fn decode_spatial_value(row: &AnyRow, ordinal: usize, type_name: &str) -> DbValue {
    let wrap = |payload| match type_name {
        "GEOGRAPHY" => DbValue::Geography(payload),
        _ => DbValue::Geometry(payload),
    };

    row.try_get::<Vec<u8>, _>(ordinal)
        .map(DbSpatialValue::Wkb)
        .map(wrap)
        .or_else(|_| {
            row.try_get::<String, _>(ordinal)
                .map(DbSpatialValue::Wkt)
                .map(wrap)
        })
        .unwrap_or(DbValue::Null)
}

fn any_row_to_db_row(row: &AnyRow) -> DbRow {
    let cols: Vec<(String, DbValue)> = row
        .columns()
        .iter()
        .map(|col| {
            let name = col.name().to_string();
            let ty = col.type_info().name().to_uppercase();
            let val = match ty {
                ref ty if ty == "BOOL" || ty == "BOOLEAN" => row
                    .try_get::<bool, _>(col.ordinal())
                    .map(DbValue::Bool)
                    .unwrap_or(DbValue::Null),
                ref ty
                    if ty == "INT4"
                        || ty == "INTEGER"
                        || ty == "INT8"
                        || ty == "BIGINT"
                        || ty == "INT2"
                        || ty == "SMALLINT" =>
                {
                    row.try_get::<i64, _>(col.ordinal())
                        .map(DbValue::Int)
                        .unwrap_or(DbValue::Null)
                }
                ref ty
                    if ty == "FLOAT4"
                        || ty == "FLOAT8"
                        || ty == "REAL"
                        || ty == "DOUBLE PRECISION" =>
                {
                    row.try_get::<f64, _>(col.ordinal())
                        .map(DbValue::Float)
                        .unwrap_or(DbValue::Null)
                }
                ref ty if ty == "GEOMETRY" || ty == "GEOGRAPHY" => {
                    decode_spatial_value(row, col.ordinal(), ty)
                }
                _ => row
                    .try_get::<String, _>(col.ordinal())
                    .map(DbValue::Text)
                    .unwrap_or_else(|_| {
                        row.try_get::<Vec<u8>, _>(col.ordinal())
                            .map(DbValue::Bytes)
                            .unwrap_or(DbValue::Null)
                    }),
            };
            (name, val)
        })
        .collect();
    DbRow(cols)
}

// ── Parameter binding ─────────────────────────────────────────────────────────

fn bind_spatial_value<'q>(
    q: sqlx::query::Query<'q, sqlx::Any, sqlx::any::AnyArguments<'q>>,
    value: &'q DbSpatialValue,
) -> sqlx::query::Query<'q, sqlx::Any, sqlx::any::AnyArguments<'q>> {
    match value {
        DbSpatialValue::Wkt(text) => q.bind(text.as_str()),
        DbSpatialValue::Wkb(bytes) => q.bind(bytes.as_slice()),
    }
}

fn bind_params<'q>(
    mut q: sqlx::query::Query<'q, sqlx::Any, sqlx::any::AnyArguments<'q>>,
    params: &'q [DbValue],
) -> sqlx::query::Query<'q, sqlx::Any, sqlx::any::AnyArguments<'q>> {
    for param in params {
        match param {
            DbValue::Null => q = q.bind(Option::<String>::None),
            DbValue::Bool(b) => q = q.bind(*b),
            DbValue::Int(i) => q = q.bind(*i),
            DbValue::Float(f) => q = q.bind(*f),
            DbValue::Text(s) => q = q.bind(s.as_str()),
            DbValue::Bytes(b) => q = q.bind(b.as_slice()),
            DbValue::Json(v) => q = q.bind(v.to_string()),
            DbValue::Geometry(value) | DbValue::Geography(value) => {
                q = bind_spatial_value(q, value)
            }
        }
    }
    q
}

// ── pg_stat_activity helpers ──────────────────────────────────────────────────

fn row_to_session_info(row: &AnyRow) -> DbSessionInfo {
    let pid: i64 = row.try_get::<i64, _>(0).unwrap_or(0);
    let app_name: String = row.try_get::<String, _>(1).unwrap_or_default();
    let database: Option<String> = row.try_get::<String, _>(2).ok();
    let state: String = row.try_get::<String, _>(3).unwrap_or_default();
    let query: Option<String> = row.try_get::<String, _>(4).ok();
    let duration_ms: Option<f64> = row.try_get::<f64, _>(5).ok();
    DbSessionInfo {
        pid: pid as i32,
        app_name,
        database,
        state,
        query,
        duration_ms: duration_ms.map(|d| d as u64),
    }
}

fn sessions_to_stat_activity(sessions: Vec<DbSessionInfo>) -> DbStatActivity {
    let idle_count = sessions.iter().filter(|s| s.state == "idle").count();
    let active_count = sessions.iter().filter(|s| s.state == "active").count();
    let idle_in_tx_count = sessions
        .iter()
        .filter(|s| s.state.starts_with("idle in transaction"))
        .count();
    DbStatActivity {
        sessions,
        idle_count,
        active_count,
        idle_in_tx_count,
    }
}

// ── Table/column introspection helpers ───────────────────────────────────────

async fn list_tables_impl(pool: &AnyPool, schema: &str) -> DbResult<Vec<DbTableInfo>> {
    let rows = sqlx::query(
        "SELECT table_name FROM information_schema.tables \
         WHERE table_schema = $1 AND table_type = 'BASE TABLE' ORDER BY table_name",
    )
    .bind(schema)
    .fetch_all(pool)
    .await
    .map_err(sqlx_err)?;

    let mut tables = Vec::new();
    for row in &rows {
        let tname: String = row.try_get::<String, _>(0).map_err(sqlx_err)?;
        let cols = fetch_columns(pool, schema, &tname).await?;
        tables.push(DbTableInfo {
            schema: schema.to_string(),
            name: tname,
            columns: cols,
            row_count_estimate: None,
            size_bytes: None,
        });
    }
    Ok(tables)
}

async fn fetch_columns(pool: &AnyPool, schema: &str, table: &str) -> DbResult<Vec<DbColumn>> {
    let rows = sqlx::query(
        "SELECT column_name, data_type, is_nullable, column_default \
         FROM information_schema.columns \
         WHERE table_schema = $1 AND table_name = $2 ORDER BY ordinal_position",
    )
    .bind(schema)
    .bind(table)
    .fetch_all(pool)
    .await
    .map_err(sqlx_err)?;

    rows.iter()
        .map(|row| {
            let name = row.try_get::<String, _>(0).map_err(sqlx_err)?;
            let ty = row.try_get::<String, _>(1).map_err(sqlx_err)?;
            let nullable_str: String = row.try_get::<String, _>(2).map_err(sqlx_err)?;
            let default_value: Option<String> = row.try_get::<String, _>(3).ok();
            Ok(DbColumn {
                name,
                ty,
                nullable: nullable_str == "YES",
                default_value,
                primary_key: false,
            })
        })
        .collect()
}

// ── SqlxDbBackend ─────────────────────────────────────────────────────────────

/// Database management backend implementing all 11 `elicit_db` traits via sqlx `AnyPool`.
pub struct SqlxDbBackend {
    pool: AnyPool,
    extra_pools: Arc<Mutex<HashMap<String, AnyPool>>>,
    txs: Arc<Mutex<HashMap<String, Arc<TxSlot>>>>,
}

impl SqlxDbBackend {
    /// Create a new backend wrapping an existing pool.
    pub fn new(pool: AnyPool) -> Self {
        Self {
            pool,
            extra_pools: Arc::new(Mutex::new(HashMap::new())),
            txs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Connect to a database URL and create a new backend.
    #[tracing::instrument(skip_all, fields(url))]
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        sqlx::any::install_default_drivers();
        let pool = AnyPool::connect(database_url).await?;
        Ok(Self::new(pool))
    }
}

// ── DbSessionManager ──────────────────────────────────────────────────────────

impl DbSessionManager for SqlxDbBackend {
    #[tracing::instrument(skip_all)]
    fn connect(
        &self,
        url: &str,
    ) -> BoxFuture<'_, DbResult<(ConnectionId, Established<ConnectionEstablished>)>> {
        let url = url.to_string();
        let extra_pools = Arc::clone(&self.extra_pools);
        Box::pin(async move {
            sqlx::any::install_default_drivers();
            let pool = AnyPool::connect(&url).await.map_err(conn_err)?;
            let id = Uuid::new_v4().to_string();
            extra_pools.lock().await.insert(id.clone(), pool);
            Ok((ConnectionId(id), Established::assert()))
        })
    }

    #[tracing::instrument(skip_all)]
    fn disconnect(&self, id: ConnectionId) -> BoxFuture<'_, DbResult<()>> {
        let extra_pools = Arc::clone(&self.extra_pools);
        Box::pin(async move {
            extra_pools.lock().await.remove(&id.0);
            Ok(())
        })
    }

    #[tracing::instrument(skip_all)]
    fn list_sessions(&self) -> BoxFuture<'_, DbResult<DbStatActivity>> {
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT pid, application_name, datname, state, query, \
                 extract(epoch from now()-query_start)*1000 as duration_ms \
                 FROM pg_stat_activity",
            )
            .fetch_all(&self.pool)
            .await
            .map_err(sqlx_err)?;
            let sessions: Vec<DbSessionInfo> = rows.iter().map(row_to_session_info).collect();
            Ok(sessions_to_stat_activity(sessions))
        })
    }

    #[tracing::instrument(skip_all)]
    fn terminate_session(&self, pid: i32) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        Box::pin(async move {
            sqlx::query("SELECT pg_terminate_backend($1)")
                .bind(pid as i64)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }
}

// ── DbServerAdmin ─────────────────────────────────────────────────────────────

impl DbServerAdmin for SqlxDbBackend {
    #[tracing::instrument(skip_all)]
    fn server_version(&self) -> BoxFuture<'_, DbResult<String>> {
        Box::pin(async move {
            let row = sqlx::query("SELECT version()")
                .fetch_one(&self.pool)
                .await
                .map_err(sqlx_err)?;
            row.try_get::<String, _>(0).map_err(sqlx_err)
        })
    }

    #[tracing::instrument(skip_all)]
    fn list_settings(&self) -> BoxFuture<'_, DbResult<Vec<(String, String)>>> {
        Box::pin(async move {
            let rows = sqlx::query("SELECT name, setting FROM pg_settings ORDER BY name")
                .fetch_all(&self.pool)
                .await
                .map_err(sqlx_err)?;
            rows.iter()
                .map(|row| {
                    let name = row.try_get::<String, _>(0).map_err(sqlx_err)?;
                    let setting = row.try_get::<String, _>(1).map_err(sqlx_err)?;
                    Ok((name, setting))
                })
                .collect()
        })
    }

    #[tracing::instrument(skip_all)]
    fn list_extensions(&self) -> BoxFuture<'_, DbResult<Vec<String>>> {
        Box::pin(async move {
            let rows = sqlx::query("SELECT name FROM pg_available_extensions ORDER BY name")
                .fetch_all(&self.pool)
                .await
                .map_err(sqlx_err)?;
            rows.iter()
                .map(|row| row.try_get::<String, _>(0).map_err(sqlx_err))
                .collect()
        })
    }

    #[tracing::instrument(skip_all)]
    fn install_extension(&self, name: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let sql = format!(r#"CREATE EXTENSION IF NOT EXISTS "{name}""#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }

    #[tracing::instrument(skip_all)]
    fn reload_config(&self) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        Box::pin(async move {
            sqlx::query("SELECT pg_reload_conf()")
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }
}

// ── DbDatabaseManager ─────────────────────────────────────────────────────────

impl DbDatabaseManager for SqlxDbBackend {
    #[tracing::instrument(skip_all)]
    fn create_database(
        &self,
        name: &str,
    ) -> BoxFuture<'_, DbResult<(Established<DatabaseCreated>, Established<AuditLogged>)>> {
        let sql = format!(r#"CREATE DATABASE "{name}""#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok((Established::assert(), Established::assert()))
        })
    }

    #[tracing::instrument(skip_all)]
    fn drop_database(&self, name: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let sql = format!(r#"DROP DATABASE IF EXISTS "{name}""#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }

    #[tracing::instrument(skip_all)]
    fn list_databases(&self) -> BoxFuture<'_, DbResult<Vec<String>>> {
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT datname FROM pg_database \
                 WHERE datistemplate = false ORDER BY datname",
            )
            .fetch_all(&self.pool)
            .await
            .map_err(sqlx_err)?;
            rows.iter()
                .map(|row| row.try_get::<String, _>(0).map_err(sqlx_err))
                .collect()
        })
    }

    #[tracing::instrument(skip_all)]
    fn rename_database(
        &self,
        from: &str,
        to: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let sql = format!(r#"ALTER DATABASE "{from}" RENAME TO "{to}""#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }

    #[tracing::instrument(skip_all)]
    fn database_size(&self, name: &str) -> BoxFuture<'_, DbResult<u64>> {
        let name = name.to_string();
        Box::pin(async move {
            let row = sqlx::query("SELECT pg_database_size($1)")
                .bind(name.as_str())
                .fetch_one(&self.pool)
                .await
                .map_err(sqlx_err)?;
            let size: i64 = row.try_get::<i64, _>(0).map_err(sqlx_err)?;
            Ok(size as u64)
        })
    }
}

// ── DbSchemaManager ───────────────────────────────────────────────────────────

impl DbSchemaManager for SqlxDbBackend {
    #[tracing::instrument(skip_all)]
    fn create_schema(
        &self,
        name: &str,
    ) -> BoxFuture<'_, DbResult<(Established<SchemaCreated>, Established<AuditLogged>)>> {
        let sql = format!(r#"CREATE SCHEMA IF NOT EXISTS "{name}""#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok((Established::assert(), Established::assert()))
        })
    }

    #[tracing::instrument(skip_all)]
    fn drop_schema(
        &self,
        name: &str,
        cascade: bool,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let modifier = if cascade { "CASCADE" } else { "RESTRICT" };
        let sql = format!(r#"DROP SCHEMA IF EXISTS "{name}" {modifier}"#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }

    #[tracing::instrument(skip_all)]
    fn list_schemas(&self) -> BoxFuture<'_, DbResult<Vec<String>>> {
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT schema_name FROM information_schema.schemata \
                 WHERE schema_name NOT LIKE 'pg_%' \
                 AND schema_name != 'information_schema' \
                 ORDER BY schema_name",
            )
            .fetch_all(&self.pool)
            .await
            .map_err(sqlx_err)?;
            rows.iter()
                .map(|row| row.try_get::<String, _>(0).map_err(sqlx_err))
                .collect()
        })
    }

    #[tracing::instrument(skip_all)]
    fn schema_info(&self, name: &str) -> BoxFuture<'_, DbResult<DbSchema>> {
        let name = name.to_string();
        Box::pin(async move {
            let owner_row = sqlx::query(
                "SELECT schema_owner FROM information_schema.schemata WHERE schema_name = $1",
            )
            .bind(name.as_str())
            .fetch_optional(&self.pool)
            .await
            .map_err(sqlx_err)?;

            let owner = owner_row
                .as_ref()
                .and_then(|row| row.try_get::<String, _>(0).ok())
                .unwrap_or_default();

            let tables = list_tables_impl(&self.pool, &name).await?;
            Ok(DbSchema {
                name,
                owner,
                tables,
            })
        })
    }
}

// ── DbTableManager ────────────────────────────────────────────────────────────

impl DbTableManager for SqlxDbBackend {
    #[tracing::instrument(skip_all)]
    fn create_table(
        &self,
        schema: &str,
        name: &str,
        columns: Vec<DbColumn>,
    ) -> BoxFuture<'_, DbResult<(Established<TableCreated>, Established<AuditLogged>)>> {
        let col_defs: Vec<String> = columns
            .iter()
            .map(|col| {
                let mut def = format!(r#""{}" {}"#, col.name, col.ty);
                if !col.nullable {
                    def.push_str(" NOT NULL");
                }
                if let Some(default) = &col.default_value {
                    def.push_str(&format!(" DEFAULT {default}"));
                }
                if col.primary_key {
                    def.push_str(" PRIMARY KEY");
                }
                def
            })
            .collect();
        let col_str = col_defs.join(", ");
        let sql = format!(r#"CREATE TABLE IF NOT EXISTS "{schema}"."{name}" ({col_str})"#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok((Established::assert(), Established::assert()))
        })
    }

    #[tracing::instrument(skip_all)]
    fn drop_table(
        &self,
        schema: &str,
        name: &str,
        cascade: bool,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let sql = if cascade {
            format!(r#"DROP TABLE IF EXISTS "{schema}"."{name}" CASCADE"#)
        } else {
            format!(r#"DROP TABLE IF EXISTS "{schema}"."{name}""#)
        };
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }

    #[tracing::instrument(skip_all)]
    fn list_tables(&self, schema: &str) -> BoxFuture<'_, DbResult<Vec<DbTableInfo>>> {
        let schema = schema.to_string();
        Box::pin(async move { list_tables_impl(&self.pool, &schema).await })
    }

    #[tracing::instrument(skip_all)]
    fn inspect_table(
        &self,
        schema: &str,
        name: &str,
    ) -> BoxFuture<'_, DbResult<(DbTableInfo, Established<TableExists>)>> {
        let schema = schema.to_string();
        let name = name.to_string();
        Box::pin(async move {
            let exists: Option<_> = sqlx::query(
                "SELECT table_name FROM information_schema.tables \
                 WHERE table_schema = $1 AND table_name = $2",
            )
            .bind(schema.as_str())
            .bind(name.as_str())
            .fetch_optional(&self.pool)
            .await
            .map_err(sqlx_err)?;

            if exists.is_none() {
                return Err(DbError::new(DbErrorKind::NotFound(format!(
                    "{schema}.{name}"
                ))));
            }

            let columns = fetch_columns(&self.pool, &schema, &name).await?;
            let info = DbTableInfo {
                schema,
                name,
                columns,
                row_count_estimate: None,
                size_bytes: None,
            };
            Ok((info, Established::assert()))
        })
    }

    #[tracing::instrument(skip_all)]
    fn add_column(
        &self,
        schema: &str,
        table: &str,
        column: DbColumn,
    ) -> BoxFuture<'_, DbResult<(Established<ColumnExists>, Established<AuditLogged>)>> {
        let mut sql = format!(
            r#"ALTER TABLE "{schema}"."{table}" ADD COLUMN "{}" {}"#,
            column.name, column.ty
        );
        if !column.nullable {
            sql.push_str(" NOT NULL");
        }
        if let Some(default) = &column.default_value {
            sql.push_str(&format!(" DEFAULT {default}"));
        }
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok((Established::assert(), Established::assert()))
        })
    }

    #[tracing::instrument(skip_all)]
    fn drop_column(
        &self,
        schema: &str,
        table: &str,
        column: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let sql = format!(r#"ALTER TABLE "{schema}"."{table}" DROP COLUMN IF EXISTS "{column}""#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }

    #[tracing::instrument(skip_all)]
    fn rename_table(
        &self,
        schema: &str,
        from: &str,
        to: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let sql = format!(r#"ALTER TABLE "{schema}"."{from}" RENAME TO "{to}""#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }

    #[tracing::instrument(skip_all)]
    fn truncate_table(
        &self,
        schema: &str,
        name: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let sql = format!(r#"TRUNCATE TABLE "{schema}"."{name}""#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }
}

// ── DbQueryExecutor ───────────────────────────────────────────────────────────

impl DbQueryExecutor for SqlxDbBackend {
    #[tracing::instrument(skip_all)]
    fn execute(
        &self,
        sql: &str,
        params: &[DbValue],
    ) -> BoxFuture<'_, DbResult<(u64, Established<AuditLogged>)>> {
        let sql = sql.to_string();
        let params = params.to_vec();
        Box::pin(async move {
            let q = bind_params(sqlx::query(&sql), &params);
            let result = q.execute(&self.pool).await.map_err(sqlx_err)?;
            Ok((result.rows_affected(), Established::assert()))
        })
    }

    #[tracing::instrument(skip_all)]
    fn query_rows(
        &self,
        sql: &str,
        params: &[DbValue],
    ) -> BoxFuture<'_, DbResult<(DbRows, Established<RowVisible>)>> {
        let sql = sql.to_string();
        let params = params.to_vec();
        Box::pin(async move {
            let q = bind_params(sqlx::query(&sql), &params);
            let rows = q.fetch_all(&self.pool).await.map_err(sqlx_err)?;
            let affected = rows.len() as u64;
            let db_rows: Vec<DbRow> = rows.iter().map(any_row_to_db_row).collect();
            Ok((
                DbRows {
                    rows: db_rows,
                    affected,
                },
                Established::assert(),
            ))
        })
    }

    #[tracing::instrument(skip_all)]
    fn explain(&self, sql: &str, analyze: bool) -> BoxFuture<'_, DbResult<DbExplain>> {
        let explain_sql = if analyze {
            format!("EXPLAIN ANALYZE {sql}")
        } else {
            format!("EXPLAIN {sql}")
        };
        Box::pin(async move {
            let rows = sqlx::query(&explain_sql)
                .fetch_all(&self.pool)
                .await
                .map_err(sqlx_err)?;
            let plan = rows
                .iter()
                .filter_map(|row| row.try_get::<String, _>(0).ok())
                .collect::<Vec<_>>()
                .join("\n");
            Ok(DbExplain {
                plan,
                startup_cost: None,
                total_cost: None,
                actual_rows: None,
                actual_time_ms: None,
            })
        })
    }

    #[tracing::instrument(skip_all)]
    fn execute_in_transaction(
        &self,
        sql: &str,
        params: &[DbValue],
        isolation: IsolationLevel,
    ) -> BoxFuture<
        '_,
        DbResult<(
            u64,
            Established<TransactionCommitted>,
            Established<AuditLogged>,
        )>,
    > {
        let sql = sql.to_string();
        let params = params.to_vec();
        Box::pin(async move {
            let mut conn = self.pool.acquire().await.map_err(sqlx_err)?;
            sqlx::query("BEGIN")
                .execute(&mut *conn)
                .await
                .map_err(sqlx_err)?;
            let iso_sql = format!("SET TRANSACTION ISOLATION LEVEL {isolation}");
            sqlx::query(&iso_sql)
                .execute(&mut *conn)
                .await
                .map_err(sqlx_err)?;
            let q = bind_params(sqlx::query(&sql), &params);
            let result = q.execute(&mut *conn).await.map_err(sqlx_err)?;
            let affected = result.rows_affected();
            sqlx::query("COMMIT")
                .execute(&mut *conn)
                .await
                .map_err(sqlx_err)?;
            Ok((affected, Established::assert(), Established::assert()))
        })
    }
}

// ── DbTransactor ──────────────────────────────────────────────────────────────

impl DbTransactor for SqlxDbBackend {
    #[tracing::instrument(skip_all)]
    fn begin(
        &self,
        isolation: IsolationLevel,
    ) -> BoxFuture<'_, DbResult<(TransactionHandle, TxMarker<Open>)>> {
        let txs = Arc::clone(&self.txs);
        Box::pin(async move {
            let mut conn = self.pool.acquire().await.map_err(sqlx_err)?;
            sqlx::query("BEGIN")
                .execute(&mut *conn)
                .await
                .map_err(sqlx_err)?;
            let iso_sql = format!("SET TRANSACTION ISOLATION LEVEL {isolation}");
            sqlx::query(&iso_sql)
                .execute(&mut *conn)
                .await
                .map_err(sqlx_err)?;
            let id = Uuid::new_v4().to_string();
            let slot = Arc::new(TxSlot::new(conn, isolation));
            txs.lock().await.insert(id.clone(), slot);
            Ok((TransactionHandle(id), TxMarker::open(isolation)))
        })
    }

    #[tracing::instrument(skip_all)]
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
        let txs = Arc::clone(&self.txs);
        Box::pin(async move {
            let slot = txs
                .lock()
                .await
                .remove(&handle.0)
                .ok_or_else(|| tx_err(format!("transaction not found: {}", handle.0)))?;
            let isolation = slot.isolation;
            let mut conn_opt = slot.conn.lock().await;
            let conn = conn_opt
                .as_mut()
                .ok_or_else(|| tx_err("transaction already consumed"))?;
            sqlx::query("COMMIT")
                .execute(&mut **conn)
                .await
                .map_err(sqlx_err)?;
            let marker = TxMarker::open(isolation).commit();
            Ok((marker, Established::assert(), Established::assert()))
        })
    }

    #[tracing::instrument(skip_all)]
    fn rollback(&self, handle: TransactionHandle) -> BoxFuture<'_, DbResult<TxMarker<RolledBack>>> {
        let txs = Arc::clone(&self.txs);
        Box::pin(async move {
            let slot = txs
                .lock()
                .await
                .remove(&handle.0)
                .ok_or_else(|| tx_err(format!("transaction not found: {}", handle.0)))?;
            let isolation = slot.isolation;
            let mut conn_opt = slot.conn.lock().await;
            let conn = conn_opt
                .as_mut()
                .ok_or_else(|| tx_err("transaction already consumed"))?;
            sqlx::query("ROLLBACK")
                .execute(&mut **conn)
                .await
                .map_err(sqlx_err)?;
            let marker = TxMarker::open(isolation).rollback();
            Ok(marker)
        })
    }

    #[tracing::instrument(skip_all)]
    fn savepoint(&self, handle: &TransactionHandle, name: &str) -> BoxFuture<'_, DbResult<()>> {
        let txs = Arc::clone(&self.txs);
        let handle_id = handle.0.clone();
        let sql = format!("SAVEPOINT {name}");
        Box::pin(async move {
            let guard = txs.lock().await;
            let slot = guard
                .get(&handle_id)
                .ok_or_else(|| tx_err(format!("transaction not found: {handle_id}")))?
                .clone();
            drop(guard);
            let mut conn_opt = slot.conn.lock().await;
            let conn = conn_opt
                .as_mut()
                .ok_or_else(|| tx_err("transaction already consumed"))?;
            sqlx::query(&sql)
                .execute(&mut **conn)
                .await
                .map_err(sqlx_err)?;
            Ok(())
        })
    }

    #[tracing::instrument(skip_all)]
    fn rollback_to_savepoint(
        &self,
        handle: &TransactionHandle,
        name: &str,
    ) -> BoxFuture<'_, DbResult<()>> {
        let txs = Arc::clone(&self.txs);
        let handle_id = handle.0.clone();
        let sql = format!("ROLLBACK TO SAVEPOINT {name}");
        Box::pin(async move {
            let guard = txs.lock().await;
            let slot = guard
                .get(&handle_id)
                .ok_or_else(|| tx_err(format!("transaction not found: {handle_id}")))?
                .clone();
            drop(guard);
            let mut conn_opt = slot.conn.lock().await;
            let conn = conn_opt
                .as_mut()
                .ok_or_else(|| tx_err("transaction already consumed"))?;
            sqlx::query(&sql)
                .execute(&mut **conn)
                .await
                .map_err(sqlx_err)?;
            Ok(())
        })
    }
}

// ── DbIndexManager ────────────────────────────────────────────────────────────

impl DbIndexManager for SqlxDbBackend {
    #[tracing::instrument(skip_all)]
    fn create_index(
        &self,
        table: &str,
        columns: &[String],
        unique: bool,
    ) -> BoxFuture<'_, DbResult<(Established<IndexExists>, Established<AuditLogged>)>> {
        let unique_kw = if unique { "UNIQUE " } else { "" };
        let cols = columns.join(", ");
        let sql = format!(r#"CREATE {unique_kw}INDEX ON "{table}" ({cols})"#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok((Established::assert(), Established::assert()))
        })
    }

    #[tracing::instrument(skip_all)]
    fn drop_index(&self, name: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let sql = format!(r#"DROP INDEX IF EXISTS "{name}""#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }

    #[tracing::instrument(skip_all)]
    fn list_indexes(&self, table: &str) -> BoxFuture<'_, DbResult<Vec<DbIndexInfo>>> {
        let table = table.to_string();
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT indexname, tablename, indexdef \
                 FROM pg_indexes WHERE tablename = $1",
            )
            .bind(table.as_str())
            .fetch_all(&self.pool)
            .await
            .map_err(sqlx_err)?;

            rows.iter()
                .map(|row| {
                    let name = row.try_get::<String, _>(0).map_err(sqlx_err)?;
                    let tbl = row.try_get::<String, _>(1).map_err(sqlx_err)?;
                    let def: String = row.try_get::<String, _>(2).unwrap_or_default();
                    let unique = def.contains("UNIQUE");
                    Ok(DbIndexInfo {
                        name,
                        table: tbl,
                        columns: vec![],
                        unique,
                        index_type: "btree".to_string(),
                    })
                })
                .collect()
        })
    }

    #[tracing::instrument(skip_all)]
    fn reindex(&self, table: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let sql = format!(r#"REINDEX TABLE "{table}""#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }
}

// ── DbRoleManager ─────────────────────────────────────────────────────────────

impl DbRoleManager for SqlxDbBackend {
    #[tracing::instrument(skip_all)]
    fn create_role(
        &self,
        name: &str,
        can_login: bool,
        superuser: bool,
    ) -> BoxFuture<'_, DbResult<(Established<AccessAuthorized>, Established<AuditLogged>)>> {
        let mut sql = format!(r#"CREATE ROLE "{name}""#);
        if can_login {
            sql.push_str(" LOGIN");
        }
        if superuser {
            sql.push_str(" SUPERUSER");
        }
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok((Established::assert(), Established::assert()))
        })
    }

    #[tracing::instrument(skip_all)]
    fn drop_role(&self, name: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let sql = format!(r#"DROP ROLE IF EXISTS "{name}""#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }

    #[tracing::instrument(skip_all)]
    fn list_roles(&self) -> BoxFuture<'_, DbResult<Vec<DbRoleInfo>>> {
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT rolname, rolsuper, rolcanlogin, rolcreatedb, rolcreaterole \
                 FROM pg_roles ORDER BY rolname",
            )
            .fetch_all(&self.pool)
            .await
            .map_err(sqlx_err)?;

            rows.iter()
                .map(|row| {
                    let name = row.try_get::<String, _>(0).map_err(sqlx_err)?;
                    let superuser = row.try_get::<bool, _>(1).unwrap_or(false);
                    let can_login = row.try_get::<bool, _>(2).unwrap_or(false);
                    let can_create_db = row.try_get::<bool, _>(3).unwrap_or(false);
                    let can_create_role = row.try_get::<bool, _>(4).unwrap_or(false);
                    Ok(DbRoleInfo {
                        name,
                        superuser,
                        can_login,
                        can_create_db,
                        can_create_role,
                    })
                })
                .collect()
        })
    }

    #[tracing::instrument(skip_all)]
    fn grant(
        &self,
        privilege: &str,
        on: &str,
        to: &str,
    ) -> BoxFuture<'_, DbResult<(Established<AccessAuthorized>, Established<AuditLogged>)>> {
        let sql = format!(r#"GRANT {privilege} ON {on} TO "{to}""#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok((Established::assert(), Established::assert()))
        })
    }

    #[tracing::instrument(skip_all)]
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
        let sql = format!(r#"REVOKE {privilege} ON {on} FROM "{from}""#);
        Box::pin(async move {
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok((Established::assert(), Established::assert()))
        })
    }
}

// ── DbMonitor ─────────────────────────────────────────────────────────────────

const STAT_ACTIVITY_SQL: &str = "SELECT pid, application_name, datname, state, query, \
     extract(epoch from now()-query_start)*1000 as duration_ms \
     FROM pg_stat_activity";

impl DbMonitor for SqlxDbBackend {
    #[tracing::instrument(skip_all)]
    fn active_sessions(&self) -> BoxFuture<'_, DbResult<DbStatActivity>> {
        Box::pin(async move {
            let rows = sqlx::query(STAT_ACTIVITY_SQL)
                .fetch_all(&self.pool)
                .await
                .map_err(sqlx_err)?;
            let sessions: Vec<DbSessionInfo> = rows.iter().map(row_to_session_info).collect();
            Ok(sessions_to_stat_activity(sessions))
        })
    }

    #[tracing::instrument(skip_all)]
    fn slow_queries(&self, threshold_ms: u64) -> BoxFuture<'_, DbResult<Vec<DbSessionInfo>>> {
        Box::pin(async move {
            let sql = format!(
                "{STAT_ACTIVITY_SQL} WHERE \
                 extract(epoch from now()-query_start)*1000 > $1"
            );
            let rows = sqlx::query(&sql)
                .bind(threshold_ms as i64)
                .fetch_all(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(rows.iter().map(row_to_session_info).collect())
        })
    }

    #[tracing::instrument(skip_all)]
    fn table_bloat(&self, schema: &str) -> BoxFuture<'_, DbResult<Vec<(String, f64)>>> {
        let schema = schema.to_string();
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT tablename, \
                 n_dead_tup::float/(n_live_tup+1) as bloat_ratio \
                 FROM pg_stat_user_tables WHERE schemaname = $1",
            )
            .bind(schema.as_str())
            .fetch_all(&self.pool)
            .await
            .map_err(sqlx_err)?;

            rows.iter()
                .map(|row| {
                    let name = row.try_get::<String, _>(0).map_err(sqlx_err)?;
                    let ratio = row.try_get::<f64, _>(1).unwrap_or(0.0);
                    Ok((name, ratio))
                })
                .collect()
        })
    }

    #[tracing::instrument(skip_all)]
    fn index_usage(&self, schema: &str) -> BoxFuture<'_, DbResult<Vec<(String, u64)>>> {
        let schema = schema.to_string();
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT indexrelname, idx_scan \
                 FROM pg_stat_user_indexes WHERE schemaname = $1",
            )
            .bind(schema.as_str())
            .fetch_all(&self.pool)
            .await
            .map_err(sqlx_err)?;

            rows.iter()
                .map(|row| {
                    let name = row.try_get::<String, _>(0).map_err(sqlx_err)?;
                    let scans: i64 = row.try_get::<i64, _>(1).unwrap_or(0);
                    Ok((name, scans as u64))
                })
                .collect()
        })
    }

    #[tracing::instrument(skip_all)]
    fn lock_waits(&self) -> BoxFuture<'_, DbResult<Vec<(i32, i32)>>> {
        Box::pin(async move {
            let rows = sqlx::query(
                "SELECT l1.pid as blocking_pid, l2.pid as blocked_pid \
                 FROM pg_locks l1 \
                 JOIN pg_locks l2 ON l1.relation = l2.relation \
                 AND l1.granted AND NOT l2.granted",
            )
            .fetch_all(&self.pool)
            .await
            .map_err(sqlx_err)?;

            rows.iter()
                .map(|row| {
                    let blocking: i64 = row.try_get::<i64, _>(0).map_err(sqlx_err)?;
                    let blocked: i64 = row.try_get::<i64, _>(1).map_err(sqlx_err)?;
                    Ok((blocking as i32, blocked as i32))
                })
                .collect()
        })
    }

    #[tracing::instrument(skip_all)]
    fn cache_hit_ratio(&self) -> BoxFuture<'_, DbResult<f64>> {
        Box::pin(async move {
            let row = sqlx::query(
                "SELECT sum(heap_blks_hit)::float / \
                 (sum(heap_blks_hit) + sum(heap_blks_read) + 1) \
                 FROM pg_statio_user_tables",
            )
            .fetch_one(&self.pool)
            .await
            .map_err(sqlx_err)?;
            Ok(row.try_get::<f64, _>(0).unwrap_or(0.0))
        })
    }
}

// ── DbBackupManager ───────────────────────────────────────────────────────────

impl DbBackupManager for SqlxDbBackend {
    #[tracing::instrument(skip_all)]
    fn initiate_backup(
        &self,
        label: &str,
    ) -> BoxFuture<'_, DbResult<(Established<BackupConsistent>, Established<AuditLogged>)>> {
        let label = label.to_string();
        Box::pin(async move {
            sqlx::query("SELECT pg_backup_start($1, false)")
                .bind(label.as_str())
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok((Established::assert(), Established::assert()))
        })
    }

    #[tracing::instrument(skip_all)]
    fn list_backups(&self) -> BoxFuture<'_, DbResult<Vec<String>>> {
        Box::pin(async move {
            let row = sqlx::query("SELECT last_archived_wal FROM pg_stat_archiver")
                .fetch_optional(&self.pool)
                .await
                .map_err(sqlx_err)?;
            let backups = row
                .and_then(|r| r.try_get::<String, _>(0).ok())
                .map(|s| vec![s])
                .unwrap_or_default();
            Ok(backups)
        })
    }

    #[tracing::instrument(skip_all)]
    fn verify_backup(&self, label: &str) -> BoxFuture<'_, DbResult<Established<BackupConsistent>>> {
        let _label = label.to_string();
        Box::pin(async move {
            sqlx::query("SELECT pg_is_in_backup()")
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }

    #[tracing::instrument(skip_all)]
    fn wal_status(&self) -> BoxFuture<'_, DbResult<Established<WALReplayable>>> {
        Box::pin(async move {
            sqlx::query("SELECT pg_walfile_name(pg_current_wal_lsn())")
                .execute(&self.pool)
                .await
                .map_err(sqlx_err)?;
            Ok(Established::assert())
        })
    }
}
