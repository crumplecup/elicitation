//! Gallery level C16: nested struct field in enum variant.
//!
//! **Hypothesis**: Pearlite can express a consistency invariant that descends
//! into a **nested struct** embedded in an enum variant, and Why3 can
//! propagate field-level facts through a `match` that moves the struct from
//! one variant to another.
//!
//! This directly models `ArchiveConnectionState.SqlConnected { db: DatabaseDescriptor }`
//! and `Reconnecting { db: DatabaseDescriptor }`: the real production types use
//! a nested struct in two variants, and the `reconnect` transition moves the
//! struct field intact.
//!
//! ## New types
//!
//! ```text
//! C16Descriptor { name: String, port: i32 }
//! ```
//!
//! Consistency: `name@.len() > 0 && port@ > 0`
//!
//! ## State enum
//!
//! Same 6-variant shape as C15, but `SqlConnected` and `Reconnecting` carry a
//! `C16Descriptor` instead of a bare String:
//!
//! ```text
//! Disconnected
//! Connecting { profile_name: String, backend_tag: String }   (same as C15)
//! SqlConnected { desc: C16Descriptor }                       (struct field)
//! KvConnected  { path: String }                              (unchanged)
//! Reconnecting { desc: C16Descriptor }                       (struct field)
//! ConnectionError { message: String }                        (unchanged)
//! ```
//!
//! ## Consistency invariant
//!
//! ```text
//! c16_consistent(s) ≡
//!   match s {
//!     Disconnected                         → true
//!     Connecting { profile_name, backend_tag } → both non-empty
//!     SqlConnected { desc }                → c16_desc_consistent(desc)
//!     KvConnected  { path }                → path non-empty
//!     Reconnecting { desc }                → c16_desc_consistent(desc)
//!     ConnectionError { message }          → message non-empty
//!   }
//! ```
//!
//! ## Key new patterns
//!
//! 1. `c16_desc_consistent` — a `#[logic]` predicate over a plain struct; called
//!    from within the match arms of `c16_consistent`.
//! 2. Nested struct propagation: `reconnect_sql` moves `C16Descriptor` from
//!    `SqlConnected { desc }` → `Reconnecting { desc }`.  Why3 must transfer
//!    `c16_desc_consistent(desc)` across the boundary.
//! 3. `c16_begin_sql` — constructor that requires `c16_desc_consistent` on the
//!    incoming descriptor, returning `SqlConnected`.
//!
//! ## Experiment table
//!
//! | ID    | What                                                      | Expected |
//! |-------|-----------------------------------------------------------|----------|
//! | C16a  | `c16_desc_consistent` over plain struct                   | ✓        |
//! | C16b  | Constructor: `c16_new` → Disconnected                     | ✓        |
//! | C16c  | `begin_connect`: Connecting with two-field consistency    | ✓        |
//! | C16d  | `begin_sql`: directly construct `SqlConnected { desc }`  | ✓        |
//! | C16e  | `finish_sql`: Connecting → SqlConnected (desc parameter)  | ✓        |
//! | C16f  | `reconnect_sql`: `SqlConnected.desc` → `Reconnecting.desc`| ✓        |
//! | C16g  | `finish_reconnect_sql`: Reconnecting → SqlConnected       | ✓        |
//! | C16h  | `disconnect`: 4-variant guard                             | ✓        |
//! | C16i  | Full lifecycle: new→begin→finish_sql→reconnect→finish→disco| ✓        |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

// ── Nested struct ─────────────────────────────────────────────────────────────

/// Simplified analogue of `DatabaseDescriptor`.
///
/// Both fields must be positive for the descriptor to be consistent.
pub struct C16Descriptor {
    /// Human-readable connection name (non-empty).
    pub name: String,
    /// Port number (positive).
    pub port: i32,
}

/// Consistency predicate for `C16Descriptor`.
///
/// Called from within `c16_consistent` match arms — tested as a same-module
/// `#[logic]` call (transparent in Why3, unlike cross-module calls).
#[logic]
pub fn c16_desc_consistent(d: &C16Descriptor) -> bool {
    pearlite! { d.name@.len() > 0 && d.port@ > 0 }
}

// ── State enum ────────────────────────────────────────────────────────────────

/// Six-variant connection state with nested struct in two variants.
pub enum C16ConnState {
    /// No active backend connection.
    Disconnected,
    /// A connection attempt is in flight.
    Connecting {
        /// Profile name (non-empty).
        profile_name: String,
        /// Backend technology tag (non-empty).
        backend_tag: String,
    },
    /// Connected to a SQL backend; carries a full descriptor.
    SqlConnected {
        /// Connection descriptor (must be consistent).
        desc: C16Descriptor,
    },
    /// Connected to a key-value store.
    KvConnected {
        /// Filesystem path (non-empty).
        path: String,
    },
    /// A reconnect attempt is in flight; carries the previous descriptor.
    Reconnecting {
        /// Previous connection descriptor (must be consistent).
        desc: C16Descriptor,
    },
    /// The last connection attempt failed.
    ConnectionError {
        /// Human-readable error message (non-empty).
        message: String,
    },
}

// ── Logic predicates ──────────────────────────────────────────────────────────

/// Composite consistency invariant.
///
/// For variants carrying `C16Descriptor`, delegates to `c16_desc_consistent`.
/// Since both predicates live in the same module, Why3 can unfold the call
/// and discharge field-level goals without cross-module opacity.
#[logic]
pub fn c16_consistent(s: &C16ConnState) -> bool {
    pearlite! {
        match s {
            C16ConnState::Disconnected                              => true,
            C16ConnState::Connecting { profile_name, backend_tag } =>
                profile_name@.len() > 0 && backend_tag@.len() > 0,
            C16ConnState::SqlConnected { desc }                     => c16_desc_consistent(desc),
            C16ConnState::KvConnected  { path }                     => path@.len() > 0,
            C16ConnState::Reconnecting { desc }                     => c16_desc_consistent(desc),
            C16ConnState::ConnectionError { message }               => message@.len() > 0,
        }
    }
}

#[logic]
pub fn c16_is_disconnected(s: &C16ConnState) -> bool {
    pearlite! { match s { C16ConnState::Disconnected => true, _ => false } }
}

#[logic]
pub fn c16_is_connecting(s: &C16ConnState) -> bool {
    pearlite! { match s { C16ConnState::Connecting { .. } => true, _ => false } }
}

#[logic]
pub fn c16_is_sql_connected(s: &C16ConnState) -> bool {
    pearlite! { match s { C16ConnState::SqlConnected { .. } => true, _ => false } }
}

#[logic]
pub fn c16_is_reconnecting(s: &C16ConnState) -> bool {
    pearlite! { match s { C16ConnState::Reconnecting { .. } => true, _ => false } }
}

/// Positive-form guard for disconnect: covers the four exit-able states.
#[logic]
pub fn c16_is_disconnectable(s: &C16ConnState) -> bool {
    pearlite! {
        match s {
            C16ConnState::SqlConnected { .. }    => true,
            C16ConnState::KvConnected  { .. }    => true,
            C16ConnState::Reconnecting { .. }    => true,
            C16ConnState::ConnectionError { .. } => true,
            _                                    => false,
        }
    }
}

// ── Transitions ───────────────────────────────────────────────────────────────

/// C16b: fresh disconnected state.
#[ensures(c16_consistent(&result))]
#[ensures(c16_is_disconnected(&result))]
pub fn c16_new() -> C16ConnState {
    C16ConnState::Disconnected
}

/// C16c: begin a connection attempt.
///
/// `Disconnected` → `Connecting { profile_name, backend_tag }`
#[requires(c16_consistent(&s))]
#[requires(c16_is_disconnected(&s))]
#[requires(profile_name@.len() > 0)]
#[requires(backend_tag@.len() > 0)]
#[ensures(c16_consistent(&result))]
#[ensures(c16_is_connecting(&result))]
pub fn c16_begin_connect(s: C16ConnState, profile_name: String, backend_tag: String) -> C16ConnState {
    let _ = s;
    C16ConnState::Connecting { profile_name, backend_tag }
}

/// C16d: directly construct a `SqlConnected` state from a descriptor.
///
/// Used in tests; requires `c16_desc_consistent` on the incoming descriptor.
#[requires(c16_desc_consistent(&desc))]
#[ensures(c16_consistent(&result))]
#[ensures(c16_is_sql_connected(&result))]
pub fn c16_begin_sql(desc: C16Descriptor) -> C16ConnState {
    C16ConnState::SqlConnected { desc }
}

/// C16e: finish connecting to a SQL backend.
///
/// `Connecting` → `SqlConnected { desc }`
///
/// The caller supplies a `C16Descriptor`; it must already be consistent.
#[requires(c16_consistent(&s))]
#[requires(c16_is_connecting(&s))]
#[requires(c16_desc_consistent(&desc))]
#[ensures(c16_consistent(&result))]
#[ensures(c16_is_sql_connected(&result))]
pub fn c16_finish_sql(s: C16ConnState, desc: C16Descriptor) -> C16ConnState {
    let _ = s;
    C16ConnState::SqlConnected { desc }
}

/// C16f: begin reconnecting from an established SQL connection.
///
/// `SqlConnected { desc }` → `Reconnecting { desc }`
///
/// Key test: `C16Descriptor` is moved from `SqlConnected` into `Reconnecting`.
/// Why3 must see that `c16_desc_consistent(desc)` holds for the source (from
/// `c16_consistent` on `SqlConnected { desc }`), and transfer it to the result.
#[requires(c16_consistent(&s))]
#[requires(c16_is_sql_connected(&s))]
#[ensures(c16_consistent(&result))]
#[ensures(c16_is_reconnecting(&result))]
pub fn c16_reconnect_sql(s: C16ConnState) -> C16ConnState {
    match s {
        C16ConnState::SqlConnected { desc } => C16ConnState::Reconnecting { desc },
        _ => C16ConnState::Disconnected, // unreachable given precondition
    }
}

/// C16g: finish a reconnect to a SQL backend.
///
/// `Reconnecting` → `SqlConnected { desc }`
#[requires(c16_consistent(&s))]
#[requires(c16_is_reconnecting(&s))]
#[requires(c16_desc_consistent(&desc))]
#[ensures(c16_consistent(&result))]
#[ensures(c16_is_sql_connected(&result))]
pub fn c16_finish_reconnect_sql(s: C16ConnState, desc: C16Descriptor) -> C16ConnState {
    let _ = s;
    C16ConnState::SqlConnected { desc }
}

/// C16h: disconnect from any active or error state.
///
/// `SqlConnected | KvConnected | Reconnecting | ConnectionError` → `Disconnected`
#[requires(c16_consistent(&s))]
#[requires(c16_is_disconnectable(&s))]
#[ensures(c16_consistent(&result))]
#[ensures(c16_is_disconnected(&result))]
pub fn c16_disconnect(s: C16ConnState) -> C16ConnState {
    let _ = s;
    C16ConnState::Disconnected
}

// ── Full lifecycle ────────────────────────────────────────────────────────────

/// C16i: full reconnect lifecycle with nested struct.
///
/// `new → begin_connect → finish_sql(desc1) → reconnect_sql → finish_reconnect_sql(desc2) → disconnect`
///
/// Checks that `C16Descriptor` consistency is maintained end-to-end, including
/// the reconnect move where `desc` propagates from `SqlConnected` to `Reconnecting`.
///
/// All Strings are parameters (Creusot generates `{false}` for literals).
/// `C16Descriptor` values are also parameters so Why3 can reason about them
/// without construction primitives.
#[requires(profile_name@.len() > 0)]
#[requires(backend_tag@.len() > 0)]
#[requires(c16_desc_consistent(&desc1))]
#[requires(c16_desc_consistent(&desc2))]
#[ensures(c16_consistent(&result))]
#[ensures(c16_is_disconnected(&result))]
pub fn c16_full_lifecycle(
    profile_name: String,
    backend_tag: String,
    desc1: C16Descriptor,
    desc2: C16Descriptor,
) -> C16ConnState {
    let s0 = c16_new();                                       // Disconnected
    let s1 = c16_begin_connect(s0, profile_name, backend_tag); // Connecting
    let s2 = c16_finish_sql(s1, desc1);                        // SqlConnected
    let s3 = c16_reconnect_sql(s2);                            // Reconnecting
    let s4 = c16_finish_reconnect_sql(s3, desc2);              // SqlConnected
    c16_disconnect(s4)                                          // Disconnected
}
