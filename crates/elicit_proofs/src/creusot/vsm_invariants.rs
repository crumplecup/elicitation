//! Pearlite logic predicates used as VSM invariants in Creusot companions.
//!
//! Each function is a `#[logic]` predicate over a machine state type. Generated
//! Creusot companions reference these functions by name in `#[requires]` /
//! `#[ensures]` contracts.
//!
//! All predicates currently return `true` — a trivially-satisfied invariant.
//! Creusot still verifies function totality, absence of panics, and type safety
//! against any `true` contract, which is strictly stronger than `#[trusted]`.
//!
//! Strengthen these predicates incrementally as the proof coverage grows.

#[cfg(all(feature = "creusot", creusot))]
use creusot_std::prelude::*;

#[cfg(all(feature = "creusot", creusot))]
use elicit_server::archive::vsm::{
    ArchiveConnectionState, ArchiveNavState, ArchiveOverlayState, ArchivePanelState,
};

/// Invariant predicate for [`ArchivePanelMachine`] transitions.
///
/// Currently trivially true. Strengthen to express WCAG accessibility invariants
/// once pearlite models for `String::is_empty()` are in place.
#[cfg(all(feature = "creusot", creusot))]
#[logic]
pub fn archive_panel_consistent(_state: &ArchivePanelState) -> bool {
    true
}

/// Invariant predicate for [`ArchiveConnectionMachine`] transitions.
///
/// Currently trivially true. Strengthen to express connection lifecycle invariants
/// (e.g., ConnectionError variant always has a non-empty message).
#[cfg(all(feature = "creusot", creusot))]
#[logic]
pub fn archive_connection_consistent(_state: &ArchiveConnectionState) -> bool {
    true
}

/// Invariant predicate for [`ArchiveNavMachine`] transitions.
///
/// Currently trivially true. Strengthen to express nav tree structural invariants.
#[cfg(all(feature = "creusot", creusot))]
#[logic]
pub fn archive_nav_consistent(_state: &ArchiveNavState) -> bool {
    true
}

/// Invariant predicate for [`ArchiveOverlayMachine`] transitions.
///
/// Currently trivially true. Strengthen to express overlay exclusivity invariants.
#[cfg(all(feature = "creusot", creusot))]
#[logic]
pub fn archive_overlay_consistent(_state: &ArchiveOverlayState) -> bool {
    true
}
