//! `ArchiveRoutinePlugin` — stored routine browsing via [`DbRoutineMeta`].
//!
//! Showcases the [`elicit_db`] trait abstraction: callers work through the
//! `DbRoutineMeta` trait rather than raw SQL, so every result carries the
//! formal contract guarantees defined in `elicit_db`.

use elicit_db::{DbRoutineDescriptor, DbRoutineMeta};
use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::archive::ArchiveDbBackend;

// ── helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        serde_json::to_string(value).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `archive_routine__list_functions`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListFunctionsParams {
    /// Database connection URL.
    pub url: String,
    /// Schema to list functions from.
    pub schema: String,
}

/// Parameters for `archive_routine__list_procedures`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListProceduresParams {
    /// Database connection URL.
    pub url: String,
    /// Schema to list procedures from.
    pub schema: String,
}

/// Parameters for `archive_routine__routine_info`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RoutineInfoParams {
    /// Database connection URL.
    pub url: String,
    /// Schema containing the routine.
    pub schema: String,
    /// Routine name.
    pub name: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "archive_routine",
    name = "archive_routine__list_functions",
    description = "List all stored functions in a schema. Uses DbRoutineMeta trait to query \
                   information_schema.routines. Returns Vec<DbRoutineDescriptor>."
)]
#[instrument]
async fn list_functions(p: ListFunctionsParams) -> Result<CallToolResult, ErrorData> {
    let backend = ArchiveDbBackend::connect(&p.url)
        .await
        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
    let routines: Vec<DbRoutineDescriptor> = backend
        .list_functions(&p.schema)
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&routines)
}

#[elicit_tool(
    plugin = "archive_routine",
    name = "archive_routine__list_procedures",
    description = "List all stored procedures in a schema. Uses DbRoutineMeta trait to query \
                   information_schema.routines. Returns Vec<DbRoutineDescriptor>."
)]
#[instrument]
async fn list_procedures(p: ListProceduresParams) -> Result<CallToolResult, ErrorData> {
    let backend = ArchiveDbBackend::connect(&p.url)
        .await
        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
    let routines: Vec<DbRoutineDescriptor> = backend
        .list_procedures(&p.schema)
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&routines)
}

#[elicit_tool(
    plugin = "archive_routine",
    name = "archive_routine__routine_info",
    description = "Get detailed info about a specific routine by schema and name. \
                   Uses DbRoutineMeta trait. Returns DbRoutineDescriptor."
)]
#[instrument]
async fn routine_info(p: RoutineInfoParams) -> Result<CallToolResult, ErrorData> {
    let backend = ArchiveDbBackend::connect(&p.url)
        .await
        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
    let descriptor: DbRoutineDescriptor = backend
        .routine_info(&p.schema, &p.name, &[])
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&descriptor)
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for stored routine introspection via the `DbRoutineMeta` trait.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "archive_routine")]
pub struct ArchiveRoutinePlugin;

impl ArchiveRoutinePlugin {
    /// Create a new routine plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for ArchiveRoutinePlugin {
    fn default() -> Self {
        Self::new()
    }
}
