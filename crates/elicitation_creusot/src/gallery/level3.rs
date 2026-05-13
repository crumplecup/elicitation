//! Gallery level C3: unit enum, `match` in `#[logic]`.
//!
//! **Hypothesis**: Pearlite can `match` over a unit (no-data) enum inside a
//! `#[logic]` function, and contracts can require/ensure a specific variant.
//! This validates the foundation for VSM state predicates.
//!
//! Enums with unit variants are the simplest case — no heap allocation, no
//! nested types.  A data-carrying enum is tested in C5.
//!
//! ## Experiment table
//!
//! | ID   | Predicate                           | Expected |
//! |------|-------------------------------------|----------|
//! | C3a  | `match` in `#[logic]` over unit enum | ✓        |
//! | C3b  | Transition Active → Inactive        | ✓        |
//! | C3c  | Transition preserves Active→        | ✓        |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

/// Simple two-state machine.
pub enum GState {
    Active,
    Inactive,
}

/// C3a: predicate — is the machine in the Active state?

#[logic]
pub fn c3_is_active(s: &GState) -> bool {
    match s {
        GState::Active => true,
        GState::Inactive => false,
    }
}

/// C3b: deactivate — requires Active, ensures Inactive.
///
/// Verifies: variant-specific pre/postconditions on a unit enum.

#[requires(c3_is_active(&s))]
#[ensures(!c3_is_active(&result))]
pub fn c3_deactivate(s: GState) -> GState {
    let _ = s;
    GState::Inactive
}

/// C3c: activate — requires Inactive, ensures Active.

#[requires(!c3_is_active(&s))]
#[ensures(c3_is_active(&result))]
pub fn c3_activate(s: GState) -> GState {
    let _ = s;
    GState::Active
}

/// Trivial invariant over a unit enum — always true.
///
/// Verifies: composition of `c3_is_active` with the identity property
/// (either variant satisfies a `|| true` predicate).

#[logic]
pub fn c3_invariant(s: &GState) -> bool {
    match s {
        GState::Active => true,
        GState::Inactive => true,
    }
}

/// Identity preserves the trivial invariant.

#[requires(c3_invariant(&s))]
#[ensures(c3_invariant(&result))]
pub fn c3_identity(s: GState) -> GState {
    s
}
