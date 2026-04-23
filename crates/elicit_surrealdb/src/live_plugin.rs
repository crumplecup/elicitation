//! SurrealLivePlugin — MCP tools for SurrealDB `LIVE SELECT` and `KILL` statements.

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

// ── Parameters ────────────────────────────────────────────────────────────────

/// Parameters for `surreal_live__select`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LiveSelectParams {
    /// Table to watch (e.g. `"orders"`).
    pub table: String,
    /// Optional WHERE clause filter (e.g. `"status = 'open'"`).
    pub filter: Option<String>,
}

/// Parameters for `surreal_live__select_diff`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LiveSelectDiffParams {
    /// Table to watch with `DIFF` output.
    pub table: String,
    /// Optional WHERE clause filter.
    pub filter: Option<String>,
}

/// Parameters for `surreal_live__kill`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LiveKillParams {
    /// The UUID of the live query to kill.
    pub query_id: String,
}

/// Parameters for `surreal_live__select_rust`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LiveSelectRustParams {
    /// Table to watch.
    pub table: String,
    /// The Rust type to deserialize notifications into (e.g. `"MyRecord"`).
    pub record_type: String,
}

/// Parameters for `surreal_live__select_diff_rust`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LiveSelectDiffRustParams {
    /// Table to watch.
    pub table: String,
}

// ── Tools ────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "surreal_live",
    name = "select",
    description = "Emit a SurrealQL LIVE SELECT statement to subscribe to table changes."
)]
#[instrument]
async fn live_select(p: LiveSelectParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let filter = p.filter.map(|f| format!(" WHERE {f}")).unwrap_or_default();
    ok_text(format!(
        "LIVE SELECT * FROM {table}{filter};",
        table = p.table
    ))
}

#[elicit_tool(
    plugin = "surreal_live",
    name = "select_diff",
    description = "Emit a SurrealQL LIVE SELECT DIFF statement to receive JSON-patch diffs on table changes."
)]
#[instrument]
async fn live_select_diff(
    p: LiveSelectDiffParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let filter = p.filter.map(|f| format!(" WHERE {f}")).unwrap_or_default();
    ok_text(format!(
        "LIVE SELECT DIFF FROM {table}{filter};",
        table = p.table
    ))
}

#[elicit_tool(
    plugin = "surreal_live",
    name = "kill",
    description = "Emit a SurrealQL KILL statement to stop a live query by its UUID."
)]
#[instrument]
async fn live_kill(p: LiveKillParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("KILL \"{}\";", p.query_id))
}

#[elicit_tool(
    plugin = "surreal_live",
    name = "select_rust",
    description = "Emit a Rust snippet for subscribing to live table changes using the SurrealDB SDK."
)]
#[instrument]
async fn live_select_rust(
    p: LiveSelectRustParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "use surrealdb::Notification;\n\n\
         let mut stream = db\n    \
             .select(\"{table}\")\n    \
             .live()\n    \
             .await?;\n\n\
         while let Some(notification) = stream.next().await {{\n    \
             let notification: Notification<{ty}> = notification?;\n    \
             match notification.action {{\n        \
                 surrealdb::Action::Create => println!(\"Created: {{:?}}\", notification.data),\n        \
                 surrealdb::Action::Update => println!(\"Updated: {{:?}}\", notification.data),\n        \
                 surrealdb::Action::Delete => println!(\"Deleted: {{:?}}\", notification.data),\n        \
                 _ => {{}},\n    \
             }}\n\
         }}",
        table = p.table,
        ty = p.record_type
    ))
}

#[elicit_tool(
    plugin = "surreal_live",
    name = "select_diff_rust",
    description = "Emit a Rust snippet for subscribing to live DIFF notifications using the SurrealDB SDK."
)]
#[instrument]
async fn live_select_diff_rust(
    p: LiveSelectDiffRustParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "use surrealdb::{{Notification, Patch}};\n\n\
         let mut stream = db\n    \
             .query(\"LIVE SELECT DIFF FROM {table}\")\n    \
             .await?\n    \
             .stream::<Notification<Vec<Patch>>>(0)?;\n\n\
         while let Some(notification) = stream.next().await {{\n    \
             let notification = notification?;\n    \
             for patch in &notification.data {{\n        \
                 println!(\"Patch: {{:?}}\", patch);\n    \
             }}\n\
         }}",
        table = p.table
    ))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing SurrealDB `LIVE SELECT` and `KILL` statement tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "surreal_live")]
pub struct SurrealLivePlugin;
