//! Gallery level C8: machine wrapper struct — invariant over state + metadata.
//!
//! **Hypothesis**: Pearlite can express invariants over a *struct* that wraps
//! a state enum together with numeric metadata (transition counter, timestamp
//! counter, etc.).  This is the shape of the real `ArchiveConnectionMachine`
//! which wraps `ArchiveConnectionState` plus auxiliary fields.
//!
//! ## Experiment table
//!
//! | ID   | What                                               | Expected |
//! |------|----------------------------------------------------|----------|
//! | C8a  | Invariant over struct field of enum type            | ✓        |
//! | C8b  | Invariant over two struct fields simultaneously     | ✓        |
//! | C8c  | Constructor satisfying the combined invariant       | ✓        |
//! | C8d  | Transition preserving the combined invariant        | ✓        |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

// ── State enum ────────────────────────────────────────────────────────────────

/// Minimal two-variant connection state (unit variants only for C8 focus).
pub enum C8ConnState {
    Disconnected,
    Connected,
}

/// Predicate: the connection state is itself consistent (trivially true here).

#[logic]
pub fn c8_state_ok(s: &C8ConnState) -> bool {
    match s {
        C8ConnState::Disconnected => true,
        C8ConnState::Connected => true,
    }
}

// ── Machine wrapper ────────────────────────────────────────────────────────────

/// A machine struct: state + a non-negative transition counter.
///
/// Mirrors the shape of `ArchiveConnectionMachine { state, .. }`.
pub struct C8Machine {
    /// Current connection state.
    pub state: C8ConnState,
    /// Number of transitions performed (monotonically non-decreasing).
    transition_count: i64,
}

/// C8 invariant: the state is consistent AND the counter is non-negative.
///
/// C8b: simultaneous predicate over two distinct fields.

#[logic]
pub fn c8_consistent(m: &C8Machine) -> bool {
    pearlite! {
        c8_state_ok(&m.state) && m.transition_count@ >= 0
    }
}

/// Guard: transition_count is below `i64::MAX` — prevents overflow in transitions.

#[logic]
pub fn c8_below_max(m: &C8Machine) -> bool {
    pearlite! { m.transition_count@ < 9223372036854775807i64@ }
}

// ── Constructor ────────────────────────────────────────────────────────────────

/// C8c: create a fresh machine in the Disconnected state.
///
/// Initial `transition_count` is 0 — satisfies `>= 0`.

#[requires(true)]
#[ensures(c8_consistent(&result))]
pub fn c8_new() -> C8Machine {
    C8Machine {
        state: C8ConnState::Disconnected,
        transition_count: 0,
    }
}

// ── Transition ─────────────────────────────────────────────────────────────────

/// C8d: connect — Disconnected → Connected, incrementing the counter.
///
/// Verifies: a transition that mutates both fields preserves `c8_consistent`.
/// The upper-bound guard prevents overflow of `transition_count`.

#[requires(c8_consistent(&m))]
#[requires(c8_below_max(&m))]
#[ensures(c8_consistent(&result))]
pub fn c8_connect(m: C8Machine) -> C8Machine {
    C8Machine {
        state: C8ConnState::Connected,
        transition_count: m.transition_count + 1,
    }
}

/// Disconnect — Connected → Disconnected, incrementing the counter.

#[requires(c8_consistent(&m))]
#[requires(c8_below_max(&m))]
#[ensures(c8_consistent(&result))]
pub fn c8_disconnect(m: C8Machine) -> C8Machine {
    C8Machine {
        state: C8ConnState::Disconnected,
        transition_count: m.transition_count + 1,
    }
}
