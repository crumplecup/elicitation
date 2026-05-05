//! Gallery level C10: full mini connection state machine.
//!
//! **Hypothesis**: A complete VSM lifecycle — all variants, all transitions,
//! invariant maintained throughout — is expressible and provable in
//! Creusot/Why3.  This level mirrors the real `ArchiveConnectionMachine` at
//! reduced scale (4 variants instead of 6, same structural shape).
//!
//! The key question for the archive VSM: once we extract the state enum and
//! its consistency predicate from the production code, can Creusot discharge
//! all the transition goals?
//!
//! ## State diagram
//!
//! ```text
//! Disconnected ──begin──► Connecting { name }
//!                              │
//!             ┌────────────────┤ succeed / fail
//!             ▼                ▼
//!         Connected { name }  Error { message }
//!             │
//!             └──disconnect──► Disconnected
//! ```
//!
//! ## Consistency invariant
//!
//! - `Disconnected`       → always consistent
//! - `Connecting { name }` → `name` is non-empty
//! - `Connected { name }` → `name` is non-empty
//! - `Error { message }`  → `message` is non-empty
//!
//! ## Experiment table
//!
//! | ID    | What                                              | Expected |
//! |-------|---------------------------------------------------|----------|
//! | C10a  | Constructor for each consistent variant            | ✓        |
//! | C10b  | `begin`: Disconnected → Connecting                 | ✓        |
//! | C10c  | `succeed`: Connecting → Connected                  | ✓        |
//! | C10d  | `fail`: Connecting → Error                         | ✓        |
//! | C10e  | `disconnect`: Connected → Disconnected             | ✓        |
//! | C10f  | `identity` on a consistent state preserves it      | ✓        |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

// ── State enum ────────────────────────────────────────────────────────────────

/// Mini connection state — four-variant simplification of
/// `ArchiveConnectionState`.
pub enum MiniConnState {
    /// No active connection.
    Disconnected,
    /// A connection is in flight.
    Connecting {
        /// Profile name (non-empty).
        name: String,
    },
    /// A live connection exists.
    Connected {
        /// Profile name that was used to connect (non-empty).
        name: String,
    },
    /// The last attempt failed.
    Error {
        /// Human-readable error message (non-empty).
        message: String,
    },
}

// ── Consistency predicate ─────────────────────────────────────────────────────

/// All variants with String payloads require non-empty strings.
///
/// This is the Creusot equivalent of the Kani `archive_connection_consistent`
/// predicate — but expressed as a `#[logic]` function and proved once for all
/// transitions, rather than checked per-harness.

#[logic]
pub fn c10_consistent(s: &MiniConnState) -> bool {
    pearlite! {
        match s {
            MiniConnState::Disconnected => true,
            MiniConnState::Connecting { name } => name@.len() > 0,
            MiniConnState::Connected { name }  => name@.len() > 0,
            MiniConnState::Error { message }   => message@.len() > 0,
        }
    }
}

// ── C10a: direct constructors ─────────────────────────────────────────────────

/// Construct a consistent `Connecting` state.

#[requires(name@.len() > 0)]
#[ensures(c10_consistent(&result))]
pub fn c10_mk_connecting(name: String) -> MiniConnState {
    MiniConnState::Connecting { name }
}

/// Construct a consistent `Connected` state.

#[requires(name@.len() > 0)]
#[ensures(c10_consistent(&result))]
pub fn c10_mk_connected(name: String) -> MiniConnState {
    MiniConnState::Connected { name }
}

/// Construct a consistent `Error` state.

#[requires(message@.len() > 0)]
#[ensures(c10_consistent(&result))]
pub fn c10_mk_error(message: String) -> MiniConnState {
    MiniConnState::Error { message }
}

// ── C10b: begin ───────────────────────────────────────────────────────────────

/// C10b: Disconnected → Connecting.
///
/// The caller supplies the profile `name`; we require it to be non-empty so
/// that the `Connecting` invariant is immediately satisfied.

#[requires(name@.len() > 0)]
#[ensures(c10_consistent(&result))]
pub fn c10_begin(_s: MiniConnState, name: String) -> MiniConnState {
    MiniConnState::Connecting { name }
}

// ── C10c: succeed ─────────────────────────────────────────────────────────────

/// C10c: Connecting → Connected.
///
/// Why3 uses `c10_begin`'s postcondition to discharge `c10_consistent(&s)`
/// automatically when this function is called after `c10_begin`.

#[requires(c10_consistent(&s))]
#[ensures(c10_consistent(&result))]
pub fn c10_succeed(s: MiniConnState) -> MiniConnState {
    match s {
        MiniConnState::Connecting { name } => MiniConnState::Connected { name },
        other => other,
    }
}

// ── C10d: fail ────────────────────────────────────────────────────────────────

/// C10d: Connecting → Error.
///
/// The caller provides the error message; it must be non-empty.

#[requires(c10_consistent(&s))]
#[requires(message@.len() > 0)]
#[ensures(c10_consistent(&result))]
pub fn c10_fail(s: MiniConnState, message: String) -> MiniConnState {
    let _ = s;
    MiniConnState::Error { message }
}

// ── C10e: disconnect ──────────────────────────────────────────────────────────

/// C10e: Connected → Disconnected.

#[requires(c10_consistent(&s))]
#[ensures(c10_consistent(&result))]
pub fn c10_disconnect(s: MiniConnState) -> MiniConnState {
    let _ = s;
    MiniConnState::Disconnected
}

// ── C10f: identity ────────────────────────────────────────────────────────────

/// C10f: identity preserves any consistent state.
///
/// This proves the "frame condition": a no-op on the state does not break
/// consistency.  All transitions in the real VSM must satisfy this as a
/// minimum bar.

#[requires(c10_consistent(&s))]
#[ensures(c10_consistent(&result))]
pub fn c10_identity(s: MiniConnState) -> MiniConnState {
    s
}

// ── Full lifecycle (C10b→c→e chained) ────────────────────────────────────────

/// Full round-trip: Disconnected → Connecting → Connected → Disconnected.
///
/// Each postcondition flows freely into the next precondition (C6 insight).
/// No stub_verified or proof tokens needed.

#[requires(name@.len() > 0)]
#[ensures(c10_consistent(&result))]
pub fn c10_full_lifecycle(name: String) -> MiniConnState {
    let s0 = MiniConnState::Disconnected;
    let s1 = c10_begin(s0, name);
    let s2 = c10_succeed(s1);
    c10_disconnect(s2)
}
