//! Pearlite logic predicates used as VSM invariants in Creusot companions.
//!
//! Each function is a `#[logic]` predicate over a machine state type.  Generated
//! Creusot companions reference these functions by name in `#[requires]` /
//! `#[ensures]` contracts.
//!
//! ## Invariant strength
//!
//! | Machine | Invariant | Strength |
//! |---------|-----------|----------|
//! | Panel | `SqlEditor.running ==> result.is_none()` | Non-trivial — constrains state shape |
//! | Overlay | `Picker/Browser idx <= collection.len()` | Non-trivial — cursor-in-bounds |
//! | Connection | `true` | Trivial — states are well-formed by construction |
//! | Nav | `true` | Trivial — cursor bounds need per-transition preconditions |
//!
//! ## Proof obligations
//!
//! The generated Creusot companions in `generated/` carry `#[requires]` /
//! `#[ensures]` contracts referencing these predicates.  Running
//! `cargo creusot -p elicit_proofs --features creusot` emits `.coma` files;
//! `why3find prove` closes the goals with Alt-Ergo.

#[cfg(creusot)]
use creusot_std::prelude::*;

#[cfg(creusot)]
use elicit_server::archive::vsm::{
    ArchiveConnectionState, ArchiveNavState, ArchiveOverlayState, ArchivePanelState,
};

/// Invariant predicate for [`ArchivePanelMachine`] transitions.
///
/// The core claim: if the SQL editor is marked as running, no query result can
/// be present yet.  Formally:
///
/// ```text
/// SqlEditor { running, result, .. }  ⊨  running → result = None
/// ```
///
/// This is non-trivial: it is violated by a state where `running = true` and
/// `result = Some(_)` simultaneously.  Every panel transition that constructs
/// or modifies `SqlEditor` preserves the invariant:
///
/// - `open_sql_editor` always produces `running = false, result = None` → trivially true.
/// - `query_complete` always produces `running = false, result = Some(_)` → trivially true.
/// - All other transitions leave `SqlEditor` unchanged (via `other => other`),
///   so the precondition suffices.
#[cfg(creusot)]
#[logic]
pub fn archive_panel_consistent(state: &ArchivePanelState) -> bool {
    pearlite! {
        match state {
            ArchivePanelState::SqlEditor { running, result, .. } =>
                *running ==> match result { None => true, Some(_) => false },
            _ => true,
        }
    }
}

/// Invariant predicate for [`ArchiveConnectionMachine`] transitions.
///
/// All connection states are well-formed by construction.  The interesting
/// invariants (e.g. `ConnectionError.message` is non-empty) require
/// non-empty-string preconditions on the generating transitions; those are
/// deferred until the generated companions are extended with explicit
/// `#[requires(message@.len() > 0)]` clauses.
#[cfg(creusot)]
#[logic]
pub fn archive_connection_consistent(_state: &ArchiveConnectionState) -> bool {
    true
}

/// Invariant predicate for [`ArchiveNavMachine`] transitions.
///
/// All nav tree states are well-formed by construction.  Cursor-in-bounds
/// invariants require per-transition `#[requires(cursor@ <= schemas@.len())]`
/// preconditions in the generated companions; those are deferred.
#[cfg(creusot)]
#[logic]
pub fn archive_nav_consistent(_state: &ArchiveNavState) -> bool {
    true
}

/// Invariant predicate for [`ArchiveOverlayMachine`] transitions.
///
/// The cursor/index in picker and browser overlays is always within the
/// bounds of the available items:
///
/// - `ExportPickerOpen { idx, formats }` ⊨ `idx ≤ formats.len()`
/// - `SavedBrowserOpen { entries, idx }` ⊨ `idx ≤ entries.len()`
///
/// These follow from:
///
/// - `open_export_picker` / `open_saved_browser` initialise `idx = 0 ≤ len`.
/// - `*_move_up` uses `saturating_sub(1)`, so `idx` can only decrease → still bounded.
/// - `*_move_down` uses `saturating_add(1).min(len.saturating_sub(1))`:
///   - if `len = 0`: result is `0 ≤ 0` ✓
///   - if `len > 0`: result is `≤ len − 1 < len` ✓
/// - All other transitions leave these variants via `other => other`, so the
///   precondition carries through.
#[cfg(creusot)]
#[logic]
pub fn archive_overlay_consistent(state: &ArchiveOverlayState) -> bool {
    pearlite! {
        match state {
            ArchiveOverlayState::ExportPickerOpen { idx, formats } =>
                idx@ <= formats@.len(),
            ArchiveOverlayState::SavedBrowserOpen { entries, idx } =>
                idx@ <= entries@.len(),
            _ => true,
        }
    }
}
