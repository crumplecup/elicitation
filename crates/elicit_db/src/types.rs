//! Common data types used in database trait signatures.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A single scalar database value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum DbValue {
    /// SQL NULL.
    Null,
    /// Boolean value.
    Bool(bool),
    /// 64-bit signed integer.
    Int(i64),
    /// 64-bit floating point.
    Float(f64),
    /// UTF-8 text.
    Text(String),
    /// Raw bytes.
    Bytes(Vec<u8>),
    /// Arbitrary JSON value.
    Json(serde_json::Value),
}

/// A single row from a query result — ordered named columns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DbRow(pub Vec<(String, DbValue)>);

impl DbRow {
    /// Look up a column value by name.
    pub fn get(&self, col: &str) -> Option<&DbValue> {
        self.0.iter().find(|(k, _)| k == col).map(|(_, v)| v)
    }
}

/// A collection of query rows with affected-row count.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DbRows {
    /// The result rows.
    pub rows: Vec<DbRow>,
    /// Number of rows affected or returned.
    pub affected: u64,
}

/// Column definition metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct DbColumn {
    /// Column name.
    pub name: String,
    /// SQL data type name.
    pub ty: String,
    /// Whether the column accepts NULL.
    pub nullable: bool,
    /// Default value expression, if any.
    pub default_value: Option<String>,
    /// Whether the column is part of the primary key.
    pub primary_key: bool,
}

/// Table metadata including columns and statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DbTableInfo {
    /// Schema that owns this table.
    pub schema: String,
    /// Table name.
    pub name: String,
    /// Column definitions.
    pub columns: Vec<DbColumn>,
    /// Estimated row count from statistics (may be stale).
    pub row_count_estimate: Option<i64>,
    /// Total size on disk in bytes.
    pub size_bytes: Option<u64>,
}

/// Schema metadata including contained tables.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DbSchema {
    /// Schema name.
    pub name: String,
    /// Owning role name.
    pub owner: String,
    /// Tables in this schema.
    pub tables: Vec<DbTableInfo>,
}

/// Index metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct DbIndexInfo {
    /// Index name.
    pub name: String,
    /// Table the index covers.
    pub table: String,
    /// Ordered list of indexed column names.
    pub columns: Vec<String>,
    /// Whether the index enforces uniqueness.
    pub unique: bool,
    /// Access method: `"btree"`, `"hash"`, `"gin"`, `"gist"`, etc.
    pub index_type: String,
}

/// Role / user metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct DbRoleInfo {
    /// Role name.
    pub name: String,
    /// Whether the role has superuser privileges.
    pub superuser: bool,
    /// Whether the role can log in.
    pub can_login: bool,
    /// Whether the role can create databases.
    pub can_create_db: bool,
    /// Whether the role can create other roles.
    pub can_create_role: bool,
}

/// Active session info from `pg_stat_activity`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct DbSessionInfo {
    /// Backend process ID.
    pub pid: i32,
    /// Application name.
    pub app_name: String,
    /// Connected database, if known.
    pub database: Option<String>,
    /// Session state: `"active"`, `"idle"`, `"idle in transaction"`, etc.
    pub state: String,
    /// Current or most recent query text.
    pub query: Option<String>,
    /// Duration of current state in milliseconds.
    pub duration_ms: Option<u64>,
}

/// Aggregate session activity from `pg_stat_activity`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DbStatActivity {
    /// All tracked sessions.
    pub sessions: Vec<DbSessionInfo>,
    /// Count of idle sessions.
    pub idle_count: usize,
    /// Count of active sessions.
    pub active_count: usize,
    /// Count of sessions idle in a transaction.
    pub idle_in_tx_count: usize,
}

/// Result of `EXPLAIN [ANALYZE]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DbExplain {
    /// Full plan text.
    pub plan: String,
    /// Estimated startup cost.
    pub startup_cost: Option<f64>,
    /// Estimated total cost.
    pub total_cost: Option<f64>,
    /// Actual row count (only with ANALYZE).
    pub actual_rows: Option<i64>,
    /// Actual execution time in milliseconds (only with ANALYZE).
    pub actual_time_ms: Option<f64>,
}

/// Opaque transaction handle — passed to commit/rollback.
///
/// The actual connection state lives in the implementation.
/// This value is an implementation-defined identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct TransactionHandle(pub String);

/// ANSI/ISO transaction isolation level.
///
/// Source: ISO/IEC 9075-2 §4.32 — Transaction isolation levels
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum IsolationLevel {
    /// Allows dirty reads, non-repeatable reads, and phantom reads.
    #[display("READ UNCOMMITTED")]
    ReadUncommitted,
    /// Prevents dirty reads; allows non-repeatable reads and phantom reads.
    #[display("READ COMMITTED")]
    ReadCommitted,
    /// Prevents dirty and non-repeatable reads; allows phantom reads.
    #[display("REPEATABLE READ")]
    RepeatableRead,
    /// Prevents all anomalies: dirty reads, non-repeatable reads, and phantom reads.
    #[display("SERIALIZABLE")]
    Serializable,
}

/// Connection identifier returned by `connect()`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct ConnectionId(pub String);
