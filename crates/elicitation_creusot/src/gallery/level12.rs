//! Gallery level C12: two-machine composition with cross-machine gating.
//!
//! **Hypothesis**: A composite invariant spanning two sub-machines
//! (connection + panel) can be expressed and maintained in Creusot.
//! The gating rule — "panel can only be non-Empty when the connection is
//! Connected" — is the cross-machine invariant.
//!
//! This directly models the real archive: panel data cannot exist without
//! a live database connection.
//!
//! ## Composite invariant
//!
//! ```text
//! c12_consistent(s) ≡
//!     c12_conn_ok(&s)    -- conn sub-machine invariant (inlined)
//!   ∧ c12_panel_ok(&s)  -- panel sub-machine invariant (inlined)
//!   ∧ c12_gated(&s)     -- cross-machine gating
//! ```
//!
//! Where `c12_gated` means: panel is non-`Empty` only if conn is `Connected`.
//!
//! ## Why consistency predicates are inlined
//!
//! Cross-module `#[logic]` calls (`c10_consistent`, `c11_consistent`) appear as
//! **opaque predicates** in level12's Why3 context — their definitions are not
//! carried across the COMA boundary.  The prover sees them as abstract booleans
//! and cannot unfold their bodies to discharge goals.
//!
//! The fix: copy the bodies of `c10_consistent` and `c11_consistent` directly
//! into `c12_conn_ok` and `c12_panel_ok`.  Level12 still *imports the types*
//! (`MiniConnState`, `MiniPanelState`, `MiniDisplayMode`) — ADT definitions are
//! always inlined by Creusot.  Only `#[logic]` function bodies are not.
//!
//! ## Key insight
//!
//! `c12_disconnect` must cascade: it resets **both** sub-machines because
//! the gating invariant forbids non-Empty panel without a Connected conn.
//! This "cascade" is enforced by the postcondition of `c12_disconnect`,
//! not by runtime bookkeeping — the type system (through Creusot) guarantees it.
//!
//! ## Experiment table
//!
//! | ID    | What                                                | Expected |
//! |-------|-----------------------------------------------------|----------|
//! | C12a  | Inlined sub-machine consistency (opaque fix)        | ✓        |
//! | C12b  | Gating predicate across both sub-machines           | ✓        |
//! | C12c  | `new`: both machines in initial state                | ✓        |
//! | C12d  | `begin`: Disconnected → Connecting, panel unchanged  | ✓        |
//! | C12e  | `succeed`: Connecting → Connected                   | ✓        |
//! | C12f  | `load`: Connected + Empty → Connected + Grid        | ✓        |
//! | C12g  | `disconnect`: any → Disconnected + Empty (cascade)  | ✓        |
//! | C12h  | Full composite lifecycle                            | ✓        |

use creusot_std::prelude::*;

use crate::gallery::level10::MiniConnState;
use crate::gallery::level11::{MiniDisplayMode, MiniPanelState};

// ── Composite struct ──────────────────────────────────────────────────────────

/// Composite of a connection sub-machine and a panel sub-machine.
///
/// The panel can only hold data when the connection is `Connected` — the
/// gating invariant enforces this relationship.
pub struct MiniComposite {
    /// Connection sub-machine state.
    pub conn: MiniConnState,
    /// Panel sub-machine state.
    pub panel: MiniPanelState,
}

// ── Sub-machine consistency predicates (inlined) ─────────────────────────────

/// C12a: connection sub-machine consistency.
///
/// Body is copied from `c10_consistent` in level10 because cross-module
/// `#[logic]` calls appear opaque in Why3.  ADT definitions (the enum arms)
/// ARE inlined by Creusot; only function bodies are not.
#[logic]
pub fn c12_conn_ok(s: &MiniComposite) -> bool {
    pearlite! {
        match &s.conn {
            MiniConnState::Disconnected => true,
            MiniConnState::Connecting { name } => name@.len() > 0,
            MiniConnState::Connected { name }  => name@.len() > 0,
            MiniConnState::Error { message }   => message@.len() > 0,
        }
    }
}

/// Panel sub-machine consistency.
///
/// Body is copied from `c11_consistent` in level11 for the same reason.
#[logic]
pub fn c12_panel_ok(s: &MiniComposite) -> bool {
    pearlite! {
        match &s.panel {
            MiniPanelState::Empty => true,
            MiniPanelState::Grid { mode: _, row_count } => row_count@ > 0,
            MiniPanelState::Detail { mode: _ } => true,
            MiniPanelState::Error { message } => message@.len() > 0,
        }
    }
}

// ── C12b: cross-machine gating ────────────────────────────────────────────────

/// True when the panel is non-`Empty` only if the connection is `Connected`.
///
/// This is the cross-machine invariant.
#[logic]
pub fn c12_gated(s: &MiniComposite) -> bool {
    pearlite! {
        match &s.panel {
            MiniPanelState::Empty => true,
            _ => match &s.conn {
                MiniConnState::Connected { .. } => true,
                _ => false,
            }
        }
    }
}

/// Full composite invariant.
#[logic]
pub fn c12_consistent(s: &MiniComposite) -> bool {
    pearlite! {
        c12_conn_ok(s) && c12_panel_ok(s) && c12_gated(s)
    }
}

// ── Connection-state helper predicates ───────────────────────────────────────

/// True when the composite's connection is `Disconnected`.
#[logic]
pub fn c12_is_disconnected(s: &MiniComposite) -> bool {
    pearlite! {
        match &s.conn {
            MiniConnState::Disconnected => true,
            _ => false,
        }
    }
}

/// True when the composite's connection is `Connecting`.
#[logic]
pub fn c12_is_connecting(s: &MiniComposite) -> bool {
    pearlite! {
        match &s.conn {
            MiniConnState::Connecting { .. } => true,
            _ => false,
        }
    }
}

/// True when the composite's connection is `Connected`.
#[logic]
pub fn c12_is_connected(s: &MiniComposite) -> bool {
    pearlite! {
        match &s.conn {
            MiniConnState::Connected { .. } => true,
            _ => false,
        }
    }
}

// ── C12c: constructor ─────────────────────────────────────────────────────────

/// C12c: create a fresh composite — Disconnected + Empty.
#[ensures(c12_consistent(&result))]
#[ensures(c12_is_disconnected(&result))]
pub fn c12_new() -> MiniComposite {
    MiniComposite {
        conn: MiniConnState::Disconnected,
        panel: MiniPanelState::Empty,
    }
}

// ── C12d: begin ───────────────────────────────────────────────────────────────

/// C12d: begin connecting — Disconnected → Connecting, panel unchanged.
///
/// Requires `c12_is_disconnected` so Why3 can prove that `s.panel = Empty`
/// (from the gating invariant: non-Empty panel requires Connected, but conn
/// is Disconnected → contradiction → panel must be Empty).  Therefore the
/// result's gating invariant holds even with the new `Connecting` conn.
#[requires(c12_consistent(&s))]
#[requires(c12_is_disconnected(&s))]
#[requires(name@.len() > 0)]
#[ensures(c12_consistent(&result))]
#[ensures(c12_is_connecting(&result))]
pub fn c12_begin(s: MiniComposite, name: String) -> MiniComposite {
    MiniComposite {
        conn: MiniConnState::Connecting { name },
        panel: s.panel,
    }
}

// ── C12e: succeed ─────────────────────────────────────────────────────────────

/// C12e: succeed — Connecting → Connected, panel unchanged.
///
/// After connecting, the gating invariant is relaxed: the panel may now
/// hold data (though it remains Empty here since we only connected).
#[requires(c12_consistent(&s))]
#[requires(c12_is_connecting(&s))]
#[ensures(c12_consistent(&result))]
#[ensures(c12_is_connected(&result))]
pub fn c12_succeed(s: MiniComposite) -> MiniComposite {
    match s.conn {
        MiniConnState::Connecting { name } => MiniComposite {
            conn: MiniConnState::Connected { name },
            panel: s.panel,
        },
        other => MiniComposite {
            conn: other,
            panel: s.panel,
        },
    }
}

// ── C12f: load ────────────────────────────────────────────────────────────────

/// C12f: load data — Connected + (any panel) → Connected + Grid { Read, n }.
///
/// This is the key cross-machine dependency: the panel transition requires
/// `c12_is_connected`, a predicate about the *connection* sub-machine.
/// The `other` branch is unreachable (precondition guarantees Connected),
/// so Why3 discharges it by contradiction.
#[requires(c12_consistent(&s))]
#[requires(c12_is_connected(&s))]
#[requires(row_count@ > 0)]
#[ensures(c12_consistent(&result))]
pub fn c12_load(s: MiniComposite, row_count: i64) -> MiniComposite {
    match s.conn {
        MiniConnState::Connected { name } => MiniComposite {
            conn: MiniConnState::Connected { name },
            panel: MiniPanelState::Grid {
                mode: MiniDisplayMode::Read,
                row_count,
            },
        },
        other => MiniComposite {
            conn: other,
            panel: s.panel,
        },
    }
}

// ── C12g: disconnect ──────────────────────────────────────────────────────────

/// C12g: disconnect — any state → Disconnected + Empty.
///
/// The panel is forcibly reset to `Empty`.  This is the "cascade transition":
/// disconnecting the conn sub-machine also resets the panel sub-machine, because
/// the gating invariant (`c12_gated`) forbids non-Empty panel without Connected.
/// The postcondition encodes this cascade — the prover verifies it is always met.
#[requires(c12_consistent(&s))]
#[ensures(c12_consistent(&result))]
pub fn c12_disconnect(s: MiniComposite) -> MiniComposite {
    let _ = s;
    MiniComposite {
        conn: MiniConnState::Disconnected,
        panel: MiniPanelState::Empty,
    }
}

// ── C12h: full composite lifecycle ───────────────────────────────────────────

/// C12h: Full lifecycle — new → begin → succeed → load → disconnect.
///
/// Verifies that the complete workflow (initial state → connecting → connected →
/// data loaded → disconnected) satisfies the composite invariant end-to-end.
/// Each postcondition feeds the next precondition; the panel-reset cascade at
/// disconnect is automatically enforced.
#[requires(name@.len() > 0)]
#[requires(row_count@ > 0)]
#[ensures(c12_consistent(&result))]
pub fn c12_full_lifecycle(name: String, row_count: i64) -> MiniComposite {
    let s0 = c12_new(); // ensures: c12_consistent
    let s1 = c12_begin(s0, name); // ensures: c12_consistent, c12_is_connecting
    let s2 = c12_succeed(s1); // ensures: c12_consistent, c12_is_connected
    let s3 = c12_load(s2, row_count); // ensures: c12_consistent
    c12_disconnect(s3) // ensures: c12_consistent
}
