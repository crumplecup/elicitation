//! `ArchiveConstraintPlugin` — SQL constraint inspection via [`DbConstraintMeta`].
//!
//! Showcases the [`elicit_db`] trait abstraction: callers work through
//! `DbConstraintMeta` to list and verify constraints, receiving formal
//! `Established<ConstraintSatisfied>` proof tokens on success.

use elicit_db::{ConstraintSatisfied, DbConstraintMeta};
use elicitation::{ElicitPlugin, contracts::Established, elicit_tool};
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

/// Parameters for `archive_constraint__list`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListConstraintsParams {
    /// Database connection URL.
    pub url: String,
    /// Schema containing the table.
    pub schema: String,
    /// Table name to list constraints for.
    pub table: String,
}

/// Parameters for `archive_constraint__verify`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct VerifyConstraintsParams {
    /// Database connection URL.
    pub url: String,
    /// Schema containing the table.
    pub schema: String,
    /// Table name to verify constraints on.
    pub table: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "archive_constraint",
    name = "archive_constraint__list",
    description = "List all constraints on a table by name and type. Uses DbConstraintMeta trait \
                   against information_schema.table_constraints. Returns Vec<(name, type)>."
)]
#[instrument]
async fn list_constraints(p: ListConstraintsParams) -> Result<CallToolResult, ErrorData> {
    let backend = ArchiveDbBackend::connect(&p.url)
        .await
        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
    let constraints: Vec<(String, String)> = backend
        .list_constraints(&p.schema, &p.table)
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&constraints)
}

#[elicit_tool(
    plugin = "archive_constraint",
    name = "archive_constraint__verify",
    description = "Verify that all constraints on a table are satisfied by performing a \
                   count query. Uses DbConstraintMeta trait. Returns confirmation string on success."
)]
#[instrument]
async fn verify_constraints(p: VerifyConstraintsParams) -> Result<CallToolResult, ErrorData> {
    let backend = ArchiveDbBackend::connect(&p.url)
        .await
        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
    // The DbConstraintMeta trait returns a formal proof token on success.
    let _proof: Established<ConstraintSatisfied> = backend
        .verify_constraints(&p.schema, &p.table)
        .await
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&format!("constraint_satisfied on {}.{}", p.schema, p.table))
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for constraint inspection via the `DbConstraintMeta` trait.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "archive_constraint")]
pub struct ArchiveConstraintPlugin;

impl ArchiveConstraintPlugin {
    /// Create a new constraint plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for ArchiveConstraintPlugin {
    fn default() -> Self {
        Self::new()
    }
}
