//! `ArchiveSpatialPlugin` — spatial column detection and extraction.
//!
//! Pure transformations over [`QueryResult`] — no database connection needed.
//! Detects geometry / geography columns and extracts their `DbSpatialValue` payloads.

use elicit_db::{DbSpatialValue, DbValue};
use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::archive::QueryResult;

// ── helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        serde_json::to_string(value).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `archive_spatial__spatial_columns`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SpatialColumnsParams {
    /// The query result to inspect.
    pub result: QueryResult,
}

/// Parameters for `archive_spatial__extract_geometry`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExtractGeometryParams {
    /// The query result containing spatial data.
    pub result: QueryResult,
    /// Name of the spatial column to extract.
    pub column: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "archive_spatial",
    name = "archive_spatial__spatial_columns",
    description = "Return the names of all spatial columns in a QueryResult. \
                   Detects geometry and geography types by column type name."
)]
#[instrument]
async fn spatial_columns(p: SpatialColumnsParams) -> Result<CallToolResult, ErrorData> {
    let names: Vec<String> = p.result.spatial_column_names.clone();
    json_result(&names)
}

#[elicit_tool(
    plugin = "archive_spatial",
    name = "archive_spatial__extract_geometry",
    description = "Extract all non-null spatial (geometry/geography) values from a named column \
                   in a QueryResult. Returns Vec<DbSpatialValue> as JSON."
)]
#[instrument]
async fn extract_geometry(p: ExtractGeometryParams) -> Result<CallToolResult, ErrorData> {
    let col = &p.column;
    let values: Vec<DbSpatialValue> = p
        .result
        .rows
        .rows
        .iter()
        .filter_map(|row| {
            let val = row.get(col)?;
            match val {
                DbValue::Geometry(sv) | DbValue::Geography(sv) => Some(sv.clone()),
                _ => None,
            }
        })
        .collect();
    json_result(&values)
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for spatial column detection and extraction.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "archive_spatial")]
pub struct ArchiveSpatialPlugin;

impl ArchiveSpatialPlugin {
    /// Create a new spatial plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for ArchiveSpatialPlugin {
    fn default() -> Self {
        Self::new()
    }
}
