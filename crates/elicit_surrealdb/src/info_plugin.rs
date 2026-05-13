//! SurrealInfoPlugin — MCP tools for SurrealDB `INFO FOR *` introspection queries.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

fn ok_text(s: impl Into<String>) -> Result<rmcp::model::CallToolResult, ErrorData> {
    Ok(rmcp::model::CallToolResult::success(vec![
        rmcp::model::Content::text(s.into()),
    ]))
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ForRootParams {}

#[derive(Debug, Deserialize, JsonSchema)]
struct ForNsParams {}

#[derive(Debug, Deserialize, JsonSchema)]
struct ForDbParams {}

// ── Parameters ────────────────────────────────────────────────────────────────

/// Parameters for `surreal_info__for_table`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct InfoForTableParams {
    /// Table name to introspect.
    pub table: String,
}

/// Parameters for `surreal_info__for_user`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct InfoForUserParams {
    /// Username to introspect.
    pub username: String,
    /// Scope: `"ROOT"`, `"NS"`, or `"DB"`.
    #[serde(default = "default_scope")]
    pub scope: String,
}

fn default_scope() -> String {
    "DB".to_string()
}

/// Parameters for `surreal_info__for_access`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct InfoForAccessParams {
    /// Access name to introspect.
    pub access: String,
    /// Scope: `"ROOT"`, `"NS"`, or `"DB"`.
    #[serde(default = "default_scope")]
    pub scope: String,
}

// ── Tools ────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "surreal_info",
    name = "for_root",
    description = "Emit the SurrealQL query to inspect the root-level schema."
)]
#[instrument]
async fn for_root(_p: ForRootParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text("INFO FOR ROOT;")
}

#[elicit_tool(
    plugin = "surreal_info",
    name = "for_ns",
    description = "Emit the SurrealQL query to inspect the current namespace schema."
)]
#[instrument]
async fn for_ns(_p: ForNsParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text("INFO FOR NS;")
}

#[elicit_tool(
    plugin = "surreal_info",
    name = "for_db",
    description = "Emit the SurrealQL query to inspect the current database schema (tables, functions, params, etc.)."
)]
#[instrument]
async fn for_db(_p: ForDbParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text("INFO FOR DB;")
}

#[elicit_tool(
    plugin = "surreal_info",
    name = "for_table",
    description = "Emit the SurrealQL query to inspect a specific table's fields, indexes, and events."
)]
#[instrument]
async fn for_table(p: InfoForTableParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("INFO FOR TABLE {};", p.table))
}

#[elicit_tool(
    plugin = "surreal_info",
    name = "for_user",
    description = "Emit the SurrealQL query to inspect a specific user's roles and permissions."
)]
#[instrument]
async fn for_user(p: InfoForUserParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("INFO FOR USER {} ON {};", p.username, p.scope))
}

#[elicit_tool(
    plugin = "surreal_info",
    name = "for_access",
    description = "Emit the SurrealQL query to inspect a specific access method's configuration."
)]
#[instrument]
async fn for_access(p: InfoForAccessParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("INFO FOR ACCESS {} ON {};", p.access, p.scope))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing SurrealDB `INFO FOR *` introspection query tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "surreal_info")]
pub struct SurrealInfoPlugin;
