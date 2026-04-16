//! `ArchiveDisplayPlugin` — render any archive descriptor to an AccessKit tree.
//!
//! Stateless pure transformation: takes a descriptor type and a display mode
//! string, invokes [`ArchiveDisplay::to_ak_nodes`], and returns a serialisable
//! `Vec<AkNodeEntry>` that feeds directly into `elicit_ui` frontends.

use elicit_accesskit::{NodeId, NodeJson};
use elicitation::{Elicit, ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::archive::display::{
    AdminSnapshotMode, ArchiveDisplay, ColumnDescriptorMode, ColumnStatsMode,
    CompositeTypeDescriptorMode, ConnectionProfileMode, ConstraintDescriptorMode,
    DatabaseDescriptorMode, DdlDescriptorMode, DomainDescriptorMode, EnumDescriptorMode,
    ErdColumnMode, ErdDiagramMode, ErdEdgeMode, ErdNodeMode, ExplainNodeMode,
    ForeignKeyDescriptorMode, FunctionDescriptorMode, IndexDescriptorMode, MonitorSnapshotMode,
    QueryHistoryEntryMode, QueryResultMode, SavedQueryMode, SchemaDescriptorMode,
    SequenceDescriptorMode, StagedEditMode, TableDescriptorMode, TableInspectionMode,
    TriggerDescriptorMode,
};
use crate::archive::{
    AdminSnapshot, ColumnDescriptor, ColumnStats, CompositeTypeDescriptor, ConnectionProfile,
    ConstraintDescriptor, DatabaseDescriptor, DdlDescriptor, DomainDescriptor, EnumDescriptor,
    ErdColumn, ErdDiagram, ErdEdge, ErdNode, ExplainNode, ForeignKeyDescriptor, FunctionDescriptor,
    IndexDescriptor, MonitorSnapshot, QueryHistoryEntry, QueryResult, SavedQuery, SchemaDescriptor,
    SequenceDescriptor, StagedEdit, TableDescriptor, TableInspection, TriggerDescriptor,
};

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

// ── additional params ─────────────────────────────────────────────────────────

/// Parameters for `archive_display__column`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayColumnParams {
    /// Column descriptor to render.
    pub descriptor: ColumnDescriptor,
    /// Display mode: `"Inline"` (default) | `"Detailed"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__foreign_key`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayForeignKeyParams {
    /// Foreign-key descriptor to render.
    pub descriptor: ForeignKeyDescriptor,
    /// Display mode: `"Inline"` (default) | `"Detailed"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__constraint`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayConstraintParams {
    /// Constraint descriptor to render.
    pub descriptor: ConstraintDescriptor,
    /// Display mode: `"Inline"` (default) | `"Detailed"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__index`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayIndexParams {
    /// Index descriptor to render.
    pub descriptor: IndexDescriptor,
    /// Display mode: `"Row"` (default) | `"Detailed"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__ddl`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayDdlParams {
    /// DDL descriptor to render.
    pub descriptor: DdlDescriptor,
    /// Display mode: `"Block"` (only mode).
    pub mode: Option<String>,
}

/// Parameters for `archive_display__table_inspection`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayTableInspectionParams {
    /// Table inspection to render.
    pub inspection: TableInspection,
    /// Display mode: `"FkList"` (default) | `"ConstraintList"` | `"IndexList"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__column_stats`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayColumnStatsParams {
    /// Column stats to render.
    pub stats: ColumnStats,
    /// Display mode: `"Summary"` (default) | `"Detailed"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__explain_node`.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct DisplayExplainNodeParams {
    /// ExplainNode plan node to render.
    pub node: ExplainNode,
    /// Display mode: `"TreeNode"` (only mode).
    pub mode: Option<String>,
}

/// Parameters for `archive_display__query_history_entry`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayQueryHistoryEntryParams {
    /// Query history entry to render.
    pub entry: QueryHistoryEntry,
    /// Display mode: `"Row"` (default) | `"Detailed"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__saved_query`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplaySavedQueryParams {
    /// Saved query to render.
    pub query: SavedQuery,
    /// Display mode: `"Row"` (default) | `"Detailed"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__staged_edit`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayStagedEditParams {
    /// Staged edit to render.
    pub edit: StagedEdit,
    /// Display mode: `"Row"` (only mode).
    pub mode: Option<String>,
}

/// Parameters for `archive_display__connection_profile`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayConnectionProfileParams {
    /// Connection profile to render.
    pub profile: ConnectionProfile,
    /// Display mode: `"Card"` (default) | `"Row"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__function`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayFunctionParams {
    /// Function descriptor to render.
    pub descriptor: FunctionDescriptor,
    /// Display mode: `"Row"` (default) | `"Detailed"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__trigger`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayTriggerParams {
    /// Trigger descriptor to render.
    pub descriptor: TriggerDescriptor,
    /// Display mode: `"Row"` (default) | `"Detailed"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__sequence`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplaySequenceParams {
    /// Sequence descriptor to render.
    pub descriptor: SequenceDescriptor,
    /// Display mode: `"Row"` (default) | `"Detailed"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__enum_type`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayEnumTypeParams {
    /// Enum type descriptor to render.
    pub descriptor: EnumDescriptor,
    /// Display mode: `"Row"` (default) | `"Detailed"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__domain`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayDomainParams {
    /// Domain descriptor to render.
    pub descriptor: DomainDescriptor,
    /// Display mode: `"Row"` (default) | `"Detailed"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__composite_type`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayCompositeTypeParams {
    /// Composite type descriptor to render.
    pub descriptor: CompositeTypeDescriptor,
    /// Display mode: `"Row"` (default) | `"Detailed"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__monitor_snapshot`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayMonitorSnapshotParams {
    /// Monitor snapshot to render.
    pub snapshot: MonitorSnapshot,
    /// Display mode: `"Dashboard"` (default) | `"SessionList"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__admin_snapshot`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayAdminSnapshotParams {
    /// Admin snapshot to render.
    pub snapshot: AdminSnapshot,
    /// Display mode: `"RoleList"` (default) | `"BackupList"` | `"WalStatus"` | `"ExtList"` | `"Settings"`.
    pub mode: Option<String>,
}

/// Parameters for `archive_display__erd_column`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayErdColumnParams {
    /// ERD column to render.
    pub column: ErdColumn,
    /// Display mode: `"Row"` (only mode).
    pub mode: Option<String>,
}

/// Parameters for `archive_display__erd_node`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayErdNodeParams {
    /// ERD node (table box) to render.
    pub node: ErdNode,
    /// Display mode: `"TableBox"` (only mode).
    pub mode: Option<String>,
}

/// Parameters for `archive_display__erd_edge`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayErdEdgeParams {
    /// ERD edge (FK relationship) to render.
    pub edge: ErdEdge,
    /// Display mode: `"Row"` (only mode).
    pub mode: Option<String>,
}

/// Parameters for `archive_display__erd_diagram`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DisplayErdDiagramParams {
    /// ERD diagram to render.
    pub diagram: ErdDiagram,
    /// Display mode: `"NodeList"` (default) | `"EdgeList"` | `"Visual"`.
    pub mode: Option<String>,
}

// ── additional tools ──────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__column",
    description = "Render a ColumnDescriptor as an AccessKit node tree. \
                   Mode: Inline (default) | Detailed."
)]
#[instrument]
async fn display_column(p: DisplayColumnParams) -> Result<CallToolResult, ErrorData> {
    let mode: ColumnDescriptorMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.descriptor.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__foreign_key",
    description = "Render a ForeignKeyDescriptor as an AccessKit node tree. \
                   Mode: Inline (default) | Detailed."
)]
#[instrument]
async fn display_foreign_key(p: DisplayForeignKeyParams) -> Result<CallToolResult, ErrorData> {
    let mode: ForeignKeyDescriptorMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.descriptor.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__constraint",
    description = "Render a ConstraintDescriptor as an AccessKit node tree. \
                   Mode: Inline (default) | Detailed."
)]
#[instrument]
async fn display_constraint(p: DisplayConstraintParams) -> Result<CallToolResult, ErrorData> {
    let mode: ConstraintDescriptorMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.descriptor.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__index",
    description = "Render an IndexDescriptor as an AccessKit node tree. \
                   Mode: Row (default) | Detailed."
)]
#[instrument]
async fn display_index(p: DisplayIndexParams) -> Result<CallToolResult, ErrorData> {
    let mode: IndexDescriptorMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.descriptor.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__ddl",
    description = "Render a DdlDescriptor as an AccessKit node tree. \
                   Mode: Block (only mode)."
)]
#[instrument]
async fn display_ddl(p: DisplayDdlParams) -> Result<CallToolResult, ErrorData> {
    let mode: DdlDescriptorMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.descriptor.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__table_inspection",
    description = "Render a TableInspection as an AccessKit node tree. \
                   Mode: FkList (default) | ConstraintList | IndexList."
)]
#[instrument]
async fn display_table_inspection(
    p: DisplayTableInspectionParams,
) -> Result<CallToolResult, ErrorData> {
    let mode: TableInspectionMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.inspection.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__column_stats",
    description = "Render ColumnStats as an AccessKit node tree. \
                   Mode: Summary (default) | Detailed."
)]
#[instrument]
async fn display_column_stats(p: DisplayColumnStatsParams) -> Result<CallToolResult, ErrorData> {
    let mode: ColumnStatsMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.stats.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__explain_node",
    description = "Render an ExplainNode plan tree as an AccessKit node tree. \
                   Mode: TreeNode (only mode)."
)]
#[instrument]
async fn display_explain_node(p: DisplayExplainNodeParams) -> Result<CallToolResult, ErrorData> {
    let mode: ExplainNodeMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.node.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__query_history_entry",
    description = "Render a QueryHistoryEntry as an AccessKit node tree. \
                   Mode: Row (default) | Detailed."
)]
#[instrument]
async fn display_query_history_entry(
    p: DisplayQueryHistoryEntryParams,
) -> Result<CallToolResult, ErrorData> {
    let mode: QueryHistoryEntryMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.entry.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__saved_query",
    description = "Render a SavedQuery as an AccessKit node tree. \
                   Mode: Row (default) | Detailed."
)]
#[instrument]
async fn display_saved_query(p: DisplaySavedQueryParams) -> Result<CallToolResult, ErrorData> {
    let mode: SavedQueryMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.query.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__staged_edit",
    description = "Render a StagedEdit as an AccessKit node tree. \
                   Mode: Row (only mode)."
)]
#[instrument]
async fn display_staged_edit(p: DisplayStagedEditParams) -> Result<CallToolResult, ErrorData> {
    let mode: StagedEditMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.edit.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__connection_profile",
    description = "Render a ConnectionProfile as an AccessKit node tree. \
                   Mode: Card (default) | Row."
)]
#[instrument]
async fn display_connection_profile(
    p: DisplayConnectionProfileParams,
) -> Result<CallToolResult, ErrorData> {
    let mode: ConnectionProfileMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.profile.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__function",
    description = "Render a FunctionDescriptor as an AccessKit node tree. \
                   Mode: Row (default) | Detailed."
)]
#[instrument]
async fn display_function(p: DisplayFunctionParams) -> Result<CallToolResult, ErrorData> {
    let mode: FunctionDescriptorMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.descriptor.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__trigger",
    description = "Render a TriggerDescriptor as an AccessKit node tree. \
                   Mode: Row (default) | Detailed."
)]
#[instrument]
async fn display_trigger(p: DisplayTriggerParams) -> Result<CallToolResult, ErrorData> {
    let mode: TriggerDescriptorMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.descriptor.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__sequence",
    description = "Render a SequenceDescriptor as an AccessKit node tree. \
                   Mode: Row (default) | Detailed."
)]
#[instrument]
async fn display_sequence(p: DisplaySequenceParams) -> Result<CallToolResult, ErrorData> {
    let mode: SequenceDescriptorMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.descriptor.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__enum_type",
    description = "Render an EnumDescriptor as an AccessKit node tree. \
                   Mode: Row (default) | Detailed."
)]
#[instrument]
async fn display_enum_type(p: DisplayEnumTypeParams) -> Result<CallToolResult, ErrorData> {
    let mode: EnumDescriptorMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.descriptor.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__domain",
    description = "Render a DomainDescriptor as an AccessKit node tree. \
                   Mode: Row (default) | Detailed."
)]
#[instrument]
async fn display_domain(p: DisplayDomainParams) -> Result<CallToolResult, ErrorData> {
    let mode: DomainDescriptorMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.descriptor.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__composite_type",
    description = "Render a CompositeTypeDescriptor as an AccessKit node tree. \
                   Mode: Row (default) | Detailed."
)]
#[instrument]
async fn display_composite_type(
    p: DisplayCompositeTypeParams,
) -> Result<CallToolResult, ErrorData> {
    let mode: CompositeTypeDescriptorMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.descriptor.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__monitor_snapshot",
    description = "Render a MonitorSnapshot as an AccessKit node tree. \
                   Mode: Dashboard (default) | SessionList."
)]
#[instrument]
async fn display_monitor_snapshot(
    p: DisplayMonitorSnapshotParams,
) -> Result<CallToolResult, ErrorData> {
    let mode: MonitorSnapshotMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.snapshot.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__admin_snapshot",
    description = "Render an AdminSnapshot as an AccessKit node tree. \
                   Mode: RoleList (default) | BackupList | WalStatus | ExtList | Settings."
)]
#[instrument]
async fn display_admin_snapshot(
    p: DisplayAdminSnapshotParams,
) -> Result<CallToolResult, ErrorData> {
    let mode: AdminSnapshotMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.snapshot.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__erd_column",
    description = "Render an ErdColumn as an AccessKit node tree. \
                   Mode: Row (only mode)."
)]
#[instrument]
async fn display_erd_column(p: DisplayErdColumnParams) -> Result<CallToolResult, ErrorData> {
    let mode: ErdColumnMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.column.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__erd_node",
    description = "Render an ErdNode (table box) as an AccessKit node tree. \
                   Mode: TableBox (only mode)."
)]
#[instrument]
async fn display_erd_node(p: DisplayErdNodeParams) -> Result<CallToolResult, ErrorData> {
    let mode: ErdNodeMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.node.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__erd_edge",
    description = "Render an ErdEdge (FK relationship) as an AccessKit node tree. \
                   Mode: Row (only mode)."
)]
#[instrument]
async fn display_erd_edge(p: DisplayErdEdgeParams) -> Result<CallToolResult, ErrorData> {
    let mode: ErdEdgeMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.edge.to_ak_nodes(&mode, 1);
    json_result(&to_entries(nodes))
}

#[elicit_tool(
    plugin = "archive_display",
    name = "archive_display__erd_diagram",
    description = "Render an ErdDiagram as an AccessKit node tree. \
                   Mode: NodeList (default) | EdgeList | Visual."
)]
#[instrument]
async fn display_erd_diagram(p: DisplayErdDiagramParams) -> Result<CallToolResult, ErrorData> {
    let mode: ErdDiagramMode = p
        .mode
        .as_deref()
        .and_then(|s| serde_json::from_value(serde_json::Value::String(s.to_string())).ok())
        .unwrap_or_default();
    let (_root, nodes) = p.diagram.to_ak_nodes(&mode, 1);
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
