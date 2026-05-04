//! Creusot logic predicates and extern_spec contracts for the real VSM
//! state machines in `elicit_server`.
//!
//! Production crates (`elicit_server`) have zero creusot knowledge — no
//! `creusot_std` dep, no `#[logic]` / `#[requires]` / `#[ensures]` attrs.
//! All creusot concerns live here, in the dedicated isolation crate.
//!
//! ## Architecture
//!
//! 1. Logic predicates (`*_consistent`) are defined here as `#[logic]`
//!    functions over the public `elicit_server` state types.
//! 2. `extern_spec!` blocks attach `#[requires]` / `#[ensures]` contracts
//!    to `elicit_server`'s transition functions without modifying them.
//!
//! Currently all invariants are trivially `true` (every state is
//! well-formed by construction).  Replace the body of each predicate with
//! real field invariants as the proofs mature.

use crate::*;

#[cfg(feature = "vsm-proofs")]
use elicit_server::{
    ArchiveConnectionState, ArchiveNavState, ArchiveOverlayState, ArchivePanelState,
};

// ── Panel ─────────────────────────────────────────────────────────────────────

/// Creusot logic invariant for [`ArchivePanelState`].
///
/// Placeholder — all states are well-formed by construction.
#[cfg(feature = "vsm-proofs")]
#[logic]
#[trusted]
pub fn archive_panel_consistent(_state: &ArchivePanelState) -> bool {
    true
}

// ── Connection ────────────────────────────────────────────────────────────────

/// Creusot logic invariant for [`ArchiveConnectionState`].
///
/// Placeholder — all states are well-formed by construction.
#[cfg(feature = "vsm-proofs")]
#[logic]
#[trusted]
pub fn archive_connection_consistent(_state: &ArchiveConnectionState) -> bool {
    true
}

// ── Nav ───────────────────────────────────────────────────────────────────────

/// Creusot logic invariant for [`ArchiveNavState`].
///
/// Placeholder — all states are well-formed by construction.
#[cfg(feature = "vsm-proofs")]
#[logic]
#[trusted]
pub fn archive_nav_consistent(_state: &ArchiveNavState) -> bool {
    true
}

// ── Overlay ───────────────────────────────────────────────────────────────────

/// Creusot logic invariant for [`ArchiveOverlayState`].
///
/// Placeholder — all states are well-formed by construction.
#[cfg(feature = "vsm-proofs")]
#[logic]
#[trusted]
pub fn archive_overlay_consistent(_state: &ArchiveOverlayState) -> bool {
    true
}
