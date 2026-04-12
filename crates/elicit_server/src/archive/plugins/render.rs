//! `ArchiveDisplayPlugin` — render any archive descriptor to an AccessKit tree.
//!
//! Stateless pure transformation: takes a descriptor type and a display mode
//! string, invokes [`ArchiveDisplay::to_ak_nodes`], and returns a serialisable
//! `Vec<AkNodeEntry>` that feeds directly into `elicit_ui` frontends.

use elicit_accesskit::{NodeId, NodeJson};
use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::archive::display::{
    ArchiveDisplay, DatabaseDescriptorMode, QueryResultMode, SchemaDescriptorMode,
    TableDescriptorMode,
};
use crate::archive::{DatabaseDescriptor, QueryResult, SchemaDescriptor, TableDescriptor};

// ── helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        serde_json::to_string(value).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

// ── output type ───────────────────────────────────────────────────────────────

/// A single AccessKit node entry: ID + node descriptor.
///
/// `NodeId` serialises as a `u64` (transparent wrapper over `accesskit::NodeId(u64)`).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AkNodeEntry {
    /// Stable u64 identifier for this node.
    pub id: NodeId,
    /// Node descriptor with role, label, children, and optional fields.
    pub node: NodeJson,
}

fn to_entries(nodes: Vec<(NodeId, NodeJson)>) -> Vec<AkNodeEntry> {
    nodes
        .into_iter()
        .map(|(id, node)| AkNodeEntry { id, node })
        .collect()
}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `archive_display__database`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayDatabaseParams {
    /// Database descriptor to render.
    pub descriptor: DatabaseDescriptor,
    /// Display mode: `"Overview"` (default) or `"ConnectionCard"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__schema`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplaySchemaParams {
    /// Schema descriptor to render.
    pub descriptor: SchemaDescriptor,
    /// Display mode: `"TreeView"` (default) or `"FlatList"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__table`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayTableParams {
    /// Table descriptor to render.
    pub descriptor: TableDescriptor,
    /// Display mode: `"GridView"` (default), `"ColumnList"`, or `"SummaryCard"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__query_result`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayQueryResultParams {
    /// Query result to render.
    pub result: QueryResult,
    /// Display mode: `"DataGrid"` (default), `"StatsSummary"`, or `"SpatialMap"`.
    pub mode: Option<String>,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__database",
    description = "Render a DatabaseDescriptor as an AccessKit node tree. \
                   Mode: Overview (default) | ConnectionCard."
)]
#[instrument]
async fn display_database(p: DisplayDatabaseParams) -> Result<CallToolResult, ErrorData> {
    let mode: DatabaseDescriptorMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();

    let (_root, nodes) = p.descriptor.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__schema",
    description = "Render a SchemaDescriptor as an AccessKit node tree. \
                   Mode: TreeView (default) | FlatList."
)]
#[instrument]
async fn display_schema(p: DisplaySchemaParams) -> Result<CallToolResult, ErrorData> {
    let mode: SchemaDescriptorMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();

    let (_root, nodes) = p.descriptor.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__table",
    description = "Render a TableDescriptor as an AccessKit node tree. \
                   Mode: GridView (default) | ColumnList | SummaryCard."
)]
#[instrument]
async fn display_table(p: DisplayTableParams) -> Result<CallToolResult, ErrorData> {
    let mode: TableDescriptorMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();

    let (_root, nodes) = p.descriptor.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__query_result",
    description = "Render a QueryResult as an AccessKit node tree. \
                   Mode: DataGrid (default) | StatsSummary | SpatialMap."
)]
#[instrument]
async fn display_query_result(p: DisplayQueryResultParams) -> Result<CallToolResult, ErrorData> {
    let mode: QueryResultMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();

    let (_root, nodes) = p.result.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for rendering archive descriptors as AccessKit node trees.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "archive_display")]
pub struct ArchiveDisplayPlugin;

impl ArchiveDisplayPlugin {
    /// Create a new display plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for ArchiveDisplayPlugin {
    fn default() -> Self {
        Self::new()
    }
}
