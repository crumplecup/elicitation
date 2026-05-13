//! Gallery level C29: trivially-true `#[logic]` predicate as inline axiom.
//!
//! **Hypothesis**: A `#[logic] fn inv(_s: &S) -> bool { true }` defined in the
//! **same file** as the companions using it gives why3find a transparent body,
//! so `inv(result)` is provable without examining the body of the transition.
//!
//! This is the pattern needed for `archive_nav_consistent` and
//! `archive_connection_consistent`, both of which are currently `= true`.
//!
//! ## Experiments
//!
//! | ID    | What                                              | Expected |
//! |-------|---------------------------------------------------|----------|
//! | C29a  | Same-file non-trusted `#[logic] fn = true`        | ✓ proved |
//! | C29b  | Same-file `#[trusted] #[logic] fn = true`         | ✓ proved |
//! | C29c  | Companion body with no String: postcond provable  | ✓ proved |
//! | C29d  | Cross-module import of the same predicate         | ✗ fails  |
//!
//! C29d deliberately reproduces the production failure to confirm the diagnosis:
//! importing a `#[logic]` fn from another module makes it opaque.
//!
//! ## Key finding
//!
//! Same-file `#[logic]` predicates (trusted or not) have transparent bodies in
//! Why3.  Their bodies can be unfolded to discharge postconditions.  Cross-module
//! imports are opaque: the body is not carried across the COMA boundary.
//!
//! For the production fix: define the invariant predicate in the **same
//! generated file** as the companions that use it.
//!
//! ## Run
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot
//! why3find prove -p creusot verif/elicitation_creusot_rlib/gallery/level29/*.coma
//! ```

use creusot_std::prelude::*;

// ── State ────────────────────────────────────────────────────────────────────

/// A simple two-state machine: Off or On.
pub enum C29State {
    /// Machine is inactive.
    Off,
    /// Machine is active with a positive counter.
    On { counter: u64 },
}

// ── C29a: same-file non-trusted logic fn = true ───────────────────────────

/// Trivially-true invariant — body is `true` regardless of state.
///
/// This is the pattern for `archive_nav_consistent` (all states well-formed).
/// Defined in the same file so its body is transparent to Why3.
#[logic]
pub fn c29a_inv(_s: &C29State) -> bool {
    true
}

/// Activate: Off → On.
#[requires(c29a_inv(&s))]
#[ensures(c29a_inv(&result))]
pub fn c29a_activate(s: C29State, v: u64) -> C29State {
    let _ = s;
    C29State::On { counter: v }
}

/// Deactivate: On → Off.
#[requires(c29a_inv(&s))]
#[ensures(c29a_inv(&result))]
pub fn c29a_deactivate(s: C29State) -> C29State {
    let _ = s;
    C29State::Off
}

// ── C29b: same-file #[trusted] #[logic] fn = true ────────────────────────

/// Trivially-true invariant declared `#[trusted]`.
///
/// `#[trusted]` turns the function into an axiom: why3find never checks the
/// body but can still use `forall x, c29b_inv(x) = true` to discharge
/// postconditions.
#[trusted]
#[logic]
pub fn c29b_inv(_s: &C29State) -> bool {
    true
}

/// Transition using the trusted invariant.
#[requires(c29b_inv(&s))]
#[ensures(c29b_inv(&result))]
pub fn c29b_activate(s: C29State, v: u64) -> C29State {
    let _ = s;
    C29State::On { counter: v }
}

// ── C29c: body with non-String construction, postcond from axiom ──────────

/// Non-trivial invariant: On requires counter > 0.
///
/// This variant tests that the body is used to discharge the postcondition
/// when the invariant is not trivially true.
#[logic]
pub fn c29c_inv(s: &C29State) -> bool {
    pearlite! {
        match s {
            C29State::Off => true,
            C29State::On { counter } => counter@ > 0,
        }
    }
}

/// Activate with non-trivial invariant — requires non-zero counter.
#[requires(v@ > 0)]
#[ensures(c29c_inv(&result))]
pub fn c29c_activate(v: u64) -> C29State {
    C29State::On { counter: v }
}

/// Deactivate is trivially consistent.
#[requires(c29c_inv(&s))]
#[ensures(c29c_inv(&result))]
pub fn c29c_deactivate(s: C29State) -> C29State {
    let _ = s;
    C29State::Off
}
