//! Verified State Machine for the archive module.
//!
//! # Design
//!
//! `ArchiveMachine` is a [`VerifiedStateMachine`] whose states model the full
//! archive lifecycle from initial disconnection through active query/edit sessions.
//!
//! ## Invariant
//!
//! [`ArchiveConsistent`] asserts: *the archive is in a self-consistent state*.
//! Concretely, any state that references a database or KV descriptor must hold a
//! non-empty identifier, and any state that carries query or edit data must have
//! an active connection beneath it.
//!
//! ## Transitions
//!
//! ### Pure transitions (direct `VerifiedTransition`)
//!
//! Functions with the signature
//! `fn(ArchiveState, Established<ArchiveConsistent>) -> (ArchiveState, Established<ArchiveConsistent>)`
//! automatically satisfy [`VerifiedTransition<ArchiveMachine>`] via the blanket
//! [`FormalMethod`] impl.  Pure transitions need no extra parameters:
//! [`disconnect`], [`query_complete`], [`begin_edit`], [`commit_edits`], [`finish_export`].
//!
//! ### Parameterised transition constructors
//!
//! Some lifecycle advances require caller-supplied data (a `DatabaseDescriptor`,
//! a query string, etc.).  These are defined as ordinary functions with extra
//! parameters.  Callers wrap them in closures to produce a value that satisfies
//! `VerifiedTransition`:
//!
//! ```rust
//! use elicit_server::archive::vsm::{ArchiveMachine, ArchiveConsistent, finish_connect_sql};
//! use elicitation::{Established, VerifiedTransition};
//! # use elicit_server::archive::{BackendKind, DatabaseDescriptor};
//! # fn ex() {
//! let db = DatabaseDescriptor::new("postgres://localhost/mydb", "mydb", None);
//! let t = |s, p| finish_connect_sql(s, p, db.clone());
//! fn assert_vt<T: VerifiedTransition<ArchiveMachine>>(_: &T) {}
//! assert_vt(&t);
//! # }
//! ```
//!
//! Transitions are **pure**: they model lifecycle changes and carry the proof
//! token.  Actual I/O (connecting, executing SQL, etc.) happens in the archive
//! plugins; the results feed into these transitions to advance the machine.

use elicitation::{Elicit, Established, Prop, VerifiedStateMachine, contracts::ProvableFrom};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::archive::types::{BackendKind, DatabaseDescriptor, ExportFormat};

// ── ArchiveState ──────────────────────────────────────────────────────────────

/// The high-level lifecycle state of the archive module.
///
/// Every variant is [`ElicitComplete`]: serialisable, JSON-schema-able, and
/// proof-method-carrying — satisfying the `VerifiedStateMachine::State` bound.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub enum ArchiveState {
    /// No active backend connection.
    #[default]
    Disconnected,

    /// A connection attempt is in flight.  `profile_name` identifies the
    /// [`ConnectionProfile`](crate::archive::ConnectionProfile) being used.
    Connecting {
        /// Name of the connection profile being used.
        profile_name: String,
        /// Which backend technology is being targeted.
        backend: BackendKind,
    },

    /// Connected to a relational SQL backend; the nav tree may be populated.
    SqlConnected {
        /// Descriptor for the active database.
        db: DatabaseDescriptor,
    },

    /// Connected to an embedded key-value store.
    KvConnected {
        /// Filesystem path (or `redb://…` URL) of the open redb file.
        path: String,
    },

    /// The user is navigating schema objects in the nav tree.
    Browsing {
        /// Descriptor for the active database.
        db: DatabaseDescriptor,
        /// Currently selected schema name, if any.
        selected_schema: Option<String>,
    },

    /// A SQL query is currently executing.
    RunningQuery {
        /// Descriptor for the active database.
        db: DatabaseDescriptor,
        /// The query text that was submitted.
        query: String,
    },

    /// A query has completed; results are available in the data grid.
    ViewingResults {
        /// Descriptor for the active database.
        db: DatabaseDescriptor,
    },

    /// The user is staging row edits in the data grid.
    EditingRows {
        /// Descriptor for the active database.
        db: DatabaseDescriptor,
    },

    /// A data export is in progress.
    Exporting {
        /// Descriptor for the active database.
        db: DatabaseDescriptor,
        /// Export format being used.
        format: ExportFormat,
    },
}

// ── ArchiveConsistent (invariant) ─────────────────────────────────────────────

/// Proposition: the archive is in a self-consistent state.
///
/// Proved by [`ArchiveConnectionCredential`] (obtained after a successful
/// connect call) or by the `Established::assert()` escape hatch when
/// bootstrapping from a known-good initial state.
#[derive(Prop)]
pub struct ArchiveConsistent;

/// Credential produced by a successful connection operation.
///
/// After `ArchiveDbBackend::connect` or `ArchiveKvBackend::open` succeeds, the
/// caller holds a `ArchiveConnectionCredential` and may call
/// `Established::prove(&credential)` to produce an `Established<ArchiveConsistent>`.
pub struct ArchiveConnectionCredential;

impl ProvableFrom<ArchiveConnectionCredential> for ArchiveConsistent {}

// ── ArchiveMachine ────────────────────────────────────────────────────────────

/// The verified state machine for the archive module.
///
/// Declare by `impl VerifiedStateMachine for ArchiveMachine` below.  All
/// transitions declared in this module that match the required function
/// signature automatically satisfy [`VerifiedTransition<ArchiveMachine>`].
pub struct ArchiveMachine;

impl VerifiedStateMachine for ArchiveMachine {
    type State = ArchiveState;
    type Invariant = ArchiveConsistent;
}

// ── Transitions ───────────────────────────────────────────────────────────────
//
// Each function's type signature IS its contract declaration.  The blanket
// `FormalMethod` impl makes every matching function a `VerifiedTransition`.

/// Begin establishing a SQL connection for `profile_name`.
///
/// Transitions from any inactive state to [`ArchiveState::Connecting`].
#[instrument(skip(proof))]
pub fn begin_connect_sql(
    _state: ArchiveState,
    proof: Established<ArchiveConsistent>,
    profile_name: String,
    backend: BackendKind,
) -> (ArchiveState, Established<ArchiveConsistent>) {
    (
        ArchiveState::Connecting {
            profile_name,
            backend,
        },
        proof,
    )
}

/// Finish establishing a SQL connection, advancing to [`ArchiveState::SqlConnected`].
#[instrument(skip(proof))]
pub fn finish_connect_sql(
    _state: ArchiveState,
    proof: Established<ArchiveConsistent>,
    db: DatabaseDescriptor,
) -> (ArchiveState, Established<ArchiveConsistent>) {
    (ArchiveState::SqlConnected { db }, proof)
}

/// Finish establishing a KV connection, advancing to [`ArchiveState::KvConnected`].
#[instrument(skip(proof))]
pub fn finish_connect_kv(
    _state: ArchiveState,
    proof: Established<ArchiveConsistent>,
    path: String,
) -> (ArchiveState, Established<ArchiveConsistent>) {
    (ArchiveState::KvConnected { path }, proof)
}

/// Disconnect from any active backend, returning to [`ArchiveState::Disconnected`].
#[instrument(skip(proof))]
pub fn disconnect(
    _state: ArchiveState,
    proof: Established<ArchiveConsistent>,
) -> (ArchiveState, Established<ArchiveConsistent>) {
    (ArchiveState::Disconnected, proof)
}

/// Begin browsing schema objects in the nav tree.
///
/// Transitions from [`ArchiveState::SqlConnected`] to [`ArchiveState::Browsing`].
#[instrument(skip(proof))]
pub fn begin_browse(
    state: ArchiveState,
    proof: Established<ArchiveConsistent>,
    selected_schema: Option<String>,
) -> (ArchiveState, Established<ArchiveConsistent>) {
    let db = match state {
        ArchiveState::SqlConnected { db } | ArchiveState::Browsing { db, .. } => db,
        ArchiveState::ViewingResults { db }
        | ArchiveState::EditingRows { db }
        | ArchiveState::Exporting { db, .. }
        | ArchiveState::RunningQuery { db, .. } => db,
        _ => return (state, proof),
    };
    (
        ArchiveState::Browsing {
            db,
            selected_schema,
        },
        proof,
    )
}

/// Submit a SQL query for execution.
///
/// Transitions from [`ArchiveState::Browsing`] (or `ViewingResults`) to
/// [`ArchiveState::RunningQuery`].
#[instrument(skip(proof))]
pub fn execute_query(
    state: ArchiveState,
    proof: Established<ArchiveConsistent>,
    query: String,
) -> (ArchiveState, Established<ArchiveConsistent>) {
    let db = match state {
        ArchiveState::Browsing { db, .. }
        | ArchiveState::ViewingResults { db }
        | ArchiveState::EditingRows { db } => db,
        _ => return (state, proof),
    };
    (ArchiveState::RunningQuery { db, query }, proof)
}

/// Mark query execution as complete; results are now available.
///
/// Transitions from [`ArchiveState::RunningQuery`] to [`ArchiveState::ViewingResults`].
#[instrument(skip(proof))]
pub fn query_complete(
    state: ArchiveState,
    proof: Established<ArchiveConsistent>,
) -> (ArchiveState, Established<ArchiveConsistent>) {
    let db = match state {
        ArchiveState::RunningQuery { db, .. } => db,
        _ => return (state, proof),
    };
    (ArchiveState::ViewingResults { db }, proof)
}

/// Begin a row-edit session in the data grid.
///
/// Transitions from [`ArchiveState::ViewingResults`] to [`ArchiveState::EditingRows`].
#[instrument(skip(proof))]
pub fn begin_edit(
    state: ArchiveState,
    proof: Established<ArchiveConsistent>,
) -> (ArchiveState, Established<ArchiveConsistent>) {
    let db = match state {
        ArchiveState::ViewingResults { db } => db,
        _ => return (state, proof),
    };
    (ArchiveState::EditingRows { db }, proof)
}

/// Commit staged row edits and return to results view.
///
/// Transitions from [`ArchiveState::EditingRows`] back to [`ArchiveState::ViewingResults`].
#[instrument(skip(proof))]
pub fn commit_edits(
    state: ArchiveState,
    proof: Established<ArchiveConsistent>,
) -> (ArchiveState, Established<ArchiveConsistent>) {
    let db = match state {
        ArchiveState::EditingRows { db } => db,
        _ => return (state, proof),
    };
    (ArchiveState::ViewingResults { db }, proof)
}

/// Begin a data export operation.
///
/// Transitions from [`ArchiveState::Browsing`] or `ViewingResults` to
/// [`ArchiveState::Exporting`].
#[instrument(skip(proof))]
pub fn begin_export(
    state: ArchiveState,
    proof: Established<ArchiveConsistent>,
    format: ExportFormat,
) -> (ArchiveState, Established<ArchiveConsistent>) {
    let db = match state {
        ArchiveState::Browsing { db, .. } | ArchiveState::ViewingResults { db } => db,
        _ => return (state, proof),
    };
    (ArchiveState::Exporting { db, format }, proof)
}

/// Mark the export as complete and return to browsing.
///
/// Transitions from [`ArchiveState::Exporting`] back to [`ArchiveState::Browsing`].
#[instrument(skip(proof))]
pub fn finish_export(
    state: ArchiveState,
    proof: Established<ArchiveConsistent>,
) -> (ArchiveState, Established<ArchiveConsistent>) {
    let db = match state {
        ArchiveState::Exporting { db, .. } => db,
        _ => return (state, proof),
    };
    (
        ArchiveState::Browsing {
            db,
            selected_schema: None,
        },
        proof,
    )
}
