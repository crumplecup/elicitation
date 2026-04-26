//! Archive descriptor types — `ElicitComplete` data model for database objects.
//!
//! All types implement `Serialize`, `Deserialize`, `JsonSchema`, and `Elicit`
//! (the derive macro that produces proof methods), satisfying the full
//! `ElicitComplete` contract.  They are wire-safe for MCP tool responses and
//! can be dropped into tool call chains as first-class values.

use elicitation::Elicit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use elicit_db::{DbIndexInfo, DbRoleInfo, DbRows, DbSessionInfo, DbTableInfo};

use chrono::{DateTime, Utc};

// ── BackendKind ───────────────────────────────────────────────────────────────

/// Database backend detected from the connection URL.
#[cfg_attr(kani, derive(kani::Arbitrary))]
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
    Elicit,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum BackendKind {
    /// PostgreSQL / CockroachDB / compatible.
    #[display("PostgreSQL")]
    Postgres,
    /// SQLite (file-based).
    #[display("SQLite")]
    Sqlite,
    /// MySQL / MariaDB / compatible.
    #[display("MySQL")]
    Mysql,
    /// Embedded key-value store (redb file path or `redb://…`).
    #[display("redb")]
    Redb,
    /// Backend could not be identified from the connection URL.
    #[display("Unknown")]
    Unknown,
}

impl BackendKind {
    /// Infer the backend from the scheme prefix of a connection URL.
    pub fn from_url(url: &str) -> Self {
        let lower = url.to_ascii_lowercase();
        if lower.starts_with("postgres") || lower.starts_with("pg") {
            Self::Postgres
        } else if lower.starts_with("sqlite") {
            Self::Sqlite
        } else if lower.starts_with("mysql") || lower.starts_with("mariadb") {
            Self::Mysql
        } else if lower.starts_with("redb") {
            Self::Redb
        } else {
            Self::Unknown
        }
    }
}

// ── TableType ─────────────────────────────────────────────────────────────────

/// Distinguishes tables from views and materialised views.
#[cfg_attr(kani, derive(kani::Arbitrary))]
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
    Elicit,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum TableType {
    /// A regular base table.
    #[display("TABLE")]
    Table,
    /// A non-materialised view.
    #[display("VIEW")]
    View,
    /// A materialised view (PostgreSQL).
    #[display("MATERIALIZED VIEW")]
    MaterializedView,
    /// Table type not recognised.
    #[display("UNKNOWN")]
    Unknown,
}

impl TableType {
    /// Parse the `table_type` column returned by `information_schema.tables`.
    pub fn from_information_schema(s: &str) -> Self {
        match s.to_uppercase().trim() {
            "BASE TABLE" | "TABLE" => Self::Table,
            "VIEW" => Self::View,
            "MATERIALIZED VIEW" => Self::MaterializedView,
            _ => Self::Unknown,
        }
    }
}

// ── ColumnDescriptor ──────────────────────────────────────────────────────────

/// Descriptor for a single database column, including spatial detection.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ColumnDescriptor {
    /// Column name.
    pub name: String,
    /// SQL data type as reported by `information_schema.columns`.
    pub sql_type: String,
    /// Whether the column accepts `NULL`.
    pub nullable: bool,
    /// Whether the column is (part of) the primary key.
    pub is_primary_key: bool,
    /// Whether the column is a foreign key.
    pub is_foreign_key: bool,
    /// Default value expression, if any.
    pub default_value: Option<String>,
    /// `true` when `sql_type` indicates a PostGIS geometry / geography column
    /// or any other well-known spatial type.
    pub is_spatial: bool,
}

#[cfg(kani)]
impl kani::Arbitrary for ColumnDescriptor {
    fn any() -> Self {
        Self {
            name: String::new(),
            sql_type: String::new(),
            nullable: kani::any(),
            is_primary_key: kani::any(),
            is_foreign_key: kani::any(),
            default_value: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
            is_spatial: kani::any(),
        }
    }
}

impl ColumnDescriptor {
    /// Detect spatial columns from their SQL type name.
    pub fn is_spatial_type(sql_type: &str) -> bool {
        let lower = sql_type.to_ascii_lowercase();
        matches!(
            lower.trim(),
            "geometry"
                | "geography"
                | "point"
                | "multipoint"
                | "linestring"
                | "multilinestring"
                | "polygon"
                | "multipolygon"
                | "geometrycollection"
        ) || lower.contains("geometry")
            || lower.contains("geography")
    }

    /// Build from a [`elicit_db::DbColumn`], inferring `is_spatial` and
    /// leaving foreign-key status as `false` (callers must set it from
    /// `information_schema.key_column_usage`).
    pub fn from_db_column(col: &elicit_db::DbColumn) -> Self {
        Self {
            is_spatial: Self::is_spatial_type(&col.ty),
            name: col.name.clone(),
            sql_type: col.ty.clone(),
            nullable: col.nullable,
            is_primary_key: col.primary_key,
            is_foreign_key: false,
            default_value: col.default_value.clone(),
        }
    }
}

// ── TableDescriptor ───────────────────────────────────────────────────────────

/// Descriptor for a database table or view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct TableDescriptor {
    /// Owning schema name.
    pub schema: String,
    /// Table / view name.
    pub table_name: String,
    /// Column metadata.
    pub columns: Vec<ColumnDescriptor>,
    /// Estimated row count from database statistics (may be stale).
    pub estimated_rows: Option<i64>,
    /// Object type: table, view, or materialised view.
    pub table_type: TableType,
}

#[cfg(kani)]
impl kani::Arbitrary for TableDescriptor {
    fn any() -> Self {
        Self {
            schema: String::new(),
            table_name: String::new(),
            columns: Vec::new(),
            estimated_rows: kani::any(),
            table_type: kani::any(),
        }
    }
}

impl TableDescriptor {
    /// Construct from a [`elicit_db::DbTableInfo`].
    pub fn from_db_table_info(info: &DbTableInfo) -> Self {
        Self {
            schema: info.schema.clone(),
            table_name: info.name.clone(),
            columns: info
                .columns
                .iter()
                .map(ColumnDescriptor::from_db_column)
                .collect(),
            estimated_rows: info.row_count_estimate,
            table_type: TableType::Table,
        }
    }

    /// Returns only columns whose `is_spatial` flag is set.
    pub fn spatial_columns(&self) -> Vec<&ColumnDescriptor> {
        self.columns.iter().filter(|c| c.is_spatial).collect()
    }

    /// Returns `true` if any column in this table is a spatial type.
    pub fn has_spatial(&self) -> bool {
        self.columns.iter().any(|c| c.is_spatial)
    }
}

// ── SchemaDescriptor ──────────────────────────────────────────────────────────

/// Descriptor for a database schema and its contained tables.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct SchemaDescriptor {
    /// Schema name.
    pub schema_name: String,
    /// Role that owns the schema.
    pub owner: String,
    /// Names of tables in this schema (lazily populated; may be empty).
    pub table_names: Vec<String>,
}

#[cfg(kani)]
impl kani::Arbitrary for SchemaDescriptor {
    fn any() -> Self {
        Self {
            schema_name: String::new(),
            owner: String::new(),
            table_names: Vec::new(),
        }
    }
}

impl SchemaDescriptor {
    /// Construct from a [`elicit_db::DbSchema`].
    pub fn from_db_schema(schema: &elicit_db::DbSchema) -> Self {
        Self {
            schema_name: schema.name.clone(),
            owner: schema.owner.clone(),
            table_names: schema.tables.iter().map(|t| t.name.clone()).collect(),
        }
    }
}

// ── DatabaseDescriptor ────────────────────────────────────────────────────────

/// Top-level descriptor for a connected database.
///
/// The raw connection URL is **never** stored; only a stable hash is kept so
/// that descriptors are safe to serialise and log.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct DatabaseDescriptor {
    /// Stable identifier derived from the connection URL (not the URL itself).
    pub connection_id: String,
    /// Database name.
    pub db_name: String,
    /// Server version string, if available.
    pub version: Option<String>,
    /// Backend technology detected from the connection URL.
    pub backend: BackendKind,
}

#[cfg(kani)]
impl kani::Arbitrary for DatabaseDescriptor {
    fn any() -> Self {
        Self {
            connection_id: String::new(),
            db_name: String::new(),
            version: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
            backend: kani::any(),
        }
    }
}

impl DatabaseDescriptor {
    /// Construct a descriptor, hashing `url` to produce the `connection_id`.
    pub fn new(url: &str, db_name: impl Into<String>, version: Option<String>) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        url.hash(&mut h);
        Self {
            connection_id: format!("{:x}", h.finish()),
            db_name: db_name.into(),
            version,
            backend: BackendKind::from_url(url),
        }
    }
}

// ── FkAction ──────────────────────────────────────────────────────────────────

/// Referential action for a foreign key constraint.
#[cfg_attr(kani, derive(kani::Arbitrary))]
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
    Elicit,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum FkAction {
    /// Cascade the change to referencing rows.
    Cascade,
    /// Set referencing columns to `NULL`.
    SetNull,
    /// Prevent the action if referencing rows exist.
    Restrict,
    /// Prevent the action after the statement completes.
    NoAction,
    /// Set referencing columns to their defaults.
    SetDefault,
}

impl FkAction {
    /// Parse from the `information_schema.referential_constraints` rule string.
    pub fn from_rule(rule: &str) -> Self {
        match rule {
            "CASCADE" => Self::Cascade,
            "SET NULL" => Self::SetNull,
            "RESTRICT" => Self::Restrict,
            "SET DEFAULT" => Self::SetDefault,
            _ => Self::NoAction,
        }
    }
}

// ── ForeignKeyDescriptor ──────────────────────────────────────────────────────

/// A single foreign-key constraint (one from-column → one to-column mapping).
///
/// Multi-column FK constraints are represented as multiple descriptors sharing
/// the same `constraint_name`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ForeignKeyDescriptor {
    /// Constraint name in the database.
    pub constraint_name: String,
    /// Source column in the current table.
    pub from_column: String,
    /// Target schema.
    pub to_schema: String,
    /// Target table.
    pub to_table: String,
    /// Target column.
    pub to_column: String,
    /// Action on `DELETE` of the referenced row.
    pub on_delete: FkAction,
    /// Action on `UPDATE` of the referenced row.
    pub on_update: FkAction,
}

#[cfg(kani)]
impl kani::Arbitrary for ForeignKeyDescriptor {
    fn any() -> Self {
        Self {
            constraint_name: String::new(),
            from_column: String::new(),
            to_schema: String::new(),
            to_table: String::new(),
            to_column: String::new(),
            on_delete: kani::any(),
            on_update: kani::any(),
        }
    }
}

// ── ConstraintKind ────────────────────────────────────────────────────────────

/// Discriminator for database constraint types.
#[cfg_attr(kani, derive(kani::Arbitrary))]
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
    Elicit,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum ConstraintKind {
    /// Primary key constraint.
    PrimaryKey,
    /// Foreign key constraint.
    ForeignKey,
    /// Unique constraint.
    Unique,
    /// Check constraint.
    Check,
    /// Exclusion constraint (PostgreSQL-specific).
    Exclusion,
}

impl ConstraintKind {
    /// Parse from `information_schema.table_constraints.constraint_type`.
    pub fn from_pg_type(s: &str) -> Self {
        match s {
            "PRIMARY KEY" => Self::PrimaryKey,
            "FOREIGN KEY" => Self::ForeignKey,
            "UNIQUE" => Self::Unique,
            "CHECK" => Self::Check,
            "EXCLUSION" => Self::Exclusion,
            _ => Self::Check,
        }
    }
}

// ── ConstraintDescriptor ──────────────────────────────────────────────────────

/// Descriptor for a single table constraint.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ConstraintDescriptor {
    /// Constraint name.
    pub name: String,
    /// Constraint type.
    pub kind: ConstraintKind,
    /// Columns covered by the constraint (ordered).
    pub columns: Vec<String>,
    /// For CHECK constraints, the check expression; `None` otherwise.
    pub definition: Option<String>,
}

#[cfg(kani)]
impl kani::Arbitrary for ConstraintDescriptor {
    fn any() -> Self {
        Self {
            name: String::new(),
            kind: kani::any(),
            columns: Vec::new(),
            definition: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
        }
    }
}

// ── DdlDescriptor ─────────────────────────────────────────────────────────────

/// The DDL text for a schema object (table, view, index, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct DdlDescriptor {
    /// Schema containing the object.
    pub schema: String,
    /// Object name.
    pub object_name: String,
    /// Full DDL text (e.g. `CREATE TABLE …`).
    pub ddl: String,
}

#[cfg(kani)]
impl kani::Arbitrary for DdlDescriptor {
    fn any() -> Self {
        Self {
            schema: String::new(),
            object_name: String::new(),
            ddl: String::new(),
        }
    }
}

// ── TableInspection ───────────────────────────────────────────────────────────

/// Rich metadata for a table fetched on demand (not loaded at nav-tree time).
///
/// Loaded lazily when the user selects a table node. Stored in
/// `ArchiveNavModel::table_inspections`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct TableInspection {
    /// Foreign keys defined on this table.
    pub foreign_keys: Vec<ForeignKeyDescriptor>,
    /// All constraints on this table.
    pub constraints: Vec<ConstraintDescriptor>,
    /// All indexes on this table.
    pub indexes: Vec<IndexDescriptor>,
}

#[cfg(kani)]
impl kani::Arbitrary for TableInspection {
    fn any() -> Self {
        Self {
            foreign_keys: Vec::new(),
            constraints: Vec::new(),
            indexes: Vec::new(),
        }
    }
}

impl TableInspection {
    /// Returns `true` when no FK, constraint, or index data was found.
    pub fn is_empty(&self) -> bool {
        self.foreign_keys.is_empty() && self.constraints.is_empty() && self.indexes.is_empty()
    }
}

/// Descriptor for a database index.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct IndexDescriptor {
    /// Index name.
    pub index_name: String,
    /// Schema that contains the indexed table.
    pub schema: String,
    /// Table the index covers.
    pub table_name: String,
    /// Ordered list of indexed column names.
    pub column_names: Vec<String>,
    /// Whether the index enforces uniqueness.
    pub is_unique: bool,
    /// Access method: `"btree"`, `"hash"`, `"gin"`, `"gist"`, etc.
    pub index_method: String,
}

#[cfg(kani)]
impl kani::Arbitrary for IndexDescriptor {
    fn any() -> Self {
        Self {
            index_name: String::new(),
            schema: String::new(),
            table_name: String::new(),
            column_names: Vec::new(),
            is_unique: kani::any(),
            index_method: String::new(),
        }
    }
}

impl IndexDescriptor {
    /// Construct from a [`elicit_db::DbIndexInfo`] and a schema name.
    pub fn from_db_index_info(info: &DbIndexInfo, schema: impl Into<String>) -> Self {
        Self {
            index_name: info.name.clone(),
            schema: schema.into(),
            table_name: info.table.clone(),
            column_names: info.columns.clone(),
            is_unique: info.unique,
            index_method: info.index_type.clone(),
        }
    }
}

// ── ColumnStats ───────────────────────────────────────────────────────────────

/// PostgreSQL planner statistics for a single column (from `pg_stats`).
///
/// Only populated for PostgreSQL backends; other backends return empty stats.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ColumnStats {
    /// Column name.
    pub column_name: String,
    /// Fraction of NULL values in the column (0.0–1.0).
    pub null_fraction: f64,
    /// Average storage size of a non-null value in bytes.
    pub avg_width_bytes: i32,
    /// Estimated number of distinct values.
    ///
    /// Positive: absolute count.  Negative: fraction of total rows (e.g. `-0.5`
    /// means 50% of rows are distinct).  `0` means unknown.
    pub n_distinct: f64,
    /// Correlation between physical and logical order (-1.0 to 1.0).
    ///
    /// Values near ±1 mean sequential scans will be efficient.  Near 0 means
    /// heap fetches will be scattered (index scan may be slower than seq scan).
    pub correlation: Option<f64>,
}

#[cfg(kani)]
impl kani::Arbitrary for ColumnStats {
    fn any() -> Self {
        Self {
            column_name: String::new(),
            null_fraction: kani::any(),
            avg_width_bytes: kani::any(),
            n_distinct: kani::any(),
            correlation: kani::any(),
        }
    }
}

// ── ExplainNode ───────────────────────────────────────────────────────────────

/// One node in a PostgreSQL `EXPLAIN (FORMAT JSON)` plan tree.
///
/// Populated by parsing the JSON array returned by PostgreSQL.
/// Nesting mirrors the `Plans` arrays in the EXPLAIN output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ExplainNode {
    /// Node type, e.g. `"Seq Scan"`, `"Hash Join"`, `"Index Scan"`.
    pub node_type: String,
    /// Relation name for scan nodes.
    pub relation_name: Option<String>,
    /// Alias used in the query.
    pub alias: Option<String>,
    /// Estimated startup cost.
    pub startup_cost: f64,
    /// Estimated total cost.
    pub total_cost: f64,
    /// Estimated output rows.
    pub plan_rows: i64,
    /// Estimated average output row width in bytes.
    pub plan_width: i32,
    /// Actual startup time in ms (`EXPLAIN ANALYZE` only).
    pub actual_startup_time: Option<f64>,
    /// Actual total time in ms (`EXPLAIN ANALYZE` only).
    pub actual_total_time: Option<f64>,
    /// Actual rows output.
    pub actual_rows: Option<i64>,
    /// Number of loops executed.
    pub actual_loops: Option<i64>,
    /// Child plan nodes (skipped during elicitation; populated from database JSON).
    #[skip]
    pub children: Vec<ExplainNode>,
}

#[cfg(kani)]
impl kani::Arbitrary for ExplainNode {
    fn any() -> Self {
        Self {
            node_type: String::new(),
            relation_name: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
            alias: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
            startup_cost: kani::any(),
            total_cost: kani::any(),
            plan_rows: kani::any(),
            plan_width: kani::any(),
            actual_startup_time: kani::any(),
            actual_total_time: kani::any(),
            actual_rows: kani::any(),
            actual_loops: kani::any(),
            children: Vec::new(),
        }
    }
}

impl ExplainNode {
    /// Parse from a single EXPLAIN JSON plan object.
    pub fn from_json(v: &serde_json::Value) -> Self {
        let get_str = |k: &str| v[k].as_str().map(str::to_string);
        let get_f64 = |k: &str| v[k].as_f64().unwrap_or(0.0);
        let get_i64 = |k: &str| v[k].as_i64().unwrap_or(0);
        let get_i32 = |k: &str| v[k].as_i64().unwrap_or(0) as i32;

        let children = v["Plans"]
            .as_array()
            .map(|arr| arr.iter().map(ExplainNode::from_json).collect())
            .unwrap_or_default();

        Self {
            node_type: get_str("Node Type").unwrap_or_else(|| "Unknown".to_string()),
            relation_name: get_str("Relation Name"),
            alias: get_str("Alias"),
            startup_cost: get_f64("Startup Cost"),
            total_cost: get_f64("Total Cost"),
            plan_rows: get_i64("Plan Rows"),
            plan_width: get_i32("Plan Width"),
            actual_startup_time: v["Actual Startup Time"].as_f64(),
            actual_total_time: v["Actual Total Time"].as_f64(),
            actual_rows: v["Actual Rows"].as_i64(),
            actual_loops: v["Actual Loops"].as_i64(),
            children,
        }
    }

    /// Parse from the top-level EXPLAIN JSON array (first element's "Plan").
    pub fn parse_explain_output(json: &str) -> Result<Self, String> {
        let val: serde_json::Value =
            serde_json::from_str(json).map_err(|e| format!("JSON parse error: {e}"))?;
        let plan = &val[0]["Plan"];
        if plan.is_null() {
            return Err("EXPLAIN output missing 'Plan' key".to_string());
        }
        Ok(Self::from_json(plan))
    }
}

/// Side-by-side comparison of two EXPLAIN plans.
///
/// Built when the user runs a second EXPLAIN while a plan is already visible.
/// Cost-delta annotations (`▲`/`▼`) are computed at IR build time when the
/// root nodes' total costs diverge by more than 10 %.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ExplainComparison {
    /// Left (original) plan root.
    pub left: ExplainNode,
    /// Right (new) plan root.
    pub right: ExplainNode,
    /// Human-readable label for the left plan.
    pub label_left: String,
    /// Human-readable label for the right plan.
    pub label_right: String,
}

#[cfg(kani)]
impl kani::Arbitrary for ExplainComparison {
    fn any() -> Self {
        Self {
            left: kani::any::<ExplainNode>(),
            right: kani::any::<ExplainNode>(),
            label_left: String::new(),
            label_right: String::new(),
        }
    }
}

/// The result of executing a SQL query: column metadata + row data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct QueryResult {
    /// Column descriptors for the result set.
    pub columns: Vec<ColumnDescriptor>,
    /// Raw row data from the database.
    pub rows: DbRows,
    /// Number of rows returned or affected.
    pub row_count: u64,
    /// Column names where `is_spatial = true` (pre-computed for display routing).
    pub spatial_column_names: Vec<String>,
}

#[cfg(kani)]
impl kani::Arbitrary for QueryResult {
    fn any() -> Self {
        Self {
            columns: Vec::new(),
            rows: kani::any::<DbRows>(),
            row_count: kani::any(),
            spatial_column_names: Vec::new(),
        }
    }
}

impl QueryResult {
    /// Build a `QueryResult` from column descriptors and raw rows.
    pub fn new(columns: Vec<ColumnDescriptor>, rows: DbRows) -> Self {
        let row_count = rows.affected;
        let spatial_column_names = columns
            .iter()
            .filter(|c| c.is_spatial)
            .map(|c| c.name.clone())
            .collect();
        Self {
            columns,
            rows,
            row_count,
            spatial_column_names,
        }
    }

    /// Returns `true` when at least one column is a spatial type.
    pub fn has_spatial(&self) -> bool {
        !self.spatial_column_names.is_empty()
    }
}

// ── Export ────────────────────────────────────────────────────────────────────

/// Output format for data export.
#[cfg_attr(kani, derive(kani::Arbitrary))]
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
    Elicit,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum ExportFormat {
    /// Comma-separated values with header row.
    #[display("csv")]
    Csv,
    /// JSON array of objects.
    #[display("json")]
    Json,
    /// Newline-delimited JSON (one object per line).
    #[display("ndjson")]
    Ndjson,
    /// Tab-separated values with header row.
    #[display("tsv")]
    Tsv,
}

impl ExportFormat {
    /// File extension for this format.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Csv => "csv",
            Self::Tsv => "tsv",
            Self::Json => "json",
            Self::Ndjson => "ndjson",
        }
    }
}

/// Result of a data export operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ExportResult {
    /// Format used.
    pub format: ExportFormat,
    /// Number of rows written.
    pub row_count: u64,
    /// Exported content.
    pub content: String,
}

#[cfg(kani)]
impl kani::Arbitrary for ExportResult {
    fn any() -> Self {
        Self {
            format: kani::any(),
            row_count: kani::any(),
            content: String::new(),
        }
    }
}

// ── QueryHistoryEntry ─────────────────────────────────────────────────────────

/// A single entry in the persistent query history log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct QueryHistoryEntry {
    /// Auto-increment row ID.
    pub id: i64,
    /// UTC timestamp when the query was executed.
    pub executed_at: DateTime<Utc>,
    /// The SQL text that was executed.
    pub sql: String,
    /// Wall-clock duration in milliseconds.
    pub duration_ms: u64,
    /// Number of rows returned or affected (None on error).
    pub row_count: Option<u64>,
    /// Error message if the query failed.
    pub error: Option<String>,
}

#[cfg(kani)]
impl kani::Arbitrary for QueryHistoryEntry {
    fn any() -> Self {
        Self {
            id: kani::any(),
            executed_at: chrono::DateTime::UNIX_EPOCH,
            sql: String::new(),
            duration_ms: kani::any(),
            row_count: kani::any(),
            error: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
        }
    }
}

/// A user-saved SQL snippet stored in the local SQLite database.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct SavedQuery {
    /// Auto-increment row ID.
    pub id: i64,
    /// User-assigned name for this snippet.
    pub name: String,
    /// The SQL text.
    pub sql: String,
    /// UTC timestamp when first saved.
    pub created_at: DateTime<Utc>,
    /// UTC timestamp of the most recent update.
    pub updated_at: DateTime<Utc>,
}

#[cfg(kani)]
impl kani::Arbitrary for SavedQuery {
    fn any() -> Self {
        Self {
            id: kani::any(),
            name: String::new(),
            sql: String::new(),
            created_at: chrono::DateTime::UNIX_EPOCH,
            updated_at: chrono::DateTime::UNIX_EPOCH,
        }
    }
}

// ── Phase 3.1 — Inline Row Edit ───────────────────────────────────────────────

/// The kind of mutation in a [`StagedEdit`].
///
/// Values for `pk_values` and `row` fields are serialised as `String`; callers
/// must pass `"NULL"` (the four-character literal) to represent SQL `NULL`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum RowEditKind {
    /// Update a single cell identified by the row's primary-key values.
    Update {
        /// `(column_name, serialised_value)` pairs identifying the row.
        pk_values: Vec<(String, String)>,
        /// The column whose value is being changed.
        column: String,
        /// New serialised value (use `"NULL"` for SQL NULL).
        new_value: String,
    },
    /// Insert a new row.
    Insert {
        /// `(column_name, serialised_value)` pairs for every column.
        row: Vec<(String, String)>,
    },
    /// Delete the row identified by its primary-key values.
    Delete {
        /// `(column_name, serialised_value)` pairs identifying the row.
        pk_values: Vec<(String, String)>,
    },
}

#[cfg(kani)]
impl kani::Arbitrary for RowEditKind {
    fn any() -> Self {
        Self::Delete {
            pk_values: Vec::new(),
        }
    }
}

/// A single row mutation staged for a future transactional commit.
///
/// Serialisable so that the `archive_query__edit_row` MCP tool can accept a
/// batch of staged edits as JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct StagedEdit {
    /// Schema that owns the target table.
    pub schema: String,
    /// Target table name.
    pub table: String,
    /// The mutation to apply.
    pub kind: RowEditKind,
}

#[cfg(kani)]
impl kani::Arbitrary for StagedEdit {
    fn any() -> Self {
        Self {
            schema: String::new(),
            table: String::new(),
            kind: kani::any::<RowEditKind>(),
        }
    }
}

/// Transient UI state for an active in-grid row-edit session.
///
/// Lives inside `ArchivePanelState::DataGrid::edit_state` and is `None` when
/// no edit session is active.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct RowEditState {
    /// Mutations staged but not yet committed (ready to send to the tool).
    pub pending_edits: Vec<StagedEdit>,
    /// `(row_index, col_index)` of the cell currently being typed into.
    pub editing_cell: Option<(usize, usize)>,
    /// Character buffer accumulating the in-progress cell value.
    pub input_buffer: String,
    /// Row indices (within the current page) marked for deletion.
    pub rows_marked_deleted: Vec<usize>,
    /// New-row form being filled in: `(column_name, typed_value)` pairs.
    pub inserting_row: Option<Vec<(String, String)>>,
    /// Which column the cursor is on within the new-row insertion form.
    pub insert_col_cursor: usize,
}

#[cfg(kani)]
impl kani::Arbitrary for RowEditState {
    fn any() -> Self {
        Self {
            pending_edits: Vec::new(),
            editing_cell: None,
            input_buffer: String::new(),
            rows_marked_deleted: Vec::new(),
            inserting_row: None,
            insert_col_cursor: kani::any(),
        }
    }
}

impl RowEditState {
    /// Create a fresh edit session with no pending mutations.
    pub fn new() -> Self {
        Self {
            pending_edits: Vec::new(),
            editing_cell: None,
            input_buffer: String::new(),
            rows_marked_deleted: Vec::new(),
            inserting_row: None,
            insert_col_cursor: 0,
        }
    }

    /// Return `true` if there are any un-committed mutations or active input.
    pub fn is_dirty(&self) -> bool {
        !self.pending_edits.is_empty()
            || self.editing_cell.is_some()
            || !self.rows_marked_deleted.is_empty()
            || self.inserting_row.is_some()
    }
}

impl Default for RowEditState {
    fn default() -> Self {
        Self::new()
    }
}

// ── Phase 3.5 — Multi-Connection ─────────────────────────────────────────────

/// A named database connection profile.
///
/// The raw connection URL is never stored directly; instead, `url_env_key`
/// names the environment variable that holds it (e.g. `"DATABASE_URL"`).
/// At runtime, [`ConnectionSet`] resolves the key via `std::env::var`.
/// SSL connection mode, matching PostgreSQL `sslmode` parameter semantics.
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum SslMode {
    /// Never use SSL.
    #[display("disable")]
    Disable,
    /// Try non-SSL first, fall back to SSL.
    #[display("allow")]
    Allow,
    /// Try SSL first, fall back to non-SSL.
    #[default]
    #[display("prefer")]
    Prefer,
    /// Always use SSL; do not verify certificate.
    #[display("require")]
    Require,
    /// Always use SSL; verify CA certificate.
    #[display("verify-ca")]
    VerifyCa,
    /// Always use SSL; verify CA certificate and hostname.
    #[display("verify-full")]
    VerifyFull,
}

///
/// [`ConnectionSet`]: crate::archive::nav_model::ConnectionSet
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ConnectionProfile {
    /// Human-visible label shown in the tab bar.
    pub name: String,
    /// Environment variable whose value is the connection URL.
    /// Pass the URL string directly here if not using env-var indirection.
    pub url_env_key: String,
    /// Database backend type (drives icon, dialect hints, etc.).
    pub backend: BackendKind,
    /// Optional Catppuccin accent colour for the tab badge (e.g. `"blue"`).
    pub color: Option<String>,

    // ── SSH tunnel (all optional; tunnel is active when `ssh_host` is set) ──
    /// Hostname or IP of the SSH bastion / jump host.
    pub ssh_host: Option<String>,
    /// SSH port on the bastion (default 22 when `None`).
    pub ssh_port: Option<u16>,
    /// SSH username on the bastion.
    pub ssh_user: Option<String>,
    /// Environment variable naming the path to the SSH private key file.
    /// The actual key path stays in the environment; never stored here.
    pub ssh_key_env: Option<String>,

    // ── SSL ──────────────────────────────────────────────────────────────────
    /// PostgreSQL `sslmode` setting.
    pub ssl_mode: SslMode,
    /// Env var naming the client certificate file path (PEM).
    pub ssl_cert_env: Option<String>,
    /// Env var naming the client private key file path (PEM).
    pub ssl_key_env: Option<String>,
    /// Env var naming the CA certificate bundle path (PEM).
    pub ssl_ca_env: Option<String>,
}

#[cfg(kani)]
impl kani::Arbitrary for ConnectionProfile {
    fn any() -> Self {
        Self {
            name: String::new(),
            url_env_key: String::new(),
            backend: kani::any(),
            color: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
            ssh_host: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
            ssh_port: kani::any(),
            ssh_user: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
            ssh_key_env: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
            ssl_mode: kani::any(),
            ssl_cert_env: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
            ssl_key_env: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
            ssl_ca_env: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
        }
    }
}

// ── Phase 4 — Advanced Object Types ──────────────────────────────────────────

/// Volatility classification for PostgreSQL functions.
#[cfg_attr(kani, derive(kani::Arbitrary))]
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
    Elicit,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum FunctionVolatility {
    /// Result depends only on arguments (safe for optimisation).
    #[display("immutable")]
    Immutable,
    /// Result may vary between calls within the same statement.
    #[display("stable")]
    Stable,
    /// Result may change across any call (default for most functions).
    #[display("volatile")]
    Volatile,
}

/// A PostgreSQL function or stored procedure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct FunctionDescriptor {
    /// OID of the function in `pg_proc`.
    pub oid: i64,
    /// Containing schema.
    pub schema: String,
    /// Function name.
    pub name: String,
    /// Full SQL argument list (e.g. `"IN x integer, IN y text"`).
    pub arguments: String,
    /// Return type declaration (e.g. `"TABLE(id integer, name text)"`).
    pub return_type: String,
    /// Implementation language (`plpgsql`, `sql`, `c`, …).
    pub language: String,
    /// Volatility classification.
    pub volatility: FunctionVolatility,
    /// Whether this is a procedure (no return value).
    pub is_procedure: bool,
    /// First 512 characters of the function body for quick inspection.
    pub body_preview: String,
}

#[cfg(kani)]
impl kani::Arbitrary for FunctionDescriptor {
    fn any() -> Self {
        Self {
            oid: kani::any(),
            schema: String::new(),
            name: String::new(),
            arguments: String::new(),
            return_type: String::new(),
            language: String::new(),
            volatility: kani::any(),
            is_procedure: kani::any(),
            body_preview: String::new(),
        }
    }
}

/// The DML event(s) a trigger fires on.
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct TriggerEvents {
    /// Fires on `INSERT`.
    pub on_insert: bool,
    /// Fires on `UPDATE`.
    pub on_update: bool,
    /// Fires on `DELETE`.
    pub on_delete: bool,
    /// Fires on `TRUNCATE`.
    pub on_truncate: bool,
}

/// A PostgreSQL trigger attached to a table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct TriggerDescriptor {
    /// Containing schema.
    pub schema: String,
    /// Table the trigger is attached to.
    pub table: String,
    /// Trigger name.
    pub name: String,
    /// Fires `BEFORE` or `AFTER` (or `INSTEAD OF`).
    pub timing: String,
    /// Which DML events fire this trigger.
    pub events: TriggerEvents,
    /// Whether the trigger fires per-row (`true`) or per-statement (`false`).
    pub row_level: bool,
    /// Fully-qualified name of the trigger function.
    pub function: String,
    /// Whether the trigger is currently enabled.
    pub enabled: bool,
}

#[cfg(kani)]
impl kani::Arbitrary for TriggerDescriptor {
    fn any() -> Self {
        Self {
            schema: String::new(),
            table: String::new(),
            name: String::new(),
            timing: String::new(),
            events: kani::any(),
            row_level: kani::any(),
            function: String::new(),
            enabled: kani::any(),
        }
    }
}

/// A PostgreSQL sequence object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct SequenceDescriptor {
    /// Containing schema.
    pub schema: String,
    /// Sequence name.
    pub name: String,
    /// Most recently returned value (may be `None` if never called).
    pub current_value: Option<i64>,
    /// Starting value.
    pub start_value: i64,
    /// Increment per call.
    pub increment_by: i64,
    /// Minimum value.
    pub min_value: i64,
    /// Maximum value.
    pub max_value: i64,
    /// Whether the sequence wraps on overflow.
    pub cycle: bool,
    /// Table.column this sequence is owned by (if any).
    pub owned_by: Option<String>,
}

#[cfg(kani)]
impl kani::Arbitrary for SequenceDescriptor {
    fn any() -> Self {
        Self {
            schema: String::new(),
            name: String::new(),
            current_value: kani::any(),
            start_value: kani::any(),
            increment_by: kani::any(),
            min_value: kani::any(),
            max_value: kani::any(),
            cycle: kani::any(),
            owned_by: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
        }
    }
}

/// A PostgreSQL enum type with its ordered labels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct EnumDescriptor {
    /// Containing schema.
    pub schema: String,
    /// Type name.
    pub name: String,
    /// Ordered list of enum labels.
    pub labels: Vec<String>,
}

#[cfg(kani)]
impl kani::Arbitrary for EnumDescriptor {
    fn any() -> Self {
        Self {
            schema: String::new(),
            name: String::new(),
            labels: Vec::new(),
        }
    }
}

/// A PostgreSQL domain type (scalar with constraints).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct DomainDescriptor {
    /// Containing schema.
    pub schema: String,
    /// Domain name.
    pub name: String,
    /// Underlying base type (e.g. `"integer"`, `"text"`).
    pub base_type: String,
    /// `NOT NULL` constraint present.
    pub not_null: bool,
    /// Default expression if any.
    pub default_expr: Option<String>,
    /// CHECK constraint expressions.
    pub check_constraints: Vec<String>,
}

#[cfg(kani)]
impl kani::Arbitrary for DomainDescriptor {
    fn any() -> Self {
        Self {
            schema: String::new(),
            name: String::new(),
            base_type: String::new(),
            not_null: kani::any(),
            default_expr: if kani::any::<bool>() {
                Some(String::new())
            } else {
                None
            },
            check_constraints: Vec::new(),
        }
    }
}

/// One column of a PostgreSQL composite type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct CompositeTypeAttribute {
    /// Attribute (column) name.
    pub name: String,
    /// Data type name.
    pub type_name: String,
}

#[cfg(kani)]
impl kani::Arbitrary for CompositeTypeAttribute {
    fn any() -> Self {
        Self {
            name: String::new(),
            type_name: String::new(),
        }
    }
}

/// A PostgreSQL composite (record) type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct CompositeTypeDescriptor {
    /// Containing schema.
    pub schema: String,
    /// Type name.
    pub name: String,
    /// Ordered list of attributes.
    pub attributes: Vec<CompositeTypeAttribute>,
}

#[cfg(kani)]
impl kani::Arbitrary for CompositeTypeDescriptor {
    fn any() -> Self {
        Self {
            schema: String::new(),
            name: String::new(),
            attributes: Vec::new(),
        }
    }
}

// ── MonitorTab ────────────────────────────────────────────────────────────────

/// Which tab is active inside the monitor panel.
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub enum MonitorTab {
    /// Session activity (default view).
    #[default]
    Sessions,
    /// Slow queries exceeding a threshold.
    SlowQueries,
    /// Current lock-wait chains.
    LockWaits,
    /// Table bloat ratios.
    TableBloat,
    /// Index usage (scan counts).
    IndexUsage,
}

impl MonitorTab {
    /// Cycle forward.
    pub fn next(&self) -> Self {
        match self {
            Self::Sessions => Self::SlowQueries,
            Self::SlowQueries => Self::LockWaits,
            Self::LockWaits => Self::TableBloat,
            Self::TableBloat => Self::IndexUsage,
            Self::IndexUsage => Self::Sessions,
        }
    }

    /// Cycle backward.
    pub fn prev(&self) -> Self {
        match self {
            Self::Sessions => Self::IndexUsage,
            Self::SlowQueries => Self::Sessions,
            Self::LockWaits => Self::SlowQueries,
            Self::TableBloat => Self::LockWaits,
            Self::IndexUsage => Self::TableBloat,
        }
    }

    /// Human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Sessions => "Sessions",
            Self::SlowQueries => "Slow Queries",
            Self::LockWaits => "Lock Waits",
            Self::TableBloat => "Table Bloat",
            Self::IndexUsage => "Index Usage",
        }
    }
}

// ── MonitorSnapshot ───────────────────────────────────────────────────────────

/// A point-in-time snapshot of live database monitoring data.
///
/// Populated by `ArchiveMonitorPlugin` tools and cached in
/// `PanelMode::MonitorPanel`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct MonitorSnapshot {
    /// Active sessions from `pg_stat_activity`.
    pub sessions: Vec<DbSessionInfo>,
    /// Roles from `pg_roles`.
    pub roles: Vec<DbRoleInfo>,
    /// Buffer cache hit ratio (0.0–1.0), or `None` if not yet fetched.
    pub cache_hit: Option<f64>,
    /// Available backup labels.
    pub backups: Vec<String>,
    /// Sessions whose query duration exceeded the slow-query threshold.
    pub slow_queries: Vec<DbSessionInfo>,
    /// `(blocking_pid, blocked_pid)` pairs from current lock waits.
    pub lock_waits: Vec<(i32, i32)>,
    /// `(table_name, bloat_ratio)` pairs.
    pub table_bloat: Vec<(String, f64)>,
    /// `(index_name, scan_count)` pairs.
    pub index_usage: Vec<(String, u64)>,
    /// Currently active monitor tab.
    pub active_tab: MonitorTab,
}

#[cfg(kani)]
impl kani::Arbitrary for MonitorSnapshot {
    fn any() -> Self {
        Self {
            sessions: Vec::new(),
            roles: Vec::new(),
            cache_hit: kani::any(),
            backups: Vec::new(),
            slow_queries: Vec::new(),
            lock_waits: Vec::new(),
            table_bloat: Vec::new(),
            index_usage: Vec::new(),
            active_tab: kani::any(),
        }
    }
}

// ── AdminPanel types ──────────────────────────────────────────────────────────

/// Which tab is active inside the admin panel.
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub enum AdminTab {
    /// Role and privilege matrix.
    #[default]
    Roles,
    /// Backup inventory and WAL status.
    Backups,
    /// Server configuration settings and extensions.
    Settings,
}

impl AdminTab {
    /// Cycle forward through tabs: Roles → Backups → Settings → Roles.
    pub fn next(&self) -> Self {
        match self {
            Self::Roles => Self::Backups,
            Self::Backups => Self::Settings,
            Self::Settings => Self::Roles,
        }
    }

    /// Cycle backward through tabs: Roles → Settings → Backups → Roles.
    pub fn prev(&self) -> Self {
        match self {
            Self::Roles => Self::Settings,
            Self::Backups => Self::Roles,
            Self::Settings => Self::Backups,
        }
    }

    /// Human-readable label for display.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Roles => "Roles",
            Self::Backups => "Backups",
            Self::Settings => "Settings",
        }
    }
}

/// A point-in-time snapshot of database administration data.
///
/// Populated by `ArchiveAdminPlugin` tools and cached in
/// `PanelMode::AdminPanel`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct AdminSnapshot {
    /// All cluster roles from `pg_roles`.
    pub roles: Vec<DbRoleInfo>,
    /// Available backup labels.
    pub backups: Vec<String>,
    /// Whether WAL archiving is healthy (from `pg_backup_start` probe).
    pub wal_ready: bool,
    /// PostgreSQL server version string.
    pub server_version: String,
    /// Installed extension names from `pg_available_extensions`.
    pub extensions: Vec<String>,
    /// Top GUC settings (name → current value pairs).
    pub settings: Vec<(String, String)>,
    /// Currently active admin tab.
    pub active_tab: AdminTab,
}

#[cfg(kani)]
impl kani::Arbitrary for AdminSnapshot {
    fn any() -> Self {
        Self {
            roles: Vec::new(),
            backups: Vec::new(),
            wal_ready: kani::any(),
            server_version: String::new(),
            extensions: Vec::new(),
            settings: Vec::new(),
            active_tab: kani::any(),
        }
    }
}

// ── ERD types ─────────────────────────────────────────────────────────────────

/// A single column in an ERD node.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ErdColumn {
    /// Column name.
    pub name: String,
    /// SQL data type.
    pub sql_type: String,
    /// Whether the column is part of the primary key.
    pub is_pk: bool,
    /// Whether the column participates in a foreign key.
    pub is_fk: bool,
}

#[cfg(kani)]
impl kani::Arbitrary for ErdColumn {
    fn any() -> Self {
        Self {
            name: String::new(),
            sql_type: String::new(),
            is_pk: kani::any(),
            is_fk: kani::any(),
        }
    }
}

/// A table node in an ERD diagram.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ErdNode {
    /// Owning schema.
    pub schema: String,
    /// Table name.
    pub table: String,
    /// Ordered list of columns.
    pub columns: Vec<ErdColumn>,
}

#[cfg(kani)]
impl kani::Arbitrary for ErdNode {
    fn any() -> Self {
        Self {
            schema: String::new(),
            table: String::new(),
            columns: Vec::new(),
        }
    }
}

/// A directed foreign-key edge between two ERD nodes.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ErdEdge {
    /// Constraint name.
    pub constraint_name: String,
    /// Source schema.
    pub from_schema: String,
    /// Source table.
    pub from_table: String,
    /// Source column.
    pub from_column: String,
    /// Target schema.
    pub to_schema: String,
    /// Target table.
    pub to_table: String,
    /// Target column.
    pub to_column: String,
}

#[cfg(kani)]
impl kani::Arbitrary for ErdEdge {
    fn any() -> Self {
        Self {
            constraint_name: String::new(),
            from_schema: String::new(),
            from_table: String::new(),
            from_column: String::new(),
            to_schema: String::new(),
            to_table: String::new(),
            to_column: String::new(),
        }
    }
}

/// A complete entity-relationship diagram for a single schema.
///
/// Produced by [`fetch_erd`](crate::archive::nav_tree::fetch_erd) and
/// cached in [`PanelMode::ErdPanel`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ErdDiagram {
    /// Schema this diagram covers.
    pub schema: String,
    /// Table nodes (one per table in the schema).
    pub nodes: Vec<ErdNode>,
    /// FK edges between nodes.
    pub edges: Vec<ErdEdge>,
}

#[cfg(kani)]
impl kani::Arbitrary for ErdDiagram {
    fn any() -> Self {
        Self {
            schema: String::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

/// Spatial layout computed from an [`ErdDiagram`] for visual rendering.
///
/// Coordinates are in logical pixels (no DPI scaling).  The origin (0, 0)
/// is the top-left corner of the canvas.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ErdLayout {
    /// Canvas width needed to contain all boxes.
    pub canvas_w: f32,
    /// Canvas height needed to contain all boxes.
    pub canvas_h: f32,
    /// Per-table bounding box: (x, y, width, height).
    ///
    /// Key is `"schema.table"`.
    pub boxes: std::collections::HashMap<String, (f32, f32, f32, f32)>,
}

#[cfg(kani)]
impl kani::Arbitrary for ErdLayout {
    fn any() -> Self {
        Self {
            canvas_w: kani::any(),
            canvas_h: kani::any(),
            boxes: std::collections::HashMap::new(),
        }
    }
}

impl ErdLayout {
    /// Compute a simple grid layout for the given diagram.
    ///
    /// Tables are sorted alphabetically, then placed in a square-ish grid.
    /// Each box is 200 px wide and 80 + 20 × col_count px tall.
    /// Horizontal gap is 40 px; vertical gap is 40 px.
    pub fn from_diagram(diagram: &ErdDiagram) -> Self {
        let n = diagram.nodes.len();
        if n == 0 {
            return Self::default();
        }

        let cols = (n as f32).sqrt().ceil() as usize;
        let box_w: f32 = 200.0;
        let gap_x: f32 = 40.0;
        let gap_y: f32 = 40.0;
        let header_h: f32 = 32.0;
        let row_h: f32 = 20.0;
        let min_h: f32 = 80.0;
        let pad: f32 = 20.0; // outer canvas padding

        let mut sorted: Vec<&ErdNode> = diagram.nodes.iter().collect();
        sorted.sort_by(|a, b| a.table.cmp(&b.table));

        let mut boxes = std::collections::HashMap::new();
        let mut max_x: f32 = 0.0;
        let mut max_y: f32 = 0.0;

        for (i, node) in sorted.iter().enumerate() {
            let col = i % cols;
            let row = i / cols;

            // Height is proportional to the number of columns.
            let box_h = (header_h + node.columns.len() as f32 * row_h).max(min_h);
            let x = pad + col as f32 * (box_w + gap_x);
            let y = pad + row as f32 * (box_h + gap_y);

            let key = format!("{}.{}", node.schema, node.table);
            boxes.insert(key, (x, y, box_w, box_h));
            if x + box_w > max_x {
                max_x = x + box_w;
            }
            if y + box_h > max_y {
                max_y = y + box_h;
            }
        }

        Self {
            canvas_w: max_x + pad,
            canvas_h: max_y + pad,
            boxes,
        }
    }

    /// Centre-bottom point of a box (connection source/sink for FK lines).
    pub fn centre_bottom(&self, key: &str) -> Option<(f32, f32)> {
        self.boxes.get(key).map(|(x, y, w, h)| (x + w / 2.0, y + h))
    }

    /// Centre-top point of a box.
    pub fn centre_top(&self, key: &str) -> Option<(f32, f32)> {
        self.boxes.get(key).map(|(x, y, w, _h)| (x + w / 2.0, *y))
    }
}
