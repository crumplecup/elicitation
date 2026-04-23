//! Verified State Machine for the archive main content panel.
//!
//! [`ArchivePanelMachine`] models the 18-variant panel state and embeds
//! AccessKit display modes in every data-carrying variant so that
//! [`ArchivePanelConsistent`] (the WCAG invariant) is provable by construction.
//!
//! ## State → display mode mapping
//!
//! | State variant | Embedded `*Mode` |
//! |---|---|
//! | `DataGrid` | `QueryResultMode` |
//! | `DdlView` | `DdlDescriptorMode` |
//! | `ExplainView` | `ExplainNodeMode` |
//! | `HistoryView` | `QueryHistoryEntryMode` |
//! | `SavedView` | `SavedQueryMode` |
//! | `MonitorView` | `MonitorSnapshotMode` |
//! | `AdminView` | `AdminSnapshotMode` |
//! | `ErdView` | `ErdDiagramMode` |
//! | `ConstraintView` | `ConstraintDescriptorMode` |
//! | `IndexView` | `IndexDescriptorMode` |
//! | `ConnectionEdit` | `ConnectionProfileMode` |
//!
//! ## Transitions
//!
//! Pure 2-param transitions satisfy [`VerifiedTransition<ArchivePanelMachine>`]
//! directly.  Parameterised constructors are ordinary functions; callers wrap
//! them in a closure.

use elicitation::{Elicit, Established, Prop, VerifiedStateMachine, formal_method};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::archive::display::{
    AdminSnapshotMode, ConnectionProfileMode, ConstraintDescriptorMode, DdlDescriptorMode,
    ErdDiagramMode, ExplainNodeMode, IndexDescriptorMode, MonitorSnapshotMode,
    QueryHistoryEntryMode, QueryResultMode, SavedQueryMode,
};
use crate::archive::types::{
    AdminSnapshot, ConnectionProfile, ConstraintDescriptor, DdlDescriptor, ErdDiagram, ErdLayout,
    ExplainComparison, ExplainNode, ExportResult, IndexDescriptor, MonitorSnapshot,
    QueryHistoryEntry, QueryResult, RowEditState, SavedQuery,
};

// ── ArchivePanelState ─────────────────────────────────────────────────────────

/// State of the main content panel.
///
/// Every variant that carries displayable data also carries the `*Mode` that
/// determines which AccessKit node tree is emitted, enforcing WCAG compliance
/// at the type level.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub enum ArchivePanelState {
    /// Column-detail view for the selected nav item (default landing).
    #[default]
    ColumnDetail,

    /// A data fetch is in progress.
    Loading {
        /// Schema being loaded.
        schema: String,
        /// Table or object being loaded.
        label: String,
    },

    /// A data grid showing a previewed or queried table.
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
        /// Active AccessKit display mode (WCAG contract).
        display_mode: QueryResultMode,
    },

    /// SQL editor pane.
    SqlEditor {
        /// Current editor text.
        text: String,
        /// Most recent query result, if any.
        result: Option<QueryResult>,
        /// Whether a query is currently executing.
        running: bool,
        /// Last execution error, if any.
        error: Option<String>,
    },

    /// DDL viewer pane.
    DdlView {
        /// Schema containing the object.
        schema: String,
        /// Object name.
        table: String,
        /// Reconstructed DDL descriptor.
        ddl: DdlDescriptor,
        /// Active AccessKit display mode (WCAG contract).
        display_mode: DdlDescriptorMode,
    },

    /// EXPLAIN plan viewer.
    ExplainView {
        /// Schema context.
        schema: String,
        /// Table name (or `"(custom)"` for SQL editor plans).
        table: String,
        /// Root node of the parsed plan tree.
        root: ExplainNode,
        /// Active AccessKit display mode (WCAG contract).
        display_mode: ExplainNodeMode,
    },

    /// Side-by-side EXPLAIN plan comparison.
    ExplainCompare {
        /// Schema context (from the newer plan).
        schema: String,
        /// Table context (from the newer plan).
        table: String,
        /// Comparison holding both plan roots and labels.
        comparison: ExplainComparison,
    },

    /// Query history browser.
    HistoryView {
        /// Cached history entries (newest first).
        entries: Vec<QueryHistoryEntry>,
        /// Active AccessKit display mode (WCAG contract).
        display_mode: QueryHistoryEntryMode,
    },

    /// Saved queries browser.
    SavedView {
        /// Cached saved queries (alphabetical).
        entries: Vec<SavedQuery>,
        /// Active AccessKit display mode (WCAG contract).
        display_mode: SavedQueryMode,
    },

    /// Export format picker / result viewer.
    ExportView {
        /// Schema of the table to export.
        schema: String,
        /// Table name to export.
        table: String,
        /// Completed export result, if available.
        result: Option<ExportResult>,
    },

    /// Key bindings / help panel.
    HelpView,

    /// Live database monitoring panel.
    MonitorView {
        /// Cached monitoring snapshot.
        snapshot: MonitorSnapshot,
        /// Whether a refresh fetch is in progress.
        loading: bool,
        /// Active AccessKit display mode (WCAG contract).
        display_mode: MonitorSnapshotMode,
    },

    /// Database administration panel.
    AdminView {
        /// Cached administration snapshot.
        snapshot: AdminSnapshot,
        /// Whether a refresh fetch is in progress.
        loading: bool,
        /// Active AccessKit display mode (WCAG contract).
        display_mode: AdminSnapshotMode,
    },

    /// Entity-relationship diagram panel.
    ErdView {
        /// Schema being visualised.
        schema: String,
        /// Cached ERD diagram.
        diagram: ErdDiagram,
        /// Computed grid layout; `None` until first fetch.
        layout: Option<ErdLayout>,
        /// Whether a fetch is in progress.
        loading: bool,
        /// Active AccessKit display mode (WCAG contract).
        display_mode: ErdDiagramMode,
    },

    /// Constraint browser for the selected table.
    ConstraintView {
        /// Schema of the table.
        schema: String,
        /// Table name.
        table: String,
        /// Constraints for the table.
        constraints: Vec<ConstraintDescriptor>,
        /// Whether a fetch is in progress.
        loading: bool,
        /// Active AccessKit display mode (WCAG contract).
        display_mode: ConstraintDescriptorMode,
    },

    /// Index browser for the selected table.
    IndexView {
        /// Schema of the table.
        schema: String,
        /// Table name.
        table: String,
        /// Indexes for the table.
        indexes: Vec<IndexDescriptor>,
        /// Whether a fetch is in progress.
        loading: bool,
        /// Active AccessKit display mode (WCAG contract).
        display_mode: IndexDescriptorMode,
    },

    /// Connection profile editor.
    ConnectionEdit {
        /// Clone of the profile being edited; changes not persisted until saved.
        profile: ConnectionProfile,
        /// Active AccessKit display mode (WCAG contract).
        display_mode: ConnectionProfileMode,
    },

    /// User-facing error message in the content area.
    ErrorView {
        /// Human-readable error message.
        message: String,
    },
}

// ── ArchivePanelConsistent (invariant) ────────────────────────────────────────

/// Proposition: the panel is rendering WCAG-compliant AccessKit nodes.
///
/// Proved when the active state's `*Mode` field is a valid mode for the
/// data it accompanies — i.e. `data.to_ak_nodes(&mode, 0)` is callable.
/// Bootstrap via `Established::assert()` for the default `ColumnDetail` state.
#[derive(Prop)]
pub struct ArchivePanelConsistent;

// ── ArchivePanelMachine ───────────────────────────────────────────────────────

/// Verified state machine for the archive main content panel.
#[derive(VerifiedStateMachine)]
#[vsm(transitions = [
    column_detail, panel_loading, panel_error,
    data_grid_ready, query_complete,
    begin_edit, commit_edits, abort_edits,
    open_sql_editor, open_export_panel, open_help_panel,
    open_saved_panel, open_connection_editor,
    ddl_ready, explain_ready, export_ready,
    history_ready, saved_ready, monitor_ready,
    admin_ready, erd_ready, constraints_ready, indexes_ready,
])]
pub struct ArchivePanelMachine;

// ── Transitions ───────────────────────────────────────────────────────────────

// ─── Navigation ───────────────────────────────────────────────────────────────

/// Return to the default column-detail view.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn column_detail(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (ArchivePanelState::ColumnDetail, proof)
}

/// Begin loading data for a table or object (spinner state).
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn panel_loading(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    schema: String,
    label: String,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (ArchivePanelState::Loading { schema, label }, proof)
}

/// Show a user-facing error in the panel.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn panel_error(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    message: String,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (ArchivePanelState::ErrorView { message }, proof)
}

// ─── Data Grid ────────────────────────────────────────────────────────────────

/// Async data fetch complete — show the data grid.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn data_grid_ready(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    schema: String,
    table: String,
    result: QueryResult,
    display_mode: QueryResultMode,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (
        ArchivePanelState::DataGrid {
            schema,
            table,
            result,
            page: 0,
            grid_row: 0,
            grid_col: 0,
            edit_state: None,
            display_mode,
        },
        proof,
    )
}

/// SQL query complete — update the SQL editor result.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn query_complete(
    state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    result: QueryResult,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    let next = match state {
        ArchivePanelState::SqlEditor { text, error, .. } => ArchivePanelState::SqlEditor {
            text,
            result: Some(result),
            running: false,
            error,
        },
        other => other,
    };
    (next, proof)
}

/// Begin a row-edit session in the data grid.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn begin_edit(
    state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    let next = match state {
        ArchivePanelState::DataGrid {
            schema,
            table,
            result,
            page,
            grid_row,
            grid_col,
            display_mode,
            ..
        } => ArchivePanelState::DataGrid {
            schema,
            table,
            result,
            page,
            grid_row,
            grid_col,
            edit_state: Some(RowEditState::default()),
            display_mode,
        },
        other => other,
    };
    (next, proof)
}

/// Commit staged edits and clear edit state.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn commit_edits(
    state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    let next = match state {
        ArchivePanelState::DataGrid {
            schema,
            table,
            result,
            page,
            grid_row,
            grid_col,
            display_mode,
            ..
        } => ArchivePanelState::DataGrid {
            schema,
            table,
            result,
            page,
            grid_row,
            grid_col,
            edit_state: None,
            display_mode,
        },
        other => other,
    };
    (next, proof)
}

/// Abort a row-edit session without committing.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn abort_edits(
    state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    commit_edits(state, proof) // same structural change: clear edit_state
}

// ─── Panel openers ────────────────────────────────────────────────────────────

/// Open the SQL editor pane.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn open_sql_editor(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    initial_text: String,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (
        ArchivePanelState::SqlEditor {
            text: initial_text,
            result: None,
            running: false,
            error: None,
        },
        proof,
    )
}

/// Open the export format picker for a table.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn open_export_panel(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    schema: String,
    table: String,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (
        ArchivePanelState::ExportView {
            schema,
            table,
            result: None,
        },
        proof,
    )
}

/// Open the help / key-bindings panel.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn open_help_panel(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (ArchivePanelState::HelpView, proof)
}

/// Open the query history browser.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn open_saved_panel(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    entries: Vec<SavedQuery>,
    display_mode: SavedQueryMode,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (
        ArchivePanelState::SavedView {
            entries,
            display_mode,
        },
        proof,
    )
}

/// Open the connection profile editor.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn open_connection_editor(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    profile: ConnectionProfile,
    display_mode: ConnectionProfileMode,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (
        ArchivePanelState::ConnectionEdit {
            profile,
            display_mode,
        },
        proof,
    )
}

// ─── Async result arrivals (PanelEvent equivalents) ───────────────────────────

/// DDL generation complete.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn ddl_ready(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    schema: String,
    table: String,
    ddl: DdlDescriptor,
    display_mode: DdlDescriptorMode,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (
        ArchivePanelState::DdlView {
            schema,
            table,
            ddl,
            display_mode,
        },
        proof,
    )
}

/// EXPLAIN plan ready.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn explain_ready(
    state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    schema: String,
    table: String,
    root: ExplainNode,
    display_mode: ExplainNodeMode,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    // If already showing an explain plan, promote to comparison view.
    let next = match state {
        ArchivePanelState::ExplainView {
            schema: old_schema,
            table: old_table,
            root: old_root,
            ..
        } => ArchivePanelState::ExplainCompare {
            schema: schema.clone(),
            table: table.clone(),
            comparison: ExplainComparison {
                left: old_root,
                right: root,
                label_left: format!("{old_schema}.{old_table}"),
                label_right: format!("{schema}.{table}"),
            },
        },
        _ => ArchivePanelState::ExplainView {
            schema,
            table,
            root,
            display_mode,
        },
    };
    (next, proof)
}

/// Export completed.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn export_ready(
    state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    result: ExportResult,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    let next = match state {
        ArchivePanelState::ExportView { schema, table, .. } => ArchivePanelState::ExportView {
            schema,
            table,
            result: Some(result),
        },
        other => other,
    };
    (next, proof)
}

/// History entries loaded.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn history_ready(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    entries: Vec<QueryHistoryEntry>,
    display_mode: QueryHistoryEntryMode,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (
        ArchivePanelState::HistoryView {
            entries,
            display_mode,
        },
        proof,
    )
}

/// Saved queries loaded.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn saved_ready(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    entries: Vec<SavedQuery>,
    display_mode: SavedQueryMode,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (
        ArchivePanelState::SavedView {
            entries,
            display_mode,
        },
        proof,
    )
}

/// Monitor snapshot ready.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn monitor_ready(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    snapshot: MonitorSnapshot,
    display_mode: MonitorSnapshotMode,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (
        ArchivePanelState::MonitorView {
            snapshot,
            loading: false,
            display_mode,
        },
        proof,
    )
}

/// Admin snapshot ready.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn admin_ready(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    snapshot: AdminSnapshot,
    display_mode: AdminSnapshotMode,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (
        ArchivePanelState::AdminView {
            snapshot,
            loading: false,
            display_mode,
        },
        proof,
    )
}

/// ERD diagram ready.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn erd_ready(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    schema: String,
    diagram: ErdDiagram,
    layout: Option<ErdLayout>,
    display_mode: ErdDiagramMode,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (
        ArchivePanelState::ErdView {
            schema,
            diagram,
            layout,
            loading: false,
            display_mode,
        },
        proof,
    )
}

/// Constraints loaded.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn constraints_ready(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    schema: String,
    table: String,
    constraints: Vec<ConstraintDescriptor>,
    display_mode: ConstraintDescriptorMode,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (
        ArchivePanelState::ConstraintView {
            schema,
            table,
            constraints,
            loading: false,
            display_mode,
        },
        proof,
    )
}

/// Indexes loaded.
#[formal_method(contracts = [ArchivePanelConsistent])]
#[instrument(skip(proof))]
pub fn indexes_ready(
    _state: ArchivePanelState,
    proof: Established<ArchivePanelConsistent>,
    schema: String,
    table: String,
    indexes: Vec<IndexDescriptor>,
    display_mode: IndexDescriptorMode,
) -> (ArchivePanelState, Established<ArchivePanelConsistent>) {
    (
        ArchivePanelState::IndexView {
            schema,
            table,
            indexes,
            loading: false,
            display_mode,
        },
        proof,
    )
}
