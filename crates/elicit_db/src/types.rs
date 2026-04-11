//! Common data types used in database trait signatures.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{AuditLogged, Committed, DbResult, Durable, TransactionCommitted, TxMarker};
use elicitation::Established;

#[cfg(feature = "geo-types")]
use elicit_geo_types::Geometry as GeoTypesGeometry;
#[cfg(feature = "geo-types")]
use elicit_geojson::{GeoJson as ShadowGeoJson, Geometry as ShadowGeoJsonGeometry};
#[cfg(feature = "geo-types")]
use elicit_wkb::{WriteOptions as WkbWriteOptions, write_geometry};
#[cfg(feature = "geo-types")]
use elicit_wkt::trait_factories::{ToWktF64, TryFromWktF64};

/// A spatial payload encoded as WKT text or WKB bytes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum DbSpatialValue {
    /// Well-known text representation.
    Wkt(String),
    /// Well-known binary representation.
    Wkb(Vec<u8>),
}

#[cfg(feature = "geo-types")]
impl DbSpatialValue {
    /// Encodes a geometry as validated WKT text for database transport.
    #[tracing::instrument(skip(geom))]
    pub fn from_geo_as_wkt(geom: &elicitation::GeoGeometry) -> Self {
        let geom: geo_types::Geometry<f64> = geom.clone().into();
        Self::Wkt(geom.wkt_string())
    }

    /// Encodes a geometry as validated WKB bytes for database transport.
    #[tracing::instrument(skip(geom))]
    pub fn from_geo_as_wkb(geom: &elicitation::GeoGeometry) -> Result<Self, String> {
        let bytes =
            write_geometry(geom, &WkbWriteOptions::default()).map_err(|error| error.to_string())?;
        Ok(Self::Wkb(bytes.bytes))
    }

    /// Converts a WKT-backed spatial payload back into a `GeoGeometry`.
    ///
    /// WKB payloads remain transport-only until a faithful reverse conversion
    /// layer lands for the current shadow stack.
    #[tracing::instrument]
    pub fn try_to_geo_geometry(&self) -> Result<elicitation::GeoGeometry, String> {
        match self {
            Self::Wkt(text) => {
                let geom: geo_types::Geometry<f64> =
                    <geo_types::Geometry<f64> as TryFromWktF64>::try_from_wkt_str(text)?;
                Ok(geom.into())
            }
            Self::Wkb(_) => Err(
                "WKB spatial payloads are transport-only for now; use WKT when GeoGeometry roundtrip is required"
                    .to_string(),
            ),
        }
    }
}

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
    /// PostGIS `geometry` payload.
    Geometry(DbSpatialValue),
    /// PostGIS `geography` payload.
    Geography(DbSpatialValue),
}

#[cfg(feature = "geo-types")]
impl DbValue {
    /// Stores a geometry payload encoded as WKT.
    #[tracing::instrument(skip(geom))]
    pub fn geometry_from_geo_as_wkt(geom: &elicitation::GeoGeometry) -> Self {
        Self::Geometry(DbSpatialValue::from_geo_as_wkt(geom))
    }

    /// Stores a geometry payload encoded as WKB.
    #[tracing::instrument(skip(geom))]
    pub fn geometry_from_geo_as_wkb(geom: &elicitation::GeoGeometry) -> Result<Self, String> {
        Ok(Self::Geometry(DbSpatialValue::from_geo_as_wkb(geom)?))
    }

    /// Stores a geography payload encoded as WKT.
    #[tracing::instrument(skip(geom))]
    pub fn geography_from_geo_as_wkt(geom: &elicitation::GeoGeometry) -> Self {
        Self::Geography(DbSpatialValue::from_geo_as_wkt(geom))
    }

    /// Stores a geography payload encoded as WKB.
    #[tracing::instrument(skip(geom))]
    pub fn geography_from_geo_as_wkb(geom: &elicitation::GeoGeometry) -> Result<Self, String> {
        Ok(Self::Geography(DbSpatialValue::from_geo_as_wkb(geom)?))
    }

    /// Stores a GeoJSON document as a JSON/JSONB-style DB value.
    #[tracing::instrument(skip(geojson))]
    pub fn json_from_geojson(geojson: &ShadowGeoJson) -> Self {
        Self::Json(geojson.clone().to_json_value())
    }

    /// Stores a geometry as a GeoJSON geometry document inside a JSON/JSONB-style DB value.
    #[tracing::instrument(skip(geom))]
    pub fn json_from_geo_as_geojson(geom: &elicitation::GeoGeometry) -> Self {
        let geometry = GeoTypesGeometry::from(geom.clone());
        let document = ShadowGeoJson::from(ShadowGeoJsonGeometry::from(&geometry));
        Self::json_from_geojson(&document)
    }

    /// Attempts to recover a `GeoGeometry` from a spatial DB value.
    #[tracing::instrument]
    pub fn try_to_geo_geometry(&self) -> Result<elicitation::GeoGeometry, String> {
        match self {
            Self::Geometry(value) | Self::Geography(value) => value.try_to_geo_geometry(),
            _ => Err("DbValue is not a geometry/geography payload".to_string()),
        }
    }

    /// Attempts to recover a GeoJSON document from a JSON DB value.
    #[tracing::instrument]
    pub fn try_to_geojson(&self) -> Result<ShadowGeoJson, String> {
        match self {
            Self::Json(value) => {
                ShadowGeoJson::from_json_value(value.clone()).map_err(|error| error.to_string())
            }
            _ => Err("DbValue is not a JSON payload".to_string()),
        }
    }

    /// Attempts to recover a `GeoGeometry` from a JSON payload containing GeoJSON.
    #[tracing::instrument]
    pub fn try_json_to_geo_geometry(&self) -> Result<elicitation::GeoGeometry, String> {
        let document = self.try_to_geojson()?;
        let geometry: GeoTypesGeometry =
            GeoTypesGeometry::try_from(document).map_err(|error| error.to_string())?;
        Ok(geometry.as_ref().clone())
    }
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

/// Result shape for executing a statement plus audit confirmation.
pub type DbExecuteResult = DbResult<(u64, Established<AuditLogged>)>;

/// Result shape for fetching rows plus row-visibility confirmation.
pub type DbQueryRowsResult = DbResult<(DbRows, Established<crate::RowVisible>)>;

/// Result shape for an auto-managed transactional execute.
pub type DbTransactionalExecuteResult = DbResult<(
    u64,
    Established<TransactionCommitted>,
    Established<AuditLogged>,
)>;

/// Result shape for a durable commit of an explicit transaction.
pub type DbCommitResult = DbResult<(
    TxMarker<Committed>,
    Established<TransactionCommitted>,
    Established<Durable>,
)>;

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
