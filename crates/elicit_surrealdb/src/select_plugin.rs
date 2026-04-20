//! `SurrealSelectPlugin` — stateful SELECT query builder.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use elicitation::{
    PluginContext, PluginToolRegistration, StatefulPlugin, ToCodeLiteral, ToolDescriptor,
    elicit_tool,
};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content, Tool},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

fn ok_text(text: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(text.into())]))
}

fn not_found(id: &str) -> ErrorData {
    ErrorData::invalid_params(format!("no SELECT descriptor for id {id}"), None)
}

// ── descriptor ────────────────────────────────────────────────────────────────

/// Order direction for SELECT ORDER BY.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum OrderDirection {
    /// Ascending order (default).
    Asc,
    /// Descending order.
    Desc,
}

/// A single ORDER BY term.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct OrderByTerm {
    /// Field name.
    pub field: String,
    /// Direction.
    #[serde(default)]
    pub direction: Option<OrderDirection>,
    /// Apply COLLATE option.
    #[serde(default)]
    pub collate: bool,
    /// Apply NUMERIC option.
    #[serde(default)]
    pub numeric: bool,
}

/// Full descriptor for a SELECT query being built incrementally.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectDescriptor {
    /// Unique ID for this descriptor.
    pub id: String,
    /// Projections (e.g. `"*"` or `"name, age"`).
    pub projections: String,
    /// FROM target.
    pub from: String,
    /// WHERE conditions (combined with AND).
    pub where_clauses: Vec<String>,
    /// FETCH fields.
    pub fetch_fields: Vec<String>,
    /// ORDER BY terms.
    pub order_by: Vec<OrderByTerm>,
    /// GROUP BY fields.
    pub group_by: Vec<String>,
    /// LIMIT count.
    pub limit: Option<u64>,
    /// START offset.
    pub start: Option<u64>,
    /// SPLIT AT field.
    pub split: Option<String>,
    /// VERSION datetime string.
    pub version: Option<String>,
}

impl SelectDescriptor {
    fn new(id: String, projections: String, from: String) -> Self {
        Self {
            id,
            projections,
            from,
            where_clauses: Vec::new(),
            fetch_fields: Vec::new(),
            order_by: Vec::new(),
            group_by: Vec::new(),
            limit: None,
            start: None,
            split: None,
            version: None,
        }
    }

    fn build_surreal(&self) -> String {
        let mut s = format!("SELECT {} FROM {}", self.projections, self.from);
        if !self.where_clauses.is_empty() {
            s.push_str(&format!(" WHERE {}", self.where_clauses.join(" AND ")));
        }
        if !self.group_by.is_empty() {
            s.push_str(&format!(" GROUP BY {}", self.group_by.join(", ")));
        }
        if !self.order_by.is_empty() {
            let terms: Vec<String> = self
                .order_by
                .iter()
                .map(|t| {
                    let mut term = t.field.clone();
                    if t.collate {
                        term.push_str(" COLLATE");
                    }
                    if t.numeric {
                        term.push_str(" NUMERIC");
                    }
                    match &t.direction {
                        Some(OrderDirection::Desc) => term.push_str(" DESC"),
                        _ => term.push_str(" ASC"),
                    }
                    term
                })
                .collect();
            s.push_str(&format!(" ORDER BY {}", terms.join(", ")));
        }
        if let Some(l) = self.limit {
            s.push_str(&format!(" LIMIT {l}"));
        }
        if let Some(st) = self.start {
            s.push_str(&format!(" START {st}"));
        }
        if !self.fetch_fields.is_empty() {
            s.push_str(&format!(" FETCH {}", self.fetch_fields.join(", ")));
        }
        if let Some(v) = &self.version {
            s.push_str(&format!(" VERSION d\"{v}\""));
        }
        if let Some(sp) = &self.split {
            s.push_str(&format!(" SPLIT AT {sp}"));
        }
        s.push(';');
        s
    }
}

// ── context ───────────────────────────────────────────────────────────────────

/// Shared state for `SurrealSelectPlugin`.
pub struct SurrealSelectCtx {
    items: Mutex<HashMap<Uuid, SelectDescriptor>>,
}

impl PluginContext for SurrealSelectCtx {}

impl SurrealSelectCtx {
    fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
        }
    }
}

// ── plugin struct ─────────────────────────────────────────────────────────────

/// Stateful MCP plugin for building SurrealDB SELECT queries incrementally.
pub struct SurrealSelectPlugin(Arc<SurrealSelectCtx>);

impl SurrealSelectPlugin {
    /// Creates a new select plugin.
    pub fn new() -> Self {
        Self(Arc::new(SurrealSelectCtx::new()))
    }
}

impl Default for SurrealSelectPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulPlugin for SurrealSelectPlugin {
    type Context = SurrealSelectCtx;

    fn name(&self) -> &'static str {
        "surreal_select"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|r| r.plugin == "surreal_select")
            .map(|r| (r.constructor)().as_tool())
            .collect()
    }

    fn tool_descriptors(&self) -> Vec<ToolDescriptor> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|r| r.plugin == "surreal_select")
            .map(|r| (r.constructor)())
            .collect()
    }

    fn context(&self) -> Arc<Self::Context> {
        self.0.clone()
    }
}

// ── parameter structs ─────────────────────────────────────────────────────────

/// Parameters to start a new SELECT descriptor.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectStartParams {
    /// Projection list (e.g. `"*"` or `"name, age"`).
    pub projections: String,
    /// FROM target.
    pub from: String,
}

/// Parameters to override projections.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectSetProjectionsParams {
    /// Descriptor UUID.
    pub id: String,
    /// New projection list.
    pub projections: String,
}

/// Parameters to set the FROM clause.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectSetFromParams {
    /// Descriptor UUID.
    pub id: String,
    /// New FROM target.
    pub from: String,
}

/// Parameters to add a WHERE condition.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectAddWhereParams {
    /// Descriptor UUID.
    pub id: String,
    /// Condition expression.
    pub condition: String,
}

/// Parameters to add a FETCH field.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectAddFetchParams {
    /// Descriptor UUID.
    pub id: String,
    /// Field to fetch.
    pub field: String,
}

/// Parameters to set ORDER BY.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectSetOrderByParams {
    /// Descriptor UUID.
    pub id: String,
    /// Order terms.
    pub terms: Vec<OrderByTerm>,
}

/// Parameters to set GROUP BY.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectSetGroupByParams {
    /// Descriptor UUID.
    pub id: String,
    /// Field names.
    pub fields: Vec<String>,
}

/// Parameters to set LIMIT.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectSetLimitParams {
    /// Descriptor UUID.
    pub id: String,
    /// Limit count.
    pub limit: u64,
}

/// Parameters to set START offset.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectSetStartParams {
    /// Descriptor UUID.
    pub id: String,
    /// Offset value.
    pub start: u64,
}

/// Parameters to set SPLIT AT.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectSetSplitParams {
    /// Descriptor UUID.
    pub id: String,
    /// Field to split on.
    pub field: String,
}

/// Parameters to set VERSION.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectSetVersionParams {
    /// Descriptor UUID.
    pub id: String,
    /// ISO 8601 datetime string.
    pub datetime: String,
}

/// Parameters to inspect a descriptor.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectInspectParams {
    /// Descriptor UUID.
    pub id: String,
}

/// Parameters to emit SurrealQL from a descriptor.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectEmitSurrealParams {
    /// Descriptor UUID.
    pub id: String,
}

/// Parameters to emit Rust SDK code from a descriptor.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectEmitRustParams {
    /// Descriptor UUID.
    pub id: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "surreal_select",
    name = "start",
    description = "Begin a new SELECT query descriptor. Returns a UUID to reference in subsequent calls."
)]
#[instrument(skip(ctx))]
async fn select_start(
    ctx: Arc<SurrealSelectCtx>,
    p: SelectStartParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    let descriptor = SelectDescriptor::new(id.to_string(), p.projections, p.from);
    ctx.items.lock().unwrap().insert(id, descriptor);
    ok_text(id.to_string())
}

#[elicit_tool(
    plugin = "surreal_select",
    name = "set_projections",
    description = "Override the SELECT projection list on an existing descriptor."
)]
#[instrument(skip(ctx))]
async fn select_set_projections(
    ctx: Arc<SurrealSelectCtx>,
    p: SelectSetProjectionsParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let mut items = ctx.items.lock().unwrap();
    let entry = items.get_mut(&id).ok_or_else(|| not_found(&p.id))?;
    entry.projections = p.projections;
    ok_text("ok")
}

#[elicit_tool(
    plugin = "surreal_select",
    name = "set_from",
    description = "Override the FROM clause on an existing descriptor."
)]
#[instrument(skip(ctx))]
async fn select_set_from(
    ctx: Arc<SurrealSelectCtx>,
    p: SelectSetFromParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let mut items = ctx.items.lock().unwrap();
    let entry = items.get_mut(&id).ok_or_else(|| not_found(&p.id))?;
    entry.from = p.from;
    ok_text("ok")
}

#[elicit_tool(
    plugin = "surreal_select",
    name = "add_where",
    description = "Append a WHERE condition to an existing descriptor (combined with AND)."
)]
#[instrument(skip(ctx))]
async fn select_add_where(
    ctx: Arc<SurrealSelectCtx>,
    p: SelectAddWhereParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let mut items = ctx.items.lock().unwrap();
    let entry = items.get_mut(&id).ok_or_else(|| not_found(&p.id))?;
    entry.where_clauses.push(p.condition);
    ok_text("ok")
}

#[elicit_tool(
    plugin = "surreal_select",
    name = "add_fetch",
    description = "Append a FETCH field to an existing descriptor for graph traversal resolution."
)]
#[instrument(skip(ctx))]
async fn select_add_fetch(
    ctx: Arc<SurrealSelectCtx>,
    p: SelectAddFetchParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let mut items = ctx.items.lock().unwrap();
    let entry = items.get_mut(&id).ok_or_else(|| not_found(&p.id))?;
    entry.fetch_fields.push(p.field);
    ok_text("ok")
}

#[elicit_tool(
    plugin = "surreal_select",
    name = "set_order_by",
    description = "Set the ORDER BY terms on an existing descriptor."
)]
#[instrument(skip(ctx))]
async fn select_set_order_by(
    ctx: Arc<SurrealSelectCtx>,
    p: SelectSetOrderByParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let mut items = ctx.items.lock().unwrap();
    let entry = items.get_mut(&id).ok_or_else(|| not_found(&p.id))?;
    entry.order_by = p.terms;
    ok_text("ok")
}

#[elicit_tool(
    plugin = "surreal_select",
    name = "set_group_by",
    description = "Set the GROUP BY fields on an existing descriptor."
)]
#[instrument(skip(ctx))]
async fn select_set_group_by(
    ctx: Arc<SurrealSelectCtx>,
    p: SelectSetGroupByParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let mut items = ctx.items.lock().unwrap();
    let entry = items.get_mut(&id).ok_or_else(|| not_found(&p.id))?;
    entry.group_by = p.fields;
    ok_text("ok")
}

#[elicit_tool(
    plugin = "surreal_select",
    name = "set_limit",
    description = "Set the LIMIT count on an existing descriptor."
)]
#[instrument(skip(ctx))]
async fn select_set_limit(
    ctx: Arc<SurrealSelectCtx>,
    p: SelectSetLimitParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let mut items = ctx.items.lock().unwrap();
    let entry = items.get_mut(&id).ok_or_else(|| not_found(&p.id))?;
    entry.limit = Some(p.limit);
    ok_text("ok")
}

#[elicit_tool(
    plugin = "surreal_select",
    name = "set_start",
    description = "Set the START offset on an existing descriptor for pagination."
)]
#[instrument(skip(ctx))]
async fn select_set_start(
    ctx: Arc<SurrealSelectCtx>,
    p: SelectSetStartParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let mut items = ctx.items.lock().unwrap();
    let entry = items.get_mut(&id).ok_or_else(|| not_found(&p.id))?;
    entry.start = Some(p.start);
    ok_text("ok")
}

#[elicit_tool(
    plugin = "surreal_select",
    name = "set_split",
    description = "Set the SPLIT AT field on an existing descriptor."
)]
#[instrument(skip(ctx))]
async fn select_set_split(
    ctx: Arc<SurrealSelectCtx>,
    p: SelectSetSplitParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let mut items = ctx.items.lock().unwrap();
    let entry = items.get_mut(&id).ok_or_else(|| not_found(&p.id))?;
    entry.split = Some(p.field);
    ok_text("ok")
}

#[elicit_tool(
    plugin = "surreal_select",
    name = "set_version",
    description = "Set the VERSION datetime on an existing descriptor for temporal queries."
)]
#[instrument(skip(ctx))]
async fn select_set_version(
    ctx: Arc<SurrealSelectCtx>,
    p: SelectSetVersionParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let mut items = ctx.items.lock().unwrap();
    let entry = items.get_mut(&id).ok_or_else(|| not_found(&p.id))?;
    entry.version = Some(p.datetime);
    ok_text("ok")
}

#[elicit_tool(
    plugin = "surreal_select",
    name = "inspect",
    description = "Return the current SELECT descriptor as a JSON summary."
)]
#[instrument(skip(ctx))]
async fn select_inspect(
    ctx: Arc<SurrealSelectCtx>,
    p: SelectInspectParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let items = ctx.items.lock().unwrap();
    let entry = items.get(&id).ok_or_else(|| not_found(&p.id))?;
    let json = serde_json::to_string_pretty(entry)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    ok_text(json)
}

#[elicit_tool(
    plugin = "surreal_select",
    name = "emit_surreal",
    description = "Emit the final SurrealQL SELECT statement for the descriptor."
)]
#[instrument(skip(ctx))]
async fn select_emit_surreal(
    ctx: Arc<SurrealSelectCtx>,
    p: SelectEmitSurrealParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let items = ctx.items.lock().unwrap();
    let entry = items.get(&id).ok_or_else(|| not_found(&p.id))?;
    ok_text(entry.build_surreal())
}

#[elicit_tool(
    plugin = "surreal_select",
    name = "emit_rust",
    description = "Emit a db.query(\"SELECT …\").await? Rust snippet for the descriptor."
)]
#[instrument(skip(ctx))]
async fn select_emit_rust(
    ctx: Arc<SurrealSelectCtx>,
    p: SelectEmitRustParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;
    let items = ctx.items.lock().unwrap();
    let entry = items.get(&id).ok_or_else(|| not_found(&p.id))?;
    let surreal = entry.build_surreal();
    ok_text(format!(
        "let mut response = db\n    .query(\"{surreal}\")\n    .await?;\nlet result: Vec<T> = response.take(0)?;\n"
    ))
}
