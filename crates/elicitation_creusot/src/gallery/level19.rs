//! Gallery level C19: `ArchiveConnectionMachine` wrapper — counter + full archive state.
//!
//! **Hypothesis**: Why3 can simultaneously track exact transition counts (C13) and
//! routing guard postconditions (C17/C18) inside a machine struct wrapper, and can
//! close a 5-step lifecycle proof where both a counter postcondition
//! (`result.count@ == 5`) and a state postcondition (`is_disconnected`) hold.
//!
//! This is the final gallery composition: all individual patterns working together
//! in a single proof, structurally identical to the production
//! `ArchiveConnectionMachine`.
//!
//! ## New combination
//!
//! | C13 proven | C18 proven | C19 combines |
//! |------------|------------|--------------|
//! | counter increments per transition | routing guards + struct propagation | both together in every transition |
//! | exact count in lifecycle | full 6-variant state | count = 5 after 5-step full lifecycle |
//!
//! ## Types
//!
//! Imports `C18BackendKind`, `C18Descriptor`, `C18ConnState` (ADT definitions cross
//! COMA module boundaries). Logic predicates are re-defined as `c19_*` inlining
//! the same bodies (cross-module `#[logic]` calls are opaque in Why3).
//!
//! ```text
//! C19Machine { state: C18ConnState, count: i64 }
//! ```
//!
//! ## Machine consistency
//!
//! ```text
//! c19_machine_consistent(m) ≡
//!   c19_state_consistent(&m.state) && m.count@ >= 0
//! ```
//!
//! ## Transitions (all increment `count` by 1)
//!
//! ```text
//! c19_new()                         → count = 0, Disconnected
//! c19_begin_sql(m, profile_name)    → count += 1, Connecting { Sql }
//! c19_finish_sql(m, desc)           → count += 1, SqlConnected
//! c19_reconnect_sql(m)              → count += 1, Reconnecting
//! c19_finish_reconnect_sql(m, desc) → count += 1, SqlConnected
//! c19_disconnect(m)                 → count += 1, Disconnected
//! ```
//!
//! ## Experiment table
//!
//! | ID    | What                                                                      | Expected |
//! |-------|---------------------------------------------------------------------------|----------|
//! | C19a  | `c19_machine_consistent` wrapping inlined `c19_state_consistent`          | ✓        |
//! | C19b  | `c19_new` → count = 0, Disconnected                                       | ✓        |
//! | C19c  | `c19_begin_sql` → count + 1 AND routing guard postcondition               | ✓        |
//! | C19d  | `c19_finish_sql` requires routing guard AND ensures SqlConnected + count   | ✓        |
//! | C19e  | `c19_reconnect_sql` → struct propagation + counter                        | ✓        |
//! | C19f  | `c19_disconnect` → Disconnected + counter                                 | ✓        |
//! | C19g  | Full lifecycle: count = 5 AND is_disconnected                             | ✓        |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use super::level18::{C18BackendKind, C18ConnState, C18Descriptor};
use creusot_std::prelude::*;

// ── Machine struct ────────────────────────────────────────────────────────────

/// Archive connection machine — state + transition counter.
///
/// Mirrors the production `ArchiveConnectionMachine`.
pub struct C19Machine {
    /// Current connection state.
    pub state: C18ConnState,
    /// Number of transitions executed since creation.
    pub count: i64,
}

// ── Inlined logic predicates ──────────────────────────────────────────────────
// Cross-module #[logic] calls are opaque in Why3, so we re-define everything
// inline. The ADT types (C18*) cross module boundaries; their bodies do not.

/// C19a (part 1): descriptor consistency (inlined from C18).
#[logic]
pub fn c19_desc_consistent(d: &C18Descriptor) -> bool {
    pearlite! { d.name@.len() > 0 && d.port@ > 0 }
}

/// C19a (part 2): state-level consistency (inlined from `c18_consistent`).
#[logic]
pub fn c19_state_consistent(s: &C18ConnState) -> bool {
    pearlite! {
        match s {
            C18ConnState::Disconnected                        => true,
            C18ConnState::Connecting { profile_name, .. }     => profile_name@.len() > 0,
            C18ConnState::SqlConnected { desc }                => c19_desc_consistent(desc),
            C18ConnState::KvConnected  { path }                => path@.len() > 0,
            C18ConnState::Reconnecting { desc }                => c19_desc_consistent(desc),
            C18ConnState::ConnectionError { message }          => message@.len() > 0,
        }
    }
}

/// C19a (part 3): machine-level consistency.
#[logic]
pub fn c19_machine_consistent(m: &C19Machine) -> bool {
    pearlite! { c19_state_consistent(&m.state) && m.count@ >= 0 }
}

/// Guard: count is below `i64::MAX` — prevents overflow on `count + 1`.
///
/// In lifecycle proofs each step's exact count value (0, 1, 2, …) is known
/// from the previous postcondition, so Why3 discharges this trivially.
#[logic]
pub fn c19_count_bounded(m: &C19Machine) -> bool {
    pearlite! { m.count@ < i64::MAX@ }
}

#[logic]
pub fn c19_is_disconnected(s: &C18ConnState) -> bool {
    pearlite! { match s { C18ConnState::Disconnected => true, _ => false } }
}

#[logic]
pub fn c19_is_connecting(s: &C18ConnState) -> bool {
    pearlite! { match s { C18ConnState::Connecting { .. } => true, _ => false } }
}

#[logic]
pub fn c19_is_sql_connected(s: &C18ConnState) -> bool {
    pearlite! { match s { C18ConnState::SqlConnected { .. } => true, _ => false } }
}

#[logic]
pub fn c19_is_reconnecting(s: &C18ConnState) -> bool {
    pearlite! { match s { C18ConnState::Reconnecting { .. } => true, _ => false } }
}

/// `Connecting` with a `Sql` backend (nested enum destructuring, inlined).
#[logic]
pub fn c19_backend_is_sql(s: &C18ConnState) -> bool {
    pearlite! {
        match s {
            C18ConnState::Connecting { backend: C18BackendKind::Sql, .. } => true,
            _ => false,
        }
    }
}

/// Positive-form disconnect guard (inlined from `c18_is_disconnectable`).
#[logic]
pub fn c19_is_disconnectable(s: &C18ConnState) -> bool {
    pearlite! {
        match s {
            C18ConnState::SqlConnected { .. }    => true,
            C18ConnState::KvConnected  { .. }    => true,
            C18ConnState::Reconnecting { .. }    => true,
            C18ConnState::ConnectionError { .. } => true,
            _                                    => false,
        }
    }
}

// ── Transitions ───────────────────────────────────────────────────────────────

/// C19b: construct the initial machine state.
#[ensures(c19_machine_consistent(&result))]
#[ensures(c19_is_disconnected(&result.state))]
#[ensures(result.count@ == 0)]
pub fn c19_new() -> C19Machine {
    C19Machine {
        state: C18ConnState::Disconnected,
        count: 0,
    }
}

/// C19c: begin a SQL connection attempt.
///
/// **New combination**: routing guard postcondition + counter increment in one transition.
#[requires(c19_machine_consistent(&m))]
#[requires(c19_count_bounded(&m))]
#[requires(c19_is_disconnected(&m.state))]
#[requires(profile_name@.len() > 0)]
#[ensures(c19_machine_consistent(&result))]
#[ensures(c19_is_connecting(&result.state))]
#[ensures(c19_backend_is_sql(&result.state))]
#[ensures(result.count@ == m.count@ + 1)]
pub fn c19_begin_sql(m: C19Machine, profile_name: String) -> C19Machine {
    C19Machine {
        state: C18ConnState::Connecting {
            profile_name,
            backend: C18BackendKind::Sql,
        },
        count: m.count + 1,
    }
}

/// C19d: finish connecting to a SQL backend.
///
/// **New combination**: routing guard precondition + SqlConnected postcondition + counter.
#[requires(c19_machine_consistent(&m))]
#[requires(c19_count_bounded(&m))]
#[requires(c19_backend_is_sql(&m.state))]
#[requires(c19_desc_consistent(&desc))]
#[ensures(c19_machine_consistent(&result))]
#[ensures(c19_is_sql_connected(&result.state))]
#[ensures(result.count@ == m.count@ + 1)]
pub fn c19_finish_sql(m: C19Machine, desc: C18Descriptor) -> C19Machine {
    C19Machine {
        state: C18ConnState::SqlConnected { desc },
        count: m.count + 1,
    }
}

/// C19e: begin reconnecting — struct propagation + counter.
///
/// **New combination**: struct move (C16/C18 pattern) + counter increment.
#[requires(c19_machine_consistent(&m))]
#[requires(c19_count_bounded(&m))]
#[requires(c19_is_sql_connected(&m.state))]
#[ensures(c19_machine_consistent(&result))]
#[ensures(c19_is_reconnecting(&result.state))]
#[ensures(result.count@ == m.count@ + 1)]
pub fn c19_reconnect_sql(m: C19Machine) -> C19Machine {
    match m.state {
        C18ConnState::SqlConnected { desc } => C19Machine {
            state: C18ConnState::Reconnecting { desc },
            count: m.count + 1,
        },
        _ => C19Machine {
            state: C18ConnState::Disconnected,
            count: m.count + 1,
        },
    }
}

/// Finish a reconnect to a SQL backend.
#[requires(c19_machine_consistent(&m))]
#[requires(c19_count_bounded(&m))]
#[requires(c19_is_reconnecting(&m.state))]
#[requires(c19_desc_consistent(&desc))]
#[ensures(c19_machine_consistent(&result))]
#[ensures(c19_is_sql_connected(&result.state))]
#[ensures(result.count@ == m.count@ + 1)]
pub fn c19_finish_reconnect_sql(m: C19Machine, desc: C18Descriptor) -> C19Machine {
    C19Machine {
        state: C18ConnState::SqlConnected { desc },
        count: m.count + 1,
    }
}

/// C19f: disconnect from any active or error state.
#[requires(c19_machine_consistent(&m))]
#[requires(c19_count_bounded(&m))]
#[requires(c19_is_disconnectable(&m.state))]
#[ensures(c19_machine_consistent(&result))]
#[ensures(c19_is_disconnected(&result.state))]
#[ensures(result.count@ == m.count@ + 1)]
pub fn c19_disconnect(m: C19Machine) -> C19Machine {
    C19Machine {
        state: C18ConnState::Disconnected,
        count: m.count + 1,
    }
}

// ── Lifecycle ─────────────────────────────────────────────────────────────────

/// C19g: full lifecycle with exact count.
///
/// `new (0) → begin_sql (1) → finish_sql (2) → reconnect_sql (3)
///  → finish_reconnect_sql (4) → disconnect (5)`
///
/// **Key postcondition**: both `is_disconnected` AND `count = 5` hold simultaneously.
/// This is the first gallery proof that chains routing guards, struct propagation,
/// and an exact counter bound in a single Why3 obligation.
#[requires(profile_name@.len() > 0)]
#[requires(c19_desc_consistent(&desc1))]
#[requires(c19_desc_consistent(&desc2))]
#[ensures(c19_machine_consistent(&result))]
#[ensures(c19_is_disconnected(&result.state))]
#[ensures(result.count@ == 5)]
pub fn c19_full_lifecycle(
    profile_name: String,
    desc1: C18Descriptor,
    desc2: C18Descriptor,
) -> C19Machine {
    let m0 = c19_new(); // count = 0, Disconnected
    let m1 = c19_begin_sql(m0, profile_name); // count = 1, Connecting { Sql }
    let m2 = c19_finish_sql(m1, desc1); // count = 2, SqlConnected
    let m3 = c19_reconnect_sql(m2); // count = 3, Reconnecting
    let m4 = c19_finish_reconnect_sql(m3, desc2); // count = 4, SqlConnected
    c19_disconnect(m4) // count = 5, Disconnected
}
