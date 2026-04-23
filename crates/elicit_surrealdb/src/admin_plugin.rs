//! SurrealAdminPlugin — MCP tools for SurrealDB server administration and maintenance.

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
struct VersionParams {}

#[derive(Debug, Deserialize, JsonSchema)]
struct HealthParams {}

// ── Parameters ────────────────────────────────────────────────────────────────

/// Parameters for `surreal_admin__export`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExportParams {
    /// Filesystem path to write the export file.
    pub path: String,
    /// Namespace to export from.
    pub namespace: String,
    /// Database to export from.
    pub database: String,
    /// Whether to use the `--pretty` flag for formatted output.
    #[serde(default)]
    pub pretty: bool,
}

/// Parameters for `surreal_admin__import`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ImportParams {
    /// Filesystem path of the import file.
    pub path: String,
    /// Namespace to import into.
    pub namespace: String,
    /// Database to import into.
    pub database: String,
}

/// Parameters for `surreal_admin__export_rust`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExportRustParams {
    /// Filesystem path to write the export file.
    pub path: String,
}

/// Parameters for `surreal_admin__import_rust`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ImportRustParams {
    /// Filesystem path of the import file.
    pub path: String,
}

/// Parameters for `surreal_admin__remove_namespace`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveNamespaceParams {
    /// Namespace name to remove.
    pub namespace: String,
}

/// Parameters for `surreal_admin__remove_database`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveDatabaseParams {
    /// Database name to remove.
    pub database: String,
}

// ── Tools ────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "surreal_admin",
    name = "version",
    description = "Emit the SurrealQL query to retrieve the current server version."
)]
#[instrument]
async fn version(_p: VersionParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(
        "INFO FOR ROOT;  -- version is returned in root INFO\n-- Or use the HTTP endpoint: GET /version",
    )
}

#[elicit_tool(
    plugin = "surreal_admin",
    name = "health",
    description = "Emit the HTTP health-check endpoint URL and a Rust SDK Reqwest snippet."
)]
#[instrument]
async fn health(_p: HealthParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(
        "// SurrealDB health check\n\
         // HTTP endpoint (returns 200 OK when healthy):\n\
         // GET http://<host>:<port>/health\n\n\
         // Using reqwest:\n\
         let response = reqwest::get(\"http://localhost:8000/health\").await?;\n\
         assert!(response.status().is_success(), \"SurrealDB is unhealthy\");",
    )
}

#[elicit_tool(
    plugin = "surreal_admin",
    name = "export",
    description = "Emit the `surreal export` CLI command to export a database to a file."
)]
#[instrument]
async fn export(p: ExportParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let pretty = if p.pretty { " --pretty" } else { "" };
    ok_text(format!(
        "surreal export{pretty} --ns {ns} --db {db} --user root --pass <password> --endpoint ws://localhost:8000 {path}",
        pretty = pretty,
        ns = p.namespace,
        db = p.database,
        path = p.path
    ))
}

#[elicit_tool(
    plugin = "surreal_admin",
    name = "import",
    description = "Emit the `surreal import` CLI command to import a database from a file."
)]
#[instrument]
async fn import(p: ImportParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "surreal import --ns {ns} --db {db} --user root --pass <password> --endpoint ws://localhost:8000 {path}",
        ns = p.namespace,
        db = p.database,
        path = p.path
    ))
}

#[elicit_tool(
    plugin = "surreal_admin",
    name = "export_rust",
    description = "Emit a Rust snippet to export the current database to a file using the SurrealDB SDK."
)]
#[instrument]
async fn export_rust(p: ExportRustParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("db.export(\"{path}\").await?;", path = p.path))
}

#[elicit_tool(
    plugin = "surreal_admin",
    name = "import_rust",
    description = "Emit a Rust snippet to import a database from a file using the SurrealDB SDK."
)]
#[instrument]
async fn import_rust(p: ImportRustParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("db.import(\"{path}\").await?;", path = p.path))
}

#[elicit_tool(
    plugin = "surreal_admin",
    name = "remove_namespace",
    description = "Emit the SurrealQL statement to remove a namespace."
)]
#[instrument]
async fn remove_namespace(
    p: RemoveNamespaceParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("REMOVE NAMESPACE {};", p.namespace))
}

#[elicit_tool(
    plugin = "surreal_admin",
    name = "remove_database",
    description = "Emit the SurrealQL statement to remove a database."
)]
#[instrument]
async fn remove_database(
    p: RemoveDatabaseParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("REMOVE DATABASE {};", p.database))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing SurrealDB server administration tools (version, health, export, import).
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "surreal_admin")]
pub struct SurrealAdminPlugin;
