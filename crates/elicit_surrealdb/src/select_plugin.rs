//! SurrealSelectPlugin — stateful MCP tools for composing SurrealDB SELECT queries.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn tool_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_text(s: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(s.into())]))
}

fn ok_json<T: serde::Serialize>(v: &T) -> Result<CallToolResult, ErrorData> {
    serde_json::to_string(v)
        .map(|s| CallToolResult::success(vec![Content::text(s)]))
        .map_err(|e| tool_err(format!("serialise: {e}")))
}

fn parse_params<T: serde::de::DeserializeOwned>(
    params: &CallToolRequestParams,
) -> Result<T, ErrorData> {
    let raw = params
        .arguments
        .as_ref()
        .map(|a| serde_json::Value::Object(a.clone()))
        .unwrap_or(serde_json::Value::Object(Default::default()));
    serde_json::from_value(raw)
        .map_err(|e| ErrorData::invalid_params(format!("param parse: {e}"), None))
}

fn build_tool(
    name: impl Into<std::borrow::Cow<'static, str>>,
    description: impl Into<std::borrow::Cow<'static, str>>,
    schema: serde_json::Value,
) -> Tool {
    let schema_obj: Arc<rmcp::model::JsonObject> = match schema {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => Arc::new(Default::default()),
    };
    Tool::new(name, description, schema_obj)
}

fn schema_of<T: schemars::JsonSchema>() -> serde_json::Value {
    serde_json::to_value(schemars::schema_for!(T)).unwrap_or_default()
}

fn parse_uuid(s: &str) -> Result<Uuid, ErrorData> {
    s.parse::<Uuid>()
        .map_err(|_| tool_err(format!("invalid UUID: {s}")))
}

// ── Descriptor ────────────────────────────────────────────────────────────────

struct SelectDescriptor {
    projections: String,
    from: String,
    where_clauses: Vec<String>,
    fetch_fields: Vec<String>,
    order_by: Option<String>,
    group_by: Option<String>,
    limit: Option<u32>,
    start_offset: Option<u32>,
    split_field: Option<String>,
    version: Option<String>,
}

impl Default for SelectDescriptor {
    fn default() -> Self {
        Self {
            projections: "*".to_string(),
            from: String::new(),
            where_clauses: Vec::new(),
            fetch_fields: Vec::new(),
            order_by: None,
            group_by: None,
            limit: None,
            start_offset: None,
            split_field: None,
            version: None,
        }
    }
}

fn build_select_sql(desc: &SelectDescriptor) -> String {
    let mut sql = format!("SELECT {} FROM {}", desc.projections, desc.from);
    if !desc.where_clauses.is_empty() {
        sql.push_str(&format!(" WHERE {}", desc.where_clauses.join(" AND ")));
    }
    if let Some(group) = &desc.group_by {
        sql.push_str(&format!(" GROUP BY {group}"));
    }
    if let Some(order) = &desc.order_by {
        sql.push_str(&format!(" ORDER BY {order}"));
    }
    if let Some(limit) = desc.limit {
        sql.push_str(&format!(" LIMIT {limit}"));
    }
    if let Some(start) = desc.start_offset {
        sql.push_str(&format!(" START {start}"));
    }
    if let Some(split) = &desc.split_field {
        sql.push_str(&format!(" SPLIT AT {split}"));
    }
    if let Some(version) = &desc.version {
        sql.push_str(&format!(" VERSION {version}"));
    }
    if !desc.fetch_fields.is_empty() {
        sql.push_str(&format!(" FETCH {}", desc.fetch_fields.join(", ")));
    }
    sql
}

// ── Context ───────────────────────────────────────────────────────────────────

struct SurrealSelectContext {
    descriptors: Mutex<HashMap<Uuid, SelectDescriptor>>,
}

impl SurrealSelectContext {
    fn new() -> Self {
        Self {
            descriptors: Mutex::new(HashMap::new()),
        }
    }
}

impl elicitation::PluginContext for SurrealSelectContext {}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for tools that address a stored SELECT descriptor by UUID.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SelectIdParams {
    /// UUID of the SELECT descriptor.
    pub id: String,
}

/// Parameters for `surreal_select__set_projections`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetProjectionsParams {
    /// UUID of the SELECT descriptor.
    pub id: String,
    /// Projection expression (e.g. `"*"` or `"name, age"`).
    pub projections: String,
}

/// Parameters for `surreal_select__set_from`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetFromParams {
    /// UUID of the SELECT descriptor.
    pub id: String,
    /// FROM target (table name, record ID, or subquery).
    pub from: String,
}

/// Parameters for `surreal_select__add_where`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddWhereParams {
    /// UUID of the SELECT descriptor.
    pub id: String,
    /// WHERE clause to append (combined with AND).
    pub clause: String,
}

/// Parameters for `surreal_select__add_fetch`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddFetchParams {
    /// UUID of the SELECT descriptor.
    pub id: String,
    /// Field name to add to the FETCH list.
    pub field: String,
}

/// Parameters for `surreal_select__set_order_by`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetOrderByParams {
    /// UUID of the SELECT descriptor.
    pub id: String,
    /// ORDER BY expression (e.g. `"name ASC"`).
    pub expr: String,
}

/// Parameters for `surreal_select__set_group_by`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetGroupByParams {
    /// UUID of the SELECT descriptor.
    pub id: String,
    /// GROUP BY expression (e.g. `"country"`).
    pub expr: String,
}

/// Parameters for `surreal_select__set_limit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetLimitParams {
    /// UUID of the SELECT descriptor.
    pub id: String,
    /// Maximum number of results to return.
    pub n: u32,
}

/// Parameters for `surreal_select__set_start`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetStartParams {
    /// UUID of the SELECT descriptor.
    pub id: String,
    /// Number of results to skip (START offset).
    pub n: u32,
}

/// Parameters for `surreal_select__set_split`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetSplitParams {
    /// UUID of the SELECT descriptor.
    pub id: String,
    /// Field to SPLIT AT.
    pub field: String,
}

/// Parameters for `surreal_select__set_version`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetVersionParams {
    /// UUID of the SELECT descriptor.
    pub id: String,
    /// VERSION datetime string (e.g. `"d'2024-01-01'"`).
    pub version: String,
}

// ── Tool implementations ──────────────────────────────────────────────────────

#[instrument(skip(ctx))]
async fn select_start(ctx: Arc<SurrealSelectContext>) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .insert(id, SelectDescriptor::default());
    ok_json(&serde_json::json!({ "id": id.to_string() }))
}

#[instrument(skip(ctx, p))]
async fn select_set_projections(
    ctx: Arc<SurrealSelectContext>,
    p: SetProjectionsParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let mut descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("descriptor not found: {id}")))?;
    desc.projections = p.projections;
    ok_json(&serde_json::json!({ "id": p.id, "projections": desc.projections }))
}

#[instrument(skip(ctx, p))]
async fn select_set_from(
    ctx: Arc<SurrealSelectContext>,
    p: SetFromParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let mut descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("descriptor not found: {id}")))?;
    desc.from = p.from;
    ok_json(&serde_json::json!({ "id": p.id, "from": desc.from }))
}

#[instrument(skip(ctx, p))]
async fn select_add_where(
    ctx: Arc<SurrealSelectContext>,
    p: AddWhereParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let mut descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("descriptor not found: {id}")))?;
    desc.where_clauses.push(p.clause);
    let count = desc.where_clauses.len();
    ok_json(&serde_json::json!({ "id": p.id, "where_count": count }))
}

#[instrument(skip(ctx, p))]
async fn select_add_fetch(
    ctx: Arc<SurrealSelectContext>,
    p: AddFetchParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let mut descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("descriptor not found: {id}")))?;
    desc.fetch_fields.push(p.field);
    let count = desc.fetch_fields.len();
    ok_json(&serde_json::json!({ "id": p.id, "fetch_count": count }))
}

#[instrument(skip(ctx, p))]
async fn select_set_order_by(
    ctx: Arc<SurrealSelectContext>,
    p: SetOrderByParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let mut descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("descriptor not found: {id}")))?;
    desc.order_by = Some(p.expr.clone());
    ok_json(&serde_json::json!({ "id": p.id, "order_by": p.expr }))
}

#[instrument(skip(ctx, p))]
async fn select_set_group_by(
    ctx: Arc<SurrealSelectContext>,
    p: SetGroupByParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let mut descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("descriptor not found: {id}")))?;
    desc.group_by = Some(p.expr.clone());
    ok_json(&serde_json::json!({ "id": p.id, "group_by": p.expr }))
}

#[instrument(skip(ctx, p))]
async fn select_set_limit(
    ctx: Arc<SurrealSelectContext>,
    p: SetLimitParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let mut descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("descriptor not found: {id}")))?;
    desc.limit = Some(p.n);
    ok_json(&serde_json::json!({ "id": p.id, "limit": p.n }))
}

#[instrument(skip(ctx, p))]
async fn select_set_start(
    ctx: Arc<SurrealSelectContext>,
    p: SetStartParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let mut descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("descriptor not found: {id}")))?;
    desc.start_offset = Some(p.n);
    ok_json(&serde_json::json!({ "id": p.id, "start": p.n }))
}

#[instrument(skip(ctx, p))]
async fn select_set_split(
    ctx: Arc<SurrealSelectContext>,
    p: SetSplitParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let mut descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("descriptor not found: {id}")))?;
    desc.split_field = Some(p.field.clone());
    ok_json(&serde_json::json!({ "id": p.id, "split_field": p.field }))
}

#[instrument(skip(ctx, p))]
async fn select_set_version(
    ctx: Arc<SurrealSelectContext>,
    p: SetVersionParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let mut descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("descriptor not found: {id}")))?;
    desc.version = Some(p.version.clone());
    ok_json(&serde_json::json!({ "id": p.id, "version": p.version }))
}

#[instrument(skip(ctx, p))]
async fn select_inspect(
    ctx: Arc<SurrealSelectContext>,
    p: SelectIdParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get(&id)
        .ok_or_else(|| tool_err(format!("descriptor not found: {id}")))?;
    ok_json(&serde_json::json!({
        "id": p.id,
        "projections": desc.projections,
        "from": desc.from,
        "where_clauses": desc.where_clauses,
        "fetch_fields": desc.fetch_fields,
        "order_by": desc.order_by,
        "group_by": desc.group_by,
        "limit": desc.limit,
        "start_offset": desc.start_offset,
        "split_field": desc.split_field,
        "version": desc.version,
    }))
}

#[instrument(skip(ctx, p))]
async fn select_emit_surreal(
    ctx: Arc<SurrealSelectContext>,
    p: SelectIdParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get(&id)
        .ok_or_else(|| tool_err(format!("descriptor not found: {id}")))?;
    ok_text(format!("{};", build_select_sql(desc)))
}

#[instrument(skip(ctx, p))]
async fn select_emit_rust(
    ctx: Arc<SurrealSelectContext>,
    p: SelectIdParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.id)?;
    let descs = ctx
        .descriptors
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let desc = descs
        .get(&id)
        .ok_or_else(|| tool_err(format!("descriptor not found: {id}")))?;
    let sql = build_select_sql(desc);
    ok_text(format!(
        "let results: Vec<Record> = db\n    .query(\"{sql}\")\n    .await?\n    .take(0)?;"
    ))
}

// ── Dispatch ──────────────────────────────────────────────────────────────────

async fn dispatch_select(
    ctx: Arc<SurrealSelectContext>,
    name: &str,
    params: &CallToolRequestParams,
) -> Result<CallToolResult, ErrorData> {
    match name {
        "surreal_select__start" => select_start(ctx).await,
        "surreal_select__set_projections" => {
            select_set_projections(ctx, parse_params(params)?).await
        }
        "surreal_select__set_from" => select_set_from(ctx, parse_params(params)?).await,
        "surreal_select__add_where" => select_add_where(ctx, parse_params(params)?).await,
        "surreal_select__add_fetch" => select_add_fetch(ctx, parse_params(params)?).await,
        "surreal_select__set_order_by" => select_set_order_by(ctx, parse_params(params)?).await,
        "surreal_select__set_group_by" => select_set_group_by(ctx, parse_params(params)?).await,
        "surreal_select__set_limit" => select_set_limit(ctx, parse_params(params)?).await,
        "surreal_select__set_start" => select_set_start(ctx, parse_params(params)?).await,
        "surreal_select__set_split" => select_set_split(ctx, parse_params(params)?).await,
        "surreal_select__set_version" => select_set_version(ctx, parse_params(params)?).await,
        "surreal_select__inspect" => select_inspect(ctx, parse_params(params)?).await,
        "surreal_select__emit_surreal" => select_emit_surreal(ctx, parse_params(params)?).await,
        "surreal_select__emit_rust" => select_emit_rust(ctx, parse_params(params)?).await,
        _ => Err(ErrorData::invalid_params(
            format!("unknown tool: {name}"),
            None,
        )),
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for composing SurrealDB SELECT queries step-by-step.
pub struct SurrealSelectPlugin(Arc<SurrealSelectContext>);

impl SurrealSelectPlugin {
    /// Create a new plugin with empty state.
    pub fn new() -> Self {
        Self(Arc::new(SurrealSelectContext::new()))
    }

    /// Invoke a tool by name with a JSON arguments object.
    ///
    /// Convenience method for tests and direct integration.
    pub async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<CallToolResult, ErrorData> {
        let owned = name.to_string();
        let params = if let Some(m) = args.as_object().cloned() {
            CallToolRequestParams::new(owned).with_arguments(m)
        } else {
            CallToolRequestParams::new(owned)
        };
        dispatch_select(self.0.clone(), name, &params).await
    }
}

impl Default for SurrealSelectPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for SurrealSelectPlugin {
    fn name(&self) -> &'static str {
        "surreal_select"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            build_tool(
                "surreal_select__start",
                "Create a new empty SELECT descriptor. Returns a UUID handle.",
                serde_json::json!({"type": "object", "properties": {}}),
            ),
            build_tool(
                "surreal_select__set_projections",
                "Set the projection fields (e.g. `\"*\"` or `\"name, age\"`).",
                schema_of::<SetProjectionsParams>(),
            ),
            build_tool(
                "surreal_select__set_from",
                "Set the FROM target (table, record ID, or subquery).",
                schema_of::<SetFromParams>(),
            ),
            build_tool(
                "surreal_select__add_where",
                "Append a WHERE clause (clauses are joined with AND).",
                schema_of::<AddWhereParams>(),
            ),
            build_tool(
                "surreal_select__add_fetch",
                "Add a field to the FETCH list for graph traversal.",
                schema_of::<AddFetchParams>(),
            ),
            build_tool(
                "surreal_select__set_order_by",
                "Set the ORDER BY expression (e.g. `\"name ASC\"`).",
                schema_of::<SetOrderByParams>(),
            ),
            build_tool(
                "surreal_select__set_group_by",
                "Set the GROUP BY expression.",
                schema_of::<SetGroupByParams>(),
            ),
            build_tool(
                "surreal_select__set_limit",
                "Set the LIMIT count.",
                schema_of::<SetLimitParams>(),
            ),
            build_tool(
                "surreal_select__set_start",
                "Set the START offset for pagination.",
                schema_of::<SetStartParams>(),
            ),
            build_tool(
                "surreal_select__set_split",
                "Set the SPLIT AT field for array expansion.",
                schema_of::<SetSplitParams>(),
            ),
            build_tool(
                "surreal_select__set_version",
                "Set the VERSION datetime for time-travel queries.",
                schema_of::<SetVersionParams>(),
            ),
            build_tool(
                "surreal_select__inspect",
                "Inspect the current state of a SELECT descriptor as JSON.",
                schema_of::<SelectIdParams>(),
            ),
            build_tool(
                "surreal_select__emit_surreal",
                "Emit the composed SELECT as a SurrealQL statement.",
                schema_of::<SelectIdParams>(),
            ),
            build_tool(
                "surreal_select__emit_rust",
                "Emit the composed SELECT as a Rust SurrealDB SDK snippet.",
                schema_of::<SelectIdParams>(),
            ),
        ]
    }

    #[tracing::instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let ctx = self.0.clone();
        Box::pin(async move { dispatch_select(ctx, params.name.as_ref(), &params).await })
    }
}
