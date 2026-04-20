//! `SurrealCrudPlugin` — SurrealQL DML and Rust SDK snippet tools.

use elicitation::{ElicitPlugin, ToCodeLiteral, elicit_tool};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

fn ok_text(text: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(text.into())]))
}

// ── parameter structs ─────────────────────────────────────────────────────────

/// Key–value binding pair for parameterized queries.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct BindPair {
    /// Binding key (without `$` sigil).
    pub key: String,
    /// Binding value expression.
    pub value: String,
}

/// Parameters for raw `SELECT` SurrealQL emission.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectRawParams {
    /// Projections (e.g. `"*"`, `"name, age"`).
    pub projections: String,
    /// `FROM` target (table name or record ID).
    pub from: String,
    /// Optional `WHERE` clause.
    #[serde(default)]
    pub where_clause: Option<String>,
    /// Optional `FETCH` fields (graph traversal).
    #[serde(default)]
    pub fetch: Vec<String>,
    /// Optional `ORDER BY` clause.
    #[serde(default)]
    pub order_by: Option<String>,
    /// Optional `LIMIT`.
    #[serde(default)]
    pub limit: Option<u64>,
    /// Optional `START` offset.
    #[serde(default)]
    pub start: Option<u64>,
}

/// Parameters for raw `CREATE` emission.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateRawParams {
    /// Table name or `table:id`.
    pub target: String,
    /// `SET field = val, …` or `CONTENT {…}` clause (provide raw SurrealQL after the keyword).
    pub content: String,
    /// Use `CONTENT` keyword instead of `SET`.
    #[serde(default)]
    pub use_content: bool,
}

/// Parameters for raw `INSERT` emission.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InsertRawParams {
    /// Table name.
    pub table: String,
    /// Field names.
    pub fields: Vec<String>,
    /// Row values as SurrealQL expressions (parallel to `fields`).
    pub values: Vec<String>,
}

/// Parameters for raw `UPDATE` emission.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateRawParams {
    /// Table or `table:id` target.
    pub target: String,
    /// `SET field = val, …` SurrealQL.
    pub set_clause: String,
    /// Optional `WHERE` filter.
    #[serde(default)]
    pub where_clause: Option<String>,
}

/// Parameters for raw `UPSERT … CONTENT` emission.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpsertRawParams {
    /// `table:id` target.
    pub target: String,
    /// JSON-like content object (raw SurrealQL).
    pub content: String,
}

/// Parameters for raw `DELETE` emission.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeleteRawParams {
    /// Target (`table`, `table:id`, or record range).
    pub target: String,
    /// Optional `WHERE` filter.
    #[serde(default)]
    pub where_clause: Option<String>,
}

/// Parameters for raw `UPDATE … MERGE` emission.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MergeRawParams {
    /// `table:id` target.
    pub target: String,
    /// Merge object (raw SurrealQL / JSON).
    pub merge_object: String,
}

/// Parameters for raw `UPDATE … PATCH` emission.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PatchRawParams {
    /// `table:id` target.
    pub target: String,
    /// JSON Patch operations array (raw JSON).
    pub patch_ops: String,
}

/// Parameters for raw `RELATE` emission.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RelateRawParams {
    /// Source record ID.
    pub from_record: String,
    /// Relation table:id.
    pub relation: String,
    /// Target record ID.
    pub to_record: String,
    /// Optional `SET` clause.
    #[serde(default)]
    pub set_clause: Option<String>,
}

/// Parameters for Rust SDK `db.select` snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectRustParams {
    /// Table name (or `("table", "id")` tuple string for single record).
    pub target: String,
    /// Rust type name for the result (e.g. `"Person"`).
    pub result_type: String,
}

/// Parameters for Rust SDK `db.create` snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateRustParams {
    /// Table name.
    pub table: String,
    /// Optional record ID.
    #[serde(default)]
    pub id: Option<String>,
    /// Content variable name or literal struct.
    pub content: String,
    /// Rust result type (e.g. `"Person"`).
    pub result_type: String,
}

/// Parameters for Rust SDK `db.insert` snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InsertRustParams {
    /// Table name.
    pub table: String,
    /// Content variable (a `Vec<T>` expression).
    pub content: String,
    /// Rust element type (e.g. `"Person"`).
    pub result_type: String,
}

/// Parameters for Rust SDK `db.update` snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateRustParams {
    /// Table name.
    pub table: String,
    /// Optional ID (if targeting a single record).
    #[serde(default)]
    pub id: Option<String>,
    /// Merge/content variable.
    pub merge: String,
    /// Rust result type.
    pub result_type: String,
}

/// Parameters for Rust SDK `db.delete` snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeleteRustParams {
    /// Table name.
    pub table: String,
    /// Optional ID (single record delete).
    #[serde(default)]
    pub id: Option<String>,
}

/// Parameters for Rust SDK `db.query` snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QueryRustParams {
    /// SurrealQL query string.
    pub query: String,
    /// Variable bindings.
    #[serde(default)]
    pub binds: Vec<BindPair>,
    /// Rust result type.
    pub result_type: String,
}

/// Parameters for `db.query("LIVE SELECT …")` snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LiveRustParams {
    /// Table to watch.
    pub table: String,
    /// Rust event type (e.g. `"Notification<Person>"`).
    pub event_type: String,
}

// ── plugin struct ─────────────────────────────────────────────────────────────

/// MCP plugin providing SurrealQL DML and Rust SDK code generation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "surreal_crud")]
pub struct SurrealCrudPlugin;

impl SurrealCrudPlugin {
    /// Creates a new CRUD plugin.
    pub fn new() -> Self {
        Self
    }
}

impl Default for SurrealCrudPlugin {
    fn default() -> Self {
        Self::new()
    }
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "surreal_crud",
    name = "select_raw",
    description = "Emit a SurrealQL SELECT statement with optional WHERE, FETCH, ORDER BY, LIMIT, and START."
)]
#[instrument(skip_all)]
async fn select_raw(p: SelectRawParams) -> Result<CallToolResult, ErrorData> {
    let mut s = format!("SELECT {} FROM {}", p.projections, p.from);
    if let Some(w) = &p.where_clause {
        s.push_str(&format!(" WHERE {w}"));
    }
    if !p.fetch.is_empty() {
        s.push_str(&format!(" FETCH {}", p.fetch.join(", ")));
    }
    if let Some(o) = &p.order_by {
        s.push_str(&format!(" ORDER BY {o}"));
    }
    if let Some(l) = p.limit {
        s.push_str(&format!(" LIMIT {l}"));
    }
    if let Some(st) = p.start {
        s.push_str(&format!(" START {st}"));
    }
    s.push(';');
    ok_text(s)
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "create_raw",
    description = "Emit a SurrealQL CREATE statement using SET or CONTENT clause."
)]
#[instrument(skip_all)]
async fn create_raw(p: CreateRawParams) -> Result<CallToolResult, ErrorData> {
    let kw = if p.use_content { "CONTENT" } else { "SET" };
    ok_text(format!("CREATE {} {} {};", p.target, kw, p.content))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "insert_raw",
    description = "Emit a SurrealQL INSERT INTO statement with field list and values."
)]
#[instrument(skip_all)]
async fn insert_raw(p: InsertRawParams) -> Result<CallToolResult, ErrorData> {
    let fields = p.fields.join(", ");
    let values = p.values.join(", ");
    ok_text(format!(
        "INSERT INTO {} ({fields}) VALUES ({values});",
        p.table
    ))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "update_raw",
    description = "Emit a SurrealQL UPDATE … SET statement with optional WHERE filter."
)]
#[instrument(skip_all)]
async fn update_raw(p: UpdateRawParams) -> Result<CallToolResult, ErrorData> {
    let mut s = format!("UPDATE {} SET {}", p.target, p.set_clause);
    if let Some(w) = &p.where_clause {
        s.push_str(&format!(" WHERE {w}"));
    }
    s.push(';');
    ok_text(s)
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "upsert_raw",
    description = "Emit a SurrealQL UPSERT … CONTENT statement."
)]
#[instrument(skip_all)]
async fn upsert_raw(p: UpsertRawParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("UPSERT {} CONTENT {};", p.target, p.content))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "delete_raw",
    description = "Emit a SurrealQL DELETE statement with optional WHERE filter."
)]
#[instrument(skip_all)]
async fn delete_raw(p: DeleteRawParams) -> Result<CallToolResult, ErrorData> {
    let mut s = format!("DELETE {}", p.target);
    if let Some(w) = &p.where_clause {
        s.push_str(&format!(" WHERE {w}"));
    }
    s.push(';');
    ok_text(s)
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "merge_raw",
    description = "Emit a SurrealQL UPDATE … MERGE statement."
)]
#[instrument(skip_all)]
async fn merge_raw(p: MergeRawParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("UPDATE {} MERGE {};", p.target, p.merge_object))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "patch_raw",
    description = "Emit a SurrealQL UPDATE … PATCH statement using JSON Patch operations."
)]
#[instrument(skip_all)]
async fn patch_raw(p: PatchRawParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("UPDATE {} PATCH {};", p.target, p.patch_ops))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "relate_raw",
    description = "Emit a SurrealQL RELATE statement creating a graph edge between two records."
)]
#[instrument(skip_all)]
async fn relate_raw(p: RelateRawParams) -> Result<CallToolResult, ErrorData> {
    let mut s = format!("RELATE {}->{}->{}", p.from_record, p.relation, p.to_record);
    if let Some(set) = &p.set_clause {
        s.push_str(&format!(" SET {set}"));
    }
    s.push(';');
    ok_text(s)
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "select_rust",
    description = "Emit a Rust SDK db.select(…).await? code snippet."
)]
#[instrument(skip_all)]
async fn select_rust(p: SelectRustParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!(
        "let result: Vec<{ty}> = db\n    .select(\"{target}\")\n    .await?;\n",
        ty = p.result_type,
        target = p.target,
    ))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "create_rust",
    description = "Emit a Rust SDK db.create(…).content(…).await? code snippet."
)]
#[instrument(skip_all)]
async fn create_rust(p: CreateRustParams) -> Result<CallToolResult, ErrorData> {
    let target = match &p.id {
        Some(id) => format!("(\"{}\", \"{}\")", p.table, id),
        None => format!("\"{}\"", p.table),
    };
    ok_text(format!(
        "let result: Option<{ty}> = db\n    .create({target})\n    .content({content})\n    .await?;\n",
        ty = p.result_type,
        content = p.content,
    ))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "insert_rust",
    description = "Emit a Rust SDK db.insert(…).content(…).await? code snippet."
)]
#[instrument(skip_all)]
async fn insert_rust(p: InsertRustParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!(
        "let result: Vec<{ty}> = db\n    .insert(\"{table}\")\n    .content({content})\n    .await?;\n",
        ty = p.result_type,
        table = p.table,
        content = p.content,
    ))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "update_rust",
    description = "Emit a Rust SDK db.update(…).merge(…).await? code snippet."
)]
#[instrument(skip_all)]
async fn update_rust(p: UpdateRustParams) -> Result<CallToolResult, ErrorData> {
    let target = match &p.id {
        Some(id) => format!("(\"{}\", \"{}\")", p.table, id),
        None => format!("\"{}\"", p.table),
    };
    ok_text(format!(
        "let result: Option<{ty}> = db\n    .update({target})\n    .merge({merge})\n    .await?;\n",
        ty = p.result_type,
        merge = p.merge,
    ))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "delete_rust",
    description = "Emit a Rust SDK db.delete(…).await? code snippet."
)]
#[instrument(skip_all)]
async fn delete_rust(p: DeleteRustParams) -> Result<CallToolResult, ErrorData> {
    let target = match &p.id {
        Some(id) => format!("(\"{}\", \"{}\")", p.table, id),
        None => format!("\"{}\"", p.table),
    };
    ok_text(format!("db.delete({target}).await?;\n"))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "query_rust",
    description = "Emit a Rust SDK db.query(…).bind(…).await? code snippet with optional variable bindings."
)]
#[instrument(skip_all)]
async fn query_rust(p: QueryRustParams) -> Result<CallToolResult, ErrorData> {
    let binds_str: String = p
        .binds
        .iter()
        .map(|b| format!("    .bind((\"{}\", {}))\n", b.key, b.value))
        .collect();
    ok_text(format!(
        "let mut response = db\n    .query(\"{query}\")\n{binds_str}    .await?;\nlet result: Vec<{ty}> = response.take(0)?;\n",
        query = p.query,
        ty = p.result_type,
    ))
}

#[elicit_tool(
    plugin = "surreal_crud",
    name = "live_rust",
    description = "Emit a Rust SDK live query snippet using LIVE SELECT and a notification stream."
)]
#[instrument(skip_all)]
async fn live_rust(p: LiveRustParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!(
        r#"let mut response = db
    .query("LIVE SELECT * FROM {table}")
    .await?;
let mut stream = response.stream::<Notification<{ty}>>(0)?;
while let Some(notification) = stream.next().await {{
    println!("{{notification:?}}");
}}
"#,
        table = p.table,
        ty = p.event_type,
    ))
}
