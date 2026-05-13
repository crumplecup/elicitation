//! Gallery level C20: `Option<T>` fields and bool implication in `#[logic]`.
//!
//! **Hypothesis**: Pearlite can match on `Option<String>` and `Option<Struct>`
//! fields inside enum variants, and can express boolean implication (`==>`)
//! as an invariant relating a `bool` field to `Option` fields.
//!
//! ## New patterns (relative to C1–C19)
//!
//! | Pattern | Source in panel | New question |
//! |---------|-----------------|--------------|
//! | `Option<String>` match in `#[logic]` | `SqlEditor.error`, `SqlEditor.result` | `match opt { Some(s) => s@.len() > 0, None => true }` in Pearlite? |
//! | `Option<Struct>` match in `#[logic]` | `DataGrid.edit_state` | `match opt { Some(e) => consistent(e), None => true }` in Pearlite? |
//! | Bool implication `==>` in invariant | `SqlEditor.running` gate | `*running ==> match result { None => true, ... }` in Pearlite? |
//! | Postcondition on `Option` field | transitions setting/clearing Options | `#[ensures]` on `Option`-typed fields via predicates? |
//!
//! ## Types
//!
//! ```text
//! C20EditState { tag: String }                  — nested struct inside Option
//!
//! C20State:
//!   Idle
//!   Editing { query: String, running: bool, result: Option<String>, error: Option<String> }
//!   Browsing { data: String, edit_state: Option<C20EditState> }
//! ```
//!
//! ## Consistency invariant
//!
//! ```text
//! c20_consistent(s) ≡ match s {
//!   Idle                                    → true
//!   Editing { query, running, result, error } →
//!       query non-empty
//!     ∧ (*running ==> result is None)        ← bool implication over Option
//!     ∧ (*running ==> error is None)
//!     ∧ (if Some(r), r non-empty)
//!     ∧ (if Some(e), e non-empty)
//!   Browsing { data, edit_state } →
//!       data non-empty
//!     ∧ (if Some(es), es.tag non-empty)      ← Option<Struct> consistency
//! }
//! ```
//!
//! ## Experiment table
//!
//! | ID    | What                                                              | Expected |
//! |-------|-------------------------------------------------------------------|----------|
//! | C20a  | `Option<String>` match inside `#[logic]`                         | ✓        |
//! | C20b  | `*running ==>` implication in `#[logic]`                         | ✓        |
//! | C20c  | `Option<Struct>` match with struct consistency delegate           | ✓        |
//! | C20d  | `c20_is_running` predicate matching `bool` field in variant       | ✓        |
//! | C20e  | Transition sets `Option` field: None → Some in proof chain        | ✓        |
//! | C20f  | Transition clears `Option` field: Some → None in proof chain      | ✓        |
//! | C20g  | Lifecycle: Idle → Editing(running) → Editing(done) → Idle         | ✓        |
//! | C20h  | Lifecycle: Idle → Editing(running) → Editing(failed) → Idle       | ✓        |
//! | C20i  | `Browsing` lifecycle: begin_edit → abort_edit                     | ✓        |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

// ── Auxiliary types ───────────────────────────────────────────────────────────

/// Staged row edits — mirrors `RowEditState`.
pub struct C20EditState {
    /// Non-empty tag identifying the edit session.
    pub tag: String,
}

/// C20c (part 1): consistency for the edit state struct.
#[logic]
pub fn c20_edit_consistent(e: &C20EditState) -> bool {
    pearlite! { e.tag@.len() > 0 }
}

// ── State enum ────────────────────────────────────────────────────────────────

/// Two-variant panel state — models `SqlEditor` and `DataGrid` patterns.
pub enum C20State {
    /// Default landing — no active editor or data view.
    Idle,

    /// SQL editor: query text + async run status + optional result/error.
    ///
    /// Mirrors `ArchivePanelState::SqlEditor`.
    Editing {
        /// Editor text (non-empty).
        query: String,
        /// True while a query is executing.
        running: bool,
        /// Query result string, if available (non-empty when Some).
        result: Option<String>,
        /// Last error message, if any (non-empty when Some).
        error: Option<String>,
    },

    /// Data viewer: data string + optional row-edit session.
    ///
    /// Mirrors `ArchivePanelState::DataGrid`.
    Browsing {
        /// Data payload (non-empty).
        data: String,
        /// Active edit session, if any.
        edit_state: Option<C20EditState>,
    },
}

// ── Logic predicates ──────────────────────────────────────────────────────────

/// C20a + C20b + C20c: composite consistency invariant.
///
/// Key new sub-expressions:
/// - `*running ==> ...` — bool implication (C20b)
/// - `match result { Some(r) => ..., None => true }` — Option<String> match (C20a)
/// - `match edit_state { Some(es) => c20_edit_consistent(es), None => true }` — Option<Struct> (C20c)
#[logic]
pub fn c20_consistent(s: &C20State) -> bool {
    pearlite! {
        match s {
            C20State::Idle => true,
            C20State::Editing { query, running, result, error } =>
                query@.len() > 0
                && (*running ==> match result { None => true, Some(_) => false })
                && (*running ==> match error { None => true, Some(_) => false })
                && (match result { Some(r) => r@.len() > 0, None => true })
                && (match error  { Some(e) => e@.len() > 0, None => true }),
            C20State::Browsing { data, edit_state } =>
                data@.len() > 0
                && (match edit_state { Some(es) => c20_edit_consistent(es), None => true }),
        }
    }
}

/// C20d: true when in `Editing` state with `running = true`.
#[logic]
pub fn c20_is_running(s: &C20State) -> bool {
    pearlite! {
        match s {
            C20State::Editing { running, .. } => *running,
            _ => false,
        }
    }
}

#[logic]
pub fn c20_is_idle(s: &C20State) -> bool {
    pearlite! { match s { C20State::Idle => true, _ => false } }
}

#[logic]
pub fn c20_is_browsing(s: &C20State) -> bool {
    pearlite! { match s { C20State::Browsing { .. } => true, _ => false } }
}

/// True when `Editing` and not running (a result or error has arrived).
#[logic]
pub fn c20_edit_done(s: &C20State) -> bool {
    pearlite! {
        match s {
            C20State::Editing { running, .. } => !*running,
            _ => false,
        }
    }
}

/// True when `Browsing` with an active edit session.
#[logic]
pub fn c20_is_editing(s: &C20State) -> bool {
    pearlite! {
        match s {
            C20State::Browsing { edit_state, .. } => match edit_state { Some(_) => true, None => false },
            _ => false,
        }
    }
}

// ── Transitions ───────────────────────────────────────────────────────────────

/// Idle state.
#[ensures(c20_consistent(&result))]
#[ensures(c20_is_idle(&result))]
pub fn c20_new() -> C20State {
    C20State::Idle
}

/// C20e (part 1): start a query — sets `running = true`, both Options None.
#[requires(c20_consistent(&s))]
#[requires(c20_is_idle(&s))]
#[requires(query@.len() > 0)]
#[ensures(c20_consistent(&result))]
#[ensures(c20_is_running(&result))]
pub fn c20_start_query(s: C20State, query: String) -> C20State {
    let _ = s;
    C20State::Editing {
        query,
        running: true,
        result: None,
        error: None,
    }
}

/// C20e (part 2): query succeeds — sets `result = Some(r)`, clears `running`.
///
/// **New combination**: postcondition requires `result` is Some with non-empty content,
/// while consistency demands `!running ==> ...` no longer constrains Options.
#[requires(c20_consistent(&s))]
#[requires(c20_is_running(&s))]
#[requires(result_str@.len() > 0)]
#[ensures(c20_consistent(&result))]
#[ensures(c20_edit_done(&result))]
pub fn c20_complete(s: C20State, result_str: String) -> C20State {
    match s {
        C20State::Editing { query, .. } => C20State::Editing {
            query,
            running: false,
            result: Some(result_str),
            error: None,
        },
        other => other,
    }
}

/// C20f: query fails — sets `error = Some(msg)`, clears `running`.
#[requires(c20_consistent(&s))]
#[requires(c20_is_running(&s))]
#[requires(error_msg@.len() > 0)]
#[ensures(c20_consistent(&result))]
#[ensures(c20_edit_done(&result))]
pub fn c20_fail(s: C20State, error_msg: String) -> C20State {
    match s {
        C20State::Editing { query, .. } => C20State::Editing {
            query,
            running: false,
            result: None,
            error: Some(error_msg),
        },
        other => other,
    }
}

/// Open the data browser with no active edit.
#[requires(c20_consistent(&s))]
#[requires(data@.len() > 0)]
#[ensures(c20_consistent(&result))]
#[ensures(c20_is_browsing(&result))]
pub fn c20_open_browse(s: C20State, data: String) -> C20State {
    let _ = s;
    C20State::Browsing {
        data,
        edit_state: None,
    }
}

/// Begin a row-edit session — `edit_state` goes from `None` to `Some`.
#[requires(c20_consistent(&s))]
#[requires(c20_is_browsing(&s))]
#[requires(tag@.len() > 0)]
#[ensures(c20_consistent(&result))]
#[ensures(c20_is_editing(&result))]
pub fn c20_begin_edit(s: C20State, tag: String) -> C20State {
    match s {
        C20State::Browsing { data, .. } => C20State::Browsing {
            data,
            edit_state: Some(C20EditState { tag }),
        },
        other => other,
    }
}

/// C20f: abort the edit session — `edit_state` goes from `Some` to `None`.
#[requires(c20_consistent(&s))]
#[requires(c20_is_editing(&s))]
#[ensures(c20_consistent(&result))]
#[ensures(c20_is_browsing(&result))]
pub fn c20_abort_edit(s: C20State) -> C20State {
    match s {
        C20State::Browsing { data, .. } => C20State::Browsing {
            data,
            edit_state: None,
        },
        other => other,
    }
}

/// Return to Idle from any state.
#[requires(c20_consistent(&s))]
#[ensures(c20_consistent(&result))]
#[ensures(c20_is_idle(&result))]
pub fn c20_reset(s: C20State) -> C20State {
    let _ = s;
    C20State::Idle
}

// ── Lifecycles ────────────────────────────────────────────────────────────────

/// C20g: Idle → Editing(running) → Editing(done with result) → Idle.
///
/// Tests that `running=true → Options None` then `running=false → Some(result)` chains
/// through the consistency invariant without contradiction.
#[requires(query@.len() > 0)]
#[requires(result_str@.len() > 0)]
#[ensures(c20_consistent(&result))]
#[ensures(c20_is_idle(&result))]
pub fn c20_lifecycle_success(query: String, result_str: String) -> C20State {
    let s0 = c20_new();
    let s1 = c20_start_query(s0, query); // running=true, both Options None
    let s2 = c20_complete(s1, result_str); // running=false, result=Some
    c20_reset(s2)
}

/// C20h: Idle → Editing(running) → Editing(failed) → Idle.
///
/// Tests that `running=true → Options None` then `running=false → Some(error)` chains.
#[requires(query@.len() > 0)]
#[requires(error_msg@.len() > 0)]
#[ensures(c20_consistent(&result))]
#[ensures(c20_is_idle(&result))]
pub fn c20_lifecycle_failure(query: String, error_msg: String) -> C20State {
    let s0 = c20_new();
    let s1 = c20_start_query(s0, query); // running=true, both Options None
    let s2 = c20_fail(s1, error_msg); // running=false, error=Some
    c20_reset(s2)
}

/// C20i: Browsing lifecycle — open_browse → begin_edit → abort_edit.
///
/// Tests `Option<Struct>` going None → Some → None in a proof chain.
#[requires(data@.len() > 0)]
#[requires(tag@.len() > 0)]
#[ensures(c20_consistent(&result))]
#[ensures(c20_is_browsing(&result))]
pub fn c20_lifecycle_edit(data: String, tag: String) -> C20State {
    let s0 = c20_new();
    let s1 = c20_open_browse(s0, data); // edit_state = None
    let s2 = c20_begin_edit(s1, tag); // edit_state = Some(C20EditState { tag })
    c20_abort_edit(s2) // edit_state = None
}
