//! Gallery level C11: panel machine with embedded display-mode enum.
//!
//! **Hypothesis**: Pearlite can match on and reason about enum variants that
//! carry *another* enum as a field.  This is the `ArchivePanelState`
//! `QueryResult { mode: QueryResultMode, ... }` pattern.
//!
//! Key question: when a struct variant field is itself an enum, can nested
//! `match` expressions in `#[logic]` functions produce goals that Alt-Ergo
//! closes?
//!
//! ## State diagram
//!
//! ```text
//! Empty ──load──► Grid { Read, n }
//!                     │
//!             select_row ▼
//!                 Detail { Read }
//!                     │         ▲
//!               begin_edit     save
//!                     ▼         │
//!                 Detail { Edit }
//! ```
//!
//! ## Experiment table
//!
//! | ID    | What                                                | Expected |
//! |-------|-----------------------------------------------------|----------|
//! | C11a  | Sub-enum mode predicates (`is_read`, `is_edit`)    | ✓        |
//! | C11b  | Nested match: `c11_grid_is_read`, `c11_detail_is_*`| ✓        |
//! | C11c  | Constructors satisfy mode + consistency postconds  | ✓        |
//! | C11d  | `load`: any → Grid { Read, row_count }             | ✓        |
//! | C11e  | `select_row`: Grid → Detail { Read }               | ✓        |
//! | C11f  | `begin_edit`: Detail { Read } → Detail { Edit }    | ✓        |
//! | C11g  | `save`: Detail { Edit } → Detail { Read }          | ✓        |
//! | C11h  | Full panel lifecycle (postconds chain end-to-end)  | ✓        |

use creusot_std::prelude::*;

// ── Display-mode sub-enum ─────────────────────────────────────────────────────

/// Read-only vs editing display mode — embedded in Grid and Detail variants.
///
/// The nested enum is the key test: can Pearlite access `MiniDisplayMode`
/// fields inside `MiniPanelState` struct variants?
pub enum MiniDisplayMode {
    /// View-only mode.
    Read,
    /// In-place editing mode.
    Edit,
}

// ── Panel state enum ──────────────────────────────────────────────────────────

/// Four-variant panel state mirroring `ArchivePanelState`.
pub enum MiniPanelState {
    /// No data loaded.
    Empty,
    /// A grid view of rows, each selectable.
    Grid {
        /// Current display mode for the grid.
        mode: MiniDisplayMode,
        /// Number of rows (must be positive).
        row_count: i64,
    },
    /// A single-row detail view.
    Detail {
        /// Whether the detail is view-only or being edited.
        mode: MiniDisplayMode,
    },
    /// An error condition.
    Error {
        /// Human-readable error message (non-empty).
        message: String,
    },
}

// ── C11a: mode sub-predicates ─────────────────────────────────────────────────

/// Returns `true` when the display mode is `Read`.
#[logic]
pub fn c11_is_read(m: &MiniDisplayMode) -> bool {
    pearlite! {
        match m {
            MiniDisplayMode::Read => true,
            MiniDisplayMode::Edit => false,
        }
    }
}

/// Returns `true` when the display mode is `Edit`.
#[logic]
pub fn c11_is_edit(m: &MiniDisplayMode) -> bool {
    pearlite! {
        match m {
            MiniDisplayMode::Read => false,
            MiniDisplayMode::Edit => true,
        }
    }
}

// ── C11b: nested mode predicates on panel state ───────────────────────────────

/// Returns `true` when the state is `Grid` with `Read` mode.
///
/// Tests nested enum field access: match on `MiniPanelState`, then
/// delegate to `c11_is_read` on the inner `MiniDisplayMode`.
#[logic]
pub fn c11_grid_is_read(s: &MiniPanelState) -> bool {
    pearlite! {
        match s {
            MiniPanelState::Grid { mode, .. } => c11_is_read(mode),
            _ => false,
        }
    }
}

/// Returns `true` when the state is `Detail` with `Read` mode.
#[logic]
pub fn c11_detail_is_read(s: &MiniPanelState) -> bool {
    pearlite! {
        match s {
            MiniPanelState::Detail { mode } => c11_is_read(mode),
            _ => false,
        }
    }
}

/// Returns `true` when the state is `Detail` with `Edit` mode.
#[logic]
pub fn c11_detail_is_edit(s: &MiniPanelState) -> bool {
    pearlite! {
        match s {
            MiniPanelState::Detail { mode } => c11_is_edit(mode),
            _ => false,
        }
    }
}

// ── Consistency predicate ─────────────────────────────────────────────────────

/// Structural invariant: `Grid` needs positive `row_count`; `Error` needs
/// a non-empty message.  `Empty` and `Detail` are unconditionally consistent.
#[logic]
pub fn c11_consistent(s: &MiniPanelState) -> bool {
    pearlite! {
        match s {
            MiniPanelState::Empty => true,
            MiniPanelState::Grid { mode: _, row_count } => row_count@ > 0,
            MiniPanelState::Detail { mode: _ } => true,
            MiniPanelState::Error { message } => message@.len() > 0,
        }
    }
}

// ── C11c: constructors ────────────────────────────────────────────────────────

/// Construct a `Grid` in Read mode with a positive row count.
#[requires(row_count@ > 0)]
#[ensures(c11_consistent(&result))]
#[ensures(c11_grid_is_read(&result))]
pub fn c11_mk_grid(row_count: i64) -> MiniPanelState {
    MiniPanelState::Grid {
        mode: MiniDisplayMode::Read,
        row_count,
    }
}

/// Construct a `Detail` in Read mode.
#[ensures(c11_consistent(&result))]
#[ensures(c11_detail_is_read(&result))]
pub fn c11_mk_detail_read() -> MiniPanelState {
    MiniPanelState::Detail {
        mode: MiniDisplayMode::Read,
    }
}

/// Construct a `Detail` in Edit mode.
#[ensures(c11_consistent(&result))]
#[ensures(c11_detail_is_edit(&result))]
pub fn c11_mk_detail_edit() -> MiniPanelState {
    MiniPanelState::Detail {
        mode: MiniDisplayMode::Edit,
    }
}

/// Construct an `Error` state.
#[requires(message@.len() > 0)]
#[ensures(c11_consistent(&result))]
pub fn c11_mk_error(message: String) -> MiniPanelState {
    MiniPanelState::Error { message }
}

// ── C11d: load ────────────────────────────────────────────────────────────────

/// C11d: Load data — any state → Grid { Read, row_count }.
///
/// The caller provides a positive row count; the mode starts at Read.
#[requires(row_count@ > 0)]
#[ensures(c11_consistent(&result))]
#[ensures(c11_grid_is_read(&result))]
pub fn c11_load(_s: MiniPanelState, row_count: i64) -> MiniPanelState {
    MiniPanelState::Grid {
        mode: MiniDisplayMode::Read,
        row_count,
    }
}

// ── C11e: select_row ──────────────────────────────────────────────────────────

/// C11e: Select a row — Grid → Detail { Read }.
///
/// Row data is not carried into the detail (the detail invariant does not
/// need it).  The mode starts at Read.
#[requires(c11_consistent(&s))]
#[ensures(c11_consistent(&result))]
#[ensures(c11_detail_is_read(&result))]
pub fn c11_select_row(s: MiniPanelState) -> MiniPanelState {
    let _ = s;
    MiniPanelState::Detail {
        mode: MiniDisplayMode::Read,
    }
}

// ── C11f: begin_edit ──────────────────────────────────────────────────────────

/// C11f: Begin editing — Detail { Read } → Detail { Edit }.
///
/// The `c11_detail_is_read` precondition is the Creusot equivalent of a
/// `stub_verified` or proof token: it ensures the caller is in Read mode
/// before requesting an edit.  Why3 uses this as an axiom at every call site.
#[requires(c11_detail_is_read(&s))]
#[ensures(c11_consistent(&result))]
#[ensures(c11_detail_is_edit(&result))]
pub fn c11_begin_edit(s: MiniPanelState) -> MiniPanelState {
    let _ = s;
    MiniPanelState::Detail {
        mode: MiniDisplayMode::Edit,
    }
}

// ── C11g: save ────────────────────────────────────────────────────────────────

/// C11g: Save — Detail { Edit } → Detail { Read }.
///
/// Requires edit mode (guards against spurious saves when not editing).
/// The `c11_detail_is_read` postcondition propagates to enable the caller
/// to prove that the next `begin_edit` precondition is satisfied.
#[requires(c11_detail_is_edit(&s))]
#[ensures(c11_consistent(&result))]
#[ensures(c11_detail_is_read(&result))]
pub fn c11_save(s: MiniPanelState) -> MiniPanelState {
    let _ = s;
    MiniPanelState::Detail {
        mode: MiniDisplayMode::Read,
    }
}

// ── C11h: full panel lifecycle ────────────────────────────────────────────────

/// C11h: Full lifecycle — Empty → Grid { Read } → Detail { Read } →
///       Detail { Edit } → Detail { Read }.
///
/// Why3 chains postconditions: `c11_load` → `c11_grid_is_read` (flows into
/// `c11_select_row`'s requires), then `c11_detail_is_read` (flows into
/// `c11_begin_edit`'s requires), then `c11_detail_is_edit` (flows into
/// `c11_save`'s requires).  Mode tracking through nested enum predicates
/// is automatic.
#[requires(row_count@ > 0)]
#[ensures(c11_detail_is_read(&result))]
pub fn c11_full_lifecycle(row_count: i64) -> MiniPanelState {
    let s0 = MiniPanelState::Empty;
    let s1 = c11_load(s0, row_count); // ensures: c11_consistent, c11_grid_is_read
    let s2 = c11_select_row(s1); // ensures: c11_consistent, c11_detail_is_read
    let s3 = c11_begin_edit(s2); // ensures: c11_consistent, c11_detail_is_edit
    c11_save(s3) // ensures: c11_consistent, c11_detail_is_read
}
