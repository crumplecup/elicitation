//! Gallery level C14: machine with two numeric fields and a relational invariant.
//!
//! **Hypothesis**: Pearlite can express invariants that relate two numeric
//! fields to each other (`error_count ≤ transition_count`).  Only specific
//! transitions update both counters; others update only one.  Why3 must track
//! the relationship across all branches.
//!
//! This is valuable for the real archive: tracking both total transitions and
//! error events, with a structural guarantee that errors cannot exceed attempts.
//!
//! ## Machine struct
//!
//! ```text
//! C14Machine {
//!     state: MiniConnState,
//!     transition_count: i64,  -- every transition increments this
//!     error_count: i64,       -- only c14_fail increments this
//! }
//! ```
//!
//! ## Invariant
//!
//! ```text
//! c14_consistent(m) ≡
//!     c14_state_ok(&m.state)
//!   ∧ m.transition_count@ ≥ 0
//!   ∧ m.error_count@ ≥ 0
//!   ∧ m.error_count@ ≤ m.transition_count@
//! ```
//!
//! The relational term (`error_count ≤ transition_count`) is the key new
//! hypothesis: does Alt-Ergo close goals involving inequalities between two
//! separate numeric model values?
//!
//! ## Experiment table
//!
//! | ID    | What                                                | Expected |
//! |-------|-----------------------------------------------------|----------|
//! | C14a  | Relational invariant: error_count ≤ transition_count | ✓       |
//! | C14b  | Constructor: both counters = 0                      | ✓        |
//! | C14c  | `begin`: only transition_count + 1                  | ✓        |
//! | C14d  | `succeed`: only transition_count + 1                | ✓        |
//! | C14e  | `fail`: both counters + 1 (preserves ≤ relation)   | ✓        |
//! | C14f  | `disconnect`: only transition_count + 1             | ✓        |
//! | C14g  | Full lifecycle with error: count=5, errors=1         | ✓        |

use creusot_std::prelude::*;

use crate::gallery::level10::MiniConnState;

// ── State consistency (inlined) ───────────────────────────────────────────────

/// State invariant — same body as `c10_consistent` / `c13_state_ok`, inlined.
#[logic]
pub fn c14_state_ok(s: &MiniConnState) -> bool {
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

/// Machine with state + two numeric fields.
pub struct C14Machine {
    /// Current connection state.
    pub state: MiniConnState,
    /// Total transitions performed (non-negative, monotone).
    pub transition_count: i64,
    /// Transitions that ended in `Error` (non-negative, ≤ transition_count).
    pub error_count: i64,
}

/// C14a: composite invariant with relational term.
#[logic]
pub fn c14_consistent(m: &C14Machine) -> bool {
    pearlite! {
        c14_state_ok(&m.state)
            && m.transition_count@ >= 0
            && m.error_count@ >= 0
            && m.error_count@ <= m.transition_count@
    }
}

/// Guard: transition_count is below `i64::MAX`.
#[logic]
pub fn c14_below_max(m: &C14Machine) -> bool {
    pearlite! { m.transition_count@ < i64::MAX@ }
}

/// Guard: error_count is also below `i64::MAX` (needed before c14_fail).
#[logic]
pub fn c14_errors_below_max(m: &C14Machine) -> bool {
    pearlite! { m.error_count@ < i64::MAX@ }
}

// ── State predicates ──────────────────────────────────────────────────────────

/// True when the machine is in `Disconnected` state.
#[logic]
pub fn c14_is_disconnected(m: &C14Machine) -> bool {
    pearlite! {
        match &m.state {
            MiniConnState::Disconnected => true,
            _ => false,
        }
    }
}

/// True when the machine is in `Connecting` state.
#[logic]
pub fn c14_is_connecting(m: &C14Machine) -> bool {
    pearlite! {
        match &m.state {
            MiniConnState::Connecting { .. } => true,
            _ => false,
        }
    }
}

/// True when the machine is in `Connected` state.
#[logic]
pub fn c14_is_connected(m: &C14Machine) -> bool {
    pearlite! {
        match &m.state {
            MiniConnState::Connected { .. } => true,
            _ => false,
        }
    }
}

// ── C14b: constructor ─────────────────────────────────────────────────────────

/// C14b: fresh machine — Disconnected, both counters = 0.
#[ensures(c14_consistent(&result))]
#[ensures(c14_is_disconnected(&result))]
#[ensures(result.transition_count@ == 0)]
#[ensures(result.error_count@ == 0)]
pub fn c14_new() -> C14Machine {
    C14Machine {
        state: MiniConnState::Disconnected,
        transition_count: 0,
        error_count: 0,
    }
}

// ── C14c: begin ───────────────────────────────────────────────────────────────

/// C14c: Disconnected → Connecting, transition_count + 1, error_count unchanged.
#[requires(c14_consistent(&m))]
#[requires(c14_below_max(&m))]
#[requires(name@.len() > 0)]
#[ensures(c14_consistent(&result))]
#[ensures(c14_is_connecting(&result))]
#[ensures(result.transition_count@ == m.transition_count@ + 1)]
#[ensures(result.error_count@ == m.error_count@)]
pub fn c14_begin(m: C14Machine, name: String) -> C14Machine {
    C14Machine {
        state: MiniConnState::Connecting { name },
        transition_count: m.transition_count + 1,
        error_count: m.error_count,
    }
}

// ── C14d: succeed ─────────────────────────────────────────────────────────────

/// C14d: Connecting → Connected, transition_count + 1, error_count unchanged.
#[requires(c14_consistent(&m))]
#[requires(c14_is_connecting(&m))]
#[requires(c14_below_max(&m))]
#[ensures(c14_consistent(&result))]
#[ensures(c14_is_connected(&result))]
#[ensures(result.transition_count@ == m.transition_count@ + 1)]
#[ensures(result.error_count@ == m.error_count@)]
pub fn c14_succeed(m: C14Machine) -> C14Machine {
    match m.state {
        MiniConnState::Connecting { name } => C14Machine {
            state: MiniConnState::Connected { name },
            transition_count: m.transition_count + 1,
            error_count: m.error_count,
        },
        other => C14Machine {
            state: other,
            transition_count: m.transition_count + 1,
            error_count: m.error_count,
        },
    }
}

// ── C14e: fail ────────────────────────────────────────────────────────────────

/// C14e: Connecting → Error, both counters + 1.
///
/// This is the key transition: `error_count` increments alongside
/// `transition_count`.  The relational invariant is preserved because
/// `(e+1) ≤ (t+1)` follows from `e ≤ t`.
#[requires(c14_consistent(&m))]
#[requires(c14_below_max(&m))]
#[requires(c14_errors_below_max(&m))]
#[requires(message@.len() > 0)]
#[ensures(c14_consistent(&result))]
#[ensures(result.transition_count@ == m.transition_count@ + 1)]
#[ensures(result.error_count@ == m.error_count@ + 1)]
pub fn c14_fail(m: C14Machine, message: String) -> C14Machine {
    C14Machine {
        state: MiniConnState::Error { message },
        transition_count: m.transition_count + 1,
        error_count: m.error_count + 1,
    }
}

// ── C14f: disconnect ──────────────────────────────────────────────────────────

/// C14f: any → Disconnected, transition_count + 1, error_count unchanged.
#[requires(c14_consistent(&m))]
#[requires(c14_below_max(&m))]
#[ensures(c14_consistent(&result))]
#[ensures(c14_is_disconnected(&result))]
#[ensures(result.transition_count@ == m.transition_count@ + 1)]
#[ensures(result.error_count@ == m.error_count@)]
pub fn c14_disconnect(m: C14Machine) -> C14Machine {
    C14Machine {
        state: MiniConnState::Disconnected,
        transition_count: m.transition_count + 1,
        error_count: m.error_count,
    }
}

// ── C14g: full lifecycle with error ───────────────────────────────────────────

/// C14g: new → begin → fail → disconnect → begin → succeed → disconnect.
///
/// One connection attempt fails, a second succeeds, then disconnects.
/// Final state: transition_count = 6, error_count = 1.
///
/// This verifies:
/// - error_count only moves on `fail`
/// - relational invariant holds throughout a mixed success/failure run
/// - the machine can recover from Error via `disconnect`
///
/// `name2` is the retry profile name (separate parameter — Creusot cannot
/// model `"literal".to_string()` and generates `{false}` for it in COMA).
#[requires(name@.len() > 0)]
#[requires(name2@.len() > 0)]
#[requires(message@.len() > 0)]
#[ensures(c14_consistent(&result))]
#[ensures(c14_is_disconnected(&result))]
#[ensures(result.transition_count@ == 6)]
#[ensures(result.error_count@ == 1)]
pub fn c14_full_lifecycle(name: String, name2: String, message: String) -> C14Machine {
    let m0 = c14_new(); // (0, 0)
    let m1 = c14_begin(m0, name); // (1, 0)  Connecting
    let m2 = c14_fail(m1, message); // (2, 1)  Error
    let m3 = c14_disconnect(m2); // (3, 1)  Disconnected
    let m4 = c14_begin(m3, name2); // (4, 1)  Connecting  ← retry
    let m5 = c14_succeed(m4); // (5, 1)  Connected
    c14_disconnect(m5) // (6, 1)  Disconnected
}
