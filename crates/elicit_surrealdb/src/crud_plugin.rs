//! SurrealCrudPlugin — MCP tools for SurrealDB CRUD operations and Rust SDK snippets.

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

/// Parameters for `surreal_crud__select_raw`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SelectRawParams {
    /// Fields to select (e.g. `"*"` or `"name, age"`).
    pub fields: String,
    /// Target table or record (e.g. `"person"` or `"person:john"`).
    pub from: String,
    /// Optional WHERE clause expression.
    #[serde(default)]
    pub where_clause: Option<String>,
    /// Optional LIMIT count.
    #[serde(default)]
    pub limit: Option<u32>,
    /// Optional START offset.
    #[serde(default)]
    pub start: Option<u32>,
}

/// Parameters for `surreal_crud__create_raw`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateRawParams {
    /// Target table or record ID (e.g. `"person"` or `"person:john"`).
    pub table_id: String,
    /// Optional CONTENT JSON string.
    #[serde(default)]
    pub content_json: Option<String>,
    /// Optional SET field assignments, each like `"name = 'Alice'"`.
    #[serde(default)]
    pub set_fields: Vec<String>,
}

/// Parameters for `surreal_crud__insert_raw`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct InsertRawParams {
    /// Target table name.
    pub table: String,
    /// Optional column names for positional VALUES syntax.
    #[serde(default)]
    pub fields: Vec<String>,
    /// Value rows, each a JSON-array string (e.g. `"['Alice', 30]"`).
    #[serde(default)]
    pub values: Vec<String>,
}

/// Parameters for `surreal_crud__update_raw`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateRawParams {
    /// Target table or record ID.
    pub target: String,
    /// SET field assignments, each like `"age = 31"`.
    #[serde(default)]
    pub set_fields: Vec<String>,
}

/// Parameters for `surreal_crud__upsert_raw`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpsertRawParams {
    /// Target table or record ID.
    pub target: String,
    /// CONTENT JSON string.
    pub content_json: String,
}

/// Parameters for `surreal_crud__delete_raw`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteRawParams {
    /// Target table or record ID.
    pub target: String,
    /// Optional WHERE clause expression.
    #[serde(default)]
    pub where_clause: Option<String>,
}

/// Parameters for `surreal_crud__merge_raw`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MergeRawParams {
    /// Target table or record ID.
    pub target: String,
    /// JSON object to merge in.
    pub merge_json: String,
}

/// Parameters for `surreal_crud__patch_raw`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PatchRawParams {
    /// Target table or record ID.
    pub target: String,
    /// JSON Patch operation strings (RFC 6902 objects).
    #[serde(default)]
    pub patches: Vec<String>,
}

/// Parameters for `surreal_crud__relate_raw`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RelateRawParams {
    /// Source record ID (e.g. `"person:alice"`).
    pub from_record: String,
    /// Edge table name.
    pub edge_table: String,
    /// Target record ID (e.g. `"product:42"`).
    pub to_record: String,
    /// Optional SET field assignments on the edge record.
    #[serde(default)]
    pub set_fields: Vec<String>,
}

/// Parameters for `surreal_crud__select_rust`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SelectRustParams {
    /// Table name.
    pub table: String,
    /// Optional specific record ID.
    #[serde(default)]
    pub record_id: Option<String>,
}

/// Parameters for `surreal_crud__create_rust`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateRustParams {
    /// Table name.
    pub table: String,
    /// Optional specific record ID.
    #[serde(default)]
    pub id: Option<String>,
    /// Rust type name for the content struct.
    pub content_type: String,
}

/// Parameters for `surreal_crud__insert_rust`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct InsertRustParams {
    /// Table name.
    pub table: String,
    /// Rust type name for the inserted records.
    pub content_type: String,
}

/// Parameters for `surreal_crud__update_rust`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateRustParams {
    /// Table name.
    pub table: String,
    /// Optional specific record ID.
    #[serde(default)]
    pub id: Option<String>,
    /// Update method: `"merge"`, `"content"`, or `"patch"`.
    pub method: String,
}

/// Parameters for `surreal_crud__delete_rust`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteRustParams {
    /// Table name.
    pub table: String,
    /// Optional specific record ID.
    #[serde(default)]
    pub id: Option<String>,
}

/// Parameters for `surreal_crud__query_rust`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QueryRustParams {
    /// SurrealQL query string.
    pub surreal_ql: String,
    /// Variable names to bind (each used as `bind(("name", name))`).
    #[serde(default)]
    pub bindings: Vec<String>,
}

/// Parameters for `surreal_crud__live_rust`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LiveRustParams {
    /// Table to subscribe to for live notifications.
    pub table: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "surreal_crud",
    name = "select_raw",
    description = "Generate a SELECT SurrealQL statement with optional WHERE, LIMIT, and START."
)]
#[instrument]
async fn select_raw(p: SelectRawParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let wh = p
        .where_clause
        .as_deref()
        .map(|w| format!(" WHERE {w}"))
        .unwrap_or_default();
    let lim = p.limit.map(|n| format!(" LIMIT {n}")).unwrap_or_default();
    let st = p.start.map(|n| format!(" START {n}")).unwrap_or_default();
    ok_text(format!("SELECT {} FROM {}{wh}{lim}{st};", p.fields, p.from))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "create_raw",
    description = "Generate a CREATE SurrealQL statement using CONTENT or SET syntax."
)]
#[instrument]
async fn create_raw(p: CreateRawParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let body = if let Some(json) = &p.content_json {
        format!(" CONTENT {json}")
    } else if !p.set_fields.is_empty() {
        format!(" SET {}", p.set_fields.join(", "))
    } else {
        String::new()
    };
    ok_text(format!("CREATE {}{body};", p.table_id))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "insert_raw",
    description = "Generate an INSERT INTO SurrealQL statement with optional column list."
)]
#[instrument]
async fn insert_raw(p: InsertRawParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let fields_part = if p.fields.is_empty() {
        String::new()
    } else {
        format!(" ({})", p.fields.join(", "))
    };
    let vals = p.values.join(", ");
    ok_text(format!(
        "INSERT INTO {}{fields_part} VALUES {vals};",
        p.table
    ))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "update_raw",
    description = "Generate an UPDATE … SET SurrealQL statement."
)]
#[instrument]
async fn update_raw(p: UpdateRawParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let set = p.set_fields.join(", ");
    ok_text(format!("UPDATE {} SET {set};", p.target))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "upsert_raw",
    description = "Generate an UPSERT … CONTENT SurrealQL statement."
)]
#[instrument]
async fn upsert_raw(p: UpsertRawParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("UPSERT {} CONTENT {};", p.target, p.content_json))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "delete_raw",
    description = "Generate a DELETE SurrealQL statement with optional WHERE clause."
)]
#[instrument]
async fn delete_raw(p: DeleteRawParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let wh = p
        .where_clause
        .as_deref()
        .map(|w| format!(" WHERE {w}"))
        .unwrap_or_default();
    ok_text(format!("DELETE {}{wh};", p.target))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "merge_raw",
    description = "Generate an UPDATE … MERGE SurrealQL statement."
)]
#[instrument]
async fn merge_raw(p: MergeRawParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("UPDATE {} MERGE {};", p.target, p.merge_json))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "patch_raw",
    description = "Generate an UPDATE … PATCH SurrealQL statement from a JSON Patch array."
)]
#[instrument]
async fn patch_raw(p: PatchRawParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let patch_arr = format!("[{}]", p.patches.join(", "));
    ok_text(format!("UPDATE {} PATCH {patch_arr};", p.target))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "relate_raw",
    description = "Generate a RELATE graph edge SurrealQL statement with optional SET fields."
)]
#[instrument]
async fn relate_raw(p: RelateRawParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let set = if p.set_fields.is_empty() {
        String::new()
    } else {
        format!(" SET {}", p.set_fields.join(", "))
    };
    ok_text(format!(
        "RELATE {}->{}->{}{set};",
        p.from_record, p.edge_table, p.to_record
    ))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "select_rust",
    description = "Generate a Rust SurrealDB SDK snippet for selecting records."
)]
#[instrument]
async fn select_rust(p: SelectRustParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let snippet = if let Some(id) = &p.record_id {
        format!(
            "let result: Option<MyRecord> = db\n    .select((\"{table}\", \"{id}\"))\n    .await?;",
            table = p.table,
            id = id
        )
    } else {
        format!(
            "let results: Vec<MyRecord> = db\n    .select(\"{table}\")\n    .await?;",
            table = p.table
        )
    };
    ok_text(snippet)
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "create_rust",
    description = "Generate a Rust SurrealDB SDK snippet for creating a record."
)]
#[instrument]
async fn create_rust(p: CreateRustParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let target = if let Some(id) = &p.id {
        format!(r#"("{}", "{}")"#, p.table, id)
    } else {
        format!(r#""{}""#, p.table)
    };
    ok_text(format!(
        "let result: Option<{ct}> = db\n    .create({target})\n    .content(value)\n    .await?;",
        ct = p.content_type
    ))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "insert_rust",
    description = "Generate a Rust SurrealDB SDK snippet for inserting records into a table."
)]
#[instrument]
async fn insert_rust(p: InsertRustParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "let results: Vec<{ct}> = db\n    .insert(\"{table}\")\n    .content(values)\n    .await?;",
        ct = p.content_type,
        table = p.table
    ))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "update_rust",
    description = "Generate a Rust SurrealDB SDK snippet for updating a record using merge, content, or patch."
)]
#[instrument]
async fn update_rust(p: UpdateRustParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let method_call = match p.method.as_str() {
        "content" => ".content(data)",
        "patch" => ".patch(PatchOp::add(\"/field\", value))",
        _ => ".merge(data)",
    };
    let target = if let Some(id) = &p.id {
        format!(r#"("{}", "{}")"#, p.table, id)
    } else {
        format!(r#""{}""#, p.table)
    };
    ok_text(format!(
        "let result: Option<T> = db\n    .update({target})\n    {method_call}\n    .await?;"
    ))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "delete_rust",
    description = "Generate a Rust SurrealDB SDK snippet for deleting a record or table."
)]
#[instrument]
async fn delete_rust(p: DeleteRustParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let target = if let Some(id) = &p.id {
        format!(r#"("{}", "{}")"#, p.table, id)
    } else {
        format!(r#""{}""#, p.table)
    };
    ok_text(format!(
        "let result: Option<T> = db\n    .delete({target})\n    .await?;"
    ))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "query_rust",
    description = "Generate a Rust SurrealDB SDK snippet for a parameterised query with bindings."
)]
#[instrument]
async fn query_rust(p: QueryRustParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let bind_lines = p
        .bindings
        .iter()
        .map(|b| format!("\n    .bind((\"{b}\", {b}))"))
        .collect::<Vec<_>>()
        .join("");
    ok_text(format!(
        "let mut response = db\n    .query(\"{sql}\"){bind_lines}\n    .await?;\nlet results: Vec<T> = response.take(0)?;",
        sql = p.surreal_ql
    ))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "live_rust",
    description = "Generate a Rust SurrealDB SDK snippet for a live query subscription."
)]
#[instrument]
async fn live_rust(p: LiveRustParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "let mut stream = db\n    .select(\"{table}\")\n    .live()\n    .await?;\nwhile let Some(notification) = stream.next().await {{\n    // handle notification\n}}",
        table = p.table
    ))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing SurrealDB CRUD operation and Rust SDK snippet tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "surreal_crud")]
pub struct SurrealCrudPlugin;
