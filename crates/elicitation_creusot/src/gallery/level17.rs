//! Gallery level C17: nested enum field in variant + backend-kind routing.
//!
//! **Hypothesis**: Pearlite can match on a nested enum field within an enum
//! variant pattern (`C17ConnState::Connecting { backend: C17BackendKind::Sql, .. }`),
//! and Why3 can use the resulting predicate to gate which finish transition is
//! called — routing `finish_sql` vs `finish_kv` by the backend carried in
//! the `Connecting` variant.
//!
//! This models the real `ArchiveConnectionState.Connecting { backend: BackendKind }`
//! where `BackendKind` is itself an enum, and only `finish_connect_sql` should
//! fire after a `Sql` backend was selected.
//!
//! ## New types
//!
//! ```text
//! C17BackendKind { Sql, Kv }        — unit enum, models BackendKind
//! ```
//!
//! ## State enum
//!
//! ```text
//! Disconnected
//! Connecting { profile_name: String, backend: C17BackendKind }
//! SqlConnected { tag: String }
//! KvConnected  { path: String }
//! ConnectionError { message: String }
//! ```
//!
//! ## Key new predicates
//!
//! - `c17_backend_is_sql(s)` — matches `Connecting { backend: C17BackendKind::Sql, .. }`
//! - `c17_backend_is_kv(s)` — matches `Connecting { backend: C17BackendKind::Kv, .. }`
//!
//! These use nested enum destructuring inside a match arm in `#[logic]`.
//! The key test: does Pearlite compile and Why3 discharge goals involving
//! a predicate that matches on an enum *field* inside an outer enum variant?
//!
//! ## Routing
//!
//! - `c17_finish_sql` requires `c17_backend_is_sql(&s)`
//! - `c17_finish_kv`  requires `c17_backend_is_kv(&s)`
//! - `c17_begin_sql` (dedicated constructor) ensures `c17_backend_is_sql(&result)`
//! - `c17_begin_kv`  (dedicated constructor) ensures `c17_backend_is_kv(&result)`
//!
//! Lifecycle `c17_full_lifecycle_sql` calls `begin_sql → finish_sql`, and the
//! prover must chain: `begin_sql` ensures `c17_backend_is_sql` →
//! `finish_sql` precondition satisfied.
//!
//! ## Experiment table
//!
//! | ID    | What                                                     | Expected |
//! |-------|----------------------------------------------------------|----------|
//! | C17a  | `c17_backend_is_sql`: nested enum match in `#[logic]`    | ✓        |
//! | C17b  | `c17_new` → Disconnected                                 | ✓        |
//! | C17c  | `c17_begin_sql`: ensures `c17_backend_is_sql`            | ✓        |
//! | C17d  | `c17_begin_kv`: ensures `c17_backend_is_kv`              | ✓        |
//! | C17e  | `c17_finish_sql`: requires `c17_backend_is_sql`          | ✓        |
//! | C17f  | `c17_finish_kv`: requires `c17_backend_is_kv`            | ✓        |
//! | C17g  | `c17_disconnect`: from any connected/error state         | ✓        |
//! | C17h  | SQL lifecycle: begin_sql → finish_sql → disconnect       | ✓        |
//! | C17i  | KV lifecycle: begin_kv → finish_kv → disconnect         | ✓        |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

// ── Backend kind ──────────────────────────────────────────────────────────────

/// Backend technology — unit enum modelling `BackendKind`.
pub enum C17BackendKind {
    /// Relational SQL backend.
    Sql,
    /// Embedded key-value store.
    Kv,
}

// ── State enum ────────────────────────────────────────────────────────────────

/// Five-variant connection state with an enum field in `Connecting`.
pub enum C17ConnState {
    /// No active backend connection.
    Disconnected,
    /// A connection attempt is in flight; backend type is embedded.
    Connecting {
        /// Profile name (non-empty).
        profile_name: String,
        /// Which backend technology is targeted.
        backend: C17BackendKind,
    },
    /// Connected to a SQL backend.
    SqlConnected {
        /// Connection tag (non-empty).
        tag: String,
    },
    /// Connected to a key-value store.
    KvConnected {
        /// Filesystem path (non-empty).
        path: String,
    },
    /// The last connection attempt failed.
    ConnectionError {
        /// Human-readable error message (non-empty).
        message: String,
    },
}

// ── Logic predicates ──────────────────────────────────────────────────────────

/// The state is self-consistent: all String payload fields are non-empty.
#[logic]
pub fn c17_consistent(s: &C17ConnState) -> bool {
    pearlite! {
        match s {
            C17ConnState::Disconnected                        => true,
            C17ConnState::Connecting { profile_name, .. }     => profile_name@.len() > 0,
            C17ConnState::SqlConnected { tag }                 => tag@.len() > 0,
            C17ConnState::KvConnected  { path }                => path@.len() > 0,
            C17ConnState::ConnectionError { message }          => message@.len() > 0,
        }
    }
}

#[logic]
pub fn c17_is_disconnected(s: &C17ConnState) -> bool {
    pearlite! { match s { C17ConnState::Disconnected => true, _ => false } }
}

#[logic]
pub fn c17_is_connecting(s: &C17ConnState) -> bool {
    pearlite! { match s { C17ConnState::Connecting { .. } => true, _ => false } }
}

#[logic]
pub fn c17_is_sql_connected(s: &C17ConnState) -> bool {
    pearlite! { match s { C17ConnState::SqlConnected { .. } => true, _ => false } }
}

#[logic]
pub fn c17_is_kv_connected(s: &C17ConnState) -> bool {
    pearlite! { match s { C17ConnState::KvConnected { .. } => true, _ => false } }
}

/// C17a: the state is `Connecting` with a `Sql` backend.
///
/// Key test: nested enum destructuring `Connecting { backend: C17BackendKind::Sql, .. }`
/// inside a `#[logic]` match expression.
#[logic]
pub fn c17_backend_is_sql(s: &C17ConnState) -> bool {
    pearlite! {
        match s {
            C17ConnState::Connecting { backend: C17BackendKind::Sql, .. } => true,
            _ => false,
        }
    }
}

/// The state is `Connecting` with a `Kv` backend.
#[logic]
pub fn c17_backend_is_kv(s: &C17ConnState) -> bool {
    pearlite! {
        match s {
            C17ConnState::Connecting { backend: C17BackendKind::Kv, .. } => true,
            _ => false,
        }
    }
}

/// Positive-form guard for disconnect.
#[logic]
pub fn c17_is_disconnectable(s: &C17ConnState) -> bool {
    pearlite! {
        match s {
            C17ConnState::SqlConnected { .. }    => true,
            C17ConnState::KvConnected  { .. }    => true,
            C17ConnState::ConnectionError { .. } => true,
            _                                    => false,
        }
    }
}

// ── Transitions ───────────────────────────────────────────────────────────────

/// C17b: fresh disconnected state.
#[ensures(c17_consistent(&result))]
#[ensures(c17_is_disconnected(&result))]
pub fn c17_new() -> C17ConnState {
    C17ConnState::Disconnected
}

/// C17c: begin a SQL connection attempt.
///
/// Creates `Connecting { backend: Sql }` and explicitly ensures
/// `c17_backend_is_sql` so downstream callers can discharge the routing guard.
#[requires(c17_is_disconnected(&s))]
#[requires(profile_name@.len() > 0)]
#[ensures(c17_consistent(&result))]
#[ensures(c17_is_connecting(&result))]
#[ensures(c17_backend_is_sql(&result))]
pub fn c17_begin_sql(s: C17ConnState, profile_name: String) -> C17ConnState {
    let _ = s;
    C17ConnState::Connecting { profile_name, backend: C17BackendKind::Sql }
}

/// C17d: begin a KV connection attempt.
///
/// Creates `Connecting { backend: Kv }` and ensures `c17_backend_is_kv`.
#[requires(c17_is_disconnected(&s))]
#[requires(profile_name@.len() > 0)]
#[ensures(c17_consistent(&result))]
#[ensures(c17_is_connecting(&result))]
#[ensures(c17_backend_is_kv(&result))]
pub fn c17_begin_kv(s: C17ConnState, profile_name: String) -> C17ConnState {
    let _ = s;
    C17ConnState::Connecting { profile_name, backend: C17BackendKind::Kv }
}

/// C17e: finish connecting to a SQL backend.
///
/// Requires `c17_backend_is_sql(&s)` — routing guard enforced by precondition.
/// If the backend was `Kv`, this transition is not applicable.
#[requires(c17_consistent(&s))]
#[requires(c17_backend_is_sql(&s))]
#[requires(tag@.len() > 0)]
#[ensures(c17_consistent(&result))]
#[ensures(c17_is_sql_connected(&result))]
pub fn c17_finish_sql(s: C17ConnState, tag: String) -> C17ConnState {
    let _ = s;
    C17ConnState::SqlConnected { tag }
}

/// C17f: finish connecting to a KV backend.
///
/// Requires `c17_backend_is_kv(&s)`.
#[requires(c17_consistent(&s))]
#[requires(c17_backend_is_kv(&s))]
#[requires(path@.len() > 0)]
#[ensures(c17_consistent(&result))]
#[ensures(c17_is_kv_connected(&result))]
pub fn c17_finish_kv(s: C17ConnState, path: String) -> C17ConnState {
    let _ = s;
    C17ConnState::KvConnected { path }
}

/// C17g: disconnect from any active or error state.
#[requires(c17_consistent(&s))]
#[requires(c17_is_disconnectable(&s))]
#[ensures(c17_consistent(&result))]
#[ensures(c17_is_disconnected(&result))]
pub fn c17_disconnect(s: C17ConnState) -> C17ConnState {
    let _ = s;
    C17ConnState::Disconnected
}

// ── Full lifecycles ───────────────────────────────────────────────────────────

/// C17h: SQL lifecycle — backend routing via `c17_backend_is_sql`.
///
/// `new → begin_sql → finish_sql → disconnect`
///
/// The prover chains: `begin_sql` ensures `c17_backend_is_sql` →
/// `finish_sql` routing precondition satisfied.
#[requires(profile_name@.len() > 0)]
#[requires(tag@.len() > 0)]
#[ensures(c17_consistent(&result))]
#[ensures(c17_is_disconnected(&result))]
pub fn c17_full_lifecycle_sql(profile_name: String, tag: String) -> C17ConnState {
    let s0 = c17_new();
    let s1 = c17_begin_sql(s0, profile_name); // Connecting { backend: Sql }
    let s2 = c17_finish_sql(s1, tag);          // SqlConnected
    c17_disconnect(s2)                          // Disconnected
}

/// C17i: KV lifecycle — backend routing via `c17_backend_is_kv`.
///
/// `new → begin_kv → finish_kv → disconnect`
#[requires(profile_name@.len() > 0)]
#[requires(path@.len() > 0)]
#[ensures(c17_consistent(&result))]
#[ensures(c17_is_disconnected(&result))]
pub fn c17_full_lifecycle_kv(profile_name: String, path: String) -> C17ConnState {
    let s0 = c17_new();
    let s1 = c17_begin_kv(s0, profile_name); // Connecting { backend: Kv }
    let s2 = c17_finish_kv(s1, path);          // KvConnected
    c17_disconnect(s2)                          // Disconnected
}
