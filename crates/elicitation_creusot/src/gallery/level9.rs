//! Gallery level C9: contract chains replace Kani's `Established<P>` tokens.
//!
//! **Hypothesis**: In deductive verification (Creusot/Why3), function
//! postconditions are **axioms** at every call site — the proof obligation
//! discharged when `f()` was verified is reused for free when `f()` is called.
//! There is no need for a runtime proof token (`Established<P>`) to carry the
//! witness between functions.
//!
//! This is a **fundamental architectural difference** from Kani (CBMC):
//!
//! | Tool      | Composition mechanism                       | Cost                    |
//! |-----------|---------------------------------------------|-------------------------|
//! | Kani      | `stub_verified!(f)` exempts callee body      | Must opt-in per callee  |
//! | Creusot   | Postcondition is axiom at every call site    | Free, automatic         |
//!
//! In Kani we model `Established<P>` as a ZST proof token whose construction
//! requires `P` to hold; passing the token to a downstream function avoids
//! re-checking the construction precondition.  In Creusot this pattern is
//! unnecessary: if `produce()` ensures `c9_holds(&result)` and `consume()`
//! requires `c9_holds(&s)`, calling `consume(produce())` is automatically
//! provable because Why3 **knows** `produce()` returned a consistent value.
//!
//! ## Experiment table
//!
//! | ID   | What                                               | Expected |
//! |------|----------------------------------------------------|----------|
//! | C9a  | Produce a value that satisfies a predicate          | ✓        |
//! | C9b  | Consume requires the predicate — satisfied by C9a   | ✓        |
//! | C9c  | Pipeline: produce → transform → consume, all free   | ✓        |
//! | C9d  | Three independent invariants thread through a chain | ✓        |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

// ── Data types ────────────────────────────────────────────────────────────────

/// A minimal state value — a non-negative counter.
pub struct C9State {
    value: i64,
}

/// The invariant: value is non-negative.

#[logic]
pub fn c9_holds(s: &C9State) -> bool {
    pearlite! { s.value@ >= 0 }
}

/// Guard: value is below `i64::MAX` — prevents overflow in increment.

#[logic]
pub fn c9_below_max(s: &C9State) -> bool {
    pearlite! { s.value@ < 9223372036854775807i64@ }
}

// ── C9a: producer ─────────────────────────────────────────────────────────────

/// C9a: produce a consistent state.
///
/// Why3 records `c9_holds(&result)` as an axiom for any caller.

#[requires(true)]
#[ensures(c9_holds(&result))]
#[ensures(c9_below_max(&result))]
pub fn c9_produce() -> C9State {
    C9State { value: 0 }
}

// ── C9b: consumer ─────────────────────────────────────────────────────────────

/// C9b: consume requires `c9_holds` — satisfied by anything produced by `c9_produce`.
///
/// No proof token is passed; the precondition is discharged by Why3's knowledge
/// that the argument came from a function whose postcondition is `c9_holds`.

#[requires(c9_holds(&s))]
#[ensures(result@ >= 0)]
pub fn c9_consume(s: C9State) -> i64 {
    s.value
}

// ── C9c: pipeline ─────────────────────────────────────────────────────────────

/// Increment, preserving the invariant.

#[requires(c9_holds(&s))]
#[requires(c9_below_max(&s))]
#[ensures(c9_holds(&result))]
pub fn c9_increment(s: C9State) -> C9State {
    C9State { value: s.value + 1 }
}

/// C9c: produce → increment → consume as a pipeline.
///
/// Each intermediate postcondition flows freely as a precondition for the
/// next step.  No `stub_verified` or proof token needed.

#[requires(true)]
#[ensures(result@ >= 0)]
pub fn c9_pipeline() -> i64 {
    let s0 = c9_produce();
    let s1 = c9_increment(s0);
    c9_consume(s1)
}

// ── C9d: multi-invariant chain ────────────────────────────────────────────────

/// A state that must satisfy three simultaneous predicates.
pub struct C9Triple {
    pub a: i64,
    pub b: i64,
    pub c: i64,
}

/// All three fields non-negative — three independent invariants.

#[logic]
pub fn c9_triple_holds(t: &C9Triple) -> bool {
    pearlite! { t.a@ >= 0 && t.b@ >= 0 && t.c@ >= 0 }
}

/// Guard: `b` is below `i64::MAX` — prevents overflow in `c9_triple_increment_b`.

#[logic]
pub fn c9_triple_b_below_max(t: &C9Triple) -> bool {
    pearlite! { t.b@ < 9223372036854775807i64@ }
}

/// Produce a consistent triple.

#[requires(true)]
#[ensures(c9_triple_holds(&result))]
#[ensures(c9_triple_b_below_max(&result))]
pub fn c9_triple_produce() -> C9Triple {
    C9Triple { a: 1, b: 2, c: 3 }
}

/// Transform: increment `b`, leave others unchanged.

#[requires(c9_triple_holds(&t))]
#[requires(c9_triple_b_below_max(&t))]
#[ensures(c9_triple_holds(&result))]
pub fn c9_triple_increment_b(t: C9Triple) -> C9Triple {
    C9Triple { b: t.b + 1, ..t }
}

/// C9d: three-invariant pipeline — all three simultaneously maintained.

#[requires(true)]
#[ensures(c9_triple_holds(&result))]
pub fn c9_triple_pipeline() -> C9Triple {
    let t = c9_triple_produce();
    c9_triple_increment_b(t)
}
