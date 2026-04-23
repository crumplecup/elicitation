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

use elicitation::{Elicit, Established, Prop, VerifiedStateMachine, contracts::ProvableFrom};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::archive::types::{BackendKind, DatabaseDescriptor};

// ── ArchiveConnectionState ────────────────────────────────────────────────────

/// Lifecycle state of the archive backend connection.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
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
#[derive(Prop)]
pub struct ArchiveConnectionConsistent;

/// Credential produced by a successful connection operation.
pub struct ArchiveConnectionCredential;

impl ProvableFrom<ArchiveConnectionCredential> for ArchiveConnectionConsistent {}

// ── ArchiveConnectionMachine ──────────────────────────────────────────────────

/// Verified state machine for the archive connection lifecycle.
pub struct ArchiveConnectionMachine;

impl VerifiedStateMachine for ArchiveConnectionMachine {
    type State = ArchiveConnectionState;
    type Invariant = ArchiveConnectionConsistent;
}

// ── Transitions ───────────────────────────────────────────────────────────────

/// Begin establishing a SQL connection for `profile_name`.
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
