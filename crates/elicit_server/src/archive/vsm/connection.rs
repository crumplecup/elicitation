//! Verified State Machine for the archive database connection lifecycle.
//!
//! [`ArchiveConnectionMachine`] tracks whether the archive has an active
//! backend connection, what kind it is, and proof that it is consistent.
//!
//! ## States
//!
//! ```text
//! Disconnected ──begin_connect──► Connecting
//!      ▲                              │
//!      │           ┌─────────────────┤ finish_connect_sql
//!      │           │                 │ finish_connect_kv
//!      │           ▼                 ▼
//!      │       SqlConnected     KvConnected
//!      │           │                 │
//!      │           └────reconnect────►Reconnecting
//!      │                             │
//!      │               ┌─────────────┤ finish_connect_sql / _kv
//!      │               │             │ connection_error
//!      └───disconnect───┘   ConnectionError
//! ```

use elicit_db::ConnectionEstablished;
use elicitation::{
    Elicit, Established, KaniCompose, KaniVariantState, Prop, VerifiedStateMachine,
    contracts::ProvableFrom, formal_method,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::types::{BackendKind, DatabaseDescriptor};

// ── ArchiveConnectionState ────────────────────────────────────────────────────

/// Lifecycle state of the archive backend connection.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniCompose,
    KaniVariantState,
)]
pub enum ArchiveConnectionState {
    /// No active backend connection.
    #[default]
    Disconnected,
    /// A connection attempt is in flight.
    Connecting {
        /// Name of the connection profile being used.
        profile_name: String,
        /// Which backend technology is being targeted.
        backend: BackendKind,
    },
    /// Connected to a relational SQL backend.
    SqlConnected {
        /// Descriptor for the active database.
        db: DatabaseDescriptor,
    },
    /// Connected to an embedded key-value store.
    KvConnected {
        /// Filesystem path (or `redb://…` URL) of the open redb file.
        path: String,
    },
    /// A reconnect attempt is in flight after a dropped connection.
    Reconnecting {
        /// Previous connection descriptor.
        db: DatabaseDescriptor,
    },
    /// The last connection attempt failed.
    ConnectionError {
        /// Human-readable error message.
        message: String,
    },
}

// ── ArchiveConnectionConsistent (invariant) ───────────────────────────────────

/// Proposition: the archive connection state is self-consistent.
///
/// Wired to [`ConnectionEstablished`] from `elicit_db`: the formal-method
/// harnesses will call `Established::prove(&kani::any::<ConnectionEstablished>())`
/// instead of the axiom `Established::assert()`, keeping CBMC's state space bounded.
#[derive(Prop)]
#[prop(credential = ConnectionEstablished, creusot_invariant_fn = "archive_connection_consistent", kani_invariant_fn = "archive_connection_consistent", verus_invariant_fn = "archive_connection_consistent", verus_inv_body = "true", creusot_inv_body = "true")]
pub struct ArchiveConnectionConsistent;

impl ProvableFrom<ConnectionEstablished> for ArchiveConnectionConsistent {}

/// Structural invariant predicate for [`ArchiveConnectionState`].
///
/// Runtime-evaluable form of [`ArchiveConnectionConsistent`] used by Kani
/// `#[kani::requires]` / `#[kani::ensures]` in contracted wrapper functions.
///
/// All `ArchiveConnectionState` variants are well-formed by construction —
/// there are no cross-field constraints to enforce.  Strings (profile names,
/// paths, error messages) are accepted as given by callers; the VSM does not
/// validate them.  This invariant is intentionally `true`.
pub fn archive_connection_consistent(_state: &ArchiveConnectionState) -> bool {
    true
}

/// Bridge `kani::Arbitrary` to `KaniCompose::kani_depth0()` so that
/// `stub_verified` can generate bounded symbolic return values.
#[cfg(kani)]
impl kani::Arbitrary for ArchiveConnectionState {
    fn any() -> Self {
        use elicitation::KaniCompose;
        ArchiveConnectionState::kani_depth0()
    }
}

/// Verified state machine for the archive connection lifecycle.
#[derive(VerifiedStateMachine)]
#[vsm(transitions = [
    begin_connect_sql, begin_connect_kv,
    finish_connect_sql, finish_connect_kv,
    disconnect, reconnect, connection_error,
])]
pub struct ArchiveConnectionMachine;

// ── Transitions ───────────────────────────────────────────────────────────────

/// Begin establishing a SQL connection for `profile_name`.
#[formal_method(contracts = [ArchiveConnectionConsistent])]
#[instrument(skip(proof))]
pub fn begin_connect_sql(
    _state: ArchiveConnectionState,
    proof: Established<ArchiveConnectionConsistent>,
    profile_name: String,
    backend: BackendKind,
) -> (
    ArchiveConnectionState,
    Established<ArchiveConnectionConsistent>,
) {
    (
        ArchiveConnectionState::Connecting {
            profile_name,
            backend,
        },
        proof,
    )
}

/// Begin establishing a KV connection.
#[formal_method(contracts = [ArchiveConnectionConsistent])]
#[instrument(skip(proof))]
pub fn begin_connect_kv(
    _state: ArchiveConnectionState,
    proof: Established<ArchiveConnectionConsistent>,
    profile_name: String,
) -> (
    ArchiveConnectionState,
    Established<ArchiveConnectionConsistent>,
) {
    (
        ArchiveConnectionState::Connecting {
            profile_name,
            backend: BackendKind::Redb,
        },
        proof,
    )
}

/// Finish establishing a SQL connection.
#[formal_method(contracts = [ArchiveConnectionConsistent])]
#[instrument(skip(proof))]
pub fn finish_connect_sql(
    _state: ArchiveConnectionState,
    proof: Established<ArchiveConnectionConsistent>,
    db: DatabaseDescriptor,
) -> (
    ArchiveConnectionState,
    Established<ArchiveConnectionConsistent>,
) {
    (ArchiveConnectionState::SqlConnected { db }, proof)
}

/// Finish establishing a KV connection.
#[formal_method(contracts = [ArchiveConnectionConsistent])]
#[instrument(skip(proof))]
pub fn finish_connect_kv(
    _state: ArchiveConnectionState,
    proof: Established<ArchiveConnectionConsistent>,
    path: String,
) -> (
    ArchiveConnectionState,
    Established<ArchiveConnectionConsistent>,
) {
    (ArchiveConnectionState::KvConnected { path }, proof)
}

/// Disconnect from any active backend.
#[formal_method(contracts = [ArchiveConnectionConsistent])]
#[instrument(skip(proof))]
pub fn disconnect(
    _state: ArchiveConnectionState,
    proof: Established<ArchiveConnectionConsistent>,
) -> (
    ArchiveConnectionState,
    Established<ArchiveConnectionConsistent>,
) {
    (ArchiveConnectionState::Disconnected, proof)
}

/// Begin a reconnect attempt after a dropped SQL connection.
#[formal_method(contracts = [ArchiveConnectionConsistent])]
#[instrument(skip(proof))]
pub fn reconnect(
    state: ArchiveConnectionState,
    proof: Established<ArchiveConnectionConsistent>,
) -> (
    ArchiveConnectionState,
    Established<ArchiveConnectionConsistent>,
) {
    let db = match state {
        ArchiveConnectionState::SqlConnected { db }
        | ArchiveConnectionState::Reconnecting { db } => db,
        other => return (other, proof),
    };
    (ArchiveConnectionState::Reconnecting { db }, proof)
}

/// Record a connection error.
#[formal_method(contracts = [ArchiveConnectionConsistent])]
#[instrument(skip(proof))]
pub fn connection_error(
    _state: ArchiveConnectionState,
    proof: Established<ArchiveConnectionConsistent>,
    message: String,
) -> (
    ArchiveConnectionState,
    Established<ArchiveConnectionConsistent>,
) {
    (ArchiveConnectionState::ConnectionError { message }, proof)
}
