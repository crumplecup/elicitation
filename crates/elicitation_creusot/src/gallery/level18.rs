//! Gallery level C18: full `ArchiveConnectionState` replica.
//!
//! **Hypothesis**: All patterns proved individually in C15–C17 can be combined
//! into a single state machine that is structurally identical to the production
//! `ArchiveConnectionState`, and Why3 can close all proof obligations.
//!
//! This level is the direct proof-of-concept for annotating the real archive
//! VSM.  The only simplification is that `DatabaseDescriptor` and `BackendKind`
//! are gallery-local types (`C18Descriptor`, `C18BackendKind`) instead of
//! imports from `elicit_server`.
//!
//! ## Types (mirrors production)
//!
//! ```text
//! C18BackendKind { Sql, Kv }
//! C18Descriptor  { name: String, port: i32 }
//! ```
//!
//! ## State enum (mirrors ArchiveConnectionState exactly)
//!
//! ```text
//! Disconnected
//! Connecting    { profile_name: String, backend: C18BackendKind }
//! SqlConnected  { desc: C18Descriptor }
//! KvConnected   { path: String }
//! Reconnecting  { desc: C18Descriptor }
//! ConnectionError { message: String }
//! ```
//!
//! ## Consistency invariant
//!
//! ```text
//! c18_consistent(s) ≡
//!   match s {
//!     Disconnected              → true
//!     Connecting { name, .. }   → name non-empty
//!     SqlConnected { desc }     → c18_desc_consistent(desc)
//!     KvConnected  { path }     → path non-empty
//!     Reconnecting { desc }     → c18_desc_consistent(desc)
//!     ConnectionError { msg }   → msg non-empty
//!   }
//! ```
//!
//! ## Routing (from C17)
//!
//! - `c18_begin_sql` → ensures `c18_backend_is_sql`
//! - `c18_begin_kv`  → ensures `c18_backend_is_kv`
//! - `c18_finish_sql` requires `c18_backend_is_sql`
//! - `c18_finish_kv`  requires `c18_backend_is_kv`
//!
//! ## Reconnect (from C16)
//!
//! `c18_reconnect_sql` moves `C18Descriptor` from `SqlConnected` to `Reconnecting`.
//!
//! ## Experiment table
//!
//! | ID    | What                                                        | Expected |
//! |-------|-------------------------------------------------------------|----------|
//! | C18a  | `c18_desc_consistent` + `c18_consistent` composite          | ✓        |
//! | C18b  | `c18_new` → Disconnected                                    | ✓        |
//! | C18c  | `c18_begin_sql` + `c18_begin_kv` with routing postcondition | ✓        |
//! | C18d  | `c18_finish_sql` + `c18_finish_kv` with routing precondition| ✓        |
//! | C18e  | `c18_connection_error`: Connecting → ConnectionError        | ✓        |
//! | C18f  | `c18_reconnect_sql`: C18Descriptor move SqlConnected→Reconnecting | ✓  |
//! | C18g  | `c18_finish_reconnect_sql`: Reconnecting → SqlConnected     | ✓        |
//! | C18h  | `c18_disconnect`: 4-variant guard                           | ✓        |
//! | C18i  | SQL lifecycle: new→begin_sql→finish_sql→disconnect          | ✓        |
//! | C18j  | KV lifecycle: new→begin_kv→finish_kv→disconnect             | ✓        |
//! | C18k  | Full lifecycle: SQL connect, error, reconnect, disconnect   | ✓        |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

// ── Auxiliary types ───────────────────────────────────────────────────────────

/// Backend technology — mirrors `BackendKind`.
pub enum C18BackendKind {
    /// Relational SQL backend.
    Sql,
    /// Embedded key-value store.
    Kv,
}

/// Connection descriptor — mirrors `DatabaseDescriptor`.
pub struct C18Descriptor {
    /// Human-readable connection name (non-empty).
    pub name: String,
    /// Port number (positive).
    pub port: i32,
}

/// C18a (part 1): consistency predicate over the descriptor.
#[logic]
pub fn c18_desc_consistent(d: &C18Descriptor) -> bool {
    pearlite! { d.name@.len() > 0 && d.port@ > 0 }
}

// ── State enum ────────────────────────────────────────────────────────────────

/// Full archive connection state — structurally identical to
/// `ArchiveConnectionState`.
pub enum C18ConnState {
    /// No active backend connection.
    Disconnected,
    /// A connection attempt is in flight.
    Connecting {
        /// Profile name (non-empty).
        profile_name: String,
        /// Which backend technology is targeted.
        backend: C18BackendKind,
    },
    /// Connected to a SQL backend.
    SqlConnected {
        /// Connection descriptor (consistent).
        desc: C18Descriptor,
    },
    /// Connected to a key-value store.
    KvConnected {
        /// Filesystem path (non-empty).
        path: String,
    },
    /// A reconnect attempt is in flight; carries the previous descriptor.
    Reconnecting {
        /// Previous connection descriptor (consistent).
        desc: C18Descriptor,
    },
    /// The last connection attempt failed.
    ConnectionError {
        /// Human-readable error message (non-empty).
        message: String,
    },
}

// ── Logic predicates ──────────────────────────────────────────────────────────

/// C18a (part 2): composite consistency invariant.
#[logic]
pub fn c18_consistent(s: &C18ConnState) -> bool {
    pearlite! {
        match s {
            C18ConnState::Disconnected                        => true,
            C18ConnState::Connecting { profile_name, .. }     => profile_name@.len() > 0,
            C18ConnState::SqlConnected { desc }                => c18_desc_consistent(desc),
            C18ConnState::KvConnected  { path }                => path@.len() > 0,
            C18ConnState::Reconnecting { desc }                => c18_desc_consistent(desc),
            C18ConnState::ConnectionError { message }          => message@.len() > 0,
        }
    }
}

#[logic]
pub fn c18_is_disconnected(s: &C18ConnState) -> bool {
    pearlite! { match s { C18ConnState::Disconnected => true, _ => false } }
}

#[logic]
pub fn c18_is_connecting(s: &C18ConnState) -> bool {
    pearlite! { match s { C18ConnState::Connecting { .. } => true, _ => false } }
}

#[logic]
pub fn c18_is_sql_connected(s: &C18ConnState) -> bool {
    pearlite! { match s { C18ConnState::SqlConnected { .. } => true, _ => false } }
}

#[logic]
pub fn c18_is_reconnecting(s: &C18ConnState) -> bool {
    pearlite! { match s { C18ConnState::Reconnecting { .. } => true, _ => false } }
}

/// `Connecting` with a `Sql` backend (nested enum destructuring).
#[logic]
pub fn c18_backend_is_sql(s: &C18ConnState) -> bool {
    pearlite! {
        match s {
            C18ConnState::Connecting { backend: C18BackendKind::Sql, .. } => true,
            _ => false,
        }
    }
}

/// `Connecting` with a `Kv` backend.
#[logic]
pub fn c18_backend_is_kv(s: &C18ConnState) -> bool {
    pearlite! {
        match s {
            C18ConnState::Connecting { backend: C18BackendKind::Kv, .. } => true,
            _ => false,
        }
    }
}

/// Positive-form disconnect guard.
#[logic]
pub fn c18_is_disconnectable(s: &C18ConnState) -> bool {
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

/// C18b: fresh disconnected state.
#[ensures(c18_consistent(&result))]
#[ensures(c18_is_disconnected(&result))]
pub fn c18_new() -> C18ConnState {
    C18ConnState::Disconnected
}

/// C18c: begin a SQL connection attempt.
#[requires(c18_is_disconnected(&s))]
#[requires(profile_name@.len() > 0)]
#[ensures(c18_consistent(&result))]
#[ensures(c18_is_connecting(&result))]
#[ensures(c18_backend_is_sql(&result))]
pub fn c18_begin_sql(s: C18ConnState, profile_name: String) -> C18ConnState {
    let _ = s;
    C18ConnState::Connecting { profile_name, backend: C18BackendKind::Sql }
}

/// C18c: begin a KV connection attempt.
#[requires(c18_is_disconnected(&s))]
#[requires(profile_name@.len() > 0)]
#[ensures(c18_consistent(&result))]
#[ensures(c18_is_connecting(&result))]
#[ensures(c18_backend_is_kv(&result))]
pub fn c18_begin_kv(s: C18ConnState, profile_name: String) -> C18ConnState {
    let _ = s;
    C18ConnState::Connecting { profile_name, backend: C18BackendKind::Kv }
}

/// C18d: finish connecting to a SQL backend.
#[requires(c18_consistent(&s))]
#[requires(c18_backend_is_sql(&s))]
#[requires(c18_desc_consistent(&desc))]
#[ensures(c18_consistent(&result))]
#[ensures(c18_is_sql_connected(&result))]
pub fn c18_finish_sql(s: C18ConnState, desc: C18Descriptor) -> C18ConnState {
    let _ = s;
    C18ConnState::SqlConnected { desc }
}

/// C18d: finish connecting to a KV backend.
#[requires(c18_consistent(&s))]
#[requires(c18_backend_is_kv(&s))]
#[requires(path@.len() > 0)]
#[ensures(c18_consistent(&result))]
#[ensures(c18_is_disconnectable(&result))]
pub fn c18_finish_kv(s: C18ConnState, path: String) -> C18ConnState {
    let _ = s;
    C18ConnState::KvConnected { path }
}

/// C18e: record a connection error.
#[requires(c18_consistent(&s))]
#[requires(c18_is_connecting(&s))]
#[requires(message@.len() > 0)]
#[ensures(c18_consistent(&result))]
#[ensures(c18_is_disconnectable(&result))]
pub fn c18_connection_error(s: C18ConnState, message: String) -> C18ConnState {
    let _ = s;
    C18ConnState::ConnectionError { message }
}

/// C18f: begin reconnecting — moves `C18Descriptor` from `SqlConnected` to `Reconnecting`.
#[requires(c18_consistent(&s))]
#[requires(c18_is_sql_connected(&s))]
#[ensures(c18_consistent(&result))]
#[ensures(c18_is_reconnecting(&result))]
pub fn c18_reconnect_sql(s: C18ConnState) -> C18ConnState {
    match s {
        C18ConnState::SqlConnected { desc } => C18ConnState::Reconnecting { desc },
        _ => C18ConnState::Disconnected, // unreachable given precondition
    }
}

/// C18g: finish a reconnect to a SQL backend.
#[requires(c18_consistent(&s))]
#[requires(c18_is_reconnecting(&s))]
#[requires(c18_desc_consistent(&desc))]
#[ensures(c18_consistent(&result))]
#[ensures(c18_is_sql_connected(&result))]
pub fn c18_finish_reconnect_sql(s: C18ConnState, desc: C18Descriptor) -> C18ConnState {
    let _ = s;
    C18ConnState::SqlConnected { desc }
}

/// C18h: disconnect from any active or error state.
#[requires(c18_consistent(&s))]
#[requires(c18_is_disconnectable(&s))]
#[ensures(c18_consistent(&result))]
#[ensures(c18_is_disconnected(&result))]
pub fn c18_disconnect(s: C18ConnState) -> C18ConnState {
    let _ = s;
    C18ConnState::Disconnected
}

// ── Lifecycles ────────────────────────────────────────────────────────────────

/// C18i: minimal SQL lifecycle.
///
/// `new → begin_sql → finish_sql → disconnect`
#[requires(profile_name@.len() > 0)]
#[requires(c18_desc_consistent(&desc))]
#[ensures(c18_consistent(&result))]
#[ensures(c18_is_disconnected(&result))]
pub fn c18_lifecycle_sql(profile_name: String, desc: C18Descriptor) -> C18ConnState {
    let s0 = c18_new();
    let s1 = c18_begin_sql(s0, profile_name);
    let s2 = c18_finish_sql(s1, desc);
    c18_disconnect(s2)
}

/// C18j: minimal KV lifecycle.
///
/// `new → begin_kv → finish_kv → disconnect`
#[requires(profile_name@.len() > 0)]
#[requires(path@.len() > 0)]
#[ensures(c18_consistent(&result))]
#[ensures(c18_is_disconnected(&result))]
pub fn c18_lifecycle_kv(profile_name: String, path: String) -> C18ConnState {
    let s0 = c18_new();
    let s1 = c18_begin_kv(s0, profile_name);
    let s2 = c18_finish_kv(s1, path);
    c18_disconnect(s2)
}

/// C18k: full lifecycle — SQL connect, reconnect, then disconnect.
///
/// `new → begin_sql → finish_sql → reconnect_sql → finish_reconnect_sql → disconnect`
///
/// Exercises all combined patterns: routing guard, struct propagation,
/// and 6-step composition in a single proof.
#[requires(profile_name@.len() > 0)]
#[requires(c18_desc_consistent(&desc1))]
#[requires(c18_desc_consistent(&desc2))]
#[ensures(c18_consistent(&result))]
#[ensures(c18_is_disconnected(&result))]
pub fn c18_full_lifecycle(
    profile_name: String,
    desc1: C18Descriptor,
    desc2: C18Descriptor,
) -> C18ConnState {
    let s0 = c18_new();                              // Disconnected
    let s1 = c18_begin_sql(s0, profile_name);        // Connecting { Sql }
    let s2 = c18_finish_sql(s1, desc1);              // SqlConnected
    let s3 = c18_reconnect_sql(s2);                  // Reconnecting
    let s4 = c18_finish_reconnect_sql(s3, desc2);    // SqlConnected
    c18_disconnect(s4)                                // Disconnected
}
