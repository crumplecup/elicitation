//! `DbIsolationFactory` — typed isolation-level transaction factory (Role 1a).
//!
//! Provides proof-returning `begin_*` variants for each SQL isolation level.
//! Each method mints a specific `Established<IsolationLevelP>` token that can
//! be composed into aggregate proofs via `contracts::proof_composition`.
//!
//! # Why a separate trait?
//!
//! [`crate::DbTransactor`] provides a single `begin(isolation: IsolationLevel)`
//! method for runtime-dispatched transaction starts.  `DbIsolationFactory` adds
//! statically-typed counterparts whose return types carry the isolation proof
//! at compile time.  This allows the type system to enforce the ProvableFrom
//! chain:
//!
//! ```text
//! begin_serializable() → Established<SerializableIsolation>
//!     ↓ ProvableFrom
//! SerializablePhenomenaEvidence → Established<AcidCompliant>
//! ```
//!
//! Source: ISO/IEC 9075-2 §17.1 — `<start transaction statement>`;
//!         ANSI X3.135-1992 §4.28 — Isolation levels;
//!         Berenson et al. "A Critique of ANSI SQL Isolation Levels" (1995).

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{
    DbResult, IsolationLevel, Open, ReadCommittedIsolation, ReadUncommittedIsolation,
    RepeatableReadIsolation, SerializableIsolation, SessionIsolationLevelSet, TransactionHandle,
    TransactionIsolationLevelSet, TransactionReadOnly, TransactionReadWrite, TxMarker,
};

// ── Role 1a: typed isolation factory ─────────────────────────────────────────

/// Proof-returning `begin_*` variants for each SQL isolation level.
///
/// Each method is the authority that a transaction was started at the declared
/// isolation level.  The returned `Established<_>` token can be passed to
/// Role 1b section factories in `contracts::proof_composition` to build
/// aggregate ACID or phenomena-prevention proofs.
///
/// Source: ISO/IEC 9075-2 §17.1; Berenson et al. (1995).
pub trait DbIsolationFactory: Send + Sync {
    /// Begin a `READ COMMITTED` transaction and mint the isolation proof.
    ///
    /// `ReadCommittedIsolation` guarantees prevention of dirty reads (P1)
    /// and dirty writes (P0).
    ///
    /// Source: ISO/IEC 9075-2 §17.1; ANSI X3.135-1992 §4.28
    fn begin_read_committed(
        &self,
    ) -> BoxFuture<
        '_,
        DbResult<(
            TransactionHandle,
            TxMarker<Open>,
            Established<ReadCommittedIsolation>,
        )>,
    >;

    /// Begin a `REPEATABLE READ` transaction and mint the isolation proof.
    ///
    /// `RepeatableReadIsolation` additionally prevents non-repeatable reads (P2)
    /// and lost updates (P4).
    ///
    /// Source: ISO/IEC 9075-2 §17.1; Berenson et al. (1995) §3
    fn begin_repeatable_read(
        &self,
    ) -> BoxFuture<
        '_,
        DbResult<(
            TransactionHandle,
            TxMarker<Open>,
            Established<RepeatableReadIsolation>,
        )>,
    >;

    /// Begin a `SERIALIZABLE` transaction and mint the isolation proof.
    ///
    /// The strongest level: prevents dirty writes, dirty reads, non-repeatable
    /// reads, phantom reads, write skew, and serialization anomalies.
    ///
    /// Source: ISO/IEC 9075-2 §17.1; Berenson et al. (1995) §3
    fn begin_serializable(
        &self,
    ) -> BoxFuture<
        '_,
        DbResult<(
            TransactionHandle,
            TxMarker<Open>,
            Established<SerializableIsolation>,
        )>,
    >;

    /// Begin a `READ UNCOMMITTED` transaction and mint the isolation proof.
    ///
    /// Weakest level; prevents only dirty writes (P0).  In PostgreSQL,
    /// `READ UNCOMMITTED` behaves identically to `READ COMMITTED`.
    ///
    /// Source: ISO/IEC 9075-2 §17.1; ANSI X3.135-1992 §4.28
    fn begin_read_uncommitted(
        &self,
    ) -> BoxFuture<
        '_,
        DbResult<(
            TransactionHandle,
            TxMarker<Open>,
            Established<ReadUncommittedIsolation>,
        )>,
    >;

    /// Begin a `READ ONLY` transaction at the given isolation level.
    ///
    /// Source: ISO/IEC 9075-2 §17.1 — `SET TRANSACTION READ ONLY`
    fn begin_read_only(
        &self,
        isolation: IsolationLevel,
    ) -> BoxFuture<
        '_,
        DbResult<(
            TransactionHandle,
            TxMarker<Open>,
            Established<TransactionReadOnly>,
        )>,
    >;

    /// Begin a `READ WRITE` transaction at the given isolation level.
    ///
    /// Source: ISO/IEC 9075-2 §17.1 — `SET TRANSACTION READ WRITE`
    fn begin_read_write(
        &self,
        isolation: IsolationLevel,
    ) -> BoxFuture<
        '_,
        DbResult<(
            TransactionHandle,
            TxMarker<Open>,
            Established<TransactionReadWrite>,
        )>,
    >;

    /// Set the session-level default isolation level.
    ///
    /// Source: ISO/IEC 9075-2 §17.1 — `SET SESSION CHARACTERISTICS AS TRANSACTION`
    fn set_session_isolation(
        &self,
        level: IsolationLevel,
    ) -> BoxFuture<'_, DbResult<Established<SessionIsolationLevelSet>>>;

    /// Set the isolation level for the current open transaction.
    ///
    /// Must be called before any DML in the transaction.
    ///
    /// Source: ISO/IEC 9075-2 §17.1 — `SET TRANSACTION ISOLATION LEVEL`
    fn set_transaction_isolation(
        &self,
        handle: &TransactionHandle,
        level: IsolationLevel,
    ) -> BoxFuture<'_, DbResult<Established<TransactionIsolationLevelSet>>>;
}
