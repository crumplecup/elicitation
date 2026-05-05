//! Gallery level C21: `Vec<T>` sequence predicates and `usize` cursor-in-bounds.
//!
//! **Hypothesis**: Pearlite can express universal quantifiers over `Vec<T>` elements
//! and relational invariants between a `usize` cursor and `Vec` length, and Why3
//! can discharge the cursor-in-bounds proof obligations for `saturating_add` +
//! `min` cursor movement.
//!
//! ## New patterns (relative to C1–C20)
//!
//! | Pattern | Source in panel | New question |
//! |---------|-----------------|--------------|
//! | `Vec<String>` length in `#[logic]` | `HistoryView.entries`, nav `schemas` | `entries@.len() > 0` in Pearlite? |
//! | `forall` quantifier over `Vec` | entries consistency | `forall |i: Int| 0 <= i && i < entries@.len() ==> entries@[i]@.len() > 0`? |
//! | `usize` cursor-in-bounds relational invariant | `NavReady.cursor`, `DataGrid.grid_row` | `cursor@ < entries@.len()` in Pearlite? |
//! | `saturating_add` / `min` cursor movement proof | `move_cursor_down`, `move_cursor_up` | Why3 closes cursor-in-bounds after saturating move? |
//! | `forall` in `#[requires]` | passing entries with all-non-empty precondition | quantifier propagation across transition? |
//!
//! ## Types
//!
//! ```text
//! C21State:
//!   Empty
//!   Loaded { entries: Vec<String>, cursor: usize }
//! ```
//!
//! ## Consistency invariant
//!
//! ```text
//! c21_consistent(s) ≡ match s {
//!   Empty   → true
//!   Loaded { entries, cursor } →
//!       entries@.len() > 0                                                    ← non-empty Vec
//!     ∧ cursor@ < entries@.len()                                               ← cursor in bounds
//!     ∧ forall |i: Int| 0 <= i && i < entries@.len() ==> entries@[i]@.len() > 0  ← all elements non-empty
//! }
//! ```
//!
//! ## Experiment table
//!
//! | ID    | What                                                                  | Expected |
//! |-------|-----------------------------------------------------------------------|----------|
//! | C21a  | `entries@.len() > 0` in `#[logic]`                                   | ✓        |
//! | C21b  | `cursor@ < entries@.len()` relational invariant                       | ✓        |
//! | C21c  | `forall` quantifier over `Vec<String>` elements                       | ✓        |
//! | C21d  | `forall` propagation through a forwarding transition                  | ✓        |
//! | C21e  | `move_cursor_down`: saturating_add + min closes cursor-in-bounds      | ✓        |
//! | C21f  | `move_cursor_up`: saturating_sub closes cursor-in-bounds              | ✓        |
//! | C21g  | Lifecycle: new → load → move_down → move_up → clear                   | ✓        |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

// ── State enum ────────────────────────────────────────────────────────────────

/// Two-variant list state — models `HistoryView` / `NavReady` patterns.
pub enum C21State {
    /// No data loaded.
    Empty,

    /// Data loaded with a cursor.
    ///
    /// Mirrors `HistoryView { entries, .. }` or `NavReady { schemas, cursor, .. }`.
    Loaded {
        /// Non-empty list of non-empty entry strings.
        entries: Vec<String>,
        /// Current row cursor (< entries.len()).
        cursor: usize,
    },
}

// ── Logic predicates ──────────────────────────────────────────────────────────

/// C21a + C21b + C21c: consistency invariant with Vec quantifier.
///
/// Key new sub-expressions:
/// - `entries@.len() > 0` — Vec non-empty (C21a)
/// - `cursor@ < entries@.len()` — cursor in bounds (C21b)
/// - `forall |i: Int| ... ==> entries@[i]@.len() > 0` — all elements non-empty (C21c)
#[logic]
pub fn c21_consistent(s: &C21State) -> bool {
    pearlite! {
        match s {
            C21State::Empty => true,
            C21State::Loaded { entries, cursor } =>
                entries@.len() > 0
                && cursor@ < entries@.len()
                && forall<i: Int> 0 <= i && i < entries@.len() ==> entries@[i]@.len() > 0,
        }
    }
}

#[logic]
pub fn c21_is_empty(s: &C21State) -> bool {
    pearlite! { match s { C21State::Empty => true, _ => false } }
}

#[logic]
pub fn c21_is_loaded(s: &C21State) -> bool {
    pearlite! { match s { C21State::Loaded { .. } => true, _ => false } }
}

// ── Transitions ───────────────────────────────────────────────────────────────

/// Empty initial state.
#[ensures(c21_consistent(&result))]
#[ensures(c21_is_empty(&result))]
pub fn c21_new() -> C21State {
    C21State::Empty
}

/// C21d: load entries — caller supplies a non-empty Vec with all-non-empty elements.
///
/// **Key precondition**: `forall` quantifier in `#[requires]`.
/// Cursor is initialised to 0 (always in bounds given `entries.len() > 0`).
#[requires(entries@.len() > 0)]
#[requires(forall<i: Int> 0 <= i && i < entries@.len() ==> entries@[i]@.len() > 0)]
#[ensures(c21_consistent(&result))]
#[ensures(c21_is_loaded(&result))]
pub fn c21_load(_s: C21State, entries: Vec<String>) -> C21State {
    C21State::Loaded { entries, cursor: 0 }
}

/// C21e: move cursor down one row — saturating increment capped at `len - 1`.
///
/// **Key proof obligation**: `saturating_add(1).min(len - 1) < len` when `len > 0`.
#[requires(c21_consistent(&s))]
#[requires(c21_is_loaded(&s))]
#[ensures(c21_consistent(&result))]
#[ensures(c21_is_loaded(&result))]
pub fn c21_cursor_down(s: C21State) -> C21State {
    match s {
        C21State::Loaded { entries, cursor } => {
            let max = entries.len().saturating_sub(1);
            C21State::Loaded {
                cursor: cursor.saturating_add(1).min(max),
                entries,
            }
        }
        other => other,
    }
}

/// C21f: move cursor up one row — saturating decrement (floor at 0).
///
/// `saturating_sub(1)` is always <= old cursor, so cursor-in-bounds is preserved.
#[requires(c21_consistent(&s))]
#[requires(c21_is_loaded(&s))]
#[ensures(c21_consistent(&result))]
#[ensures(c21_is_loaded(&result))]
pub fn c21_cursor_up(s: C21State) -> C21State {
    match s {
        C21State::Loaded { entries, cursor } => C21State::Loaded {
            cursor: cursor.saturating_sub(1),
            entries,
        },
        other => other,
    }
}

/// Return to empty state.
#[requires(c21_consistent(&s))]
#[ensures(c21_consistent(&result))]
#[ensures(c21_is_empty(&result))]
pub fn c21_clear(s: C21State) -> C21State {
    let _ = s;
    C21State::Empty
}

// ── Lifecycle ─────────────────────────────────────────────────────────────────

/// C21g: full lifecycle — new → load → move_down → move_up → clear.
///
/// Tests that the `forall` quantifier in `c21_load`'s precondition propagates
/// through cursor movement and back to the cleared state.
#[requires(entries@.len() > 0)]
#[requires(forall<i: Int> 0 <= i && i < entries@.len() ==> entries@[i]@.len() > 0)]
#[ensures(c21_consistent(&result))]
#[ensures(c21_is_empty(&result))]
pub fn c21_lifecycle(entries: Vec<String>) -> C21State {
    let s0 = c21_new();
    let s1 = c21_load(s0, entries); // cursor = 0, all-non-empty entries
    let s2 = c21_cursor_down(s1); // cursor = min(1, len-1)
    let s3 = c21_cursor_up(s2); // cursor = max(0, cursor-1)
    c21_clear(s3)
}
