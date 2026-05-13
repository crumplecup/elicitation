//! Gallery level C30: out-of-scope String construction — `#[trusted]` pattern.
//!
//! **Problem**: Some VSM transitions construct `String::new()` in their body
//! (e.g. `nav_loaded` sets `filter: String::new()`).  Creusot has no model for
//! `String::new()` in a program-code body; it emits `{false} any` which is an
//! unprovable goal.
//!
//! **Signal**: We cannot (and should not) prove correctness through the String
//! constructor.  Instead, mark the companion `#[trusted]` + `#[ensures]` to
//! declare the contract as an axiom.  The `ProvableFrom` mechanism in the
//! production type system enforces that the only way to obtain the output token
//! is via the canonical transition — the Creusot axiom documents the contract
//! without requiring body analysis.
//!
//! ## Experiments
//!
//! | ID    | What                                                     | Expected   |
//! |-------|----------------------------------------------------------|------------|
//! | C30a  | Companion with `String::new()` in body, NOT trusted      | ✗ `{false}` |
//! | C30b  | Same companion, marked `#[trusted]` + `#[ensures]`       | ✓ proved   |
//! | C30c  | `#[trusted]` helper wrapping the String constructor       | ✓ proved   |
//!
//! C30a is expected to fail — it is included to confirm the diagnosis.
//! C30b is the recommended pattern for out-of-scope constructors.
//! C30c shows an alternative: isolate the construction in its own `#[trusted]`
//! stub, keeping the rest of the body verifiable.
//!
//! ## Relationship to ProvableFrom
//!
//! The `#[trusted]` annotation signals: "this contract is guaranteed by the
//! `ProvableFrom` type system, not by Creusot body analysis."  A reviewer can
//! audit that:
//!
//! 1. The `ProvableFrom<C>` impl requires a credential `C` from an external
//!    verified system (e.g. WCAG accessibility checker).
//! 2. The canonical transition is the only function that can mint the output
//!    `Established<P>` token.
//! 3. Therefore the invariant holds by construction — Creusot just formalises
//!    the contract without re-verifying the implementation.
//!
//! ## Run
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot
//! why3find prove -p creusot verif/elicitation_creusot_rlib/gallery/level30/*.coma
//! ```

use creusot_std::prelude::*;

// ── State ────────────────────────────────────────────────────────────────────

/// A state that carries a String payload.
///
/// Mirrors `ArchiveNavState::NavReady { filter: String, ... }`.
pub enum C30State {
    /// No active session.
    Empty,
    /// Active session with a String filter (mirroring NavReady).
    Active {
        /// A string filter value — may be empty (default state).
        filter: String,
        /// An integer counter.
        count: u64,
    },
}

// ── Invariant ────────────────────────────────────────────────────────────────

/// Trivially-true invariant for this machine.
///
/// All states are well-formed by construction — mirrors `archive_nav_consistent`.
#[logic]
pub fn c30_inv(_s: &C30State) -> bool {
    true
}

// ── C30a: String::new() in body — EXPECTED TO FAIL ───────────────────────────

/// Companion that constructs a String in its body.
///
/// **C30a**: This companion is expected to FAIL because `String::new()` has no
/// Creusot model — it generates `{false} any` in the COMA output, which is
/// an unprovable proof obligation.
///
/// This entry is included to confirm the diagnosis.  Why3find should report
/// this function as unproved.
#[requires(c30_inv(&s))]
#[ensures(c30_inv(&result))]
pub fn c30a_activate_string(s: C30State, count: u64) -> C30State {
    let _ = s;
    // String::new() has no creusot model → {false} any in COMA
    C30State::Active {
        filter: String::new(),
        count,
    }
}

// ── C30b: #[trusted] companion — RECOMMENDED PATTERN ─────────────────────────

/// Companion with String construction, marked `#[trusted]`.
///
/// **C30b**: The contract is declared as an axiom.  Why3find accepts it
/// trivially — the VC is just "trusted contract is satisfied" which requires
/// no body analysis.
///
/// This is the recommended pattern when:
/// - The transition constructs types without a Creusot model (String, etc.)
/// - Correctness is guaranteed by the `ProvableFrom` type system
/// - We want to document the contract without re-proving the body
#[trusted]
#[requires(c30_inv(&s))]
#[ensures(c30_inv(&result))]
pub fn c30b_activate_string(s: C30State, count: u64) -> C30State {
    let _ = s;
    C30State::Active {
        filter: String::new(),
        count,
    }
}

// ── C30c: #[trusted] String helper ───────────────────────────────────────────

/// A `#[trusted]` stub that constructs an empty String.
///
/// **C30c**: By isolating the String construction in a `#[trusted]` function
/// with an explicit `#[ensures]` contract, the rest of the companion body
/// remains verifiable.  This keeps the scope of trust minimal.
#[trusted]
#[ensures(result@.len() == 0)]
pub fn c30_empty_string() -> String {
    String::new()
}

/// Companion using the trusted String helper.
///
/// The body is now fully verifiable — `c30_empty_string()` contributes a
/// concrete ensures that why3find can use.
#[requires(c30_inv(&s))]
#[ensures(c30_inv(&result))]
pub fn c30c_activate_helper(s: C30State, count: u64) -> C30State {
    let _ = s;
    let filter = c30_empty_string();
    C30State::Active { filter, count }
}
