//! Verified State Machine for the archive navigation tree.

use elicit_ui::WcagVerified;
use elicitation::{
    Elicit, Established, KaniCompose, KaniVariantState, Prop, VerifiedStateMachine,
    contracts::ProvableFrom, formal_method,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::archive::nav_model::SchemaWithExpand;
use crate::archive::nav_tree::NavTree;

// ── ArchiveNavState ───────────────────────────────────────────────────────────

/// State of the archive navigation tree panel.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniCompose,
    KaniVariantState,
)]
pub enum ArchiveNavState {
    /// No nav tree has been loaded yet.
    #[default]
    NavUnloaded,

    /// A nav tree fetch is in progress.
    NavLoading,

    /// Nav tree is loaded and ready for browsing.
    NavReady {
        /// All schemas with expand state.
        schemas: Vec<SchemaWithExpand>,
        /// Index of the currently highlighted row in the flat list.
        cursor: usize,
        /// Current filter string (empty means no filter).
        filter: String,
        /// Whether the filter bar is active (accepting keystrokes).
        filter_active: bool,
        /// Whether the help overlay is shown (nav-level).
        show_help: bool,
    },

    /// Nav tree is showing a filtered subset.
    NavFiltered {
        /// All schemas with expand state (unfiltered source).
        schemas: Vec<SchemaWithExpand>,
        /// Active filter string.
        filter: String,
        /// Index of the currently highlighted row in the filtered flat list.
        cursor: usize,
    },
}

// ── ArchiveNavConsistent (invariant) ─────────────────────────────────────────

/// Proposition: the nav tree is in a self-consistent state.
///
/// Wired to [`WcagVerified`] from `elicit_ui`: nav state produces AccessKit
/// nodes via `to_ak_nodes`, so WCAG compliance is the appropriate credential.
#[derive(Prop)]
#[prop(credential = WcagVerified, creusot_invariant_fn = "archive_nav_consistent", kani_invariant_fn = "archive_nav_consistent", verus_invariant_fn = "archive_nav_consistent", verus_inv_body = "match *state { ArchiveNavState::NavFiltered { filter, .. } => filter@.len() > 0, _ => true, }", verus_state_body = "NavFiltered { filter: String }, _Other,", creusot_inv_body = "pearlite! { match state { ArchiveNavState::NavFiltered { filter, .. } => filter@.len() > 0, _ => true, } }")]
pub struct ArchiveNavConsistent;

impl ProvableFrom<WcagVerified> for ArchiveNavConsistent {}

/// Structural invariant predicate for [`ArchiveNavState`].
///
/// Runtime-evaluable form of [`ArchiveNavConsistent`] used by Kani
/// `#[kani::requires]` / `#[kani::ensures]` in contracted wrapper functions.
///
/// Invariant: `NavFiltered` always holds a non-empty filter string.
/// `apply_filter` only creates `NavFiltered` when `!filter.is_empty()`.
pub fn archive_nav_consistent(state: &ArchiveNavState) -> bool {
    match state {
        ArchiveNavState::NavFiltered { filter, .. } => !filter.is_empty(),
        _ => true,
    }
}

/// Bridge `kani::Arbitrary` to `KaniCompose::kani_depth0()` so that
/// `stub_verified` can generate bounded symbolic return values.
#[cfg(kani)]
impl kani::Arbitrary for ArchiveNavState {
    fn any() -> Self {
        use elicitation::KaniCompose;
        ArchiveNavState::kani_depth0()
    }
}

// ── ArchiveNavMachine ─────────────────────────────────────────────────────────

/// Verified state machine for the archive navigation tree.
#[derive(VerifiedStateMachine)]
#[vsm(transitions = [
    load_nav, nav_loaded, nav_refresh,
    expand_schema, collapse_schema,
    move_cursor_up, move_cursor_down,
    apply_filter, clear_filter,
])]
pub struct ArchiveNavMachine;

// ── Transitions ───────────────────────────────────────────────────────────────

/// Start loading the nav tree (e.g. after connect).
#[formal_method(contracts = [ArchiveNavConsistent])]
#[instrument(skip(proof))]
pub fn load_nav(
    _state: ArchiveNavState,
    proof: Established<ArchiveNavConsistent>,
) -> (ArchiveNavState, Established<ArchiveNavConsistent>) {
    (ArchiveNavState::NavLoading, proof)
}

/// Nav tree fetch complete — populate from a `NavTree`.
#[formal_method(contracts = [ArchiveNavConsistent])]
#[instrument(skip(proof, nav))]
pub fn nav_loaded(
    _state: ArchiveNavState,
    proof: Established<ArchiveNavConsistent>,
    nav: NavTree,
) -> (ArchiveNavState, Established<ArchiveNavConsistent>) {
    let schemas = nav
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
    (
        ArchiveNavState::NavReady {
            schemas,
            cursor: 0,
            filter: String::new(),
            filter_active: false,
            show_help: false,
        },
        proof,
    )
}

/// Trigger a nav tree refresh (returns to loading state).
#[formal_method(contracts = [ArchiveNavConsistent])]
#[instrument(skip(proof))]
pub fn nav_refresh(
    _state: ArchiveNavState,
    proof: Established<ArchiveNavConsistent>,
) -> (ArchiveNavState, Established<ArchiveNavConsistent>) {
    (ArchiveNavState::NavLoading, proof)
}

/// Expand or collapse a schema in the nav tree.
#[formal_method(contracts = [ArchiveNavConsistent])]
#[instrument(skip(proof))]
pub fn expand_schema(
    state: ArchiveNavState,
    proof: Established<ArchiveNavConsistent>,
    schema_idx: usize,
    expanded: bool,
) -> (ArchiveNavState, Established<ArchiveNavConsistent>) {
    let next = match state {
        ArchiveNavState::NavReady {
            mut schemas,
            cursor,
            filter,
            filter_active,
            show_help,
        } => {
            if let Some(s) = schemas.get_mut(schema_idx) {
                s.expanded = expanded;
            }
            ArchiveNavState::NavReady {
                schemas,
                cursor,
                filter,
                filter_active,
                show_help,
            }
        }
        other => other,
    };
    (next, proof)
}

/// Collapse a previously-expanded schema.
#[formal_method(contracts = [ArchiveNavConsistent])]
#[instrument(skip(proof))]
pub fn collapse_schema(
    state: ArchiveNavState,
    proof: Established<ArchiveNavConsistent>,
    schema_idx: usize,
) -> (ArchiveNavState, Established<ArchiveNavConsistent>) {
    expand_schema(state, proof, schema_idx, false)
}

/// Move the nav tree cursor up one row.
#[formal_method(contracts = [ArchiveNavConsistent])]
#[instrument(skip(proof))]
pub fn move_cursor_up(
    state: ArchiveNavState,
    proof: Established<ArchiveNavConsistent>,
) -> (ArchiveNavState, Established<ArchiveNavConsistent>) {
    let next = match state {
        ArchiveNavState::NavReady {
            schemas,
            cursor,
            filter,
            filter_active,
            show_help,
        } => ArchiveNavState::NavReady {
            schemas,
            cursor: cursor.saturating_sub(1),
            filter,
            filter_active,
            show_help,
        },
        other => other,
    };
    (next, proof)
}

/// Move the nav tree cursor down one row.
#[formal_method(contracts = [ArchiveNavConsistent])]
#[instrument(skip(proof))]
pub fn move_cursor_down(
    state: ArchiveNavState,
    proof: Established<ArchiveNavConsistent>,
    max: usize,
) -> (ArchiveNavState, Established<ArchiveNavConsistent>) {
    let next = match state {
        ArchiveNavState::NavReady {
            schemas,
            cursor,
            filter,
            filter_active,
            show_help,
        } => ArchiveNavState::NavReady {
            schemas,
            cursor: cursor.saturating_add(1).min(max.saturating_sub(1)),
            filter,
            filter_active,
            show_help,
        },
        other => other,
    };
    (next, proof)
}

#[cfg_attr(not(kani), instrument)]
fn apply_filter_kernel(state: ArchiveNavState, filter: &str) -> ArchiveNavState {
    match state {
        ArchiveNavState::NavReady { schemas, .. }
        | ArchiveNavState::NavFiltered { schemas, .. } => {
            if filter.is_empty() {
                ArchiveNavState::NavReady {
                    schemas,
                    cursor: 0,
                    filter: String::new(),
                    filter_active: true,
                    show_help: false,
                }
            } else {
                ArchiveNavState::NavFiltered {
                    schemas,
                    filter: filter.to_owned(),
                    cursor: 0,
                }
            }
        }
        other => other,
    }
}

/// Start filtering the nav tree.
#[formal_method(contracts = [ArchiveNavConsistent])]
#[instrument(skip(proof))]
pub fn apply_filter(
    state: ArchiveNavState,
    proof: Established<ArchiveNavConsistent>,
    filter: String,
) -> (ArchiveNavState, Established<ArchiveNavConsistent>) {
    let next = apply_filter_kernel(state, &filter);
    (next, proof)
}

/// Clear the active filter and return to unfiltered nav.
#[formal_method(contracts = [ArchiveNavConsistent])]
#[instrument(skip(proof))]
pub fn clear_filter(
    state: ArchiveNavState,
    proof: Established<ArchiveNavConsistent>,
) -> (ArchiveNavState, Established<ArchiveNavConsistent>) {
    let next = apply_filter_kernel(state, "");
    (next, proof)
}
