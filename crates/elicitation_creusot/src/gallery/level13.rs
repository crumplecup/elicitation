//! Gallery level C13: machine wrapper struct with transition counter.
//!
//! **Hypothesis**: A machine wrapper struct can combine the full 4-variant
//! connection state (String-carrying, from C10) with a transition counter
//! (from C8).  Exact arithmetic postconditions
//! (`result.transition_count@ = m.transition_count@ + 1`) let the prover
//! chain `below_max` guards across a multi-step lifecycle with no manual
//! lemmas.
//!
//! This is the closest gallery analogue to the real `ArchiveConnectionMachine`:
//! a wrapper struct holding state + metadata, each transition preserving the
//! composite invariant and recording progress via the counter.
//!
//! ## Machine struct
//!
//! ```text
//! C13Machine {
//!     state: MiniConnState,   // 4-variant (Disconnected/Connecting/Connected/Error)
//!     transition_count: i64,  // monotonically non-decreasing
//! }
//! ```
//!
//! ## Invariant
//!
//! ```text
//! c13_consistent(m) ≡ c13_state_ok(&m.state) ∧ m.transition_count@ ≥ 0
//! ```
//!
//! ## State consistency inlined
//!
//! `c13_state_ok` is the same body as `c10_consistent` from level10.
//! Per the C12 lesson: cross-module `#[logic]` calls appear opaque in Why3;
//! only the ADT definition (enum arms + field names) crosses module boundaries.
//! We import `MiniConnState` for the type but inline the consistency body.
//!
//! ## Key new patterns
//!
//! - **Exact counter postcondition**: `result.transition_count@ = m.transition_count@ + 1`
//! - **Counter chaining**: each transition's exact count value is known by Why3,
//!   so the prover can discharge `below_max` for each subsequent step without a
//!   manual lemma.
//! - **State postconditions on wrapper**: e.g. `c13_is_connecting(&result)` after
//!   `c13_begin` feeds into `c13_succeed`'s guard.
//!
//! ## Experiment table
//!
//! | ID    | What                                                | Expected |
//! |-------|-----------------------------------------------------|----------|
//! | C13a  | State consistency inlined in wrapper invariant      | ✓        |
//! | C13b  | Constructor: count = 0, Disconnected                | ✓        |
//! | C13c  | Exact counter postcondition (`= old + 1`)           | ✓        |
//! | C13d  | `begin`: Disconnected → Connecting, count + 1       | ✓        |
//! | C13e  | `succeed`: Connecting → Connected, count + 1        | ✓        |
//! | C13f  | `fail`: Connecting → Error, count + 1               | ✓        |
//! | C13g  | `disconnect`: any → Disconnected, count + 1         | ✓        |
//! | C13h  | Full lifecycle: counter chains end-to-end            | ✓        |

use creusot_std::prelude::*;

use crate::gallery::level10::MiniConnState;

// ── State consistency (inlined from c10_consistent) ───────────────────────────

/// C13a: state invariant — String-carrying variants need non-empty payloads.
///
/// Body is the same as `c10_consistent` in level10.  Inlined here because
/// cross-module `#[logic]` calls appear opaque in Why3 (C12 lesson).
#[logic]
pub fn c13_state_ok(s: &MiniConnState) -> bool {
    pearlite! {
        match s {
            MiniConnState::Disconnected => true,
            MiniConnState::Connecting { name } => name@.len() > 0,
            MiniConnState::Connected { name }  => name@.len() > 0,
            MiniConnState::Error { message }   => message@.len() > 0,
        }
    }
}

// ── Machine wrapper ───────────────────────────────────────────────────────────

/// Machine wrapper: connection state + monotone transition counter.
///
/// Mirrors the shape of `ArchiveConnectionMachine { state, transition_count, .. }`.
pub struct C13Machine {
    /// Current connection state.
    pub state: MiniConnState,
    /// Number of transitions performed (non-negative).
    pub transition_count: i64,
}

/// C13 composite invariant.
#[logic]
pub fn c13_consistent(m: &C13Machine) -> bool {
    pearlite! {
        c13_state_ok(&m.state) && m.transition_count@ >= 0
    }
}

/// Guard: transition_count is below `i64::MAX` — prevents overflow.
#[logic]
pub fn c13_below_max(m: &C13Machine) -> bool {
    pearlite! { m.transition_count@ < i64::MAX@ }
}

// ── State predicates ──────────────────────────────────────────────────────────

/// True when the machine is in `Disconnected` state.
#[logic]
pub fn c13_is_disconnected(m: &C13Machine) -> bool {
    pearlite! {
        match &m.state {
            MiniConnState::Disconnected => true,
            _ => false,
        }
    }
}

/// True when the machine is in `Connecting` state.
#[logic]
pub fn c13_is_connecting(m: &C13Machine) -> bool {
    pearlite! {
        match &m.state {
            MiniConnState::Connecting { .. } => true,
            _ => false,
        }
    }
}

/// True when the machine is in `Connected` state.
#[logic]
pub fn c13_is_connected(m: &C13Machine) -> bool {
    pearlite! {
        match &m.state {
            MiniConnState::Connected { .. } => true,
            _ => false,
        }
    }
}

// ── C13b: constructor ─────────────────────────────────────────────────────────

/// C13b: fresh machine — Disconnected, count = 0.
#[ensures(c13_consistent(&result))]
#[ensures(c13_is_disconnected(&result))]
#[ensures(result.transition_count@ == 0)]
pub fn c13_new() -> C13Machine {
    C13Machine {
        state: MiniConnState::Disconnected,
        transition_count: 0,
    }
}

// ── C13d: begin ───────────────────────────────────────────────────────────────

/// C13d: Disconnected → Connecting, count + 1.
///
/// C13c: exact counter postcondition tests whether Why3 can track
/// `result.transition_count@ = m.transition_count@ + 1`.
#[requires(c13_consistent(&m))]
#[requires(c13_below_max(&m))]
#[requires(name@.len() > 0)]
#[ensures(c13_consistent(&result))]
#[ensures(c13_is_connecting(&result))]
#[ensures(result.transition_count@ == m.transition_count@ + 1)]
pub fn c13_begin(m: C13Machine, name: String) -> C13Machine {
    C13Machine {
        state: MiniConnState::Connecting { name },
        transition_count: m.transition_count + 1,
    }
}

// ── C13e: succeed ─────────────────────────────────────────────────────────────

/// C13e: Connecting → Connected, count + 1.
#[requires(c13_consistent(&m))]
#[requires(c13_is_connecting(&m))]
#[requires(c13_below_max(&m))]
#[ensures(c13_consistent(&result))]
#[ensures(c13_is_connected(&result))]
#[ensures(result.transition_count@ == m.transition_count@ + 1)]
pub fn c13_succeed(m: C13Machine) -> C13Machine {
    match m.state {
        MiniConnState::Connecting { name } => C13Machine {
            state: MiniConnState::Connected { name },
            transition_count: m.transition_count + 1,
        },
        other => C13Machine {
            state: other,
            transition_count: m.transition_count + 1,
        },
    }
}

// ── C13f: fail ────────────────────────────────────────────────────────────────

/// C13f: Connecting → Error, count + 1.
#[requires(c13_consistent(&m))]
#[requires(c13_below_max(&m))]
#[requires(message@.len() > 0)]
#[ensures(c13_consistent(&result))]
#[ensures(result.transition_count@ == m.transition_count@ + 1)]
pub fn c13_fail(m: C13Machine, message: String) -> C13Machine {
    C13Machine {
        state: MiniConnState::Error { message },
        transition_count: m.transition_count + 1,
    }
}

// ── C13g: disconnect ──────────────────────────────────────────────────────────

/// C13g: any → Disconnected, count + 1.
#[requires(c13_consistent(&m))]
#[requires(c13_below_max(&m))]
#[ensures(c13_consistent(&result))]
#[ensures(c13_is_disconnected(&result))]
#[ensures(result.transition_count@ == m.transition_count@ + 1)]
pub fn c13_disconnect(m: C13Machine) -> C13Machine {
    C13Machine {
        state: MiniConnState::Disconnected,
        transition_count: m.transition_count + 1,
    }
}

// ── C13h: full lifecycle ──────────────────────────────────────────────────────

/// C13h: new → begin → succeed → disconnect, with exact counter chain.
///
/// The counter goes 0 → 1 → 2 → 3.  Each step's exact count is known by Why3,
/// so `c13_below_max` is dischargeable at every step from the previous step's
/// exact postcondition alone — no manual lemma needed.
#[requires(name@.len() > 0)]
#[ensures(c13_consistent(&result))]
#[ensures(c13_is_disconnected(&result))]
#[ensures(result.transition_count@ == 3)]
pub fn c13_full_lifecycle(name: String) -> C13Machine {
    let m0 = c13_new(); // count = 0
    let m1 = c13_begin(m0, name); // count = 1
    let m2 = c13_succeed(m1); // count = 2
    c13_disconnect(m2) // count = 3
}
