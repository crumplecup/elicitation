//! Shared keyboard-navigation model for the archive frontends.
//!
//! `ArchiveNavModel` holds the flattened tree used by both the ratatui TUI and
//! the egui native frontend.  Keybinding semantics are declared once in
//! [`crate::archive::ArchiveKeyMap::default_map`] and every frontend derives
//! its key-to-action mapping from that IR definition.
//!
//! ## Flat-list model
//!
//! The tree (schemas → tables) is projected into a flat `Vec<FlatItem>` that
//! the frontends render as a scrollable list.  A single `usize` cursor tracks
//! the selected row.  Schema rows can be expanded/collapsed; when a schema is
//! collapsed its child table rows are removed from the flat list.
//!
//! ## Filter
//!
//! Press `/` to activate the filter bar.  Typing narrows the flat list to
//! items whose name contains the filter string (case-insensitive).  Esc clears
//! the filter and returns to normal navigation.

use std::collections::BTreeMap;

use elicit_accesskit::KeyBinding;
use elicitation::Established;
use elicitation::{Elicit, KaniCompose};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator as _;

use crate::archive::types::DdlDescriptor;
use crate::archive::vsm::{
    ArchiveConnectionConsistent, ArchiveConnectionState, ArchiveNavConsistent,
    ArchiveOverlayConsistent, ArchiveOverlayState, ArchivePanelConsistent, ArchivePanelState,
    close_overlay, column_detail, data_grid_ready, ddl_ready, explain_ready,
    open_connection_editor, open_export_picker, open_help,
    open_save_prompt as vsm_open_save_prompt, open_saved_browser, open_sql_editor, panel_error,
    panel_loading, picker_move_down, picker_move_up, prompt_backspace, prompt_push,
    saved_browser_down, saved_browser_up,
};
use crate::archive::{
    AdminSnapshot, ColumnStats, ConnectionProfile, ConstraintDescriptor, ErdDiagram, ErdLayout,
    ExplainNode, ExplainPlan, ExportFormat, IndexDescriptor, MonitorSnapshot, QueryHistoryEntry,
    QueryResult, RowEditState, SavedQuery, StagedEdit, TableInspection,
    actions::{ArchiveKeyMap, KeyMapMode},
    nav_tree::{NavTree, SchemaEntry},
};

// ── FlatItem ──────────────────────────────────────────────────────────────────

/// One visible row in the navigation panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlatItem {
    /// A schema row.  The payload is the schema index into
    /// [`ArchiveNavModel::schemas`].
    Schema(usize),
    /// A table/view row.  The payload is `(schema_idx, table_idx)`.
    Table(usize, usize),
    /// Collapsible group header for functions within a schema.
    FunctionsGroup(usize),
    /// A function row under its schema's functions group.  `(schema_idx, fn_idx)`.
    Function(usize, usize),
    /// Collapsible group header for sequences within a schema.
    SequencesGroup(usize),
    /// A sequence row under its schema's sequences group.  `(schema_idx, seq_idx)`.
    Sequence(usize, usize),
    /// Collapsible group header for user-defined types within a schema.
    TypesGroup(usize),
    /// A type row.  `(schema_idx, kind, idx)` where `kind` is
    /// `0` = enum, `1` = domain, `2` = composite.
    TypeEntry(usize, u8, usize),
    /// Collapsible group header for triggers within a schema.
    TriggersGroup(usize),
    /// A trigger row under its schema's triggers group.  `(schema_idx, trigger_idx)`.
    Trigger(usize, usize),
    /// Collapsible group header for database-level extensions.
    ExtensionsGroup,
    /// A database extension row.  Payload is index into
    /// [`ArchiveNavModel::extensions`].
    Extension(usize),
}

// ── SchemaWithExpand ──────────────────────────────────────────────────────────

/// A schema entry combined with its current expand/collapse state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit, KaniCompose)]
pub struct SchemaWithExpand {
    /// The underlying schema descriptor.
    pub entry: SchemaEntry,
    /// Whether child tables are currently visible.
    pub expanded: bool,
    /// Whether the functions group is expanded.
    pub functions_expanded: bool,
    /// Whether the sequences group is expanded.
    pub sequences_expanded: bool,
    /// Whether the types group is expanded.
    pub types_expanded: bool,
    /// Whether the triggers group is expanded.
    pub triggers_expanded: bool,
}

#[cfg(kani)]
impl kani::Arbitrary for SchemaWithExpand {
    fn any() -> Self {
        Self {
            entry: kani::any(),
            expanded: kani::any(),
            functions_expanded: kani::any(),
            sequences_expanded: kani::any(),
            types_expanded: kani::any(),
            triggers_expanded: kani::any(),
        }
    }
}

// ── Fetch messaging ───────────────────────────────────────────────────────────

/// Request sent from the event loop to the background fetch task.
#[derive(Debug, Clone)]
pub enum FetchRequest {
    /// Fetch the first N rows of a table.
    PreviewTable {
        /// Schema containing the table.
        schema: String,
        /// Table to preview.
        table: String,
    },
    /// Re-query the schema/table tree.
    Refresh,
    /// Execute arbitrary SQL.
    ExecuteSql {
        /// Raw SQL to execute.
        sql: String,
    },
    /// Fetch FK, constraint, and index enrichment for a table.
    InspectTable {
        /// Schema containing the table.
        schema: String,
        /// Table to inspect.
        table: String,
    },
    /// Generate DDL for a table.
    GetDdl {
        /// Schema containing the table.
        schema: String,
        /// Table to generate DDL for.
        table: String,
    },
    /// Fetch per-column planner statistics for a table.
    GetColumnStats {
        /// Schema containing the table.
        schema: String,
        /// Table to get stats for.
        table: String,
    },
    /// Run `EXPLAIN (ANALYZE, FORMAT JSON)` on a SQL string.
    ExplainSql {
        /// Schema context (for display only).
        schema: String,
        /// Table context (for display only).
        table: String,
        /// SQL to explain.
        sql: String,
    },
    /// Export the current data grid or query result.
    ExportData {
        /// Schema context (for display only).
        schema: String,
        /// Table context (for display only).
        table: String,
        /// Query result to export (cloned from the active panel).
        result: QueryResult,
        /// Desired output format.
        format: ExportFormat,
    },
    /// Switch to a new database URL without restarting the fetch task.
    ///
    /// The fetch task replaces its active URL and performs a [`Refresh`].
    ///
    /// [`Refresh`]: FetchRequest::Refresh
    UpdateUrl(String),
    /// Fetch a live monitoring snapshot (sessions, roles, cache hit, backups).
    ///
    /// `schema` is used for table-bloat and index-usage queries (defaults to
    /// `"public"` if empty).
    FetchMonitor {
        /// Schema for table-bloat and index-usage queries.
        schema: String,
    },
    /// Fetch an administration snapshot (roles, backups/WAL, settings, extensions).
    FetchAdmin,
    /// Fetch an ERD diagram for the given schema.
    FetchErd {
        /// Schema to diagram.
        schema: String,
    },
    /// Fetch constraints for a table (re-uses `inspect_table_direct` output).
    FetchConstraints {
        /// Schema containing the table.
        schema: String,
        /// Table to inspect.
        table: String,
    },
    /// Fetch indexes for a table (re-uses `inspect_table_direct` output).
    FetchIndexes {
        /// Schema containing the table.
        schema: String,
        /// Table to inspect.
        table: String,
    },
}

/// Response sent from the background fetch task back to the event loop.
#[derive(Debug, Clone)]
pub enum PanelEvent {
    /// Data grid ready.
    DataGrid {
        /// Schema the table belongs to.
        schema: String,
        /// Table name.
        table: String,
        /// Fetched query result.
        result: QueryResult,
    },
    /// Refreshed nav tree.
    NavRefreshed(NavTree),
    /// SQL query result.
    SqlResult(QueryResult),
    /// An error occurred during a fetch.
    FetchError(String),
    /// Table inspection (FK/constraints/indexes) completed.
    TableInspected {
        /// Schema the table belongs to.
        schema: String,
        /// Table name.
        table: String,
        /// Enrichment data.
        inspection: TableInspection,
    },
    /// DDL generation completed.
    DdlReady {
        /// Schema the table belongs to.
        schema: String,
        /// Table name.
        table: String,
        /// Reconstructed DDL text.
        ddl: String,
    },
    /// Column stats ready.
    ColumnStatsReady {
        /// Schema the table belongs to.
        schema: String,
        /// Table name.
        table: String,
        /// Per-column statistics.
        stats: Vec<ColumnStats>,
    },
    /// EXPLAIN plan ready.
    ExplainReady {
        /// Schema context.
        schema: String,
        /// Table context.
        table: String,
        /// Parsed plan arena.
        root: ExplainPlan,
    },
    /// Export completed; content ready to write or display.
    ExportReady {
        /// Schema context.
        schema: String,
        /// Table context.
        table: String,
        /// Serialized export content.
        content: String,
        /// Format of the content.
        format: ExportFormat,
        /// Number of rows exported.
        row_count: u64,
    },
    /// Live monitoring snapshot ready.
    MonitorReady(MonitorSnapshot),
    /// Administration snapshot ready.
    AdminReady(AdminSnapshot),
    /// ERD diagram ready.
    ErdReady(ErdDiagram),
    /// Constraints for a table are ready.
    ConstraintsReady {
        /// Schema the table belongs to.
        schema: String,
        /// Table name.
        table: String,
        /// Constraint list.
        constraints: Vec<ConstraintDescriptor>,
    },
    /// Indexes for a table are ready.
    IndexesReady {
        /// Schema the table belongs to.
        schema: String,
        /// Table name.
        table: String,
        /// Index list.
        indexes: Vec<IndexDescriptor>,
    },
}

// ── ArchiveNavModel ───────────────────────────────────────────────────────────

/// Frontend-agnostic keyboard-navigation state for the archive tree view.
///
/// Frontends call [`move_up`], [`move_down`], [`toggle_expand`] in response
/// to key events resolved via [`ArchiveKeyMap::resolve`].
///
/// [`move_up`]: ArchiveNavModel::move_up
/// [`move_down`]: ArchiveNavModel::move_down
/// [`toggle_expand`]: ArchiveNavModel::toggle_expand
#[derive(Debug)]
pub struct ArchiveNavModel {
    /// Database display name.
    pub db_name: String,
    /// Optional server version string.
    pub version: Option<String>,
    /// Backend label (e.g. `"PostgreSQL"`).
    pub backend_label: String,
    /// All schemas with their expand state.
    pub schemas: Vec<SchemaWithExpand>,
    /// Flattened visible rows (rebuilt after every expand/collapse or filter change).
    pub flat: Vec<FlatItem>,
    /// Index into [`flat`] of the currently highlighted row.
    ///
    /// [`flat`]: ArchiveNavModel::flat
    pub cursor: usize,
    /// Ephemeral status flash (e.g. refresh confirmation).
    pub flash: Option<String>,
    /// Current filter string (empty means no filter).
    pub filter: String,
    /// Whether the filter bar is active (accepting keystrokes).
    pub filter_active: bool,
    /// Cached FK/constraint/index enrichment, keyed by `(schema, table)`.
    pub table_inspections: BTreeMap<(String, String), TableInspection>,
    /// Cached per-column planner statistics, keyed by `(schema, table)`.
    pub column_stats: BTreeMap<(String, String), Vec<ColumnStats>>,
    /// Most recent export result (schema, table, content, format).
    pub last_export: Option<(String, String, String, ExportFormat)>,
    /// In-memory history cache (newest first), loaded at startup.
    pub history_cache: Vec<QueryHistoryEntry>,
    /// Current position in history navigation (0 = most recent).
    /// `None` means the user has not started cycling history.
    pub history_idx: Option<usize>,
    /// In-memory saved-query cache (alphabetical), loaded at startup.
    pub saved_cache: Vec<SavedQuery>,
    /// Database-level extensions `(name, version)`, loaded at startup.
    pub extensions: Vec<(String, String)>,
    /// Whether the extensions group in the nav tree is expanded.
    pub extensions_expanded: bool,
    /// Current panel VSM state.
    pub panel_state: ArchivePanelState,
    /// Proof that the panel state is WCAG-consistent.
    pub panel_proof: Established<ArchivePanelConsistent>,
    /// Current overlay VSM state.
    pub overlay_state: ArchiveOverlayState,
    /// Proof that the overlay state is consistent.
    pub overlay_proof: Established<ArchiveOverlayConsistent>,
    /// Current connection VSM state.
    pub conn_state: ArchiveConnectionState,
    /// Proof that the connection state is consistent.
    pub conn_proof: Established<ArchiveConnectionConsistent>,
    /// Proof that the nav tree state is consistent.
    pub nav_proof: Established<ArchiveNavConsistent>,
}

impl ArchiveNavModel {
    /// Build from a [`NavTree`], selecting the first row.
    pub fn new(nav: NavTree) -> Self {
        let schemas: Vec<SchemaWithExpand> = nav
            .schemas
            .into_iter()
            .map(|e| SchemaWithExpand {
                entry: e,
                expanded: false,
                functions_expanded: false,
                sequences_expanded: false,
                types_expanded: false,
                triggers_expanded: false,
            })
            .collect();

        let mut model = Self {
            db_name: nav.db_name,
            version: nav.version,
            backend_label: nav.backend.to_string(),
            schemas,
            flat: Vec::new(),
            cursor: 0,
            flash: None,
            filter: String::new(),
            filter_active: false,
            table_inspections: BTreeMap::new(),
            column_stats: BTreeMap::new(),
            last_export: None,
            history_cache: Vec::new(),
            history_idx: None,
            saved_cache: Vec::new(),
            extensions: Vec::new(),
            extensions_expanded: false,
            panel_state: ArchivePanelState::default(),
            panel_proof: Established::assert(),
            overlay_state: ArchiveOverlayState::default(),
            overlay_proof: Established::assert(),
            conn_state: ArchiveConnectionState::default(),
            conn_proof: Established::assert(),
            nav_proof: Established::assert(),
        };
        model.rebuild_flat();
        model
    }

    /// Replace the nav tree after a live refresh, preserving cursor position.
    pub fn apply_refresh(&mut self, nav: NavTree) {
        let old_cursor_item = self.flat.get(self.cursor).copied();
        self.schemas = nav
            .schemas
            .into_iter()
            .map(|e| SchemaWithExpand {
                entry: e,
                expanded: false,
                functions_expanded: false,
                sequences_expanded: false,
                types_expanded: false,
                triggers_expanded: false,
            })
            .collect();
        self.db_name = nav.db_name;
        self.version = nav.version;
        self.backend_label = nav.backend.to_string();
        self.rebuild_flat();
        // Try to preserve the old cursor position.
        if let Some(item) = old_cursor_item
            && let Some(pos) = self.flat.iter().position(|f| *f == item)
        {
            self.cursor = pos;
        }
        self.flash = Some("↺ Refreshed".to_string());
    }

    /// Rebuild the flat list from the current expand state, applying any active filter.
    pub fn rebuild_flat(&mut self) {
        self.flat.clear();
        let filter = self.filter.to_lowercase();
        let active = self.filter_active && !filter.is_empty();

        for (i, s) in self.schemas.iter().enumerate() {
            let schema_matches = !active || s.entry.name.to_lowercase().contains(&filter);
            // Include tables that match even if schema doesn't (show their parent schema too).
            let any_table_matches = s
                .entry
                .tables
                .iter()
                .any(|t| !active || t.table_name.to_lowercase().contains(&filter));

            if schema_matches || any_table_matches {
                self.flat.push(FlatItem::Schema(i));
            }

            if s.expanded || (active && any_table_matches) {
                for j in 0..s.entry.tables.len() {
                    let t = &s.entry.tables[j];
                    let table_matches = !active || t.table_name.to_lowercase().contains(&filter);
                    if table_matches {
                        self.flat.push(FlatItem::Table(i, j));
                    }
                }
            }

            // Phase 4 groups — only when explicitly expanded (not just filter mode)
            if s.expanded {
                if !s.entry.functions.is_empty() {
                    self.flat.push(FlatItem::FunctionsGroup(i));
                    if s.functions_expanded {
                        for j in 0..s.entry.functions.len() {
                            self.flat.push(FlatItem::Function(i, j));
                        }
                    }
                }
                if !s.entry.sequences.is_empty() {
                    self.flat.push(FlatItem::SequencesGroup(i));
                    if s.sequences_expanded {
                        for j in 0..s.entry.sequences.len() {
                            self.flat.push(FlatItem::Sequence(i, j));
                        }
                    }
                }
                let type_count =
                    s.entry.enums.len() + s.entry.domains.len() + s.entry.composites.len();
                if type_count > 0 {
                    self.flat.push(FlatItem::TypesGroup(i));
                    if s.types_expanded {
                        for j in 0..s.entry.enums.len() {
                            self.flat.push(FlatItem::TypeEntry(i, 0, j));
                        }
                        for j in 0..s.entry.domains.len() {
                            self.flat.push(FlatItem::TypeEntry(i, 1, j));
                        }
                        for j in 0..s.entry.composites.len() {
                            self.flat.push(FlatItem::TypeEntry(i, 2, j));
                        }
                    }
                }
                if !s.entry.triggers.is_empty() {
                    self.flat.push(FlatItem::TriggersGroup(i));
                    if s.triggers_expanded {
                        for j in 0..s.entry.triggers.len() {
                            self.flat.push(FlatItem::Trigger(i, j));
                        }
                    }
                }
            }
        }
        // Extensions are database-level, appended after all schemas.
        if !self.extensions.is_empty() && !active {
            self.flat.push(FlatItem::ExtensionsGroup);
            if self.extensions_expanded {
                for j in 0..self.extensions.len() {
                    self.flat.push(FlatItem::Extension(j));
                }
            }
        }
        self.cursor = self.cursor.min(self.flat.len().saturating_sub(1));
    }

    /// Move selection one row up (wraps around).
    pub fn move_up(&mut self) {
        if self.flat.is_empty() {
            return;
        }
        self.cursor = if self.cursor == 0 {
            self.flat.len() - 1
        } else {
            self.cursor - 1
        };
    }

    /// Move selection one row down (wraps around).
    pub fn move_down(&mut self) {
        if self.flat.is_empty() {
            return;
        }
        self.cursor = if self.cursor + 1 >= self.flat.len() {
            0
        } else {
            self.cursor + 1
        };
    }

    /// Expand/collapse a schema row or group header, or open a data grid for a table row.
    ///
    /// Returns a [`FetchRequest`] when the caller should initiate an async
    /// data fetch (table row was selected).
    pub fn toggle_expand(&mut self) -> Option<FetchRequest> {
        let &item = self.flat.get(self.cursor)?;
        match item {
            FlatItem::Schema(i) => {
                self.schemas[i].expanded = !self.schemas[i].expanded;
                self.rebuild_flat();
                if let Some(pos) = self
                    .flat
                    .iter()
                    .position(|f| matches!(f, FlatItem::Schema(j) if *j == i))
                {
                    self.cursor = pos;
                }
                None
            }
            FlatItem::Table(si, ti) => {
                let t = &self.schemas[si].entry.tables[ti];
                let schema = t.schema.clone();
                let table = t.table_name.clone();
                self.apply_panel(|s, p| panel_loading(s, p, schema.clone(), table.clone()));
                self.flash = None;
                Some(FetchRequest::PreviewTable { schema, table })
            }
            FlatItem::FunctionsGroup(si) => {
                self.schemas[si].functions_expanded = !self.schemas[si].functions_expanded;
                self.rebuild_flat();
                None
            }
            FlatItem::SequencesGroup(si) => {
                self.schemas[si].sequences_expanded = !self.schemas[si].sequences_expanded;
                self.rebuild_flat();
                None
            }
            FlatItem::TypesGroup(si) => {
                self.schemas[si].types_expanded = !self.schemas[si].types_expanded;
                self.rebuild_flat();
                None
            }
            FlatItem::TriggersGroup(si) => {
                self.schemas[si].triggers_expanded = !self.schemas[si].triggers_expanded;
                self.rebuild_flat();
                None
            }
            FlatItem::ExtensionsGroup => {
                self.extensions_expanded = !self.extensions_expanded;
                self.rebuild_flat();
                None
            }
            // Leaf nodes: Enter does nothing for now (detail panel in a future phase).
            FlatItem::Function(..)
            | FlatItem::Sequence(..)
            | FlatItem::TypeEntry(..)
            | FlatItem::Trigger(..)
            | FlatItem::Extension(..) => None,
        }
    }

    /// Activate the filter bar.
    pub fn open_filter(&mut self) {
        self.filter_active = true;
        self.filter.clear();
        self.rebuild_flat();
    }

    /// Push a character into the filter string.
    pub fn filter_push(&mut self, ch: char) {
        self.filter.push(ch);
        self.cursor = 0;
        self.rebuild_flat();
    }

    /// Delete the last character from the filter string.
    pub fn filter_backspace(&mut self) {
        self.filter.pop();
        self.cursor = 0;
        self.rebuild_flat();
    }

    /// Close and clear the filter bar, returning to normal navigation.
    pub fn close_filter(&mut self) {
        self.filter_active = false;
        self.filter.clear();
        self.rebuild_flat();
    }

    /// Signal a refresh (async backends send a [`FetchRequest::Refresh`]).
    pub fn request_refresh(&mut self) -> FetchRequest {
        self.flash = Some("↺ Refreshing…".to_string());
        FetchRequest::Refresh
    }

    /// Toggle the help overlay.
    pub fn toggle_help(&mut self) {
        if matches!(self.overlay_state, ArchiveOverlayState::HelpOpen) {
            self.apply_overlay(close_overlay);
        } else {
            self.apply_overlay(open_help);
        }
        self.flash = None;
    }

    /// Return the currently selected `FlatItem`, if any.
    pub fn selected(&self) -> Option<FlatItem> {
        self.flat.get(self.cursor).copied()
    }

    /// Return `(schema, table)` for the currently selected table row.
    ///
    /// Returns `None` if the cursor is on a schema row or the flat list is empty.
    pub fn selected_schema_table(&self) -> Option<(String, String)> {
        match self.selected()? {
            FlatItem::Table(si, ti) => {
                let entry = &self.schemas[si].entry;
                Some((entry.name.clone(), entry.tables[ti].table_name.clone()))
            }
            _ => None,
        }
    }

    /// If the cursor is on a table row, return an [`InspectTable`] request.
    ///
    /// Call this whenever the cursor moves to a table row to eagerly load
    /// FK/constraint/index enrichment data in the background.
    ///
    /// [`InspectTable`]: FetchRequest::InspectTable
    pub fn inspect_request(&self) -> Option<FetchRequest> {
        match self.selected()? {
            FlatItem::Table(si, ti) => {
                let t = &self.schemas[si].entry.tables[ti];
                Some(FetchRequest::InspectTable {
                    schema: t.schema.clone(),
                    table: t.table_name.clone(),
                })
            }
            _ => None,
        }
    }

    /// If the cursor is on a table row, return a [`GetDdl`] request.
    ///
    /// Triggered by the `d` key binding.
    ///
    /// [`GetDdl`]: FetchRequest::GetDdl
    pub fn ddl_request(&self) -> Option<FetchRequest> {
        match self.selected()? {
            FlatItem::Table(si, ti) => {
                let t = &self.schemas[si].entry.tables[ti];
                Some(FetchRequest::GetDdl {
                    schema: t.schema.clone(),
                    table: t.table_name.clone(),
                })
            }
            _ => None,
        }
    }

    /// If the cursor is on a table row, return a [`GetColumnStats`] request.
    ///
    /// Triggered eagerly alongside [`inspect_request`].
    ///
    /// [`GetColumnStats`]: FetchRequest::GetColumnStats
    /// [`inspect_request`]: ArchiveNavModel::inspect_request
    pub fn stats_request(&self) -> Option<FetchRequest> {
        match self.selected()? {
            FlatItem::Table(si, ti) => {
                let t = &self.schemas[si].entry.tables[ti];
                Some(FetchRequest::GetColumnStats {
                    schema: t.schema.clone(),
                    table: t.table_name.clone(),
                })
            }
            _ => None,
        }
    }

    /// If the cursor is on a table row, return an [`ExplainSql`] request for a
    /// simple `SELECT *` preview.
    ///
    /// Triggered by the `e` key binding.
    ///
    /// [`ExplainSql`]: FetchRequest::ExplainSql
    pub fn explain_request(&self) -> Option<FetchRequest> {
        match self.selected()? {
            FlatItem::Table(si, ti) => {
                let t = &self.schemas[si].entry.tables[ti];
                Some(FetchRequest::ExplainSql {
                    schema: t.schema.clone(),
                    table: t.table_name.clone(),
                    sql: format!(
                        r#"SELECT * FROM "{}"."{}" LIMIT 100"#,
                        t.schema.replace('"', ""),
                        t.table_name.replace('"', "")
                    ),
                })
            }
            _ => None,
        }
    }

    /// If the active panel contains a data grid, return an [`ExportData`] request
    /// for the given format.
    ///
    /// Returns `None` when there is no data to export.
    ///
    /// [`ExportData`]: FetchRequest::ExportData
    pub fn export_request(&self, format: ExportFormat) -> Option<FetchRequest> {
        match &self.panel_state {
            ArchivePanelState::DataGrid {
                schema,
                table,
                result,
                ..
            } => Some(FetchRequest::ExportData {
                schema: schema.clone(),
                table: table.clone(),
                result: result.clone(),
                format,
            }),
            ArchivePanelState::SqlEditor {
                result: Some(r), ..
            } => Some(FetchRequest::ExportData {
                schema: String::new(),
                table: "(query result)".to_string(),
                result: r.clone(),
                format,
            }),
            _ => None,
        }
    }

    /// Store enrichment returned by an [`InspectTable`] fetch.
    ///
    /// [`InspectTable`]: FetchRequest::InspectTable
    pub fn store_inspection(
        &mut self,
        schema: impl Into<String>,
        table: impl Into<String>,
        inspection: crate::archive::TableInspection,
    ) {
        self.table_inspections
            .insert((schema.into(), table.into()), inspection);
    }

    /// Look up cached enrichment for `(schema, table)`.
    pub fn inspection(
        &self,
        schema: &str,
        table: &str,
    ) -> Option<&crate::archive::TableInspection> {
        self.table_inspections
            .get(&(schema.to_string(), table.to_string()))
    }

    /// Store column statistics returned by a [`GetColumnStats`] fetch.
    ///
    /// [`GetColumnStats`]: FetchRequest::GetColumnStats
    pub fn store_column_stats(
        &mut self,
        schema: impl Into<String>,
        table: impl Into<String>,
        stats: Vec<crate::archive::ColumnStats>,
    ) {
        self.column_stats
            .insert((schema.into(), table.into()), stats);
    }

    /// Look up cached column statistics for `(schema, table)`.
    pub fn column_stats_for(
        &self,
        schema: &str,
        table: &str,
    ) -> Option<&Vec<crate::archive::ColumnStats>> {
        self.column_stats
            .get(&(schema.to_string(), table.to_string()))
    }

    /// Key bindings for the default navigation mode, derived from
    /// [`ArchiveKeyMap::default_map`].
    ///
    /// Every frontend should drive its status-bar rendering from this
    /// single source of truth.
    pub fn bindings() -> Vec<KeyBinding> {
        ArchiveKeyMap::default_map()
            .to_status_bar(KeyMapMode::Default)
            .bindings
    }

    /// Determine which UI mode is currently active.
    ///
    /// Used by frontends to look up the correct key bindings via
    /// [`ArchiveKeyMap::resolve`].
    pub fn current_mode(&self) -> KeyMapMode {
        if self.filter_active {
            return KeyMapMode::Filter;
        }
        if matches!(
            self.overlay_state,
            ArchiveOverlayState::SavePromptOpen { .. }
        ) {
            return KeyMapMode::SavePrompt;
        }
        if matches!(
            self.overlay_state,
            ArchiveOverlayState::SavedBrowserOpen { .. }
        ) {
            return KeyMapMode::SavedBrowser;
        }
        if matches!(
            self.overlay_state,
            ArchiveOverlayState::ExportPickerOpen { .. }
        ) {
            return KeyMapMode::ExportPicker;
        }
        if matches!(self.panel_state, ArchivePanelState::SqlEditor { .. }) {
            return KeyMapMode::SqlEditor;
        }
        KeyMapMode::Default
    }

    /// Return the SQL text of the currently-focused saved query, if any.
    ///
    /// Used by frontends to implement [`crate::archive::ArchiveAction::SavedBrowserSelect`].
    pub fn load_focused_saved_query_text(&self) -> Option<String> {
        if let ArchiveOverlayState::SavedBrowserOpen { entries, idx } = &self.overlay_state {
            entries.get(*idx).map(|q| q.sql.clone())
        } else {
            None
        }
    }

    /// Remove the currently-focused saved query from the local cache.
    ///
    /// Returns `(id, name)` of the removed entry so the caller can enqueue a
    /// deletion on the persistent store.  Returns `None` if the cache is empty.
    ///
    /// Used by frontends to implement [`crate::archive::ArchiveAction::SavedBrowserDelete`].
    pub fn remove_focused_saved_query(&mut self) -> Option<(i64, String)> {
        let idx = match &self.overlay_state {
            ArchiveOverlayState::SavedBrowserOpen { idx, .. } => *idx,
            _ => return None,
        };
        if idx >= self.saved_cache.len() {
            return None;
        }
        let q = self.saved_cache.remove(idx);
        if let ArchiveOverlayState::SavedBrowserOpen {
            entries,
            idx: curr_idx,
        } = &mut self.overlay_state
        {
            if idx < entries.len() {
                entries.remove(idx);
            }
            if *curr_idx > 0 && *curr_idx >= entries.len() {
                *curr_idx -= 1;
            }
        }
        Some((q.id, q.name))
    }

    /// Step backward through history (toward older entries) in the SQL editor.
    ///
    /// If the panel is a `SqlEditor`, replaces the current SQL with the entry
    /// at `history_idx`.  Returns `true` if the SQL was updated.
    pub fn history_prev(&mut self) -> bool {
        if self.history_cache.is_empty() {
            return false;
        }
        let next_idx = match self.history_idx {
            None => 0,
            Some(i) => (i + 1).min(self.history_cache.len().saturating_sub(1)),
        };
        self.history_idx = Some(next_idx);
        self.apply_history_entry(next_idx)
    }

    /// Step forward through history (toward more recent entries) in the SQL editor.
    ///
    /// Returns `true` if the SQL was updated.
    pub fn history_next(&mut self) -> bool {
        let idx = match self.history_idx {
            None => return false,
            Some(0) => {
                self.history_idx = None;
                return false;
            }
            Some(i) => i - 1,
        };
        self.history_idx = Some(idx);
        self.apply_history_entry(idx)
    }

    fn apply_history_entry(&mut self, idx: usize) -> bool {
        let sql = match self.history_cache.get(idx) {
            Some(e) => e.sql.clone(),
            None => return false,
        };
        if let ArchivePanelState::SqlEditor { text, .. } = &mut self.panel_state {
            *text = sql;
        } else {
            self.apply_panel(|s, p| open_sql_editor(s, p, sql));
        }
        true
    }

    // ── Overlay state helpers ─────────────────────────────────────────────────

    /// Toggle the export format picker overlay.
    pub fn toggle_export_picker(&mut self) {
        if matches!(
            self.overlay_state,
            ArchiveOverlayState::ExportPickerOpen { .. }
        ) {
            self.apply_overlay(close_overlay);
        } else {
            let formats: Vec<ExportFormat> = ExportFormat::iter().collect();
            self.apply_overlay(|s, p| open_export_picker(s, p, formats));
        }
    }

    /// Move the export picker selection up.
    pub fn export_picker_prev(&mut self) {
        self.apply_overlay(picker_move_up);
    }

    /// Move the export picker selection down.
    pub fn export_picker_next(&mut self) {
        self.apply_overlay(picker_move_down);
    }

    /// Confirm the export picker — returns the chosen [`ExportFormat`].
    pub fn confirm_export_picker(&mut self) -> ExportFormat {
        if let ArchiveOverlayState::ExportPickerOpen { idx, ref formats } = self.overlay_state {
            let format = formats.get(idx).copied().unwrap_or(ExportFormat::Csv);
            self.apply_overlay(close_overlay);
            format
        } else {
            ExportFormat::Csv
        }
    }

    /// Open the save-name prompt (SQL editor context).
    pub fn open_save_prompt(&mut self) {
        self.apply_overlay(vsm_open_save_prompt);
    }

    /// Append a character to the save-name prompt text.
    pub fn save_prompt_push(&mut self, ch: char) {
        self.apply_overlay(|s, p| prompt_push(s, p, ch));
    }

    /// Delete the last character from the save-name prompt text.
    pub fn save_prompt_backspace(&mut self) {
        self.apply_overlay(prompt_backspace);
    }

    /// Close the save-name prompt, discarding any typed text.
    pub fn close_save_prompt(&mut self) {
        self.apply_overlay(close_overlay);
    }

    /// Consume the save-name prompt text, closing the prompt.
    ///
    /// Returns `None` if the prompt is not active or the text is blank.
    pub fn take_save_prompt(&mut self) -> Option<String> {
        if let ArchiveOverlayState::SavePromptOpen { text } = &self.overlay_state {
            let name = text.trim().to_string();
            self.apply_overlay(close_overlay);
            if name.is_empty() { None } else { Some(name) }
        } else {
            None
        }
    }

    /// Toggle the saved-queries browser overlay.
    pub fn toggle_saved_browser(&mut self) {
        if matches!(
            self.overlay_state,
            ArchiveOverlayState::SavedBrowserOpen { .. }
        ) {
            self.apply_overlay(close_overlay);
        } else {
            let entries = self.saved_cache.clone();
            self.apply_overlay(|s, p| open_saved_browser(s, p, entries));
        }
    }

    // ── Phase 5 — Monitor panel ───────────────────────────────────────────────

    /// Open the live monitor panel (or close it if already open).
    ///
    /// Returns `Some(FetchRequest::FetchMonitor)` when opening so the caller
    /// can dispatch a data fetch; returns `None` when closing.
    pub fn toggle_monitor_panel(&mut self) -> Option<FetchRequest> {
        if matches!(self.panel_state, ArchivePanelState::MonitorView { .. }) {
            self.apply_panel(column_detail);
            return None;
        }
        self.panel_state = ArchivePanelState::MonitorView {
            snapshot: MonitorSnapshot::default(),
            loading: true,
            display_mode: Default::default(),
        };
        let schema = self
            .selected_schema_name()
            .unwrap_or_else(|| "public".to_string());
        Some(FetchRequest::FetchMonitor { schema })
    }

    /// Apply a completed monitoring snapshot to the monitor panel.
    ///
    /// No-ops if the panel is no longer in `MonitorPanel` mode (e.g. the user
    /// navigated away before the fetch completed).
    pub fn apply_monitor_snapshot(&mut self, mut snapshot: MonitorSnapshot) {
        if let ArchivePanelState::MonitorView {
            snapshot: s,
            loading,
            ..
        } = &mut self.panel_state
        {
            // Preserve the active tab the user may have selected.
            snapshot.active_tab = s.active_tab.clone();
            *s = snapshot;
            *loading = false;
        }
    }

    // ── Phase 5.2 — Admin panel ───────────────────────────────────────────────

    /// Open the admin panel (or close it if already open).
    ///
    /// Returns `Some(FetchRequest::FetchAdmin)` when opening so the caller can
    /// dispatch a data fetch; returns `None` when closing.
    pub fn toggle_admin_panel(&mut self) -> Option<FetchRequest> {
        if matches!(self.panel_state, ArchivePanelState::AdminView { .. }) {
            self.apply_panel(column_detail);
            return None;
        }
        self.panel_state = ArchivePanelState::AdminView {
            snapshot: AdminSnapshot::default(),
            loading: true,
            display_mode: Default::default(),
        };
        Some(FetchRequest::FetchAdmin)
    }

    /// Apply a completed admin snapshot to the admin panel.
    pub fn apply_admin_snapshot(&mut self, snapshot: AdminSnapshot) {
        if let ArchivePanelState::AdminView {
            snapshot: s,
            loading,
            ..
        } = &mut self.panel_state
        {
            *s = snapshot;
            *loading = false;
        }
    }

    /// Advance the active tab in the admin or monitor panel.
    pub fn admin_tab_next(&mut self) {
        match &mut self.panel_state {
            ArchivePanelState::AdminView { snapshot, .. } => {
                snapshot.active_tab = snapshot.active_tab.next();
            }
            ArchivePanelState::MonitorView { snapshot, .. } => {
                snapshot.active_tab = snapshot.active_tab.next();
            }
            _ => {}
        }
    }

    /// Step back to the previous tab in the admin or monitor panel.
    pub fn admin_tab_prev(&mut self) {
        match &mut self.panel_state {
            ArchivePanelState::AdminView { snapshot, .. } => {
                snapshot.active_tab = snapshot.active_tab.prev();
            }
            ArchivePanelState::MonitorView { snapshot, .. } => {
                snapshot.active_tab = snapshot.active_tab.prev();
            }
            _ => {}
        }
    }

    /// Return the name of the currently selected schema (works for schema,
    /// table, function, sequence, type, and trigger nodes).
    pub fn selected_schema_name(&self) -> Option<String> {
        use crate::archive::nav_model::FlatItem;
        match self.selected()? {
            FlatItem::Schema(si) => Some(self.schemas[si].entry.name.clone()),
            FlatItem::Table(si, _)
            | FlatItem::FunctionsGroup(si)
            | FlatItem::SequencesGroup(si)
            | FlatItem::TypesGroup(si)
            | FlatItem::TriggersGroup(si)
            | FlatItem::Function(si, _)
            | FlatItem::Sequence(si, _)
            | FlatItem::Trigger(si, _)
            | FlatItem::TypeEntry(si, _, _) => Some(self.schemas[si].entry.name.clone()),
            FlatItem::ExtensionsGroup | FlatItem::Extension(_) => None,
        }
    }

    /// Open the ERD panel for the currently selected schema, or close it if
    /// already open.
    ///
    /// Returns `Some(FetchRequest::FetchErd)` when opening so the caller can
    /// dispatch a background fetch.  Returns `None` on close.
    pub fn toggle_erd_panel(&mut self) -> Option<FetchRequest> {
        let schema = self.selected_schema_name()?;
        if matches!(self.panel_state, ArchivePanelState::ErdView { .. }) {
            self.apply_panel(column_detail);
            return None;
        }
        self.panel_state = ArchivePanelState::ErdView {
            schema: schema.clone(),
            diagram: ErdDiagram::default(),
            layout: None,
            loading: true,
            display_mode: Default::default(),
        };
        Some(FetchRequest::FetchErd { schema })
    }

    /// Apply a completed ERD diagram to the ERD panel.
    pub fn apply_erd_diagram(&mut self, diagram: ErdDiagram) {
        if let ArchivePanelState::ErdView {
            diagram: d,
            layout,
            loading,
            ..
        } = &mut self.panel_state
        {
            let new_layout = ErdLayout::from_diagram(&diagram);
            *d = diagram;
            *layout = Some(new_layout);
            *loading = false;
        }
    }

    // ── Phase 8 — Constraint / Index panels ──────────────────────────────────

    /// Open the constraint panel for the selected table, or close it.
    ///
    /// If the enrichment data is already cached, populates the panel
    /// immediately (no `FetchRequest` needed).  Otherwise returns
    /// `Some(FetchRequest::FetchConstraints)` for the caller to dispatch.
    pub fn toggle_constraint_panel(&mut self) -> Option<FetchRequest> {
        if matches!(self.panel_state, ArchivePanelState::ConstraintView { .. }) {
            self.apply_panel(column_detail);
            return None;
        }
        let (schema, table) = self.selected_schema_table()?;
        if let Some(insp) = self.table_inspections.get(&(schema.clone(), table.clone())) {
            let constraints = insp.constraints.clone();
            self.panel_state = ArchivePanelState::ConstraintView {
                schema,
                table,
                constraints,
                loading: false,
                display_mode: Default::default(),
            };
            return None;
        }
        self.panel_state = ArchivePanelState::ConstraintView {
            schema: schema.clone(),
            table: table.clone(),
            constraints: Vec::new(),
            loading: true,
            display_mode: Default::default(),
        };
        Some(FetchRequest::FetchConstraints { schema, table })
    }

    /// Apply a completed constraints list to the constraint panel.
    pub fn apply_constraints(
        &mut self,
        schema: String,
        table: String,
        constraints: Vec<ConstraintDescriptor>,
    ) {
        if let ArchivePanelState::ConstraintView {
            schema: ps,
            table: pt,
            constraints: c,
            loading,
            ..
        } = &mut self.panel_state
            && ps == &schema
            && pt == &table
        {
            *c = constraints;
            *loading = false;
        }
    }

    /// Open the index panel for the selected table, or close it if already open.
    pub fn toggle_index_panel(&mut self) -> Option<FetchRequest> {
        if matches!(self.panel_state, ArchivePanelState::IndexView { .. }) {
            self.apply_panel(column_detail);
            return None;
        }
        let (schema, table) = self.selected_schema_table()?;
        if let Some(insp) = self.table_inspections.get(&(schema.clone(), table.clone())) {
            let indexes = insp.indexes.clone();
            self.panel_state = ArchivePanelState::IndexView {
                schema,
                table,
                indexes,
                loading: false,
                display_mode: Default::default(),
            };
            return None;
        }
        self.panel_state = ArchivePanelState::IndexView {
            schema: schema.clone(),
            table: table.clone(),
            indexes: Vec::new(),
            loading: true,
            display_mode: Default::default(),
        };
        Some(FetchRequest::FetchIndexes { schema, table })
    }

    /// Apply a completed index list to the index panel.
    pub fn apply_indexes(&mut self, schema: String, table: String, indexes: Vec<IndexDescriptor>) {
        if let ArchivePanelState::IndexView {
            schema: ps,
            table: pt,
            indexes: ix,
            loading,
            ..
        } = &mut self.panel_state
            && ps == &schema
            && pt == &table
        {
            *ix = indexes;
            *loading = false;
        }
    }

    // ── Phase 13 — Connection editor ─────────────────────────────────────────

    /// Open the connection editor panel for `profile`, or close it if already open.
    pub fn toggle_connection_editor(&mut self, profile: ConnectionProfile) {
        if matches!(self.panel_state, ArchivePanelState::ConnectionEdit { .. }) {
            self.apply_panel(column_detail);
        } else {
            self.apply_panel(|s, p| open_connection_editor(s, p, profile, Default::default()));
        }
    }

    // ── Phase 8 — Data-grid pagination ────────────────────────────────────────

    /// Advance to the next page of the data grid.
    pub fn page_next(&mut self) {
        if let ArchivePanelState::DataGrid { page, .. } = &mut self.panel_state {
            *page += 1;
        }
    }

    /// Go back to the previous page of the data grid.
    pub fn page_prev(&mut self) {
        if let ArchivePanelState::DataGrid { page, .. } = &mut self.panel_state {
            *page = page.saturating_sub(1);
        }
    }

    /// Jump to the first page of the data grid.
    pub fn page_first(&mut self) {
        if let ArchivePanelState::DataGrid { page, .. } = &mut self.panel_state {
            *page = 0;
        }
    }

    /// Jump to the last page of the data grid.
    #[cfg(not(kani))]
    pub fn page_last(&mut self) {
        if let ArchivePanelState::DataGrid { result, page, .. } = &mut self.panel_state {
            const PAGE_SIZE: u32 = 100;
            let total = result.rows.rows.len() as u32;
            *page = if total == 0 {
                0
            } else {
                (total - 1) / PAGE_SIZE
            };
        }
    }

    /// Move the saved-browser selection up.
    pub fn saved_browser_prev(&mut self) {
        self.apply_overlay(saved_browser_up);
    }

    /// Move the saved-browser selection down.
    pub fn saved_browser_next(&mut self) {
        self.apply_overlay(saved_browser_down);
    }

    /// Set the nav filter string and rebuild the flat list.
    ///
    /// Used by the browser frontend to apply query-param filters.
    pub fn set_filter_str(&mut self, filter: &str) {
        self.filter = filter.to_string();
        self.filter_active = !filter.is_empty();
        self.rebuild_flat();
    }

    // ── Phase 3.1 — Inline Row Edit ──────────────────────────────────────────

    /// Move the grid cursor up one row. Returns `true` if in a data grid.
    #[cfg(not(kani))]
    pub fn grid_row_up(&mut self) -> bool {
        if let ArchivePanelState::DataGrid {
            result, grid_row, ..
        } = &mut self.panel_state
        {
            if *grid_row > 0 {
                *grid_row -= 1;
            } else {
                *grid_row = result.rows.rows.len().saturating_sub(1);
            }
            return true;
        }
        false
    }

    /// Move the grid cursor down one row. Returns `true` if in a data grid.
    #[cfg(not(kani))]
    pub fn grid_row_down(&mut self) -> bool {
        if let ArchivePanelState::DataGrid {
            result, grid_row, ..
        } = &mut self.panel_state
        {
            let max = result.rows.rows.len().saturating_sub(1);
            *grid_row = if *grid_row >= max { 0 } else { *grid_row + 1 };
            return true;
        }
        false
    }

    /// Move the grid cursor left one column. Returns `true` if in a data grid.
    #[cfg(not(kani))]
    pub fn grid_col_left(&mut self) -> bool {
        if let ArchivePanelState::DataGrid {
            result, grid_col, ..
        } = &mut self.panel_state
        {
            if *grid_col > 0 {
                *grid_col -= 1;
            } else {
                *grid_col = result.columns.len().saturating_sub(1);
            }
            return true;
        }
        false
    }

    /// Move the grid cursor right one column. Returns `true` if in a data grid.
    #[cfg(not(kani))]
    pub fn grid_col_right(&mut self) -> bool {
        if let ArchivePanelState::DataGrid {
            result, grid_col, ..
        } = &mut self.panel_state
        {
            let max = result.columns.len().saturating_sub(1);
            *grid_col = if *grid_col >= max { 0 } else { *grid_col + 1 };
            return true;
        }
        false
    }

    /// Enter row-edit mode on the data grid. Returns `true` if successful.
    pub fn begin_edit_mode(&mut self) -> bool {
        if let ArchivePanelState::DataGrid { edit_state, .. } = &mut self.panel_state
            && edit_state.is_none()
        {
            *edit_state = Some(RowEditState::new());
            return true;
        }
        false
    }

    /// Start editing the currently focused cell. Returns `true` if successful.
    #[cfg(not(kani))]
    pub fn begin_cell_edit(&mut self) -> bool {
        if let ArchivePanelState::DataGrid {
            result,
            grid_row,
            grid_col,
            edit_state: Some(es),
            ..
        } = &mut self.panel_state
        {
            if es.editing_cell.is_some() {
                return false;
            }
            let row_idx = *grid_row;
            let col_idx = *grid_col;
            if let Some(row) = result.rows.rows.get(row_idx)
                && let Some((_, val)) = row.0.get(col_idx)
            {
                es.editing_cell = Some((row_idx, col_idx));
                es.input_buffer = format!("{val:?}");
                return true;
            }
        }
        false
    }

    /// Push a character into the cell-edit or insert-row buffer.
    pub fn cell_edit_push(&mut self, ch: char) {
        if let ArchivePanelState::DataGrid {
            edit_state: Some(es),
            ..
        } = &mut self.panel_state
            && (es.editing_cell.is_some() || es.inserting_row.is_some())
        {
            es.input_buffer.push(ch);
        }
    }

    /// Remove the last character from the cell-edit or insert-row buffer.
    pub fn cell_edit_pop(&mut self) {
        if let ArchivePanelState::DataGrid {
            edit_state: Some(es),
            ..
        } = &mut self.panel_state
            && (es.editing_cell.is_some() || es.inserting_row.is_some())
        {
            es.input_buffer.pop();
        }
    }

    /// Stage the current cell edit as a pending update. Returns `true` if an edit was staged.
    #[cfg(not(kani))]
    pub fn stage_cell_edit(&mut self) -> bool {
        if let ArchivePanelState::DataGrid {
            schema,
            table,
            result,
            edit_state: Some(es),
            ..
        } = &mut self.panel_state
        {
            let Some((row_idx, col_idx)) = es.editing_cell.take() else {
                return false;
            };
            let new_value = std::mem::take(&mut es.input_buffer);
            let Some(row) = result.rows.rows.get(row_idx) else {
                return false;
            };
            let Some(col_desc) = result.columns.get(col_idx) else {
                return false;
            };

            let pk_values: Vec<(String, String)> = result
                .columns
                .iter()
                .zip(row.0.iter())
                .filter(|(c, _)| c.is_primary_key)
                .map(|(c, (_, v))| (c.name.clone(), format!("{v:?}")))
                .collect();

            es.pending_edits.push(StagedEdit {
                schema: schema.clone(),
                table: table.clone(),
                kind: crate::archive::RowEditKind::Update {
                    pk_values,
                    column: col_desc.name.clone(),
                    new_value,
                },
            });
            return true;
        }
        false
    }

    /// Begin inserting a new row. Returns `true` if successful.
    #[cfg(not(kani))]
    pub fn begin_insert_row(&mut self) -> bool {
        if let ArchivePanelState::DataGrid {
            result,
            edit_state: Some(es),
            ..
        } = &mut self.panel_state
        {
            if es.inserting_row.is_some() {
                return false;
            }
            let form: Vec<(String, String)> = result
                .columns
                .iter()
                .map(|c| (c.name.clone(), String::new()))
                .collect();
            es.inserting_row = Some(form);
            es.insert_col_cursor = 0;
            es.input_buffer.clear();
            return true;
        }
        false
    }

    /// Advance to the next column when filling in a new row's insert form. Returns `true` when the form is complete.
    pub fn insert_row_next_col(&mut self) -> bool {
        if let ArchivePanelState::DataGrid {
            edit_state: Some(es),
            ..
        } = &mut self.panel_state
            && let Some(form) = &mut es.inserting_row
        {
            let cursor = es.insert_col_cursor;
            if let Some(slot) = form.get_mut(cursor) {
                slot.1 = std::mem::take(&mut es.input_buffer);
            }
            es.insert_col_cursor += 1;
            if es.insert_col_cursor >= form.len() {
                es.insert_col_cursor = 0;
                return true;
            }
        }
        false
    }

    /// Stage the current insert-row form as a pending insert. Returns `true` if successful.
    pub fn stage_insert_row(&mut self) -> bool {
        if let ArchivePanelState::DataGrid {
            schema,
            table,
            edit_state: Some(es),
            ..
        } = &mut self.panel_state
        {
            let Some(mut form) = es.inserting_row.take() else {
                return false;
            };
            let cursor = es.insert_col_cursor;
            if let Some(slot) = form.get_mut(cursor) {
                slot.1 = std::mem::take(&mut es.input_buffer);
            }
            es.insert_col_cursor = 0;
            es.pending_edits.push(StagedEdit {
                schema: schema.clone(),
                table: table.clone(),
                kind: crate::archive::RowEditKind::Insert { row: form },
            });
            return true;
        }
        false
    }

    /// Mark the focused row for deletion. Returns `true` if the row was marked.
    #[cfg(not(kani))]
    pub fn stage_delete_row(&mut self) -> bool {
        if let ArchivePanelState::DataGrid {
            schema,
            table,
            result,
            grid_row,
            edit_state: Some(es),
            ..
        } = &mut self.panel_state
        {
            let row_idx = *grid_row;
            if es.rows_marked_deleted.contains(&row_idx) {
                return false; // already marked
            }
            let Some(row) = result.rows.rows.get(row_idx) else {
                return false;
            };

            let pk_values: Vec<(String, String)> = result
                .columns
                .iter()
                .zip(row.0.iter())
                .filter(|(c, _)| c.is_primary_key)
                .map(|(c, (_, v))| (c.name.clone(), format!("{v:?}")))
                .collect();

            es.rows_marked_deleted.push(row_idx);
            es.pending_edits.push(StagedEdit {
                schema: schema.clone(),
                table: table.clone(),
                kind: crate::archive::RowEditKind::Delete { pk_values },
            });
            return true;
        }
        false
    }

    /// Cancel all staged edits and exit edit mode.
    ///
    pub fn discard_edit_mode(&mut self) -> bool {
        if let ArchivePanelState::DataGrid { edit_state, .. } = &mut self.panel_state
            && edit_state.is_some()
        {
            *edit_state = None;
            return true;
        }
        false
    }

    /// Drain the pending edits from edit mode, returning `(schema, table, edits)` if any exist.
    pub fn take_staged_edits(&mut self) -> Option<(String, String, Vec<StagedEdit>)> {
        if let ArchivePanelState::DataGrid {
            schema,
            table,
            edit_state,
            ..
        } = &mut self.panel_state
            && let Some(es) = edit_state.take()
            && !es.pending_edits.is_empty()
        {
            return Some((schema.clone(), table.clone(), es.pending_edits));
        }
        None
    }

    /// Move the cursor to the first flat item matching the given schema + table.
    ///
    /// Returns `true` if the item was found and the cursor moved.  Used by the
    /// browser frontend to update selection highlighting after an API preview.
    pub fn select_table(&mut self, schema: &str, table: &str) -> bool {
        for (idx, item) in self.flat.iter().enumerate() {
            if let FlatItem::Table(si, ti) = item {
                let s = &self.schemas[*si].entry.name;
                let t = &self.schemas[*si].entry.tables[*ti].table_name;
                if s == schema && t == table {
                    self.cursor = idx;
                    return true;
                }
            }
        }
        false
    }

    // ── VSM helper methods ────────────────────────────────────────────────────

    fn apply_panel<F>(&mut self, f: F)
    where
        F: FnOnce(
            ArchivePanelState,
            Established<ArchivePanelConsistent>,
        ) -> (ArchivePanelState, Established<ArchivePanelConsistent>),
    {
        let proof = std::mem::replace(&mut self.panel_proof, Established::assert());
        let state = std::mem::take(&mut self.panel_state);
        let (new_state, new_proof) = f(state, proof);
        self.panel_state = new_state;
        self.panel_proof = new_proof;
    }

    fn apply_overlay<F>(&mut self, f: F)
    where
        F: FnOnce(
            ArchiveOverlayState,
            Established<ArchiveOverlayConsistent>,
        ) -> (ArchiveOverlayState, Established<ArchiveOverlayConsistent>),
    {
        let proof = std::mem::replace(&mut self.overlay_proof, Established::assert());
        let state = std::mem::take(&mut self.overlay_state);
        let (new_state, new_proof) = f(state, proof);
        self.overlay_state = new_state;
        self.overlay_proof = new_proof;
    }

    // ── Public panel convenience methods ─────────────────────────────────────

    /// Returns `true` when the panel is showing a data grid.
    #[tracing::instrument(skip(self))]
    pub fn is_data_grid(&self) -> bool {
        matches!(self.panel_state, ArchivePanelState::DataGrid { .. })
    }

    /// Set the panel to ColumnDetail.
    #[tracing::instrument(skip(self))]
    pub fn panel_go_column_detail(&mut self) {
        self.apply_panel(column_detail);
    }

    /// Apply a completed data grid result.
    #[tracing::instrument(skip(self))]
    pub fn panel_set_data_grid(&mut self, schema: String, table: String, result: QueryResult) {
        self.apply_panel(|s, p| data_grid_ready(s, p, schema, table, result, Default::default()));
    }

    /// Set an explain view (promoting to comparison if already showing one).
    #[tracing::instrument(skip(self))]
    pub fn panel_set_explain(&mut self, schema: String, table: String, root: ExplainPlan) {
        self.apply_panel(|s, p| explain_ready(s, p, schema, table, root, Default::default()));
    }

    /// Set a DDL view.
    #[tracing::instrument(skip(self))]
    pub fn panel_set_ddl(&mut self, schema: String, table: String, ddl_text: String) {
        let descriptor = DdlDescriptor {
            schema: schema.clone(),
            object_name: table.clone(),
            ddl: ddl_text,
        };
        self.apply_panel(|s, p| ddl_ready(s, p, schema, table, descriptor, Default::default()));
    }

    /// Set to loading state.
    #[tracing::instrument(skip(self))]
    pub fn panel_set_loading(&mut self, schema: String, label: String) {
        self.apply_panel(|s, p| panel_loading(s, p, schema, label));
    }

    /// Set an error view.
    #[tracing::instrument(skip(self))]
    pub fn panel_set_error(&mut self, message: String) {
        self.apply_panel(|s, p| panel_error(s, p, message));
    }

    /// Open the SQL editor.
    #[tracing::instrument(skip(self))]
    pub fn panel_open_sql_editor(&mut self, text: String) {
        self.apply_panel(|s, p| open_sql_editor(s, p, text));
    }

    // ── IR pipeline ───────────────────────────────────────────────────────────

    /// Build a fully-described [`elicit_ui::VerifiedTree`] from the current model state.
    ///
    /// This is the **only** authorised way to obtain an
    /// [`elicit_ui::IrSourced`] proof token.  Every frontend renderer must
    /// call this before rendering — the token is a compile-time contract that
    /// all frontends share the same AccessKit IR source.
    ///
    /// The returned tree encodes the full page structure:
    /// - Toolbar with all action buttons
    /// - Navigation panel (filter input + flat tree)
    /// - Content panel (current PanelMode)
    /// - Status bar
    #[cfg(not(kani))]
    #[tracing::instrument(skip(self))]
    pub fn to_verified_tree(
        &self,
    ) -> crate::archive::ArchiveResult<(
        elicit_ui::VerifiedTree,
        elicitation::Established<elicit_ui::IrSourced>,
    )> {
        use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};
        use elicit_ui::{VerifiedTree, Viewport};
        use std::collections::BTreeMap;

        let mut nodes: BTreeMap<AkNodeId, AkNode> = BTreeMap::new();
        let mut counter: u64 = 1; // NodeId(0) reserved for Window

        // ── toolbar ───────────────────────────────────────────────────────────
        let toolbar_id = self.build_toolbar_nodes(&mut nodes, &mut counter);

        // ── nav panel ─────────────────────────────────────────────────────────
        let filter_id = AkNodeId::from(counter);
        counter += 1;
        let mut filter_node = AkNode::new(AkRole::SearchInput);
        filter_node.set_label("nav filter".to_string());
        filter_node.set_value(self.filter.clone());
        filter_node.set_description(
            "id=nav-filter;name=filter;\
             hx-get=/api/nav;hx-target=#nav-tree;hx-swap=outerHTML;\
             hx-trigger=keyup changed delay:250ms;autocomplete=off;\
             placeholder=/ filter..."
                .to_string(),
        );
        nodes.insert(filter_id, filter_node);

        let (nav_tree_id, nav_item_nodes) = self.build_nav_item_nodes(&mut nodes, &mut counter);

        let mut nav_tree_node = AkNode::new(AkRole::Tree);
        nav_tree_node.set_label(self.db_name.clone());
        nav_tree_node.set_description("id=nav-tree".to_string());
        nav_tree_node.set_children(nav_item_nodes);
        nodes.insert(nav_tree_id, nav_tree_node);

        let nav_id = AkNodeId::from(counter);
        counter += 1;
        let mut nav_node = AkNode::new(AkRole::Navigation);
        nav_node.set_description("class=nav-panel".to_string());
        nav_node.set_children(vec![filter_id, nav_tree_id]);
        nodes.insert(nav_id, nav_node);

        // ── content panel ─────────────────────────────────────────────────────
        let content_id = AkNodeId::from(counter);
        counter += 1;
        let content_children = self.build_content_nodes(&mut nodes, &mut counter);
        let mut content_node = AkNode::new(AkRole::GenericContainer);
        let panel_attr = if matches!(self.panel_state, ArchivePanelState::MonitorView { .. }) {
            ";data-panel=monitor"
        } else {
            ""
        };
        content_node.set_description(format!("id=content{panel_attr}"));
        content_node.set_children(content_children);
        nodes.insert(content_id, content_node);

        // ── main split ────────────────────────────────────────────────────────
        let main_id = AkNodeId::from(counter);
        let mut main_node = AkNode::new(AkRole::Main);
        // Horizontal so the ratatui bridge lays out nav (left) + content (right).
        main_node.set_orientation(accesskit::Orientation::Horizontal);
        main_node.set_children(vec![nav_id, content_id]);
        nodes.insert(main_id, main_node);

        // ── status bar ────────────────────────────────────────────────────────
        let status = ArchiveKeyMap::default_map().to_status_bar(KeyMapMode::Default);
        let (status_root_eid, status_pairs) = status.to_ak_nodes(10_000);
        for (eid, json) in status_pairs {
            nodes.insert(eid.0, accesskit::Node::from(json));
        }

        // ── window root ───────────────────────────────────────────────────────
        let window_id = AkNodeId::from(0u64);
        let mut window = AkNode::new(AkRole::Window);
        let mut window_children = vec![toolbar_id, main_id, status_root_eid.0];
        // Append overlay dialog (TUI help/export/save/saved-browser overlays).
        if let Some(overlay_id) = self.build_overlay_node(&mut nodes, &mut counter) {
            window_children.push(overlay_id);
        }
        window.set_children(window_children);
        nodes.insert(window_id, window);

        let viewport = Viewport::new(800, 600);
        let tree = VerifiedTree::from_parts(nodes, window_id, viewport);
        Ok((tree, elicitation::Established::assert()))
    }

    /// Build the content-only subtree (root = `GenericContainer id=content`).
    ///
    /// Used by browser API endpoints that return only the content panel fragment.
    #[cfg(not(kani))]
    #[tracing::instrument(skip(self))]
    pub fn to_content_tree(
        &self,
    ) -> crate::archive::ArchiveResult<(
        elicit_ui::VerifiedTree,
        elicitation::Established<elicit_ui::IrSourced>,
    )> {
        use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};
        use elicit_ui::{VerifiedTree, Viewport};
        use std::collections::BTreeMap;

        let mut nodes: BTreeMap<AkNodeId, AkNode> = BTreeMap::new();
        let mut counter: u64 = 1;
        let content_children = self.build_content_nodes(&mut nodes, &mut counter);

        let root_id = AkNodeId::from(0u64);
        let mut content_node = AkNode::new(AkRole::GenericContainer);
        let panel_attr = if matches!(self.panel_state, ArchivePanelState::MonitorView { .. }) {
            ";data-panel=monitor"
        } else {
            ""
        };
        content_node.set_description(format!("id=content{panel_attr}"));
        content_node.set_children(content_children);
        nodes.insert(root_id, content_node);

        let viewport = Viewport::new(800, 600);
        let tree = VerifiedTree::from_parts(nodes, root_id, viewport);
        Ok((tree, elicitation::Established::assert()))
    }

    /// Build the nav-tree-only subtree (root = `Tree id=nav-tree`).
    ///
    /// Used by `/api/nav` to return the nav tree items for HTMX outerHTML swap.
    #[tracing::instrument(skip(self))]
    pub fn to_nav_tree(
        &self,
    ) -> crate::archive::ArchiveResult<(
        elicit_ui::VerifiedTree,
        elicitation::Established<elicit_ui::IrSourced>,
    )> {
        use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};
        use elicit_ui::{VerifiedTree, Viewport};
        use std::collections::BTreeMap;

        let mut nodes: BTreeMap<AkNodeId, AkNode> = BTreeMap::new();
        let mut counter: u64 = 1;
        let (root_id, nav_item_nodes) = self.build_nav_item_nodes(&mut nodes, &mut counter);

        let mut tree_node = AkNode::new(AkRole::Tree);
        tree_node.set_label(self.db_name.clone());
        tree_node.set_description("id=nav-tree".to_string());
        tree_node.set_children(nav_item_nodes);
        nodes.insert(root_id, tree_node);

        let viewport = Viewport::new(800, 600);
        let tree = VerifiedTree::from_parts(nodes, root_id, viewport);
        Ok((tree, elicitation::Established::assert()))
    }

    /// Build toolbar AccessKit nodes and return the toolbar root `NodeId`.
    fn build_toolbar_nodes(
        &self,
        nodes: &mut std::collections::BTreeMap<accesskit::NodeId, accesskit::Node>,
        counter: &mut u64,
    ) -> accesskit::NodeId {
        use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};

        let mut alloc = || {
            let id = AkNodeId::from(*counter);
            *counter += 1;
            id
        };

        let title_id = alloc();
        let mut title = AkNode::new(AkRole::Label);
        title.set_label("▦ Archive".to_string());
        title.set_description("class=title".to_string());
        nodes.insert(title_id, title);

        let mut btn = |label: &str, desc: &str| {
            let id = alloc();
            let mut n = AkNode::new(AkRole::Button);
            n.set_label(label.to_string());
            n.set_description(desc.to_string());
            (id, n)
        };

        let (sql_btn_id, sql_btn) = btn(
            "SQL Editor",
            "hx-get=/api/open-sql-editor;hx-target=#content;hx-swap=outerHTML",
        );
        nodes.insert(sql_btn_id, sql_btn);

        let (ddl_btn_id, ddl_btn) = btn(
            "DDL",
            "hx-get=/api/ddl-panel;hx-target=#content;hx-swap=outerHTML",
        );
        nodes.insert(ddl_btn_id, ddl_btn);

        let (explain_btn_id, explain_btn) = btn(
            "EXPLAIN",
            "hx-get=/api/explain-panel;hx-target=#content;hx-swap=outerHTML",
        );
        nodes.insert(explain_btn_id, explain_btn);

        let (col_btn_id, col_btn) = btn(
            "Col Detail",
            "hx-get=/api/col-detail-panel;hx-target=#content;hx-swap=outerHTML",
        );
        nodes.insert(col_btn_id, col_btn);

        let (hist_btn_id, hist_btn) = btn(
            "History",
            "hx-get=/api/history-panel;hx-target=#content;hx-swap=outerHTML",
        );
        nodes.insert(hist_btn_id, hist_btn);

        let (saved_btn_id, saved_btn) = btn(
            "Saved",
            "hx-get=/api/saved-panel;hx-target=#content;hx-swap=outerHTML",
        );
        nodes.insert(saved_btn_id, saved_btn);

        let (save_sql_btn_id, save_sql_btn) = btn("Save SQL", "data-action=save-sql");
        nodes.insert(save_sql_btn_id, save_sql_btn);

        let (export_btn_id, export_btn) = btn(
            "Export",
            "hx-get=/api/export-panel;hx-target=#content;hx-swap=outerHTML",
        );
        nodes.insert(export_btn_id, export_btn);

        let (refresh_btn_id, refresh_btn) = btn(
            "⟳ Refresh",
            "hx-post=/api/refresh;hx-target=#content;hx-swap=outerHTML",
        );
        nodes.insert(refresh_btn_id, refresh_btn);

        let (help_btn_id, help_btn) = btn(
            "?",
            "hx-get=/api/open-help;hx-target=#content;hx-swap=outerHTML",
        );
        nodes.insert(help_btn_id, help_btn);

        let toolbar_id = alloc();
        let mut toolbar = AkNode::new(AkRole::Toolbar);
        // Horizontal so the ratatui bridge lays out buttons left-to-right.
        toolbar.set_orientation(accesskit::Orientation::Horizontal);
        toolbar.set_children(vec![
            title_id,
            sql_btn_id,
            ddl_btn_id,
            explain_btn_id,
            col_btn_id,
            hist_btn_id,
            saved_btn_id,
            save_sql_btn_id,
            export_btn_id,
            refresh_btn_id,
            help_btn_id,
        ]);
        nodes.insert(toolbar_id, toolbar);
        toolbar_id
    }

    /// Build the flat nav items and return `(tree_node_id, item_ids)`.
    fn build_nav_item_nodes(
        &self,
        nodes: &mut std::collections::BTreeMap<accesskit::NodeId, accesskit::Node>,
        counter: &mut u64,
    ) -> (accesskit::NodeId, Vec<accesskit::NodeId>) {
        use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};

        let tree_id = AkNodeId::from(*counter);
        *counter += 1;

        let mut nav_children: Vec<AkNodeId> = Vec::new();
        for (idx, item) in self.flat.iter().enumerate() {
            let item_id = AkNodeId::from(*counter);
            *counter += 1;
            nav_children.push(item_id);

            let label = match item {
                FlatItem::Schema(si) => {
                    let s = &self.schemas[*si];
                    let arrow = if s.expanded { "▾" } else { "▸" };
                    format!(
                        "{} {} ({}) — {} table{}",
                        arrow,
                        s.entry.name,
                        s.entry.owner,
                        s.entry.tables.len(),
                        if s.entry.tables.len() == 1 { "" } else { "s" },
                    )
                }
                FlatItem::Table(si, ti) => {
                    let t = &self.schemas[*si].entry.tables[*ti];
                    format!("  {} [{}]", t.table_name, t.table_type)
                }
                FlatItem::FunctionsGroup(si) => {
                    let s = &self.schemas[*si];
                    let arrow = if s.functions_expanded { "▾" } else { "▸" };
                    format!("  {} Functions ({})", arrow, s.entry.functions.len())
                }
                FlatItem::Function(si, fi) => {
                    let f = &self.schemas[*si].entry.functions[*fi];
                    format!("    {} ({})", f.name, f.arguments)
                }
                FlatItem::SequencesGroup(si) => {
                    let s = &self.schemas[*si];
                    let arrow = if s.sequences_expanded { "▾" } else { "▸" };
                    format!("  {} Sequences ({})", arrow, s.entry.sequences.len())
                }
                FlatItem::Sequence(si, qi) => {
                    let q = &self.schemas[*si].entry.sequences[*qi];
                    format!("    {}", q.name)
                }
                FlatItem::TypesGroup(si) => {
                    let s = &self.schemas[*si];
                    let arrow = if s.types_expanded { "▾" } else { "▸" };
                    let total =
                        s.entry.enums.len() + s.entry.domains.len() + s.entry.composites.len();
                    format!("  {} Types ({})", arrow, total)
                }
                FlatItem::TypeEntry(si, kind, idx) => {
                    let entry = &self.schemas[*si].entry;
                    match kind {
                        0 => format!("    {} [enum]", entry.enums[*idx].name),
                        1 => format!("    {} [domain]", entry.domains[*idx].name),
                        _ => format!("    {} [composite]", entry.composites[*idx].name),
                    }
                }
                FlatItem::TriggersGroup(si) => {
                    let s = &self.schemas[*si];
                    let arrow = if s.triggers_expanded { "▾" } else { "▸" };
                    format!("  {} Triggers ({})", arrow, s.entry.triggers.len())
                }
                FlatItem::Trigger(si, ti) => {
                    let t = &self.schemas[*si].entry.triggers[*ti];
                    format!("    {} [trigger]", t.name)
                }
                FlatItem::ExtensionsGroup => {
                    let arrow = if self.extensions_expanded {
                        "▾"
                    } else {
                        "▸"
                    };
                    format!("{} Extensions ({})", arrow, self.extensions.len())
                }
                FlatItem::Extension(idx) => {
                    let (name, version) = &self.extensions[*idx];
                    format!("  {} v{}", name, version)
                }
            };

            // Machine-readable metadata in description for browser frontends.
            let meta = match item {
                FlatItem::Schema(si) => {
                    format!("schema:{}", self.schemas[*si].entry.name)
                }
                FlatItem::Table(si, ti) => {
                    let s = &self.schemas[*si].entry.name;
                    let t = &self.schemas[*si].entry.tables[*ti].table_name;
                    format!("schema:{s},table:{t}")
                }
                FlatItem::FunctionsGroup(si) => {
                    format!("schema:{},group:functions", self.schemas[*si].entry.name)
                }
                FlatItem::Function(si, fi) => {
                    let s = &self.schemas[*si].entry.name;
                    let f = &self.schemas[*si].entry.functions[*fi].name;
                    format!("schema:{s},function:{f}")
                }
                FlatItem::SequencesGroup(si) => {
                    format!("schema:{},group:sequences", self.schemas[*si].entry.name)
                }
                FlatItem::Sequence(si, qi) => {
                    let s = &self.schemas[*si].entry.name;
                    let q = &self.schemas[*si].entry.sequences[*qi].name;
                    format!("schema:{s},sequence:{q}")
                }
                FlatItem::TypesGroup(si) => {
                    format!("schema:{},group:types", self.schemas[*si].entry.name)
                }
                FlatItem::TypeEntry(si, kind, idx) => {
                    let s = &self.schemas[*si].entry.name;
                    let entry = &self.schemas[*si].entry;
                    let type_name = match kind {
                        0 => entry.enums[*idx].name.as_str(),
                        1 => entry.domains[*idx].name.as_str(),
                        _ => entry.composites[*idx].name.as_str(),
                    };
                    format!("schema:{s},type:{type_name}")
                }
                FlatItem::TriggersGroup(si) => {
                    format!("schema:{},group:triggers", self.schemas[*si].entry.name)
                }
                FlatItem::Trigger(si, ti) => {
                    let s = &self.schemas[*si].entry.name;
                    let t = &self.schemas[*si].entry.triggers[*ti].name;
                    format!("schema:{s},trigger:{t}")
                }
                FlatItem::ExtensionsGroup => "group:extensions".to_string(),
                FlatItem::Extension(idx) => {
                    let (name, _) = &self.extensions[*idx];
                    format!("extension:{name}")
                }
            };

            let mut tree_item = AkNode::new(AkRole::TreeItem);
            tree_item.set_label(label);
            tree_item.set_description(meta);
            if idx == self.cursor {
                tree_item.set_selected(true);
            }
            nodes.insert(item_id, tree_item);
        }

        (tree_id, nav_children)
    }

    /// Build AccessKit nodes for the current content panel.  Returns the list
    /// of direct children of the content group.
    #[cfg(not(kani))]
    fn build_content_nodes(
        &self,
        nodes: &mut std::collections::BTreeMap<accesskit::NodeId, accesskit::Node>,
        counter: &mut u64,
    ) -> Vec<accesskit::NodeId> {
        use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};

        let mut children: Vec<AkNodeId> = Vec::new();
        let mut alloc = || {
            let id = AkNodeId::from(*counter);
            *counter += 1;
            id
        };

        match &self.panel_state {
            ArchivePanelState::ColumnDetail => {
                let heading_id = alloc();
                let label = self
                    .selected()
                    .map(|item| match item {
                        FlatItem::Schema(si) => {
                            format!("Schema: {}", self.schemas[si].entry.name)
                        }
                        FlatItem::Table(si, ti) => {
                            let t = &self.schemas[si].entry.tables[ti];
                            format!(
                                "Column detail — {}.{}",
                                self.schemas[si].entry.name, t.table_name
                            )
                        }
                        _ => "Select a table or object".to_string(),
                    })
                    .unwrap_or_else(|| "Select a table".to_string());
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(label);
                nodes.insert(heading_id, h);
                children.push(heading_id);

                // Column list from cached inspection
                if let Some(FlatItem::Table(si, ti)) = self.selected() {
                    let key = (
                        self.schemas[si].entry.name.clone(),
                        self.schemas[si].entry.tables[ti].table_name.clone(),
                    );
                    if let Some(stats) = self.column_stats.get(&key) {
                        let list_id = alloc();
                        let mut list_children: Vec<AkNodeId> = Vec::new();
                        for cs in stats {
                            let item_id = alloc();
                            let mut item = AkNode::new(AkRole::ListItem);
                            item.set_label(format!(
                                "{} — {:.0} distinct, null_frac {:.1}%",
                                cs.column_name,
                                cs.n_distinct,
                                cs.null_fraction * 100.0,
                            ));
                            nodes.insert(item_id, item);
                            list_children.push(item_id);
                        }
                        let mut list = AkNode::new(AkRole::List);
                        list.set_label("columns");
                        list.set_children(list_children);
                        nodes.insert(list_id, list);
                        children.push(list_id);
                    }
                }
            }

            ArchivePanelState::Loading {
                schema,
                label: table,
            } => {
                let prog_id = alloc();
                let mut prog = AkNode::new(AkRole::ProgressIndicator);
                prog.set_label(format!("Loading {schema}.{table}…"));
                nodes.insert(prog_id, prog);
                children.push(prog_id);
            }

            ArchivePanelState::DataGrid {
                schema,
                table,
                result,
                page,
                ..
            } => {
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(format!("{schema}.{table} — page {}", page + 1));
                nodes.insert(heading_id, h);
                children.push(heading_id);

                let grid_id = alloc();
                let mut grid_children: Vec<AkNodeId> = Vec::new();

                // Header row
                let hdr_id = alloc();
                let mut hdr_cells: Vec<AkNodeId> = Vec::new();
                for col in &result.columns {
                    let cell_id = alloc();
                    let mut cell = AkNode::new(AkRole::ColumnHeader);
                    cell.set_label(col.name.clone());
                    nodes.insert(cell_id, cell);
                    hdr_cells.push(cell_id);
                }
                let mut hdr_row = AkNode::new(AkRole::Row);
                hdr_row.set_children(hdr_cells);
                nodes.insert(hdr_id, hdr_row);
                grid_children.push(hdr_id);

                // Data rows (cap to 200 for IR size)
                for row in result.rows.rows.iter().take(200) {
                    let row_id = alloc();
                    let mut row_cells: Vec<AkNodeId> = Vec::new();
                    for (_, val) in &row.0 {
                        let cell_id = alloc();
                        let mut cell = AkNode::new(AkRole::Cell);
                        cell.set_label(format!("{val:?}"));
                        nodes.insert(cell_id, cell);
                        row_cells.push(cell_id);
                    }
                    let mut data_row = AkNode::new(AkRole::Row);
                    data_row.set_children(row_cells);
                    nodes.insert(row_id, data_row);
                    grid_children.push(row_id);
                }

                let mut grid = AkNode::new(AkRole::Grid);
                grid.set_label(format!("{schema}.{table}"));
                grid.set_children(grid_children);
                nodes.insert(grid_id, grid);
                children.push(grid_id);
            }

            ArchivePanelState::SqlEditor {
                text,
                result,
                running,
                error,
            } => {
                // Show error banner if present
                if let Some(err_msg) = error {
                    let err_id = alloc();
                    let mut err = AkNode::new(AkRole::Alert);
                    err.set_label(err_msg.clone());
                    nodes.insert(err_id, err);
                    children.push(err_id);
                }

                let editor_id = alloc();
                let mut editor = AkNode::new(AkRole::MultilineTextInput);
                editor.set_label("SQL editor".to_string());
                editor.set_value(text.clone());
                nodes.insert(editor_id, editor);
                children.push(editor_id);

                if *running {
                    let prog_id = alloc();
                    let mut prog = AkNode::new(AkRole::ProgressIndicator);
                    prog.set_label("Query running…".to_string());
                    nodes.insert(prog_id, prog);
                    children.push(prog_id);
                }

                if let Some(res) = result {
                    let grid_id = alloc();
                    let mut grid_children: Vec<AkNodeId> = Vec::new();

                    let hdr_id = alloc();
                    let mut hdr_cells: Vec<AkNodeId> = Vec::new();
                    for col in &res.columns {
                        let cell_id = alloc();
                        let mut cell = AkNode::new(AkRole::ColumnHeader);
                        cell.set_label(col.name.clone());
                        nodes.insert(cell_id, cell);
                        hdr_cells.push(cell_id);
                    }
                    let mut hdr_row = AkNode::new(AkRole::Row);
                    hdr_row.set_children(hdr_cells);
                    nodes.insert(hdr_id, hdr_row);
                    grid_children.push(hdr_id);

                    for row in res.rows.rows.iter().take(200) {
                        let row_id = alloc();
                        let mut row_cells: Vec<AkNodeId> = Vec::new();
                        for (_, val) in &row.0 {
                            let cell_id = alloc();
                            let mut cell = AkNode::new(AkRole::Cell);
                            cell.set_label(format!("{val:?}"));
                            nodes.insert(cell_id, cell);
                            row_cells.push(cell_id);
                        }
                        let mut data_row = AkNode::new(AkRole::Row);
                        data_row.set_children(row_cells);
                        nodes.insert(row_id, data_row);
                        grid_children.push(row_id);
                    }

                    let mut grid = AkNode::new(AkRole::Grid);
                    grid.set_label("query results".to_string());
                    grid.set_children(grid_children);
                    nodes.insert(grid_id, grid);
                    children.push(grid_id);
                }
            }

            ArchivePanelState::DdlView {
                schema, table, ddl, ..
            } => {
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(format!("DDL: {schema}.{table}"));
                nodes.insert(heading_id, h);
                children.push(heading_id);

                let code_id = alloc();
                let mut code = AkNode::new(AkRole::Code);
                code.set_label(ddl.ddl.clone());
                nodes.insert(code_id, code);
                children.push(code_id);
            }

            ArchivePanelState::ExplainView {
                schema,
                table,
                root,
                ..
            } => {
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(format!("EXPLAIN: {schema}.{table}"));
                nodes.insert(heading_id, h);
                children.push(heading_id);

                let plan_root_id = alloc();
                let plan_id = self.build_explain_node(&root.nodes, root.root(), nodes, counter);
                let mut plan_tree = AkNode::new(AkRole::Tree);
                plan_tree.set_label("query plan".to_string());
                plan_tree.set_children(vec![plan_id]);
                nodes.insert(plan_root_id, plan_tree);
                children.push(plan_root_id);
            }

            ArchivePanelState::ExplainCompare {
                schema,
                table,
                comparison,
            } => {
                let left = &comparison.left;
                let right = &comparison.right;
                let label_left = &comparison.label_left;
                let label_right = &comparison.label_right;
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(format!("EXPLAIN compare: {schema}.{table}"));
                nodes.insert(heading_id, h);
                children.push(heading_id);

                // Cost-delta annotation: if total costs differ by > 10 %, prefix labels.
                let cost_l = left.root().total_cost;
                let cost_r = right.root().total_cost;
                let delta_pct = if cost_l > 0.0 {
                    (cost_r - cost_l).abs() / cost_l
                } else {
                    0.0
                };
                let (ann_l, ann_r) = if delta_pct > 0.10 {
                    if cost_r < cost_l {
                        ("", "▼ ")
                    } else {
                        ("", "▲ ")
                    }
                } else {
                    ("", "")
                };

                // Pre-allocate all IDs before calling build_explain_node (which
                // also needs a &mut counter, so alloc's borrow must end first).
                let group_id = alloc();
                let left_tree_id = alloc();
                let right_tree_id = alloc();
                // alloc is not called again; its borrow of counter ends here.

                let left_plan_id =
                    self.build_explain_node(&left.nodes, left.root(), nodes, counter);
                let mut left_tree = AkNode::new(AkRole::Tree);
                left_tree.set_label(format!("{ann_l}{label_left}"));
                left_tree.set_children(vec![left_plan_id]);
                nodes.insert(left_tree_id, left_tree);

                let right_plan_id =
                    self.build_explain_node(&right.nodes, right.root(), nodes, counter);
                let mut right_tree = AkNode::new(AkRole::Tree);
                right_tree.set_label(format!("{ann_r}{label_right}"));
                right_tree.set_children(vec![right_plan_id]);
                nodes.insert(right_tree_id, right_tree);

                let mut group = AkNode::new(AkRole::Group);
                group.set_label("plan comparison".to_string());
                group.set_description("class=plan-compare".to_string());
                group.set_children(vec![left_tree_id, right_tree_id]);
                nodes.insert(group_id, group);
                children.push(group_id);
            }

            ArchivePanelState::HistoryView { entries, .. } => {
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label("Query History".to_string());
                nodes.insert(heading_id, h);
                children.push(heading_id);

                let list_id = alloc();
                let mut list_children: Vec<AkNodeId> = Vec::new();
                for (i, entry) in entries.iter().enumerate() {
                    let item_id = alloc();
                    let mut item = AkNode::new(AkRole::ListItem);
                    let short = entry.sql.lines().next().unwrap_or("").trim();
                    item.set_label(format!("{i}. {short}"));
                    item.set_description(format!(
                        "hx-get=/api/load-history?idx={i};hx-target=#content;hx-swap=outerHTML"
                    ));
                    nodes.insert(item_id, item);
                    list_children.push(item_id);
                }
                let mut list = AkNode::new(AkRole::List);
                list.set_label("history".to_string());
                list.set_children(list_children);
                nodes.insert(list_id, list);
                children.push(list_id);
            }

            ArchivePanelState::SavedView { entries, .. } => {
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label("Saved Queries".to_string());
                nodes.insert(heading_id, h);
                children.push(heading_id);

                let list_id = alloc();
                let mut list_children: Vec<AkNodeId> = Vec::new();
                for sq in entries.iter() {
                    let row_id = alloc();
                    let mut row = AkNode::new(AkRole::Group);
                    row.set_label(sq.name.clone());

                    let load_id = alloc();
                    let mut load_btn = AkNode::new(AkRole::Button);
                    load_btn.set_label(sq.name.clone());
                    load_btn.set_description(format!(
                        "hx-get=/api/load-saved?id={};hx-target=#content;hx-swap=outerHTML",
                        sq.id
                    ));
                    nodes.insert(load_id, load_btn);

                    let del_id = alloc();
                    let mut del_btn = AkNode::new(AkRole::Button);
                    del_btn.set_label("delete".to_string());
                    del_btn.set_description(format!(
                        "hx-delete=/api/saved/{};hx-target=#content;hx-swap=outerHTML",
                        sq.id
                    ));
                    nodes.insert(del_id, del_btn);

                    row.set_children(vec![load_id, del_id]);
                    nodes.insert(row_id, row);
                    list_children.push(row_id);
                }
                let mut list = AkNode::new(AkRole::List);
                list.set_label("saved queries".to_string());
                list.set_children(list_children);
                nodes.insert(list_id, list);
                children.push(list_id);
            }

            ArchivePanelState::ExportView { schema, table, .. } => {
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(format!("Export: {schema}.{table}"));
                nodes.insert(heading_id, h);
                children.push(heading_id);

                let list_id = alloc();
                let formats = [
                    ("CSV", "csv"),
                    ("JSON", "json"),
                    ("TSV", "tsv"),
                    ("SQL", "sql"),
                ];
                let mut list_items: Vec<AkNodeId> = Vec::new();
                for (label, fmt) in formats {
                    let btn_id = alloc();
                    let mut btn = AkNode::new(AkRole::Button);
                    btn.set_label(label.to_string());
                    btn.set_description(format!(
                        "hx-get=/api/export?schema={schema}&table={table}&format={fmt}"
                    ));
                    nodes.insert(btn_id, btn);
                    list_items.push(btn_id);
                }
                let mut list = AkNode::new(AkRole::List);
                list.set_label("export formats".to_string());
                list.set_children(list_items);
                nodes.insert(list_id, list);
                children.push(list_id);
            }

            ArchivePanelState::HelpView => {
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label("Key Bindings".to_string());
                nodes.insert(heading_id, h);
                children.push(heading_id);

                let list_id = alloc();
                let mut list_items: Vec<AkNodeId> = Vec::new();
                for kb in Self::bindings() {
                    let item_id = alloc();
                    let mut item = AkNode::new(AkRole::ListItem);
                    item.set_label(format!("{} — {}", kb.key, kb.action));
                    nodes.insert(item_id, item);
                    list_items.push(item_id);
                }
                let mut list = AkNode::new(AkRole::List);
                list.set_label("bindings".to_string());
                list.set_children(list_items);
                nodes.insert(list_id, list);
                children.push(list_id);
            }
            ArchivePanelState::MonitorView {
                snapshot, loading, ..
            } => {
                use crate::archive::MonitorTab;

                let tab_bar = [
                    MonitorTab::Sessions,
                    MonitorTab::SlowQueries,
                    MonitorTab::LockWaits,
                    MonitorTab::TableBloat,
                    MonitorTab::IndexUsage,
                ]
                .iter()
                .map(|t| {
                    if *t == snapshot.active_tab {
                        format!("[{}]", t.label())
                    } else {
                        t.label().to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(" | ");

                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(if *loading {
                    "Monitor — loading…".to_string()
                } else {
                    format!("Monitor — {tab_bar}")
                });
                nodes.insert(heading_id, h);
                children.push(heading_id);

                match snapshot.active_tab {
                    MonitorTab::Sessions => {
                        let list_id = alloc();
                        let mut items: Vec<AkNodeId> = Vec::new();
                        for s in &snapshot.sessions {
                            let item_id = alloc();
                            let mut item = AkNode::new(AkRole::ListItem);
                            let dur = s
                                .duration_ms
                                .map(|ms| format!(" {}ms", ms))
                                .unwrap_or_default();
                            item.set_label(format!(
                                "pid={} {} [{}]{}",
                                s.pid, s.app_name, s.state, dur
                            ));
                            nodes.insert(item_id, item);
                            items.push(item_id);
                        }
                        let mut list = AkNode::new(AkRole::List);
                        list.set_label(format!(
                            "Sessions ({}) — cache hit {}",
                            snapshot.sessions.len(),
                            snapshot
                                .cache_hit
                                .map(|r| format!("{:.1}%", r * 100.0))
                                .unwrap_or_else(|| "n/a".to_string()),
                        ));
                        list.set_children(items);
                        nodes.insert(list_id, list);
                        children.push(list_id);
                    }
                    MonitorTab::SlowQueries => {
                        let list_id = alloc();
                        let mut items: Vec<AkNodeId> = Vec::new();
                        for s in &snapshot.slow_queries {
                            let item_id = alloc();
                            let mut item = AkNode::new(AkRole::ListItem);
                            let dur = s
                                .duration_ms
                                .map(|ms| format!(" {}ms", ms))
                                .unwrap_or_default();
                            let query = s
                                .query
                                .as_deref()
                                .map(|q| {
                                    let q = q.trim();
                                    if q.len() > 60 {
                                        format!(" — {}…", &q[..60])
                                    } else {
                                        format!(" — {q}")
                                    }
                                })
                                .unwrap_or_default();
                            item.set_label(format!("pid={} [{}]{}{query}", s.pid, s.state, dur));
                            nodes.insert(item_id, item);
                            items.push(item_id);
                        }
                        let mut list = AkNode::new(AkRole::List);
                        list.set_label(format!("Slow Queries ({})", snapshot.slow_queries.len()));
                        list.set_children(items);
                        nodes.insert(list_id, list);
                        children.push(list_id);
                    }
                    MonitorTab::LockWaits => {
                        let list_id = alloc();
                        let mut items: Vec<AkNodeId> = Vec::new();
                        for (blocking, blocked) in &snapshot.lock_waits {
                            let item_id = alloc();
                            let mut item = AkNode::new(AkRole::ListItem);
                            item.set_label(format!("pid {blocking} blocks pid {blocked}"));
                            nodes.insert(item_id, item);
                            items.push(item_id);
                        }
                        let mut list = AkNode::new(AkRole::List);
                        list.set_label(format!("Lock Waits ({})", snapshot.lock_waits.len()));
                        list.set_children(items);
                        nodes.insert(list_id, list);
                        children.push(list_id);
                    }
                    MonitorTab::TableBloat => {
                        let list_id = alloc();
                        let mut items: Vec<AkNodeId> = Vec::new();
                        for (table, ratio) in &snapshot.table_bloat {
                            let item_id = alloc();
                            let mut item = AkNode::new(AkRole::ListItem);
                            item.set_label(format!("{table} — bloat {:.1}%", ratio * 100.0));
                            nodes.insert(item_id, item);
                            items.push(item_id);
                        }
                        let mut list = AkNode::new(AkRole::List);
                        list.set_label(format!("Table Bloat ({})", snapshot.table_bloat.len()));
                        list.set_children(items);
                        nodes.insert(list_id, list);
                        children.push(list_id);
                    }
                    MonitorTab::IndexUsage => {
                        let list_id = alloc();
                        let mut items: Vec<AkNodeId> = Vec::new();
                        for (index, scans) in &snapshot.index_usage {
                            let item_id = alloc();
                            let mut item = AkNode::new(AkRole::ListItem);
                            item.set_label(format!("{index} — {scans} scans"));
                            nodes.insert(item_id, item);
                            items.push(item_id);
                        }
                        let mut list = AkNode::new(AkRole::List);
                        list.set_label(format!("Index Usage ({})", snapshot.index_usage.len()));
                        list.set_children(items);
                        nodes.insert(list_id, list);
                        children.push(list_id);
                    }
                }
            }
            ArchivePanelState::AdminView {
                snapshot, loading, ..
            } => {
                use crate::archive::AdminTab;

                // Heading: tab bar
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(if *loading {
                    "Admin — loading…".to_string()
                } else {
                    format!(
                        "Admin [{}] — {} | {} | {}{}",
                        snapshot.active_tab.label(),
                        "Roles",
                        "Backups",
                        "Settings",
                        if snapshot.server_version.is_empty() {
                            String::new()
                        } else {
                            format!(" — {}", snapshot.server_version)
                        }
                    )
                });
                nodes.insert(heading_id, h);
                children.push(heading_id);

                match snapshot.active_tab {
                    AdminTab::Roles => {
                        let list_id = alloc();
                        let mut items: Vec<AkNodeId> = Vec::new();
                        for r in &snapshot.roles {
                            let item_id = alloc();
                            let mut item = AkNode::new(AkRole::ListItem);
                            let attrs: Vec<&str> = [
                                r.superuser.then_some("superuser"),
                                r.can_login.then_some("login"),
                                r.can_create_db.then_some("createdb"),
                                r.can_create_role.then_some("createrole"),
                            ]
                            .into_iter()
                            .flatten()
                            .collect();
                            item.set_label(if attrs.is_empty() {
                                r.name.clone()
                            } else {
                                format!("{} [{}]", r.name, attrs.join(", "))
                            });
                            nodes.insert(item_id, item);
                            items.push(item_id);
                        }
                        let mut list = AkNode::new(AkRole::List);
                        list.set_label("Roles".to_string());
                        list.set_children(items);
                        nodes.insert(list_id, list);
                        children.push(list_id);
                    }
                    AdminTab::Backups => {
                        let backup_id = alloc();
                        let mut items: Vec<AkNodeId> = Vec::new();
                        // WAL status item
                        let wal_id = alloc();
                        let mut wal_item = AkNode::new(AkRole::ListItem);
                        wal_item.set_label(format!(
                            "WAL: {}",
                            if snapshot.wal_ready {
                                "ready"
                            } else {
                                "unavailable"
                            }
                        ));
                        nodes.insert(wal_id, wal_item);
                        items.push(wal_id);
                        // Backup labels
                        for label in &snapshot.backups {
                            let item_id = alloc();
                            let mut item = AkNode::new(AkRole::ListItem);
                            item.set_label(label.clone());
                            nodes.insert(item_id, item);
                            items.push(item_id);
                        }
                        if snapshot.backups.is_empty() {
                            let item_id = alloc();
                            let mut item = AkNode::new(AkRole::ListItem);
                            item.set_label("No backups found".to_string());
                            nodes.insert(item_id, item);
                            items.push(item_id);
                        }
                        let mut list = AkNode::new(AkRole::List);
                        list.set_label("Backups".to_string());
                        list.set_children(items);
                        nodes.insert(backup_id, list);
                        children.push(backup_id);
                    }
                    AdminTab::Settings => {
                        // Extensions sub-list
                        let ext_id = alloc();
                        let mut ext_items: Vec<AkNodeId> = Vec::new();
                        for ext in &snapshot.extensions {
                            let item_id = alloc();
                            let mut item = AkNode::new(AkRole::ListItem);
                            item.set_label(ext.clone());
                            nodes.insert(item_id, item);
                            ext_items.push(item_id);
                        }
                        let mut ext_list = AkNode::new(AkRole::List);
                        ext_list.set_label("Extensions".to_string());
                        ext_list.set_children(ext_items);
                        nodes.insert(ext_id, ext_list);
                        children.push(ext_id);

                        // GUC settings sub-list (top 20)
                        let settings_id = alloc();
                        let mut setting_items: Vec<AkNodeId> = Vec::new();
                        for (name, val) in snapshot.settings.iter().take(20) {
                            let item_id = alloc();
                            let mut item = AkNode::new(AkRole::ListItem);
                            item.set_label(format!("{} = {}", name, val));
                            nodes.insert(item_id, item);
                            setting_items.push(item_id);
                        }
                        let mut settings_list = AkNode::new(AkRole::List);
                        settings_list.set_label("GUC Settings".to_string());
                        settings_list.set_children(setting_items);
                        nodes.insert(settings_id, settings_list);
                        children.push(settings_id);
                    }
                }
            }
            ArchivePanelState::ErdView {
                schema,
                diagram,
                layout,
                loading,
                ..
            } => {
                use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};

                // Heading
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(if *loading {
                    format!("ERD — {} — loading…", schema)
                } else {
                    format!(
                        "ERD — {} — {} tables, {} relationships",
                        schema,
                        diagram.nodes.len(),
                        diagram.edges.len()
                    )
                });
                nodes.insert(heading_id, h);
                children.push(heading_id);

                if let Some(layout) = layout {
                    // ── Spatial figure IR ────────────────────────────────────
                    // Role::Figure root carries canvas dimensions.
                    let figure_id = alloc();
                    let mut figure_children: Vec<AkNodeId> = Vec::new();

                    // One Group per table box with coordinate metadata.
                    let mut sorted_nodes: Vec<&crate::archive::types::ErdNode> =
                        diagram.nodes.iter().collect();
                    sorted_nodes.sort_by(|a, b| a.table.cmp(&b.table));

                    for node_entry in &sorted_nodes {
                        let key = format!("{}.{}", node_entry.schema, node_entry.table);
                        let Some(&(bx, by, bw, bh)) = layout.boxes.get(&key) else {
                            continue;
                        };
                        let box_id = alloc();
                        let mut box_node = AkNode::new(AkRole::Group);
                        box_node.set_label(node_entry.table.clone());
                        box_node.set_description(format!("x={bx},y={by},w={bw},h={bh}"));

                        // Column children (accessible label row).
                        let mut col_ids: Vec<AkNodeId> = Vec::new();
                        for col in &node_entry.columns {
                            let col_id = alloc();
                            let mut col_node = AkNode::new(AkRole::ListItem);
                            let flags: Vec<&str> =
                                [col.is_pk.then_some("PK"), col.is_fk.then_some("FK")]
                                    .into_iter()
                                    .flatten()
                                    .collect();
                            col_node.set_label(if flags.is_empty() {
                                format!("{} : {}", col.name, col.sql_type)
                            } else {
                                format!("{} : {} [{}]", col.name, col.sql_type, flags.join(", "))
                            });
                            nodes.insert(col_id, col_node);
                            col_ids.push(col_id);
                        }
                        if !col_ids.is_empty() {
                            box_node.set_children(col_ids);
                        }
                        nodes.insert(box_id, box_node);
                        figure_children.push(box_id);
                    }

                    // One Group per FK edge with endpoint coordinates.
                    for edge in &diagram.edges {
                        let from_key = format!("{}.{}", edge.from_schema, edge.from_table);
                        let to_key = format!("{}.{}", edge.to_schema, edge.to_table);
                        let (Some((x1, y1)), Some((x2, y2))) =
                            (layout.centre_bottom(&from_key), layout.centre_top(&to_key))
                        else {
                            continue;
                        };
                        let edge_id = alloc();
                        let mut edge_node = AkNode::new(AkRole::Group);
                        edge_node.set_label(format!(
                            "{}.{} → {}.{}",
                            edge.from_table, edge.from_column, edge.to_table, edge.to_column
                        ));
                        edge_node.set_description(format!("x1={x1},y1={y1},x2={x2},y2={y2}"));
                        nodes.insert(edge_id, edge_node);
                        figure_children.push(edge_id);
                    }

                    let mut figure = AkNode::new(AkRole::Figure);
                    figure.set_label(format!("ERD — {schema}"));
                    figure.set_description(format!("w={},h={}", layout.canvas_w, layout.canvas_h));
                    if !figure_children.is_empty() {
                        figure.set_children(figure_children);
                    }
                    nodes.insert(figure_id, figure);
                    children.push(figure_id);
                } else {
                    // ── Fallback list view (loading / empty) ─────────────────
                    let tables_list_id = alloc();
                    let mut table_items: Vec<AkNodeId> = Vec::new();
                    for node_entry in &diagram.nodes {
                        let table_id = alloc();
                        let mut table_node = AkNode::new(AkRole::TreeItem);
                        table_node.set_label(format!(
                            "{}  ({} cols)",
                            node_entry.table,
                            node_entry.columns.len()
                        ));
                        let mut col_ids: Vec<AkNodeId> = Vec::new();
                        for col in &node_entry.columns {
                            let col_id = alloc();
                            let mut col_node = AkNode::new(AkRole::ListItem);
                            let flags: Vec<&str> =
                                [col.is_pk.then_some("PK"), col.is_fk.then_some("FK")]
                                    .into_iter()
                                    .flatten()
                                    .collect();
                            col_node.set_label(if flags.is_empty() {
                                format!("  {} : {}", col.name, col.sql_type)
                            } else {
                                format!("  {} : {} [{}]", col.name, col.sql_type, flags.join(", "))
                            });
                            nodes.insert(col_id, col_node);
                            col_ids.push(col_id);
                        }
                        if !col_ids.is_empty() {
                            table_node.set_children(col_ids);
                        }
                        nodes.insert(table_id, table_node);
                        table_items.push(table_id);
                    }
                    let mut tables_list = AkNode::new(AkRole::Tree);
                    tables_list.set_label("Tables".to_string());
                    tables_list.set_children(table_items);
                    nodes.insert(tables_list_id, tables_list);
                    children.push(tables_list_id);

                    if !diagram.edges.is_empty() {
                        let edges_list_id = alloc();
                        let mut edge_items: Vec<AkNodeId> = Vec::new();
                        for edge in &diagram.edges {
                            let edge_id = alloc();
                            let mut edge_node = AkNode::new(AkRole::ListItem);
                            edge_node.set_label(format!(
                                "{}.{} → {}.{} ({})",
                                edge.from_table,
                                edge.from_column,
                                edge.to_table,
                                edge.to_column,
                                edge.constraint_name,
                            ));
                            nodes.insert(edge_id, edge_node);
                            edge_items.push(edge_id);
                        }
                        let mut edges_list = AkNode::new(AkRole::List);
                        edges_list.set_label("Relationships".to_string());
                        edges_list.set_children(edge_items);
                        nodes.insert(edges_list_id, edges_list);
                        children.push(edges_list_id);
                    }
                }
            }
            ArchivePanelState::ConstraintView {
                schema,
                table,
                constraints,
                loading,
                ..
            } => {
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(if *loading {
                    format!("Constraints — {schema}.{table} — loading…")
                } else {
                    format!(
                        "Constraints — {schema}.{table} — {} constraint(s)",
                        constraints.len()
                    )
                });
                nodes.insert(heading_id, h);
                children.push(heading_id);

                let list_id = alloc();
                let mut items: Vec<AkNodeId> = Vec::new();
                for c in constraints {
                    let item_id = alloc();
                    let mut item = AkNode::new(AkRole::ListItem);
                    item.set_label(format!(
                        "{} [{:?}]{}",
                        c.name,
                        c.kind,
                        c.definition
                            .as_deref()
                            .map(|d| format!(": {d}"))
                            .unwrap_or_default()
                    ));
                    nodes.insert(item_id, item);
                    items.push(item_id);
                }
                let mut list = AkNode::new(AkRole::List);
                list.set_label("constraints".to_string());
                list.set_children(items);
                nodes.insert(list_id, list);
                children.push(list_id);
            }
            ArchivePanelState::IndexView {
                schema,
                table,
                indexes,
                loading,
                ..
            } => {
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(if *loading {
                    format!("Indexes — {schema}.{table} — loading…")
                } else {
                    format!("Indexes — {schema}.{table} — {} index(es)", indexes.len())
                });
                nodes.insert(heading_id, h);
                children.push(heading_id);

                let list_id = alloc();
                let mut items: Vec<AkNodeId> = Vec::new();
                for ix in indexes {
                    let item_id = alloc();
                    let mut item = AkNode::new(AkRole::ListItem);
                    item.set_label(format!(
                        "{} [{}]{}",
                        ix.index_name,
                        ix.index_method,
                        if ix.column_names.is_empty() {
                            String::new()
                        } else {
                            format!(" on ({})", ix.column_names.join(", "))
                        }
                    ));
                    nodes.insert(item_id, item);
                    items.push(item_id);
                }
                let mut list = AkNode::new(AkRole::List);
                list.set_label("indexes".to_string());
                list.set_children(items);
                nodes.insert(list_id, list);
                children.push(list_id);
            }
            ArchivePanelState::ConnectionEdit { profile, .. } => {
                // Form heading
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(format!("Edit connection: {}", profile.name));
                nodes.insert(heading_id, h);
                children.push(heading_id);

                // One TextInput per editable field
                let fields: &[(&str, String)] = &[
                    ("Name", profile.name.clone()),
                    ("URL env var", profile.url_env_key.clone()),
                    ("Color", profile.color.clone().unwrap_or_default()),
                    ("SSH host", profile.ssh_host.clone().unwrap_or_default()),
                    (
                        "SSH port",
                        profile.ssh_port.map_or(String::new(), |p| p.to_string()),
                    ),
                    ("SSH user", profile.ssh_user.clone().unwrap_or_default()),
                    (
                        "SSH key env var",
                        profile.ssh_key_env.clone().unwrap_or_default(),
                    ),
                    ("SSL mode", format!("{}", profile.ssl_mode)),
                    (
                        "SSL cert env var",
                        profile.ssl_cert_env.clone().unwrap_or_default(),
                    ),
                    (
                        "SSL key env var",
                        profile.ssl_key_env.clone().unwrap_or_default(),
                    ),
                    (
                        "SSL CA env var",
                        profile.ssl_ca_env.clone().unwrap_or_default(),
                    ),
                ];
                let form_id = alloc();
                let mut field_ids: Vec<AkNodeId> = Vec::new();
                for (label, value) in fields {
                    let field_id = alloc();
                    let mut field = AkNode::new(AkRole::TextInput);
                    field.set_label(label.to_string());
                    if !value.is_empty() {
                        field.set_value(Box::<str>::from(value.as_str()));
                    }
                    nodes.insert(field_id, field);
                    field_ids.push(field_id);
                }
                let mut form = AkNode::new(AkRole::Form);
                form.set_label(format!("Edit connection: {}", profile.name));
                form.set_children(field_ids);
                nodes.insert(form_id, form);
                children.push(form_id);
            }
            ArchivePanelState::ErrorView { message } => {
                let alert_id = alloc();
                let mut alert = AkNode::new(AkRole::Alert);
                alert.set_label(message.clone());
                nodes.insert(alert_id, alert);
                children.push(alert_id);
            }
        }

        children
    }

    /// Recursively build AccessKit nodes for an [`ExplainNode`] subtree.
    fn build_explain_node(
        &self,
        arena: &[ExplainNode],
        node: &ExplainNode,
        nodes: &mut std::collections::BTreeMap<accesskit::NodeId, accesskit::Node>,
        counter: &mut u64,
    ) -> accesskit::NodeId {
        use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};

        let id = AkNodeId::from(*counter);
        *counter += 1;

        let mut child_ids: Vec<AkNodeId> = Vec::new();
        for &idx in &node.children {
            child_ids.push(self.build_explain_node(arena, &arena[idx], nodes, counter));
        }

        let label = format!(
            "{} (cost {:.1}..{:.1} rows {})",
            node.node_type, node.startup_cost, node.total_cost, node.plan_rows,
        );
        let mut ak_node = AkNode::new(AkRole::TreeItem);
        ak_node.set_label(label);
        if !child_ids.is_empty() {
            ak_node.set_children(child_ids);
        }
        nodes.insert(id, ak_node);
        id
    }

    /// Build an AccessKit [`Role::Dialog`] node for the currently active
    /// overlay, if any.  Returns `None` when no overlay is open.
    fn build_overlay_node(
        &self,
        nodes: &mut std::collections::BTreeMap<accesskit::NodeId, accesskit::Node>,
        counter: &mut u64,
    ) -> Option<accesskit::NodeId> {
        use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};

        let mut alloc = || {
            let id = AkNodeId::from(*counter);
            *counter += 1;
            id
        };

        match &self.overlay_state {
            ArchiveOverlayState::HelpOpen => {
                let dialog_id = alloc();
                let mut dialog_children: Vec<AkNodeId> = Vec::new();

                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label("Key Bindings".to_string());
                nodes.insert(heading_id, h);
                dialog_children.push(heading_id);

                let list_id = alloc();
                let mut list_items: Vec<AkNodeId> = Vec::new();
                for kb in Self::bindings() {
                    let item_id = alloc();
                    let mut item = AkNode::new(AkRole::ListItem);
                    item.set_label(format!("{} — {}", kb.key, kb.action));
                    nodes.insert(item_id, item);
                    list_items.push(item_id);
                }
                let mut list = AkNode::new(AkRole::List);
                list.set_label("bindings".to_string());
                list.set_children(list_items);
                nodes.insert(list_id, list);
                dialog_children.push(list_id);

                let mut dialog = AkNode::new(AkRole::Dialog);
                dialog.set_label("Key Bindings".to_string());
                dialog.set_children(dialog_children);
                nodes.insert(dialog_id, dialog);
                Some(dialog_id)
            }

            ArchiveOverlayState::ExportPickerOpen { idx, formats } => {
                let dialog_id = alloc();
                let mut dialog_children: Vec<AkNodeId> = Vec::new();

                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label("Export Format".to_string());
                nodes.insert(heading_id, h);
                dialog_children.push(heading_id);

                let list_id = alloc();
                let mut list_items: Vec<AkNodeId> = Vec::new();
                for (i, fmt) in formats.iter().enumerate() {
                    let item_id = alloc();
                    let mut item = AkNode::new(AkRole::ListItem);
                    let label = if i == *idx {
                        format!("▶ {fmt}")
                    } else {
                        fmt.to_string()
                    };
                    item.set_label(label);
                    nodes.insert(item_id, item);
                    list_items.push(item_id);
                }
                let mut list = AkNode::new(AkRole::List);
                list.set_label("export formats".to_string());
                list.set_children(list_items);
                nodes.insert(list_id, list);
                dialog_children.push(list_id);

                let mut dialog = AkNode::new(AkRole::Dialog);
                dialog.set_label("Export Format".to_string());
                dialog.set_children(dialog_children);
                nodes.insert(dialog_id, dialog);
                Some(dialog_id)
            }

            ArchiveOverlayState::SavePromptOpen { text } => {
                let dialog_id = alloc();
                let mut dialog_children: Vec<AkNodeId> = Vec::new();

                let label_id = alloc();
                let mut label = AkNode::new(AkRole::Label);
                label.set_label("Save query as:".to_string());
                nodes.insert(label_id, label);
                dialog_children.push(label_id);

                let input_id = alloc();
                let mut input = AkNode::new(AkRole::TextInput);
                input.set_label("Query name".to_string());
                input.set_value(text.clone());
                nodes.insert(input_id, input);
                dialog_children.push(input_id);

                let mut dialog = AkNode::new(AkRole::Dialog);
                dialog.set_label("Save Query".to_string());
                dialog.set_children(dialog_children);
                nodes.insert(dialog_id, dialog);
                Some(dialog_id)
            }

            ArchiveOverlayState::SavedBrowserOpen { entries, idx } => {
                let dialog_id = alloc();
                let mut dialog_children: Vec<AkNodeId> = Vec::new();

                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label("Saved Queries".to_string());
                nodes.insert(heading_id, h);
                dialog_children.push(heading_id);

                let list_id = alloc();
                let mut list_items: Vec<AkNodeId> = Vec::new();
                for (i, sq) in entries.iter().enumerate() {
                    let item_id = alloc();
                    let mut item = AkNode::new(AkRole::ListItem);
                    let label = if i == *idx {
                        format!("▶ {}", sq.name)
                    } else {
                        sq.name.clone()
                    };
                    item.set_label(label);
                    nodes.insert(item_id, item);
                    list_items.push(item_id);
                }
                let mut list = AkNode::new(AkRole::List);
                list.set_label("saved queries".to_string());
                list.set_children(list_items);
                nodes.insert(list_id, list);
                dialog_children.push(list_id);

                let mut dialog = AkNode::new(AkRole::Dialog);
                dialog.set_label("Saved Queries".to_string());
                dialog.set_children(dialog_children);
                nodes.insert(dialog_id, dialog);
                Some(dialog_id)
            }

            ArchiveOverlayState::OverlayNone => None,
        }
    }
}

// ── Column width helpers for data-grid rendering ──────────────────────────────

/// Compute column display widths from a [`QueryResult`] for table rendering.
#[cfg(not(kani))]
pub fn column_widths(result: &QueryResult, max_col_width: usize) -> Vec<usize> {
    result
        .columns
        .iter()
        .enumerate()
        .map(|(ci, col)| {
            let header_w = col.name.len();
            let data_w = result
                .rows
                .rows
                .iter()
                .map(|row| {
                    row.0
                        .get(ci)
                        .map(|(_, v)| format!("{v:?}").len().min(max_col_width))
                        .unwrap_or(0)
                })
                .max()
                .unwrap_or(0);
            header_w.max(data_w).min(max_col_width)
        })
        .collect()
}

// ── Phase 3.5 — Multi-Connection Management ──────────────────────────────────

/// A set of named database connections with one active at a time.
///
/// `ConnectionSet` acts as a transparent proxy to the active [`ArchiveNavModel`]
/// via `Deref`/`DerefMut` — frontends can keep calling `self.model.foo()` after
/// renaming the field to `self.connections` with zero further changes.
///
/// The active URL is stored in the entry; `conn_active_url()` either resolves
/// `url_env_key` through `std::env::var` (for named connections) or returns the
/// stored override (for programmatic initialization via [`ConnectionSet::from_single`]).
pub struct ConnectionSet {
    entries: Vec<(ConnectionProfile, ArchiveNavModel, Option<String>)>,
    active: usize,
}

impl ConnectionSet {
    /// Build a `ConnectionSet` from a single nav model and optional URL.
    pub fn from_single(
        profile: ConnectionProfile,
        model: ArchiveNavModel,
        url: Option<String>,
    ) -> Self {
        Self {
            entries: vec![(profile, model, url)],
            active: 0,
        }
    }

    /// Add a new connection entry.
    pub fn conn_add(
        &mut self,
        profile: ConnectionProfile,
        model: ArchiveNavModel,
        url: Option<String>,
    ) {
        self.entries.push((profile, model, url));
    }

    /// Remove the active connection.  Clamps `active` to stay in bounds.
    pub fn conn_remove_active(&mut self) {
        if self.entries.len() > 1 {
            self.entries.remove(self.active);
            if self.active >= self.entries.len() {
                self.active = self.entries.len() - 1;
            }
        }
    }

    /// Set the active connection by index.  Returns `true` if `index` is in bounds.
    pub fn conn_set_active(&mut self, index: usize) -> bool {
        if index < self.entries.len() {
            self.active = index;
            true
        } else {
            false
        }
    }

    /// Advance to the next connection (wraps around).
    pub fn conn_next(&mut self) {
        self.active = (self.active + 1) % self.entries.len();
    }

    /// Go to the previous connection (wraps around).
    pub fn conn_prev(&mut self) {
        self.active = self.active.checked_sub(1).unwrap_or(self.entries.len() - 1);
    }

    /// Total number of connection entries.
    pub fn conn_len(&self) -> usize {
        self.entries.len()
    }

    /// Index of the currently active entry.
    pub fn conn_active_index(&self) -> usize {
        self.active
    }

    /// Connection profile for the active entry.
    pub fn conn_active_profile(&self) -> &ConnectionProfile {
        &self.entries[self.active].0
    }

    /// Resolve the URL for the active connection.
    ///
    /// Resolution order:
    /// 1. Stored URL override (set during programmatic initialization)
    /// 2. `std::env::var(&profile.url_env_key)` — for env-var-backed profiles
    /// 3. `None` — if neither resolves
    pub fn conn_active_url(&self) -> Option<String> {
        let (profile, _, url_override) = &self.entries[self.active];
        url_override
            .clone()
            .or_else(|| std::env::var(&profile.url_env_key).ok())
    }

    /// Tab-bar labels: `"name (active)"` for the current, `"name"` for others.
    pub fn conn_tab_labels(&self) -> Vec<String> {
        self.entries
            .iter()
            .enumerate()
            .map(|(i, (p, _, _))| {
                if i == self.active {
                    format!("{} ●", p.name)
                } else {
                    p.name.clone()
                }
            })
            .collect()
    }
}

impl std::ops::Deref for ConnectionSet {
    type Target = ArchiveNavModel;
    fn deref(&self) -> &ArchiveNavModel {
        &self.entries[self.active].1
    }
}

impl std::ops::DerefMut for ConnectionSet {
    fn deref_mut(&mut self) -> &mut ArchiveNavModel {
        &mut self.entries[self.active].1
    }
}
