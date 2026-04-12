//! Archive descriptor types — `ElicitComplete` data model for database objects.
//!
//! All types implement `Serialize`, `Deserialize`, `JsonSchema`, and `Elicit`
//! (the derive macro that produces proof methods), satisfying the full
//! `ElicitComplete` contract.  They are wire-safe for MCP tool responses and
//! can be dropped into tool call chains as first-class values.

use elicitation::{Elicit, Prompt, Select};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use elicit_db::{DbIndexInfo, DbRows, DbTableInfo};

// ── BackendKind ───────────────────────────────────────────────────────────────

/// Database backend detected from the connection URL.
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
        } else {
            Self::Unknown
        }
    }
}

// ── TableType ─────────────────────────────────────────────────────────────────

/// Distinguishes tables from views and materialised views.
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

// ── IndexDescriptor ───────────────────────────────────────────────────────────

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

// ── QueryResult ───────────────────────────────────────────────────────────────

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
