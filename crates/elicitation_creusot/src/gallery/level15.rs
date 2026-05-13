//! Gallery level C15: six-variant connection state mirroring `ArchiveConnectionState`.
//!
//! **Hypothesis**: Pearlite can express a consistency invariant over a 6-variant
//! enum that includes a **two-field variant** (`Connecting { profile_name, backend_tag }`),
//! and Why3 can propagate the payload non-empty invariant through a `match`
//! that moves a field from one variant to another (`SqlConnected { tag }` →
//! `Reconnecting { tag }`).
//!
//! This level is the direct Creusot analogue of the Kani archive connection
//! proofs.  The types are simplified (plain `String` instead of `DatabaseDescriptor`
//! / `BackendKind`) but the structural shape is identical.
//!
//! ## State diagram
//!
//! ```text
//! Disconnected ──begin_connect──► Connecting { profile_name, backend_tag }
//!      ▲                              │
//!      │         ┌───────────────────┤ finish_sql / finish_kv
//!      │         │                   │ connection_error
//!      │         ▼                   ▼
//!      │   SqlConnected { tag }   KvConnected { path }   ConnectionError { message }
//!      │         │                   │
//!      │         └────reconnect_sql──►
//!      │                       Reconnecting { tag }
//!      │                             │
//!      │         ┌───────────────────┤ finish_reconnect_sql
//!      │         │                   │ connection_error (Reconnecting)
//!      └─────────┘
//!          disconnect (from SqlConnected | KvConnected | Reconnecting | ConnectionError)
//! ```
//!
//! ## Consistency invariant
//!
//! - `Disconnected`                           → always consistent
//! - `Connecting { profile_name, backend_tag }` → both fields non-empty
//! - `SqlConnected { tag }`                   → `tag` non-empty
//! - `KvConnected { path }`                   → `path` non-empty
//! - `Reconnecting { tag }`                   → `tag` non-empty
//! - `ConnectionError { message }`            → `message` non-empty
//!
//! ## Key new patterns
//!
//! 1. Two-field variant: consistency predicate uses `&&` across two String fields.
//! 2. Payload propagation: `reconnect_sql` moves `tag` from `SqlConnected` to
//!    `Reconnecting`, and Why3 must transfer the `tag@.len() > 0` fact.
//! 3. `c15_is_disconnectable` — positive-form guard covering 4 variants, avoiding
//!    double-negation reasoning.
//!
//! ## Experiment table
//!
//! | ID    | What                                                  | Expected |
//! |-------|-------------------------------------------------------|----------|
//! | C15a  | Constructor: `Disconnected`                           | ✓        |
//! | C15b  | `begin_connect`: two String params, &&-consistency    | ✓        |
//! | C15c  | `finish_sql`: Connecting → SqlConnected               | ✓        |
//! | C15d  | `finish_kv`: Connecting → KvConnected                 | ✓        |
//! | C15e  | `connection_error`: Connecting → ConnectionError      | ✓        |
//! | C15f  | `reconnect_sql`: tag propagated from Sql → Reconnect  | ✓        |
//! | C15g  | `finish_reconnect_sql`: Reconnecting → SqlConnected   | ✓        |
//! | C15h  | `disconnect`: 4-variant guard, result = Disconnected  | ✓        |
//! | C15i  | Full reconnect lifecycle (6 steps)                    | ✓        |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

// ── State enum ────────────────────────────────────────────────────────────────

/// Six-variant connection state modelled after `ArchiveConnectionState`.
pub enum C15ConnState {
    /// No active backend connection.
    Disconnected,
    /// A connection attempt is in flight; two payload fields.
    Connecting {
        /// Name of the connection profile (non-empty).
        profile_name: String,
        /// Backend technology tag (non-empty).
        backend_tag: String,
    },
    /// Connected to a SQL backend.
    SqlConnected {
        /// Connection descriptor tag (non-empty).
        tag: String,
    },
    /// Connected to a key-value store.
    KvConnected {
        /// Filesystem path (non-empty).
        path: String,
    },
    /// A reconnect attempt is in flight.
    Reconnecting {
        /// Tag from the previous connection (non-empty).
        tag: String,
    },
    /// The last connection attempt failed.
    ConnectionError {
        /// Human-readable error message (non-empty).
        message: String,
    },
}

// ── Logic predicates ──────────────────────────────────────────────────────────

/// The state is self-consistent: all String payload fields are non-empty.
///
/// The `Connecting` arm is the key new test: two String fields joined by `&&`.
#[logic]
pub fn c15_consistent(s: &C15ConnState) -> bool {
    pearlite! {
        match s {
            C15ConnState::Disconnected                        => true,
            C15ConnState::Connecting { profile_name, backend_tag } =>
                profile_name@.len() > 0 && backend_tag@.len() > 0,
            C15ConnState::SqlConnected { tag }                => tag@.len() > 0,
            C15ConnState::KvConnected { path }                => path@.len() > 0,
            C15ConnState::Reconnecting { tag }                => tag@.len() > 0,
            C15ConnState::ConnectionError { message }         => message@.len() > 0,
        }
    }
}

#[logic]
pub fn c15_is_disconnected(s: &C15ConnState) -> bool {
    pearlite! {
        match s { C15ConnState::Disconnected => true, _ => false }
    }
}

#[logic]
pub fn c15_is_connecting(s: &C15ConnState) -> bool {
    pearlite! {
        match s { C15ConnState::Connecting { .. } => true, _ => false }
    }
}

#[logic]
pub fn c15_is_sql_connected(s: &C15ConnState) -> bool {
    pearlite! {
        match s { C15ConnState::SqlConnected { .. } => true, _ => false }
    }
}

#[logic]
pub fn c15_is_kv_connected(s: &C15ConnState) -> bool {
    pearlite! {
        match s { C15ConnState::KvConnected { .. } => true, _ => false }
    }
}

#[logic]
pub fn c15_is_reconnecting(s: &C15ConnState) -> bool {
    pearlite! {
        match s { C15ConnState::Reconnecting { .. } => true, _ => false }
    }
}

#[logic]
pub fn c15_is_error(s: &C15ConnState) -> bool {
    pearlite! {
        match s { C15ConnState::ConnectionError { .. } => true, _ => false }
    }
}

/// Positive-form guard for `disconnect`: covers the four states from which
/// disconnection is legal, avoiding double-negation reasoning in Why3.
#[logic]
pub fn c15_is_disconnectable(s: &C15ConnState) -> bool {
    pearlite! {
        match s {
            C15ConnState::SqlConnected { .. }    => true,
            C15ConnState::KvConnected { .. }     => true,
            C15ConnState::Reconnecting { .. }    => true,
            C15ConnState::ConnectionError { .. } => true,
            _                                    => false,
        }
    }
}

// ── Transitions ───────────────────────────────────────────────────────────────

/// C15a: fresh disconnected state.
#[ensures(c15_consistent(&result))]
#[ensures(c15_is_disconnected(&result))]
pub fn c15_new() -> C15ConnState {
    C15ConnState::Disconnected
}

/// C15b: begin a connection attempt.
///
/// `Disconnected` → `Connecting { profile_name, backend_tag }`
///
/// Both payload fields are required to be non-empty so `c15_consistent` holds.
#[requires(c15_consistent(&s))]
#[requires(c15_is_disconnected(&s))]
#[requires(profile_name@.len() > 0)]
#[requires(backend_tag@.len() > 0)]
#[ensures(c15_consistent(&result))]
#[ensures(c15_is_connecting(&result))]
pub fn c15_begin_connect(
    s: C15ConnState,
    profile_name: String,
    backend_tag: String,
) -> C15ConnState {
    let _ = s;
    C15ConnState::Connecting {
        profile_name,
        backend_tag,
    }
}

/// C15c: finish connecting to a SQL backend.
///
/// `Connecting` → `SqlConnected { tag }`
#[requires(c15_consistent(&s))]
#[requires(c15_is_connecting(&s))]
#[requires(tag@.len() > 0)]
#[ensures(c15_consistent(&result))]
#[ensures(c15_is_sql_connected(&result))]
pub fn c15_finish_sql(s: C15ConnState, tag: String) -> C15ConnState {
    let _ = s;
    C15ConnState::SqlConnected { tag }
}

/// C15d: finish connecting to a KV backend.
///
/// `Connecting` → `KvConnected { path }`
#[requires(c15_consistent(&s))]
#[requires(c15_is_connecting(&s))]
#[requires(path@.len() > 0)]
#[ensures(c15_consistent(&result))]
#[ensures(c15_is_kv_connected(&result))]
pub fn c15_finish_kv(s: C15ConnState, path: String) -> C15ConnState {
    let _ = s;
    C15ConnState::KvConnected { path }
}

/// C15e: record a connection error.
///
/// `Connecting` → `ConnectionError { message }`
#[requires(c15_consistent(&s))]
#[requires(c15_is_connecting(&s))]
#[requires(message@.len() > 0)]
#[ensures(c15_consistent(&result))]
#[ensures(c15_is_error(&result))]
pub fn c15_connection_error(s: C15ConnState, message: String) -> C15ConnState {
    let _ = s;
    C15ConnState::ConnectionError { message }
}

/// C15f: begin reconnecting from an established SQL connection.
///
/// `SqlConnected { tag }` → `Reconnecting { tag }`
///
/// Key test: the `tag` field is moved from `SqlConnected` into `Reconnecting`.
/// Why3 must propagate `tag@.len() > 0` (from `c15_consistent` on the input)
/// through the `match` to discharge `c15_consistent` on the result.
#[requires(c15_consistent(&s))]
#[requires(c15_is_sql_connected(&s))]
#[ensures(c15_consistent(&result))]
#[ensures(c15_is_reconnecting(&result))]
pub fn c15_reconnect_sql(s: C15ConnState) -> C15ConnState {
    match s {
        C15ConnState::SqlConnected { tag } => C15ConnState::Reconnecting { tag },
        _ => C15ConnState::Disconnected, // unreachable given precondition
    }
}

/// C15g: finish a reconnect to a SQL backend.
///
/// `Reconnecting` → `SqlConnected { tag }`
#[requires(c15_consistent(&s))]
#[requires(c15_is_reconnecting(&s))]
#[requires(tag@.len() > 0)]
#[ensures(c15_consistent(&result))]
#[ensures(c15_is_sql_connected(&result))]
pub fn c15_finish_reconnect_sql(s: C15ConnState, tag: String) -> C15ConnState {
    let _ = s;
    C15ConnState::SqlConnected { tag }
}

/// C15h: disconnect from any active or error state.
///
/// `SqlConnected | KvConnected | Reconnecting | ConnectionError` → `Disconnected`
///
/// Uses the positive-form guard `c15_is_disconnectable` instead of a double
/// negation, which makes the Why3 goal easier to discharge.
#[requires(c15_consistent(&s))]
#[requires(c15_is_disconnectable(&s))]
#[ensures(c15_consistent(&result))]
#[ensures(c15_is_disconnected(&result))]
pub fn c15_disconnect(s: C15ConnState) -> C15ConnState {
    let _ = s;
    C15ConnState::Disconnected
}

// ── Full lifecycle ────────────────────────────────────────────────────────────

/// C15i: full reconnect lifecycle.
///
/// `new → begin_connect → finish_sql → reconnect_sql → finish_reconnect_sql → disconnect`
///
/// This is 6 steps; the prover must chain:
/// - consistency through the two-field `Connecting` variant
/// - tag propagation from `SqlConnected` through `Reconnecting`
/// - final disconnect from `SqlConnected`
///
/// All Strings are parameters — Creusot generates `{false}` for literals.
#[requires(profile_name@.len() > 0)]
#[requires(backend_tag@.len() > 0)]
#[requires(tag1@.len() > 0)]
#[requires(tag2@.len() > 0)]
#[ensures(c15_consistent(&result))]
#[ensures(c15_is_disconnected(&result))]
pub fn c15_full_lifecycle(
    profile_name: String,
    backend_tag: String,
    tag1: String,
    tag2: String,
) -> C15ConnState {
    let s0 = c15_new(); // Disconnected
    let s1 = c15_begin_connect(s0, profile_name, backend_tag); // Connecting
    let s2 = c15_finish_sql(s1, tag1); // SqlConnected
    let s3 = c15_reconnect_sql(s2); // Reconnecting
    let s4 = c15_finish_reconnect_sql(s3, tag2); // SqlConnected
    c15_disconnect(s4) // Disconnected
}
