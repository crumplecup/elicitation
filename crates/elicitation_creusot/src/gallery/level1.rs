//! Gallery level C1: unit type, trivially-true invariant.
//!
//! **Hypothesis**: `cargo creusot` can compile a `#[logic]` predicate and
//! `#[requires(true)] #[ensures(true)]` contracts on a function over a ZST.
//! This validates the basic Creusot workflow before any real predicates.
//!
//! Unlike Kani (where harnesses run the CBMC engine), Creusot *validation*
//! happens in two steps:
//! 1. `cargo creusot -p elicitation_creusot` — generates WhyML.
//!    If this succeeds, the annotations are syntactically and type-correct.
//! 2. `why3 ide verif/...` (or a batch prover) — discharges proof obligations.
//!    For trivially-true contracts, Alt-Ergo closes goals in < 1 s.
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

/// Zero-sized unit type — no heap, no fields.

pub struct GUnit;

/// Trivially-true invariant. Costs nothing to evaluate; never fails.

#[logic]
pub fn c1_invariant(_: &GUnit) -> bool {
    true
}

/// Identity transition — state passes through unchanged.
///
/// Verifies: trivially-true contract compiles and the postcondition holds.

#[requires(c1_invariant(&s))]
#[ensures(c1_invariant(&result))]
pub fn c1_identity(s: GUnit) -> GUnit {
    s
}

/// Constant constructor — always produces a valid `GUnit`.
///
/// Verifies: postcondition on a `#[logic]` predicate over a freshly-created value.

#[requires(true)]
#[ensures(c1_invariant(&result))]
pub fn c1_new() -> GUnit {
    GUnit
}
