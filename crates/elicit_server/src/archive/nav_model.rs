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

use std::collections::HashMap;

use elicit_accesskit::KeyBinding;

use crate::archive::{
    AdminSnapshot, AdminTab, ColumnStats, ConnectionProfile, ErdDiagram, ExplainNode, ExportFormat,
    MonitorSnapshot, QueryHistoryEntry, QueryResult, RowEditState, SavedQuery, StagedEdit,
    TableInspection,
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
}

// ── SchemaWithExpand ──────────────────────────────────────────────────────────

/// A schema entry combined with its current expand/collapse state.
#[derive(Debug, Clone)]
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
}

// ── PanelMode ─────────────────────────────────────────────────────────────────

/// What the central content panel should display.
#[derive(Debug, Clone)]
pub enum PanelMode {
    /// Column detail for the currently selected schema or table (default).
    ColumnDetail,
    /// A data grid for a previewed table.
    DataGrid {
        /// Schema the table belongs to.
        schema: String,
        /// Table name.
        table: String,
        /// Query result holding columns + rows.
        result: QueryResult,
        /// Current page index (0-based).
        page: u32,
        /// Row cursor within the current page (0-based).
        grid_row: usize,
        /// Column cursor within the current page (0-based).
        grid_col: usize,
        /// Staged row edits awaiting commit, or `None` when not in edit mode.
        edit_state: Option<RowEditState>,
    },
    /// A data fetch is in progress (spinner state).
    Loading {
        /// Schema being loaded.
        schema: String,
        /// Table being loaded.
        table: String,
    },
    /// SQL editor pane (Phase 1.2).
    SqlEditor {
        /// Current editor text.
        text: String,
        /// Most recent query result, if any.
        result: Option<QueryResult>,
        /// Whether a query is running.
        running: bool,
        /// Last execution error, if any (web frontend only).
        error: Option<String>,
    },
    /// DDL viewer pane.
    Ddl {
        /// Schema containing the object.
        schema: String,
        /// Object name.
        table: String,
        /// Reconstructed DDL text.
        ddl: String,
    },
    /// EXPLAIN plan viewer.
    ExplainPlan {
        /// Schema containing the table.
        schema: String,
        /// Table name (or `"(custom)"` for SQL editor plans).
        table: String,
        /// Root node of the parsed plan tree.
        root: ExplainNode,
    },
    /// Query history browser panel (web frontend).
    HistoryPanel {
        /// Cached history entries to display.
        entries: Vec<QueryHistoryEntry>,
    },
    /// Saved queries browser panel (web frontend).
    SavedPanel {
        /// Cached saved queries to display.
        entries: Vec<SavedQuery>,
    },
    /// Export format picker panel (web frontend).
    ExportPanel {
        /// Schema of the table to export.
        schema: String,
        /// Table name to export.
        table: String,
    },
    /// Key bindings / help panel (web frontend).
    HelpPanel,
    /// Live database monitoring panel — sessions, roles, cache hit, backups.
    MonitorPanel {
        /// Cached monitoring snapshot; empty until first fetch completes.
        snapshot: MonitorSnapshot,
        /// Whether a fetch is currently in progress.
        loading: bool,
    },
    /// Database administration panel — roles, backups/WAL, server settings.
    AdminPanel {
        /// Cached administration snapshot; empty until first fetch completes.
        snapshot: AdminSnapshot,
        /// Whether a fetch is currently in progress.
        loading: bool,
    },
    /// Entity-relationship diagram panel for a single schema.
    ErdPanel {
        /// Schema being visualised.
        schema: String,
        /// Cached ERD diagram; empty until first fetch completes.
        diagram: ErdDiagram,
        /// Whether a fetch is currently in progress.
        loading: bool,
    },
}

impl Default for PanelMode {
    fn default() -> Self {
        Self::ColumnDetail
    }
}

impl PanelMode {
    /// Returns `true` when the panel is showing a data grid.
    pub fn is_data_grid(&self) -> bool {
        matches!(self, Self::DataGrid { .. })
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
    FetchMonitor,
    /// Fetch an administration snapshot (roles, backups/WAL, settings, extensions).
    FetchAdmin,
    /// Fetch an ERD diagram for the given schema.
    FetchErd {
        /// Schema to diagram.
        schema: String,
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
        /// Parsed plan root.
        root: ExplainNode,
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
    /// Whether the keybinding help overlay is shown.
    pub show_help: bool,
    /// Ephemeral status flash (e.g. refresh confirmation).
    pub flash: Option<String>,
    /// Current filter string (empty means no filter).
    pub filter: String,
    /// Whether the filter bar is active (accepting keystrokes).
    pub filter_active: bool,
    /// Current content panel mode.
    pub panel: PanelMode,
    /// Cached FK/constraint/index enrichment, keyed by `(schema, table)`.
    pub table_inspections: HashMap<(String, String), TableInspection>,
    /// Cached per-column planner statistics, keyed by `(schema, table)`.
    pub column_stats: HashMap<(String, String), Vec<ColumnStats>>,
    /// Most recent export result (schema, table, content, format).
    pub last_export: Option<(String, String, String, ExportFormat)>,
    /// In-memory history cache (newest first), loaded at startup.
    pub history_cache: Vec<QueryHistoryEntry>,
    /// Current position in history navigation (0 = most recent).
    /// `None` means the user has not started cycling history.
    pub history_idx: Option<usize>,
    /// In-memory saved-query cache (alphabetical), loaded at startup.
    pub saved_cache: Vec<SavedQuery>,
    /// Whether the export format picker overlay is shown.
    pub export_picker: bool,
    /// Currently highlighted option in the export picker (0–3).
    pub export_picker_idx: usize,
    /// Whether the save-name prompt modal is shown.
    pub save_prompt_active: bool,
    /// Text being typed into the save-name prompt.
    pub save_prompt_text: String,
    /// Whether the saved-queries browser overlay is shown.
    pub saved_browser_active: bool,
    /// Currently highlighted row in the saved-queries browser.
    pub saved_browser_idx: usize,
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
            })
            .collect();

        let mut model = Self {
            db_name: nav.db_name,
            version: nav.version,
            backend_label: nav.backend.to_string(),
            schemas,
            flat: Vec::new(),
            cursor: 0,
            show_help: false,
            flash: None,
            filter: String::new(),
            filter_active: false,
            panel: PanelMode::ColumnDetail,
            table_inspections: HashMap::new(),
            column_stats: HashMap::new(),
            last_export: None,
            history_cache: Vec::new(),
            history_idx: None,
            saved_cache: Vec::new(),
            export_picker: false,
            export_picker_idx: 0,
            save_prompt_active: false,
            save_prompt_text: String::new(),
            saved_browser_active: false,
            saved_browser_idx: 0,
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
            })
            .collect();
        self.db_name = nav.db_name;
        self.version = nav.version;
        self.backend_label = nav.backend.to_string();
        self.rebuild_flat();
        // Try to preserve the old cursor position.
        if let Some(item) = old_cursor_item {
            if let Some(pos) = self.flat.iter().position(|f| *f == item) {
                self.cursor = pos;
            }
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
        let Some(&item) = self.flat.get(self.cursor) else {
            return None;
        };
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
                self.panel = PanelMode::Loading {
                    schema: schema.clone(),
                    table: table.clone(),
                };
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
            // Leaf nodes: Enter does nothing for now (detail panel in a future phase).
            FlatItem::Function(..) | FlatItem::Sequence(..) | FlatItem::TypeEntry(..) => None,
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
        self.show_help = !self.show_help;
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
        match &self.panel {
            PanelMode::DataGrid {
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
            PanelMode::SqlEditor {
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
        if self.save_prompt_active {
            return KeyMapMode::SavePrompt;
        }
        if self.saved_browser_active {
            return KeyMapMode::SavedBrowser;
        }
        if self.export_picker {
            return KeyMapMode::ExportPicker;
        }
        if matches!(self.panel, PanelMode::SqlEditor { .. }) {
            return KeyMapMode::SqlEditor;
        }
        KeyMapMode::Default
    }

    /// Return the SQL text of the currently-focused saved query, if any.
    ///
    /// Used by frontends to implement [`ArchiveAction::SavedBrowserSelect`].
    pub fn load_focused_saved_query_text(&self) -> Option<String> {
        self.saved_cache
            .get(self.saved_browser_idx)
            .map(|q| q.sql.clone())
    }

    /// Remove the currently-focused saved query from the local cache.
    ///
    /// Returns `(id, name)` of the removed entry so the caller can enqueue a
    /// deletion on the persistent store.  Returns `None` if the cache is empty.
    ///
    /// Used by frontends to implement [`ArchiveAction::SavedBrowserDelete`].
    pub fn remove_focused_saved_query(&mut self) -> Option<(i64, String)> {
        let idx = self.saved_browser_idx;
        if idx >= self.saved_cache.len() {
            return None;
        }
        let q = self.saved_cache.remove(idx);
        if idx > 0 && idx >= self.saved_cache.len() {
            self.saved_browser_prev();
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
        match &mut self.panel {
            PanelMode::SqlEditor { text, .. } => {
                *text = sql;
                true
            }
            _ => {
                // Activate sql editor mode with the chosen history entry.
                self.panel = PanelMode::SqlEditor {
                    text: sql,
                    result: None,
                    running: false,
                    error: None,
                };
                true
            }
        }
    }

    // ── Overlay state helpers ─────────────────────────────────────────────────

    /// Toggle the export format picker overlay.
    pub fn toggle_export_picker(&mut self) {
        self.export_picker = !self.export_picker;
        if self.export_picker {
            self.export_picker_idx = 0;
        }
    }

    /// Move the export picker selection up.
    pub fn export_picker_prev(&mut self) {
        if self.export_picker_idx > 0 {
            self.export_picker_idx -= 1;
        }
    }

    /// Move the export picker selection down (max 3).
    pub fn export_picker_next(&mut self) {
        if self.export_picker_idx < 3 {
            self.export_picker_idx += 1;
        }
    }

    /// Confirm the export picker — returns the chosen [`ExportFormat`].
    pub fn confirm_export_picker(&mut self) -> ExportFormat {
        self.export_picker = false;
        match self.export_picker_idx {
            0 => ExportFormat::Csv,
            1 => ExportFormat::Json,
            2 => ExportFormat::Tsv,
            _ => ExportFormat::Ndjson,
        }
    }

    /// Open the save-name prompt (SQL editor context).
    pub fn open_save_prompt(&mut self) {
        self.save_prompt_active = true;
        self.save_prompt_text.clear();
    }

    /// Append a character to the save-name prompt text.
    pub fn save_prompt_push(&mut self, ch: char) {
        self.save_prompt_text.push(ch);
    }

    /// Delete the last character from the save-name prompt text.
    pub fn save_prompt_backspace(&mut self) {
        self.save_prompt_text.pop();
    }

    /// Close the save-name prompt, discarding any typed text.
    pub fn close_save_prompt(&mut self) {
        self.save_prompt_active = false;
        self.save_prompt_text.clear();
    }

    /// Consume the save-name prompt text, closing the prompt.
    ///
    /// Returns `None` if the prompt is not active or the text is blank.
    pub fn take_save_prompt(&mut self) -> Option<String> {
        if !self.save_prompt_active {
            return None;
        }
        let name = self.save_prompt_text.trim().to_string();
        self.close_save_prompt();
        if name.is_empty() { None } else { Some(name) }
    }

    /// Toggle the saved-queries browser overlay.
    pub fn toggle_saved_browser(&mut self) {
        self.saved_browser_active = !self.saved_browser_active;
        if self.saved_browser_active {
            self.saved_browser_idx = 0;
        }
    }

    // ── Phase 5 — Monitor panel ───────────────────────────────────────────────

    /// Open the live monitor panel (or close it if already open).
    ///
    /// Returns `Some(FetchRequest::FetchMonitor)` when opening so the caller
    /// can dispatch a data fetch; returns `None` when closing.
    pub fn toggle_monitor_panel(&mut self) -> Option<FetchRequest> {
        if matches!(self.panel, PanelMode::MonitorPanel { .. }) {
            self.panel = PanelMode::ColumnDetail;
            return None;
        }
        self.panel = PanelMode::MonitorPanel {
            snapshot: MonitorSnapshot::default(),
            loading: true,
        };
        Some(FetchRequest::FetchMonitor)
    }

    /// Apply a completed monitoring snapshot to the monitor panel.
    ///
    /// No-ops if the panel is no longer in `MonitorPanel` mode (e.g. the user
    /// navigated away before the fetch completed).
    pub fn apply_monitor_snapshot(&mut self, snapshot: MonitorSnapshot) {
        if let PanelMode::MonitorPanel {
            snapshot: s,
            loading,
        } = &mut self.panel
        {
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
        if matches!(self.panel, PanelMode::AdminPanel { .. }) {
            self.panel = PanelMode::ColumnDetail;
            return None;
        }
        self.panel = PanelMode::AdminPanel {
            snapshot: AdminSnapshot::default(),
            loading: true,
        };
        Some(FetchRequest::FetchAdmin)
    }

    /// Apply a completed administration snapshot to the admin panel.
    ///
    /// No-ops if the panel is no longer in `AdminPanel` mode.
    pub fn apply_admin_snapshot(&mut self, snapshot: AdminSnapshot) {
        if let PanelMode::AdminPanel {
            snapshot: s,
            loading,
        } = &mut self.panel
        {
            *s = snapshot;
            *loading = false;
        }
    }

    /// Cycle to the next admin panel tab (`]` key).
    ///
    /// No-ops when not in `AdminPanel` mode.
    pub fn admin_tab_next(&mut self) {
        if let PanelMode::AdminPanel { snapshot, .. } = &mut self.panel {
            snapshot.active_tab = snapshot.active_tab.next();
        }
    }

    /// Cycle to the previous admin panel tab (`[` key).
    ///
    /// No-ops when not in `AdminPanel` mode.
    pub fn admin_tab_prev(&mut self) {
        if let PanelMode::AdminPanel { snapshot, .. } = &mut self.panel {
            snapshot.active_tab = snapshot.active_tab.prev();
        }
    }

    /// Return the name of the currently selected schema (works for schema,
    /// table, function, sequence, and type nodes).
    pub fn selected_schema_name(&self) -> Option<String> {
        use crate::archive::nav_model::FlatItem;
        match self.selected()? {
            FlatItem::Schema(si) => Some(self.schemas[si].entry.name.clone()),
            FlatItem::Table(si, _)
            | FlatItem::FunctionsGroup(si)
            | FlatItem::SequencesGroup(si)
            | FlatItem::TypesGroup(si)
            | FlatItem::Function(si, _)
            | FlatItem::Sequence(si, _)
            | FlatItem::TypeEntry(si, _, _) => Some(self.schemas[si].entry.name.clone()),
        }
    }

    /// Open the ERD panel for the currently selected schema, or close it if
    /// already open.
    ///
    /// Returns `Some(FetchRequest::FetchErd)` when opening so the caller can
    /// dispatch a background fetch.  Returns `None` on close.
    pub fn toggle_erd_panel(&mut self) -> Option<FetchRequest> {
        let schema = self.selected_schema_name()?;
        if matches!(self.panel, PanelMode::ErdPanel { .. }) {
            self.panel = PanelMode::ColumnDetail;
            return None;
        }
        self.panel = PanelMode::ErdPanel {
            schema: schema.clone(),
            diagram: ErdDiagram::default(),
            loading: true,
        };
        Some(FetchRequest::FetchErd { schema })
    }

    /// Apply a completed ERD diagram to the panel.
    ///
    /// No-ops if the panel is no longer in `ErdPanel` mode.
    pub fn apply_erd_diagram(&mut self, diagram: ErdDiagram) {
        if let PanelMode::ErdPanel {
            diagram: d,
            loading,
            ..
        } = &mut self.panel
        {
            *d = diagram;
            *loading = false;
        }
    }

    /// Move the saved-browser selection up.
    pub fn saved_browser_prev(&mut self) {
        if self.saved_browser_idx > 0 {
            self.saved_browser_idx -= 1;
        }
    }

    /// Move the saved-browser selection down.
    pub fn saved_browser_next(&mut self) {
        if !self.saved_cache.is_empty() {
            self.saved_browser_idx = (self.saved_browser_idx + 1).min(self.saved_cache.len() - 1);
        }
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

    /// Move the data-grid row cursor up one row (wraps at top).
    ///
    /// Returns `false` if the active panel is not a `DataGrid`.
    pub fn grid_row_up(&mut self) -> bool {
        if let PanelMode::DataGrid {
            result, grid_row, ..
        } = &mut self.panel
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

    /// Move the data-grid row cursor down one row (wraps at bottom).
    ///
    /// Returns `false` if the active panel is not a `DataGrid`.
    pub fn grid_row_down(&mut self) -> bool {
        if let PanelMode::DataGrid {
            result, grid_row, ..
        } = &mut self.panel
        {
            let max = result.rows.rows.len().saturating_sub(1);
            *grid_row = if *grid_row >= max { 0 } else { *grid_row + 1 };
            return true;
        }
        false
    }

    /// Move the data-grid column cursor left one column (wraps).
    ///
    /// Returns `false` if the active panel is not a `DataGrid`.
    pub fn grid_col_left(&mut self) -> bool {
        if let PanelMode::DataGrid {
            result, grid_col, ..
        } = &mut self.panel
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

    /// Move the data-grid column cursor right one column (wraps).
    ///
    /// Returns `false` if the active panel is not a `DataGrid`.
    pub fn grid_col_right(&mut self) -> bool {
        if let PanelMode::DataGrid {
            result, grid_col, ..
        } = &mut self.panel
        {
            let max = result.columns.len().saturating_sub(1);
            *grid_col = if *grid_col >= max { 0 } else { *grid_col + 1 };
            return true;
        }
        false
    }

    /// Enter edit mode: create a [`RowEditState`] on the current `DataGrid`.
    ///
    /// Returns `false` if the active panel is not a `DataGrid` or edit mode is
    /// already active.
    pub fn begin_edit_mode(&mut self) -> bool {
        if let PanelMode::DataGrid { edit_state, .. } = &mut self.panel {
            if edit_state.is_none() {
                *edit_state = Some(RowEditState::new());
                return true;
            }
        }
        false
    }

    /// Start typing into the currently selected cell.
    ///
    /// Pre-fills the input buffer with the cell's current displayed value so
    /// the user can edit it in place.  Returns `false` if no edit session is
    /// active or the cursor is out of range.
    pub fn begin_cell_edit(&mut self) -> bool {
        if let PanelMode::DataGrid {
            result,
            grid_row,
            grid_col,
            edit_state: Some(es),
            ..
        } = &mut self.panel
        {
            if es.editing_cell.is_some() {
                return false; // already editing
            }
            let row_idx = *grid_row;
            let col_idx = *grid_col;
            if let Some(row) = result.rows.rows.get(row_idx) {
                if let Some((_, val)) = row.0.get(col_idx) {
                    es.editing_cell = Some((row_idx, col_idx));
                    es.input_buffer = format!("{val:?}");
                    return true;
                }
            }
        }
        false
    }

    /// Append a character to the in-progress cell edit buffer.
    pub fn cell_edit_push(&mut self, ch: char) {
        if let PanelMode::DataGrid {
            edit_state: Some(es),
            ..
        } = &mut self.panel
        {
            if es.editing_cell.is_some() {
                es.input_buffer.push(ch);
            } else if es.inserting_row.is_some() {
                es.input_buffer.push(ch);
            }
        }
    }

    /// Delete the last character from the in-progress cell edit buffer.
    pub fn cell_edit_pop(&mut self) {
        if let PanelMode::DataGrid {
            edit_state: Some(es),
            ..
        } = &mut self.panel
        {
            if es.editing_cell.is_some() || es.inserting_row.is_some() {
                es.input_buffer.pop();
            }
        }
    }

    /// Finalise the in-progress cell edit and add it to `pending_edits`.
    ///
    /// The primary-key values are derived from all columns whose
    /// [`ColumnDescriptor::is_primary_key`] flag is `true` in the current result.
    ///
    /// Returns `false` if no cell edit was in progress.
    pub fn stage_cell_edit(&mut self) -> bool {
        if let PanelMode::DataGrid {
            schema,
            table,
            result,
            edit_state: Some(es),
            ..
        } = &mut self.panel
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

    /// Enter new-row insertion mode: prepare an empty form with one slot per column.
    ///
    /// Returns `false` if no edit session is active or insertion is already in progress.
    pub fn begin_insert_row(&mut self) -> bool {
        if let PanelMode::DataGrid {
            result,
            edit_state: Some(es),
            ..
        } = &mut self.panel
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

    /// Advance to the next column in the new-row insertion form.
    ///
    /// Saves the current `input_buffer` into the current column slot.
    /// Returns `true` when the cursor wrapped around (all columns filled once).
    pub fn insert_row_next_col(&mut self) -> bool {
        if let PanelMode::DataGrid {
            edit_state: Some(es),
            ..
        } = &mut self.panel
        {
            if let Some(form) = &mut es.inserting_row {
                let cursor = es.insert_col_cursor;
                if let Some(slot) = form.get_mut(cursor) {
                    slot.1 = std::mem::take(&mut es.input_buffer);
                }
                es.insert_col_cursor += 1;
                if es.insert_col_cursor >= form.len() {
                    es.insert_col_cursor = 0;
                    return true; // wrapped
                }
            }
        }
        false
    }

    /// Finalise the new-row form and add an `Insert` edit to `pending_edits`.
    ///
    /// Returns `false` if no insertion was in progress.
    pub fn stage_insert_row(&mut self) -> bool {
        if let PanelMode::DataGrid {
            schema,
            table,
            edit_state: Some(es),
            ..
        } = &mut self.panel
        {
            let Some(mut form) = es.inserting_row.take() else {
                return false;
            };
            // Save whatever is still in the buffer into the current slot
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

    /// Mark the currently selected row for deletion.
    ///
    /// Returns `false` if no edit session is active or the row is already marked.
    pub fn stage_delete_row(&mut self) -> bool {
        if let PanelMode::DataGrid {
            schema,
            table,
            result,
            grid_row,
            edit_state: Some(es),
            ..
        } = &mut self.panel
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
    /// Returns `false` if no edit session was active.
    pub fn discard_edit_mode(&mut self) -> bool {
        if let PanelMode::DataGrid { edit_state, .. } = &mut self.panel {
            if edit_state.is_some() {
                *edit_state = None;
                return true;
            }
        }
        false
    }

    /// Drain and return the pending edits together with the schema and table name.
    ///
    /// Clears the staged edits and exits edit mode.  Returns `None` if there is
    /// no active edit session or no pending mutations.
    pub fn take_staged_edits(&mut self) -> Option<(String, String, Vec<StagedEdit>)> {
        if let PanelMode::DataGrid {
            schema,
            table,
            edit_state,
            ..
        } = &mut self.panel
        {
            if let Some(es) = edit_state.take() {
                if !es.pending_edits.is_empty() {
                    return Some((schema.clone(), table.clone(), es.pending_edits));
                }
            }
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

    // ── IR pipeline ───────────────────────────────────────────────────────────

    /// Build a fully-described [`VerifiedTree`] from the current model state.
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
    #[tracing::instrument(skip(self))]
    pub fn to_verified_tree(
        &self,
    ) -> crate::archive::ArchiveResult<(
        elicit_ui::VerifiedTree,
        elicitation::Established<elicit_ui::IrSourced>,
    )> {
        use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};
        use elicit_ui::{VerifiedTree, Viewport};
        use std::collections::HashMap;

        let mut nodes: HashMap<AkNodeId, AkNode> = HashMap::new();
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
        content_node.set_description("id=content".to_string());
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
    #[tracing::instrument(skip(self))]
    pub fn to_content_tree(
        &self,
    ) -> crate::archive::ArchiveResult<(
        elicit_ui::VerifiedTree,
        elicitation::Established<elicit_ui::IrSourced>,
    )> {
        use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};
        use elicit_ui::{VerifiedTree, Viewport};
        use std::collections::HashMap;

        let mut nodes: HashMap<AkNodeId, AkNode> = HashMap::new();
        let mut counter: u64 = 1;
        let content_children = self.build_content_nodes(&mut nodes, &mut counter);

        let root_id = AkNodeId::from(0u64);
        let mut content_node = AkNode::new(AkRole::GenericContainer);
        content_node.set_description("id=content".to_string());
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
        use std::collections::HashMap;

        let mut nodes: HashMap<AkNodeId, AkNode> = HashMap::new();
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
        nodes: &mut std::collections::HashMap<accesskit::NodeId, accesskit::Node>,
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
        nodes: &mut std::collections::HashMap<accesskit::NodeId, accesskit::Node>,
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
    fn build_content_nodes(
        &self,
        nodes: &mut std::collections::HashMap<accesskit::NodeId, accesskit::Node>,
        counter: &mut u64,
    ) -> Vec<accesskit::NodeId> {
        use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};

        let mut children: Vec<AkNodeId> = Vec::new();
        let mut alloc = || {
            let id = AkNodeId::from(*counter);
            *counter += 1;
            id
        };

        match &self.panel {
            PanelMode::ColumnDetail => {
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

            PanelMode::Loading { schema, table } => {
                let prog_id = alloc();
                let mut prog = AkNode::new(AkRole::ProgressIndicator);
                prog.set_label(format!("Loading {schema}.{table}…"));
                nodes.insert(prog_id, prog);
                children.push(prog_id);
            }

            PanelMode::DataGrid {
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

            PanelMode::SqlEditor {
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

            PanelMode::Ddl { schema, table, ddl } => {
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(format!("DDL: {schema}.{table}"));
                nodes.insert(heading_id, h);
                children.push(heading_id);

                let code_id = alloc();
                let mut code = AkNode::new(AkRole::Code);
                code.set_label(ddl.clone());
                nodes.insert(code_id, code);
                children.push(code_id);
            }

            PanelMode::ExplainPlan {
                schema,
                table,
                root,
            } => {
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(format!("EXPLAIN: {schema}.{table}"));
                nodes.insert(heading_id, h);
                children.push(heading_id);

                let plan_root_id = alloc();
                let plan_id = self.build_explain_node(root, nodes, counter);
                let mut plan_tree = AkNode::new(AkRole::Tree);
                plan_tree.set_label("query plan".to_string());
                plan_tree.set_children(vec![plan_id]);
                nodes.insert(plan_root_id, plan_tree);
                children.push(plan_root_id);
            }

            PanelMode::HistoryPanel { entries } => {
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

            PanelMode::SavedPanel { entries } => {
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

            PanelMode::ExportPanel { schema, table } => {
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

            PanelMode::HelpPanel => {
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
            PanelMode::MonitorPanel { snapshot, loading } => {
                let heading_id = alloc();
                let mut h = AkNode::new(AkRole::Heading);
                h.set_label(if *loading {
                    "Monitor — loading…".to_string()
                } else {
                    format!(
                        "Monitor — {} sessions, {} roles, cache hit {}",
                        snapshot.sessions.len(),
                        snapshot.roles.len(),
                        snapshot
                            .cache_hit
                            .map(|r| format!("{:.1}%", r * 100.0))
                            .unwrap_or_else(|| "n/a".to_string()),
                    )
                });
                nodes.insert(heading_id, h);
                children.push(heading_id);

                // Sessions list
                let list_id = alloc();
                let mut list_items: Vec<AkNodeId> = Vec::new();
                for s in &snapshot.sessions {
                    let item_id = alloc();
                    let mut item = AkNode::new(AkRole::ListItem);
                    item.set_label(format!("pid={} {} [{}]", s.pid, s.app_name, s.state,));
                    nodes.insert(item_id, item);
                    list_items.push(item_id);
                }
                let mut list = AkNode::new(AkRole::List);
                list.set_label("sessions".to_string());
                list.set_children(list_items);
                nodes.insert(list_id, list);
                children.push(list_id);

                // Roles list
                let roles_id = alloc();
                let mut role_items: Vec<AkNodeId> = Vec::new();
                for r in &snapshot.roles {
                    let item_id = alloc();
                    let mut item = AkNode::new(AkRole::ListItem);
                    item.set_label(format!(
                        "{}{}{} ",
                        r.name,
                        if r.superuser { " [superuser]" } else { "" },
                        if r.can_login { " [login]" } else { "" },
                    ));
                    nodes.insert(item_id, item);
                    role_items.push(item_id);
                }
                let mut roles_list = AkNode::new(AkRole::List);
                roles_list.set_label("roles".to_string());
                roles_list.set_children(role_items);
                nodes.insert(roles_id, roles_list);
                children.push(roles_id);
            }
            PanelMode::AdminPanel { snapshot, loading } => {
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
            PanelMode::ErdPanel {
                schema,
                diagram,
                loading,
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

                // Table nodes list
                let tables_list_id = alloc();
                let mut table_items: Vec<AkNodeId> = Vec::new();
                for node_entry in &diagram.nodes {
                    // Table header item
                    let table_id = alloc();
                    let mut table_node = AkNode::new(AkRole::TreeItem);
                    table_node.set_label(format!(
                        "{}  ({} cols)",
                        node_entry.table,
                        node_entry.columns.len()
                    ));
                    // Column children
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

                // Relationships list
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

        children
    }

    /// Recursively build AccessKit nodes for an [`ExplainNode`] subtree.
    fn build_explain_node(
        &self,
        node: &ExplainNode,
        nodes: &mut std::collections::HashMap<accesskit::NodeId, accesskit::Node>,
        counter: &mut u64,
    ) -> accesskit::NodeId {
        use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};

        let id = AkNodeId::from(*counter);
        *counter += 1;

        let mut child_ids: Vec<AkNodeId> = Vec::new();
        for child in &node.children {
            child_ids.push(self.build_explain_node(child, nodes, counter));
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
        nodes: &mut std::collections::HashMap<accesskit::NodeId, accesskit::Node>,
        counter: &mut u64,
    ) -> Option<accesskit::NodeId> {
        use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};

        let mut alloc = || {
            let id = AkNodeId::from(*counter);
            *counter += 1;
            id
        };

        if self.show_help {
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
            return Some(dialog_id);
        }

        if self.export_picker {
            let dialog_id = alloc();
            let mut dialog_children: Vec<AkNodeId> = Vec::new();

            let heading_id = alloc();
            let mut h = AkNode::new(AkRole::Heading);
            h.set_label("Export Format".to_string());
            nodes.insert(heading_id, h);
            dialog_children.push(heading_id);

            let list_id = alloc();
            let formats = ["CSV", "JSON", "TSV", "SQL"];
            let mut list_items: Vec<AkNodeId> = Vec::new();
            for (i, fmt) in formats.iter().enumerate() {
                let item_id = alloc();
                let mut item = AkNode::new(AkRole::ListItem);
                let label = if i == self.export_picker_idx {
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
            return Some(dialog_id);
        }

        if self.save_prompt_active {
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
            input.set_value(self.save_prompt_text.clone());
            nodes.insert(input_id, input);
            dialog_children.push(input_id);

            let mut dialog = AkNode::new(AkRole::Dialog);
            dialog.set_label("Save Query".to_string());
            dialog.set_children(dialog_children);
            nodes.insert(dialog_id, dialog);
            return Some(dialog_id);
        }

        if self.saved_browser_active {
            let dialog_id = alloc();
            let mut dialog_children: Vec<AkNodeId> = Vec::new();

            let heading_id = alloc();
            let mut h = AkNode::new(AkRole::Heading);
            h.set_label("Saved Queries".to_string());
            nodes.insert(heading_id, h);
            dialog_children.push(heading_id);

            let list_id = alloc();
            let mut list_items: Vec<AkNodeId> = Vec::new();
            for (i, sq) in self.saved_cache.iter().enumerate() {
                let item_id = alloc();
                let mut item = AkNode::new(AkRole::ListItem);
                let label = if i == self.saved_browser_idx {
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
            return Some(dialog_id);
        }

        None
    }
}

// ── Column width helpers for data-grid rendering ──────────────────────────────

/// Compute column display widths from a [`QueryResult`] for table rendering.
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
