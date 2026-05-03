//! Gallery level C23: 18-variant scale test mirroring `ArchivePanelState`.
//!
//! **Hypothesis**: Alt-Ergo can close proof obligations that arise from an
//! 18-arm `match` expression inside a `#[logic]` predicate, combining all
//! panel-machine patterns (Option, Vec, Box, bool-implication, usize cursors,
//! String, nested structs) at production scale.
//!
//! ## New patterns (relative to C1–C22)
//!
//! | Pattern | Question |
//! |---------|----------|
//! | 18-arm match in `#[logic]` | Does Alt-Ergo still close goals? |
//! | All prior patterns combined | Do compound invariants compose without timeout? |
//!
//! ## Types
//!
//! Mirrors `ArchivePanelState` with simplified placeholder types so the
//! gallery has no dependency on the production domain crates.
//!
//! ```text
//! C23Profile { name: String, port: i32 }      — boxed (ConnectionEdit)
//! C23HistoryEntry { text: String }             — Vec element
//! C23SavedQuery { name: String }               — Vec element
//! C23Constraint { name: String }               — Vec element
//! C23Index { name: String }                    — Vec element
//! C23EditState { tag: String }                 — Option element (DataGrid)
//!
//! C23State: 18 variants (see below)
//! ```
//!
//! ## Consistency invariant
//!
//! Non-trivial checks per variant:
//! - Variants with `schema` / `table` string fields: non-empty.
//! - `SqlEditor`: `*running ==> result.is_none()` (bool implication, C20).
//! - `DataGrid`: `grid_row@ <= entries@.len()` (cursor-in-bounds, C21).
//! - `ConnectionEdit`: `profile.name@.len() > 0 && profile.port@ > 0` (C22 Box deref).
//! - `ErrorView`: `message@.len() > 0`.
//! - Remaining unit / snapshot variants: `true`.
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

// ---------------------------------------------------------------------------
// Auxiliary types
// ---------------------------------------------------------------------------

/// Profile struct to be boxed inside `C23State::ConnectionEdit`.
pub struct C23Profile {
    pub name: String,
    pub port: i32,
}

/// Single history entry (simplified from `QueryHistoryEntry`).
pub struct C23HistoryEntry {
    pub text: String,
}

/// Single saved query (simplified from `SavedQuery`).
pub struct C23SavedQuery {
    pub name: String,
}

/// Single constraint descriptor (simplified from `ConstraintDescriptor`).
pub struct C23Constraint {
    pub name: String,
}

/// Single index descriptor (simplified from `IndexDescriptor`).
pub struct C23Index {
    pub name: String,
}

/// Staged row edits awaiting commit (simplified from `RowEditState`).
pub struct C23EditState {
    pub tag: String,
}

// ---------------------------------------------------------------------------
// Main 18-variant enum
// ---------------------------------------------------------------------------

/// 18-variant state mirroring `ArchivePanelState`.
pub enum C23State {
    /// 1 — column-detail landing view (default, no data payload).
    ColumnDetail,

    /// 2 — data fetch in progress.
    Loading {
        schema: String,
        label: String,
    },

    /// 3 — data grid with optional edit state and cursor.
    DataGrid {
        schema: String,
        table: String,
        entries: Vec<String>,
        grid_row: usize,
        edit_state: Option<C23EditState>,
    },

    /// 4 — SQL editor pane.
    SqlEditor {
        text: String,
        result: Option<String>,
        running: bool,
        error: Option<String>,
    },

    /// 5 — DDL viewer.
    DdlView {
        schema: String,
        table: String,
        ddl: String,
    },

    /// 6 — EXPLAIN plan viewer.
    ExplainView {
        schema: String,
        table: String,
        plan: String,
    },

    /// 7 — side-by-side EXPLAIN comparison.
    ExplainCompare {
        schema: String,
        table: String,
        left: String,
        right: String,
    },

    /// 8 — query history browser.
    HistoryView {
        entries: Vec<C23HistoryEntry>,
    },

    /// 9 — saved queries browser.
    SavedView {
        entries: Vec<C23SavedQuery>,
    },

    /// 10 — export format picker / result viewer.
    ExportView {
        schema: String,
        table: String,
        result: Option<String>,
    },

    /// 11 — key bindings / help panel.
    HelpView,

    /// 12 — live database monitoring panel.
    MonitorView {
        snapshot: String,
        loading: bool,
    },

    /// 13 — database administration panel.
    AdminView {
        snapshot: String,
        loading: bool,
    },

    /// 14 — entity-relationship diagram.
    ErdView {
        schema: String,
        diagram: String,
        layout: Option<String>,
        loading: bool,
    },

    /// 15 — constraint browser.
    ConstraintView {
        schema: String,
        table: String,
        constraints: Vec<C23Constraint>,
        loading: bool,
    },

    /// 16 — index browser.
    IndexView {
        schema: String,
        table: String,
        indexes: Vec<C23Index>,
        loading: bool,
    },

    /// 17 — connection profile editor.
    ConnectionEdit {
        profile: Box<C23Profile>,
    },

    /// 18 — error message panel.
    ErrorView {
        message: String,
    },
}

// ---------------------------------------------------------------------------
// Logic predicates
// ---------------------------------------------------------------------------

/// Whole-state consistency invariant across all 18 variants.
#[logic]
pub fn c23_consistent(s: &C23State) -> bool {
    pearlite! {
        match s {
            C23State::ColumnDetail => true,
            C23State::Loading { schema, label } =>
                schema@.len() > 0 && label@.len() > 0,
            C23State::DataGrid { schema, table, entries, grid_row, .. } =>
                schema@.len() > 0
                    && table@.len() > 0
                    && grid_row@ <= entries@.len(),
            C23State::SqlEditor { running, result, .. } =>
                *running ==> match result { None => true, Some(_) => false },
            C23State::DdlView { schema, table, .. } =>
                schema@.len() > 0 && table@.len() > 0,
            C23State::ExplainView { schema, table, .. } =>
                schema@.len() > 0 && table@.len() > 0,
            C23State::ExplainCompare { schema, table, .. } =>
                schema@.len() > 0 && table@.len() > 0,
            C23State::HistoryView { .. } => true,
            C23State::SavedView { .. } => true,
            C23State::ExportView { schema, table, .. } =>
                schema@.len() > 0 && table@.len() > 0,
            C23State::HelpView => true,
            C23State::MonitorView { .. } => true,
            C23State::AdminView { .. } => true,
            C23State::ErdView { schema, .. } => schema@.len() > 0,
            C23State::ConstraintView { schema, table, .. } =>
                schema@.len() > 0 && table@.len() > 0,
            C23State::IndexView { schema, table, .. } =>
                schema@.len() > 0 && table@.len() > 0,
            C23State::ConnectionEdit { profile } =>
                profile.name@.len() > 0 && profile.port@ > 0,
            C23State::ErrorView { message } => message@.len() > 0,
        }
    }
}

/// True when the state is `ColumnDetail`.
#[logic]
pub fn c23_is_column_detail(s: &C23State) -> bool {
    pearlite! {
        match s {
            C23State::ColumnDetail => true,
            _ => false,
        }
    }
}

/// True when the state is `Loading`.
#[logic]
pub fn c23_is_loading(s: &C23State) -> bool {
    pearlite! {
        match s {
            C23State::Loading { .. } => true,
            _ => false,
        }
    }
}

/// True when the state is `DataGrid`.
#[logic]
pub fn c23_is_data_grid(s: &C23State) -> bool {
    pearlite! {
        match s {
            C23State::DataGrid { .. } => true,
            _ => false,
        }
    }
}

/// True when the state is `SqlEditor`.
#[logic]
pub fn c23_is_sql_editor(s: &C23State) -> bool {
    pearlite! {
        match s {
            C23State::SqlEditor { .. } => true,
            _ => false,
        }
    }
}

/// True when the state is `ConnectionEdit`.
#[logic]
pub fn c23_is_connection_edit(s: &C23State) -> bool {
    pearlite! {
        match s {
            C23State::ConnectionEdit { .. } => true,
            _ => false,
        }
    }
}

/// True when the state is `ErrorView`.
#[logic]
pub fn c23_is_error_view(s: &C23State) -> bool {
    pearlite! {
        match s {
            C23State::ErrorView { .. } => true,
            _ => false,
        }
    }
}

// ---------------------------------------------------------------------------
// Transitions
// ---------------------------------------------------------------------------

/// Initial state — ColumnDetail.
#[ensures(c23_consistent(&result))]
#[ensures(c23_is_column_detail(&result))]
pub fn c23_new() -> C23State {
    C23State::ColumnDetail
}

/// Begin loading — transition from any state to Loading.
#[requires(schema@.len() > 0)]
#[requires(label@.len() > 0)]
#[ensures(c23_consistent(&result))]
#[ensures(c23_is_loading(&result))]
pub fn c23_start_loading(_s: C23State, schema: String, label: String) -> C23State {
    C23State::Loading { schema, label }
}

/// Load complete — deliver data grid results.
///
/// Precondition ensures cursor starts at 0 ≤ 0 = len(empty entries).
/// (In practice entries is non-empty; cursor 0 ≤ len trivially holds.)
#[requires(schema@.len() > 0)]
#[requires(table@.len() > 0)]
#[ensures(c23_consistent(&result))]
#[ensures(c23_is_data_grid(&result))]
pub fn c23_load_data_grid(
    _s: C23State,
    schema: String,
    table: String,
    entries: Vec<String>,
) -> C23State {
    C23State::DataGrid {
        schema,
        table,
        entries,
        grid_row: 0,
        edit_state: None,
    }
}

/// Open SQL editor — idle (not running, no result).
#[ensures(c23_consistent(&result))]
#[ensures(c23_is_sql_editor(&result))]
pub fn c23_open_sql_editor(_s: C23State) -> C23State {
    C23State::SqlEditor {
        text: String::new(),
        result: None,
        running: false,
        error: None,
    }
}

/// Start running a query — SqlEditor → SqlEditor(running=true, result=None).
#[requires(c23_is_sql_editor(&s))]
#[ensures(c23_consistent(&result))]
#[ensures(c23_is_sql_editor(&result))]
pub fn c23_run_query(s: C23State) -> C23State {
    match s {
        C23State::SqlEditor { text, .. } => C23State::SqlEditor {
            text,
            result: None,
            running: true,
            error: None,
        },
        _ => C23State::ColumnDetail,
    }
}

/// Open connection editor with a valid profile.
#[requires(name@.len() > 0)]
#[requires(port@ > 0)]
#[ensures(c23_consistent(&result))]
#[ensures(c23_is_connection_edit(&result))]
pub fn c23_open_connection_edit(_s: C23State, name: String, port: i32) -> C23State {
    C23State::ConnectionEdit {
        profile: Box::new(C23Profile { name, port }),
    }
}

/// Show error view.
#[requires(message@.len() > 0)]
#[ensures(c23_consistent(&result))]
#[ensures(c23_is_error_view(&result))]
pub fn c23_show_error(_s: C23State, message: String) -> C23State {
    C23State::ErrorView { message }
}

/// Close / navigate home — any state → ColumnDetail.
#[ensures(c23_consistent(&result))]
#[ensures(c23_is_column_detail(&result))]
pub fn c23_close(_s: C23State) -> C23State {
    C23State::ColumnDetail
}

/// Lifecycle — exercises multiple variant paths.
///
/// `new → loading → sql_editor → run_query → close`
#[requires(schema@.len() > 0)]
#[requires(label@.len() > 0)]
#[ensures(c23_consistent(&result))]
#[ensures(c23_is_column_detail(&result))]
pub fn c23_lifecycle(schema: String, label: String) -> C23State {
    let s0 = c23_new();
    let s1 = c23_start_loading(s0, schema, label);
    proof_assert!(c23_is_loading(&s1));
    let s2 = c23_open_sql_editor(s1);
    proof_assert!(c23_is_sql_editor(&s2));
    let s3 = c23_run_query(s2);
    proof_assert!(c23_is_sql_editor(&s3));
    c23_close(s3)
}
