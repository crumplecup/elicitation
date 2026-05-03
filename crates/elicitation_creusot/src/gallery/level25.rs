//! Gallery level C25: VSM transition harness + `why3find prove`.
//!
//! **Hypothesis**: A `__creusot`-style harness that *delegates* to a
//! transition function is provable by `why3find prove`, provided the
//! transition itself carries a matching `#[ensures]` contract visible to
//! Creusot.
//!
//! This level replicates the structure of the generated files in
//! `elicit_proofs/src/creusot/generated/` using only gallery-local code,
//! so we can iterate safely before touching production.
//!
//! ## The delegation pattern
//!
//! In production, `elicit_proofs` generates harnesses like:
//!
//! ```rust,ignore
//! #[cfg(creusot)]
//! #[requires(archive_overlay_consistent(&_state))]
//! #[ensures(archive_overlay_consistent(&result.0))]
//! pub(crate) fn close_overlay__creusot(
//!     _state: ArchiveOverlayState,
//!     proof: Established<ArchiveOverlayConsistent>,
//! ) -> (ArchiveOverlayState, Established<ArchiveOverlayConsistent>) {
//!     close_overlay(_state, proof)   // ← delegates to elicit_server
//! }
//! ```
//!
//! The VC for this harness says: given `archive_overlay_consistent(&_state)`,
//! prove `archive_overlay_consistent(&result.0)` after calling
//! `close_overlay`.  Creusot can only discharge this if it can see a matching
//! `#[ensures(archive_overlay_consistent(&result.0))]` on `close_overlay`.
//!
//! If `close_overlay` is **opaque** (no spec visible), Creusot generates a
//! `{false} any` body for the harness — the VC is unprovable because the
//! postcondition of the callee is unknown.
//!
//! ## Experiments
//!
//! | ID    | What                                                     | Expected |
//! |-------|----------------------------------------------------------|----------|
//! | C25a  | Harness delegates to function **with** matching ensures  | ✓ proved |
//! | C25b  | Harness delegates to `#[trusted]` stub                   | notes    |
//! | C25c  | Harness inlines the body (no delegation)                 | ✓ proved |
//!
//! **C25b** documents the failure mode: `#[trusted]` makes the callee
//! opaque — the harness VC becomes unprovable.  This is the anti-pattern
//! seen in the production generated files when `elicit_server/creusot` is not
//! active.  **Do not use `#[trusted]` to paper over this.**
//!
//! **C25c** shows the fallback: inline the body in the harness.  Always
//! provable, but generates harnesses that duplicate logic.
//!
//! ## Key finding
//!
//! The production path must ensure each transition function carries a
//! `#[ensures(invariant(&result.0))]` contract that is **visible** when
//! `cargo creusot -p elicit_proofs` compiles `elicit_server`.  Two ways:
//!
//! 1. Enable `elicit_server/creusot` feature (requires `creusot-std` in
//!    `elicit_server`'s dep tree — currently avoided due to sysroot
//!    compilation issues).
//! 2. Inline the body in each generated `__creusot` harness (simpler,
//!    no cross-crate spec dependency).
//!
//! ## Results
//!
//! All patterns proved by `why3find prove`:
//!
//! | File                        | VCs | Result |
//! |-----------------------------|-----|--------|
//! | `c25_deactivate`            | 1   | ✔      |
//! | `c25_deactivate_harness`    | 2   | ✔      |
//! | `c25_deactivate_inlined`    | 1   | ✔      |
//! | `c25_activate`              | 1   | ✔      |
//! | `c25_activate_harness`      | 2   | ✔      |
//! | `c25_activate_inlined`      | 1   | ✔      |
//! | `c25_trusted_harness`       | 2   | ✔      |
//!
//! The harness VCs have 2 goals: (1) prove the callee's requires is satisfied,
//! (2) prove the harness ensures holds given the callee's ensures as axiom.
//! Inlined bodies have 1 goal: prove the ensures directly from the body.
//!
//! ## Diagnosis: `{false} any` in production coma files
//!
//! When `cargo creusot -p elicit_proofs` runs, the generated harnesses call
//! `close_overlay(_state, proof)` from `elicit_server`.  But `elicit_server`
//! is compiled **without** `feature = "creusot"` (to avoid the `creusot-std`
//! sysroot-only crate being pulled in as a regular dep).  Without the feature,
//! `formal_method`'s `cfg_attr(all(creusot, feature = "creusot"), ...)` guards
//! are inactive — the transition functions carry no Creusot spec.
//!
//! Creusot represents an opaque (spec-less) function call as `{false} any` in
//! the generated Coma — the block's precondition is `false`, meaning the
//! prover must prove `false` to enter it.  The VC is therefore unprovable.
//!
//! ## Production fix options
//!
//! 1. **Run `cargo creusot -p elicit_server --features creusot` directly.**
//!    The `formal_method` ensures activate; transitions are verified in their
//!    own crate.  The `elicit_proofs` harnesses serve as a separate cross-check
//!    and would need inlined bodies (option 2) to be independently provable.
//!
//! 2. **Generate harnesses with inlined bodies** (option C25c).
//!    Change `formal_method`'s `creusot_contract()` to copy the function body
//!    into the harness rather than delegating.  Always provable; no cross-crate
//!    spec dependency.  Cleanest for `elicit_proofs`.
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! why3find prove -p creusot \
//!   verif/elicitation_creusot_rlib/gallery/level25/*.coma
//! ```

use creusot_std::prelude::*;

// ── State ────────────────────────────────────────────────────────────────────

/// Two-state machine: Idle or Active(value).
pub enum C25State {
    /// No active session.
    Idle,
    /// Active session with a positive value.
    Active {
        /// Must be positive.
        value: i64,
    },
}

// ── Invariant ────────────────────────────────────────────────────────────────

/// Consistency predicate: Active variant requires value > 0.
#[logic]
pub fn c25_consistent(s: &C25State) -> bool {
    pearlite! {
        match s {
            C25State::Idle => true,
            C25State::Active { value } => value@ > 0,
        }
    }
}

// ── C25a: delegation with matching ensures ────────────────────────────────────

/// The real transition function — carries a full `#[ensures]` contract.
///
/// This is the production-equivalent: a `#[formal_method]`-annotated
/// function that *does* have its Creusot spec active.
#[requires(c25_consistent(&state))]
#[ensures(c25_consistent(&result))]
pub fn c25_deactivate(state: C25State) -> C25State {
    let _ = state;
    C25State::Idle
}

/// Harness that delegates to `c25_deactivate`.
///
/// **C25a**: Creusot sees `c25_deactivate`'s `#[ensures]` and uses it to
/// discharge the harness's own ensures.  This should produce a provable VC.
#[requires(c25_consistent(&state))]
#[ensures(c25_consistent(&result))]
pub fn c25_deactivate__harness(state: C25State) -> C25State {
    c25_deactivate(state)
}

/// Activate transition — requires positive value.
#[requires(value@ > 0)]
#[ensures(c25_consistent(&result))]
pub fn c25_activate(value: i64) -> C25State {
    C25State::Active { value }
}

/// Activate harness — delegates with matching requires/ensures.
///
/// **C25a (activate variant)**: same delegation pattern, non-trivial
/// precondition (`value > 0`) forwarded to the callee.
#[requires(value@ > 0)]
#[ensures(c25_consistent(&result))]
pub fn c25_activate__harness(value: i64) -> C25State {
    c25_activate(value)
}

// ── C25b: delegation to trusted stub ─────────────────────────────────────────

/// A stub that mimics an opaque transition — `#[trusted]` makes the body
/// invisible to Creusot, so only the contract (if any) is used.
///
/// When called from a harness with `#[ensures(c25_consistent)]`, Creusot must
/// rely on this function's own ensures.  **Without** a matching ensures here,
/// the VC is `false` — unprovable, regardless of the harness contract.
///
/// This documents the failure mode: marking a callee `#[trusted]` without a
/// spec does not help the caller.
#[trusted]
#[ensures(c25_consistent(&result))]
pub fn c25_deactivate_stub(_state: C25State) -> C25State {
    C25State::Idle
}

/// Harness over the trusted stub.
///
/// **C25b**: because `c25_deactivate_stub` carries `#[ensures(c25_consistent)]`
/// even as a `#[trusted]` function, Creusot treats the ensures as an axiom and
/// can discharge the harness VC.
///
/// **Key lesson**: `#[trusted]` is *acceptable* when the callee has an
/// explicit `#[ensures]` contract.  The VC is trivially proved because the
/// callee's ensures is taken as given.  What is **not** acceptable is
/// `#[trusted]` with *no* ensures — that leaves the VC unprovable.
#[requires(c25_consistent(&state))]
#[ensures(c25_consistent(&result))]
pub fn c25_trusted_harness(state: C25State) -> C25State {
    c25_deactivate_stub(state)
}

// ── C25c: inlined body ────────────────────────────────────────────────────────

/// Harness with the body inlined — no delegation at all.
///
/// **C25c**: Creusot sees the full body and can prove the ensures
/// directly.  This is the fallback pattern when the transition function
/// cannot carry a Creusot spec (e.g., because `elicit_server/creusot` is
/// not active during `cargo creusot -p elicit_proofs`).
#[requires(c25_consistent(&state))]
#[ensures(c25_consistent(&result))]
pub fn c25_deactivate__inlined(state: C25State) -> C25State {
    let _ = state;
    C25State::Idle
}

/// Inlined activate harness — non-trivial: must build Active { value }.
#[requires(value@ > 0)]
#[ensures(c25_consistent(&result))]
pub fn c25_activate__inlined(value: i64) -> C25State {
    C25State::Active { value }
}
