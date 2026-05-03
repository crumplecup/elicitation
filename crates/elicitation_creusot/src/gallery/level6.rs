//! Gallery level C6: composition — postcondition flows free as precondition.
//!
//! **Hypothesis**: In Creusot/Why3, a function's postcondition is automatically
//! available as an axiom at any call site.  There is no `stub_verified`
//! machinery needed.  Composition is *free* in the deductive model.
//!
//! This is the critical difference from Kani:
//! - Kani (CBMC): DFCC requires explicit `stub_verified` to avoid re-checking
//!   the callee body.  Without it, the callee is inlined and the solver
//!   re-verifies the body from scratch (8× slowdown, potential timeout).
//! - Creusot (Why3): postconditions become axioms automatically.  Calling
//!   `connect()` in a harness means Why3 *assumes* its postcondition without
//!   re-checking its proof.
//!
//! ## Experiment table
//!
//! | ID   | What                                                | Expected |
//! |------|-----------------------------------------------------|----------|
//! | C6a  | `connect()` postcond feeds `transition()` precond  | ✓        |
//! | C6b  | Three-step chain: A → B → C                        | ✓        |
//! | C6c  | Callee that returns `Established<P>` equivalent     | ✓        |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```


use creusot_std::prelude::*;

// ── Re-use level 5 types ──────────────────────────────────────────────────────


pub enum C6State {
    Idle,
    Ready,
    Done,
}


#[logic]
pub fn c6_invariant(s: &C6State) -> bool {
    match s {
        C6State::Idle => true,
        C6State::Ready => true,
        C6State::Done => true,
    }
}


#[logic]
pub fn c6_is_ready(s: &C6State) -> bool {
    match s {
        C6State::Ready => true,
        _ => false,
    }
}


#[logic]
pub fn c6_is_done(s: &C6State) -> bool {
    match s {
        C6State::Done => true,
        _ => false,
    }
}

// ── Step functions ────────────────────────────────────────────────────────────

/// Step 1: Idle → Ready.

#[requires(true)]
#[ensures(c6_is_ready(&result))]
pub fn c6_start(_s: C6State) -> C6State {
    C6State::Ready
}

/// Step 2: Ready → Done.
///
/// Requires `c6_is_ready` — satisfied automatically by `c6_start`'s postcondition.

#[requires(c6_is_ready(&s))]
#[ensures(c6_is_done(&result))]
pub fn c6_finish(s: C6State) -> C6State {
    let _ = s;
    C6State::Done
}

/// C6a: two-step composition.
///
/// Verifies: `c6_finish(c6_start(s))` — Why3 uses `c6_start`'s postcondition
/// as an axiom to discharge `c6_finish`'s precondition without re-checking
/// `c6_start`'s body.

#[requires(true)]
#[ensures(c6_is_done(&result))]
pub fn c6_start_then_finish(s: C6State) -> C6State {
    let ready = c6_start(s);
    c6_finish(ready)
}

// ── Three-step chain (C6b) ────────────────────────────────────────────────────


pub enum C6Chain {
    A,
    B,
    C,
    D,
}


#[logic]
pub fn c6_chain_is_b(s: &C6Chain) -> bool {
    match s {
        C6Chain::B => true,
        _ => false,
    }
}


#[logic]
pub fn c6_chain_is_c(s: &C6Chain) -> bool {
    match s {
        C6Chain::C => true,
        _ => false,
    }
}


#[logic]
pub fn c6_chain_is_d(s: &C6Chain) -> bool {
    match s {
        C6Chain::D => true,
        _ => false,
    }
}


#[requires(true)]
#[ensures(c6_chain_is_b(&result))]
pub fn c6_step_a_to_b(_: C6Chain) -> C6Chain {
    C6Chain::B
}


#[requires(c6_chain_is_b(&s))]
#[ensures(c6_chain_is_c(&result))]
pub fn c6_step_b_to_c(s: C6Chain) -> C6Chain {
    let _ = s;
    C6Chain::C
}


#[requires(c6_chain_is_c(&s))]
#[ensures(c6_chain_is_d(&result))]
pub fn c6_step_c_to_d(s: C6Chain) -> C6Chain {
    let _ = s;
    C6Chain::D
}

/// C6b: three-step chain A → B → C → D.
///
/// Verifies: each postcondition propagates automatically through the chain.

#[requires(true)]
#[ensures(c6_chain_is_d(&result))]
pub fn c6_three_step_chain(s: C6Chain) -> C6Chain {
    let b = c6_step_a_to_b(s);
    let c = c6_step_b_to_c(b);
    c6_step_c_to_d(c)
}
