//! ANSI isolation level propositions.
//!
//! Source: ANSI X3.135-1992 (SQL-92) §4.28 — Transaction isolation.
//! Extended with anomalies from Berenson et al. "A Critique of ANSI SQL
//! Isolation Levels" (1995) and PostgreSQL SSI documentation.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
            }
        };
    }

    // -- Isolation levels --

    /// Transaction ran at READ UNCOMMITTED isolation.
    ///
    /// Source: ANSI SQL-92 Table 2 — isolation level READ UNCOMMITTED
    pub struct ReadUncommittedIsolation;
    structural_prop!(ReadUncommittedIsolation, "ReadUncommittedIsolation");

    /// Transaction ran at READ COMMITTED isolation.
    ///
    /// Source: ANSI SQL-92 Table 2 — isolation level READ COMMITTED
    pub struct ReadCommittedIsolation;
    structural_prop!(ReadCommittedIsolation, "ReadCommittedIsolation");

    /// Transaction ran at REPEATABLE READ isolation.
    ///
    /// Source: ANSI SQL-92 Table 2 — isolation level REPEATABLE READ
    pub struct RepeatableReadIsolation;
    structural_prop!(RepeatableReadIsolation, "RepeatableReadIsolation");

    /// Transaction ran at SERIALIZABLE isolation.
    ///
    /// Source: ANSI SQL-92 Table 2 — isolation level SERIALIZABLE
    pub struct SerializableIsolation;
    structural_prop!(SerializableIsolation, "SerializableIsolation");

    /// The effective isolation level was silently upgraded by the engine.
    ///
    /// PostgreSQL upgrades REPEATABLE READ to snapshot isolation rather than
    /// the ANSI definition. Source: Berenson et al. §3 — Snapshot Isolation;
    /// PostgreSQL docs §13.2.2
    pub struct IsolationLevelUpgraded;
    structural_prop!(IsolationLevelUpgraded, "IsolationLevelUpgraded");

    // -- Read phenomena (Berenson et al.) --

    /// Dirty writes (phenomenon P0) are prevented.
    ///
    /// T1 modifies x; T2 modifies x before T1 commits or rolls back.
    /// Source: Berenson et al. §2.1 — P0 Dirty Write
    pub struct PreventsDirtyWrite;
    structural_prop!(PreventsDirtyWrite, "PreventsDirtyWrite");

    /// Dirty reads (phenomenon P1) are prevented.
    ///
    /// T1 reads x written by T2 before T2 commits.
    /// Source: ANSI SQL-92 §4.28 — P1 Dirty Read
    pub struct PreventsDirtyRead;
    structural_prop!(PreventsDirtyRead, "PreventsDirtyRead");

    /// Non-repeatable reads / fuzzy reads (phenomenon P2) are prevented.
    ///
    /// T1 reads x; T2 updates or deletes x and commits; T1 re-reads x and
    /// gets a different value.
    /// Source: ANSI SQL-92 §4.28 — P2 Fuzzy / Non-Repeatable Read
    pub struct PreventsNonRepeatableRead;
    structural_prop!(PreventsNonRepeatableRead, "PreventsNonRepeatableRead");

    /// Phantom reads (phenomenon P3) are prevented.
    ///
    /// T1 reads rows matching a predicate; T2 inserts or deletes a row
    /// matching that predicate and commits; T1 re-evaluates the predicate
    /// and sees a different set.
    /// Source: ANSI SQL-92 §4.28 — P3 Phantom
    pub struct PreventsPhantomRead;
    structural_prop!(PreventsPhantomRead, "PreventsPhantomRead");

    /// Lost updates (phenomenon P4) are prevented.
    ///
    /// T1 reads x; T2 writes x and commits; T1 writes x using the value it
    /// read, silently discarding T2's update.
    /// Source: Berenson et al. §3 — P4 Lost Update
    pub struct PreventsLostUpdate;
    structural_prop!(PreventsLostUpdate, "PreventsLostUpdate");

    /// Read skew (anomaly A2) is prevented.
    ///
    /// T1 reads x; T2 updates both x and y and commits; T1 reads y and
    /// observes an inconsistent pair (new y, old x).
    /// Source: Berenson et al. §4.1 — A2 Read Skew
    pub struct PreventsReadSkew;
    structural_prop!(PreventsReadSkew, "PreventsReadSkew");

    /// Write skew (anomaly A5B) is prevented.
    ///
    /// T1 reads x and y then writes x; T2 reads x and y then writes y;
    /// both commit, violating a constraint that held over (x, y) together.
    /// Source: Berenson et al. §4.1 — A5B Write Skew
    pub struct PreventsWriteSkew;
    structural_prop!(PreventsWriteSkew, "PreventsWriteSkew");

    /// The transaction did not introduce a serialization anomaly.
    ///
    /// The committed outcome is equivalent to some serial execution of all
    /// concurrent transactions. Source: PostgreSQL SSI documentation
    pub struct PreventsSerializationAnomaly;
    structural_prop!(PreventsSerializationAnomaly, "PreventsSerializationAnomaly");

    /// No rw-anti-dependency cycle exists in the transaction's dependency graph.
    ///
    /// SSI aborts transactions that would form a dangerous structure (pivot +
    /// rw-anti-dependency cycle). Source: Ports & Grittner "Serializable
    /// Snapshot Isolation in PostgreSQL" (2012) §2
    pub struct PreventsCircularInformationFlow;
    structural_prop!(
        PreventsCircularInformationFlow,
        "PreventsCircularInformationFlow"
    );

    // -- Lock modes (PostgreSQL §13.3) --

    /// RowShareLock (SELECT FOR SHARE) is held on at least one row.
    ///
    /// Source: PostgreSQL §13.3.1 — Table-Level Lock Modes / RowShareLock
    pub struct RowShareLockAcquired;
    structural_prop!(RowShareLockAcquired, "RowShareLockAcquired");

    /// RowExclusiveLock (SELECT FOR UPDATE / INSERT / UPDATE / DELETE) is held.
    ///
    /// Source: PostgreSQL §13.3.1 — Table-Level Lock Modes / RowExclusiveLock
    pub struct RowExclusiveLockAcquired;
    structural_prop!(RowExclusiveLockAcquired, "RowExclusiveLockAcquired");

    /// ShareUpdateExclusiveLock is held (VACUUM, ANALYZE, CREATE INDEX CONCURRENTLY).
    ///
    /// Source: PostgreSQL §13.3.1 — Table-Level Lock Modes /
    /// ShareUpdateExclusiveLock
    pub struct ShareUpdateExclusiveLockAcquired;
    structural_prop!(
        ShareUpdateExclusiveLockAcquired,
        "ShareUpdateExclusiveLockAcquired"
    );

    /// ShareLock is held (non-concurrent CREATE INDEX).
    ///
    /// Source: PostgreSQL §13.3.1 — Table-Level Lock Modes / ShareLock
    pub struct ShareLockAcquired;
    structural_prop!(ShareLockAcquired, "ShareLockAcquired");

    /// ShareRowExclusiveLock is held (trigger creation).
    ///
    /// Source: PostgreSQL §13.3.1 — Table-Level Lock Modes /
    /// ShareRowExclusiveLock
    pub struct ShareRowExclusiveLockAcquired;
    structural_prop!(
        ShareRowExclusiveLockAcquired,
        "ShareRowExclusiveLockAcquired"
    );

    /// ExclusiveLock is held; only AccessShareLock is compatible.
    ///
    /// Source: PostgreSQL §13.3.1 — Table-Level Lock Modes / ExclusiveLock
    pub struct ExclusiveLockAcquired;
    structural_prop!(ExclusiveLockAcquired, "ExclusiveLockAcquired");

    /// AccessExclusiveLock is held (DROP TABLE, TRUNCATE, ALTER TABLE).
    ///
    /// Source: PostgreSQL §13.3.1 — Table-Level Lock Modes /
    /// AccessExclusiveLock
    pub struct AccessExclusiveLockAcquired;
    structural_prop!(AccessExclusiveLockAcquired, "AccessExclusiveLockAcquired");

    /// Row-level FOR UPDATE lock was acquired on the target row.
    ///
    /// Source: PostgreSQL §13.3.2 — Row-Level Lock Modes / FOR UPDATE
    pub struct ForUpdateAcquired;
    structural_prop!(ForUpdateAcquired, "ForUpdateAcquired");

    /// Row-level FOR SHARE lock was acquired on the target row.
    ///
    /// Source: PostgreSQL §13.3.2 — Row-Level Lock Modes / FOR SHARE
    pub struct ForShareAcquired;
    structural_prop!(ForShareAcquired, "ForShareAcquired");

    /// NOWAIT caused an immediate error rather than waiting on a lock conflict.
    ///
    /// Source: PostgreSQL §13.3.2 — Row-Level Lock Modes / NOWAIT
    pub struct NoWaitRespected;
    structural_prop!(NoWaitRespected, "NoWaitRespected");

    /// SKIP LOCKED skipped rows that were locked by another transaction.
    ///
    /// Source: PostgreSQL §13.3.2 — Row-Level Lock Modes / SKIP LOCKED
    pub struct SkipLockedApplied;
    structural_prop!(SkipLockedApplied, "SkipLockedApplied");

    // -- Deadlock handling --

    /// The deadlock detection algorithm identified a wait-for cycle.
    ///
    /// Source: PostgreSQL §13.3.4 — Deadlocks
    pub struct DeadlockDetected;
    structural_prop!(DeadlockDetected, "DeadlockDetected");

    /// One transaction was chosen as the deadlock victim and rolled back.
    ///
    /// Source: PostgreSQL §13.3.4 — Deadlocks
    pub struct DeadlockResolved;
    structural_prop!(DeadlockResolved, "DeadlockResolved");

    /// `lock_timeout` caused the statement to fail before a deadlock could form.
    ///
    /// Source: PostgreSQL §13.3.4 — Deadlocks / lock_timeout GUC
    pub struct DeadlockTimeoutRespected;
    structural_prop!(DeadlockTimeoutRespected, "DeadlockTimeoutRespected");

    /// `statement_timeout` terminated a long-running query.
    ///
    /// Source: PostgreSQL — Runtime Config / statement_timeout GUC
    pub struct StatementTimeoutRespected;
    structural_prop!(StatementTimeoutRespected, "StatementTimeoutRespected");

    // -- Serializable Snapshot Isolation (SSI) --

    /// An SSI predicate lock (SIRead lock) is held for the current scan.
    ///
    /// Source: Ports & Grittner §3 — SIRead locks track read sets under SSI
    pub struct SsiPredicateLockHeld;
    structural_prop!(SsiPredicateLockHeld, "SsiPredicateLockHeld");

    /// An rw-anti-dependency edge has been tracked by the SSI bookkeeping.
    ///
    /// Source: Ports & Grittner §2 — rw-anti-dependency edges in SSI conflict
    /// graph
    pub struct SsiRwAntiDependencyTracked;
    structural_prop!(SsiRwAntiDependencyTracked, "SsiRwAntiDependencyTracked");

    /// SSI found no dangerous structure (no pivot / promoter cycle) and allowed
    /// the transaction to commit.
    ///
    /// Source: Ports & Grittner §2 — dangerous structures require two
    /// consecutive rw-anti-dependency edges
    pub struct SsiDangerousStructureAvoided;
    structural_prop!(SsiDangerousStructureAvoided, "SsiDangerousStructureAvoided");

    // -- Advisory locks --

    /// A session-level advisory lock was acquired and persists until explicitly
    /// released or the session ends.
    ///
    /// Source: PostgreSQL §13.3.5 — Advisory Locks
    pub struct AdvisoryLockSessionHeld;
    structural_prop!(AdvisoryLockSessionHeld, "AdvisoryLockSessionHeld");

    /// A transaction-level advisory lock was acquired and is released
    /// automatically at transaction end.
    ///
    /// Source: PostgreSQL §13.3.5 — Advisory Locks
    pub struct AdvisoryLockTransactionHeld;
    structural_prop!(AdvisoryLockTransactionHeld, "AdvisoryLockTransactionHeld");

    /// An advisory lock was explicitly released via `pg_advisory_unlock`.
    ///
    /// Source: PostgreSQL §13.3.5 — Advisory Locks
    pub struct AdvisoryLockReleased;
    structural_prop!(AdvisoryLockReleased, "AdvisoryLockReleased");

    // -- Transaction lifecycle --

    /// A SAVEPOINT was established within the current transaction.
    ///
    /// Source: SQL:2003 §17.6 — SAVEPOINT statement
    pub struct SavepointEstablished;
    structural_prop!(SavepointEstablished, "SavepointEstablished");

    /// A DEFERRABLE INITIALLY DEFERRED constraint was validated at commit time.
    ///
    /// The constraint check was postponed to the end of the transaction rather
    /// than being checked immediately after each statement.
    /// Source: SQL:2003 §11.8 — Column constraint definition / DEFERRABLE
    pub struct DeferrableConstraintChecked;
    structural_prop!(DeferrableConstraintChecked, "DeferrableConstraintChecked");

    /// PREPARE TRANSACTION succeeded; the transaction is in the prepared state
    /// awaiting a second-phase decision (2PC).
    ///
    /// Source: PostgreSQL — PREPARE TRANSACTION command
    pub struct TwoPhaseCommitPrepared;
    structural_prop!(TwoPhaseCommitPrepared, "TwoPhaseCommitPrepared");

    /// The prepared transaction was finalised with COMMIT PREPARED or
    /// ROLLBACK PREPARED (2PC second phase).
    ///
    /// Source: PostgreSQL — COMMIT PREPARED / ROLLBACK PREPARED commands
    pub struct TwoPhaseCommitFinalized;
    structural_prop!(TwoPhaseCommitFinalized, "TwoPhaseCommitFinalized");

    /// Transaction was started in `READ ONLY` access mode.
    ///
    /// Source: ISO/IEC 9075-2 §17.1 — `<start transaction statement>` READ ONLY
    pub struct TransactionReadOnly;
    structural_prop!(TransactionReadOnly, "TransactionReadOnly");

    /// Transaction was started in `READ WRITE` access mode.
    ///
    /// Source: ISO/IEC 9075-2 §17.1 — `<start transaction statement>` READ WRITE
    pub struct TransactionReadWrite;
    structural_prop!(TransactionReadWrite, "TransactionReadWrite");

    /// Session default isolation level was set via `SET SESSION CHARACTERISTICS AS TRANSACTION ISOLATION LEVEL`.
    ///
    /// Source: ISO/IEC 9075-2 §17.1 — `<set transaction statement>` / PostgreSQL SET
    pub struct SessionIsolationLevelSet;
    structural_prop!(SessionIsolationLevelSet, "SessionIsolationLevelSet");

    /// Per-transaction isolation level was set via `SET TRANSACTION ISOLATION LEVEL`.
    ///
    /// Source: ISO/IEC 9075-2 §17.1 — `<set transaction statement>`
    pub struct TransactionIsolationLevelSet;
    structural_prop!(TransactionIsolationLevelSet, "TransactionIsolationLevelSet");
}

pub use emit_impls::{
    AccessExclusiveLockAcquired, AdvisoryLockReleased, AdvisoryLockSessionHeld,
    AdvisoryLockTransactionHeld, DeadlockDetected, DeadlockResolved, DeadlockTimeoutRespected,
    DeferrableConstraintChecked, ExclusiveLockAcquired, ForShareAcquired, ForUpdateAcquired,
    IsolationLevelUpgraded, NoWaitRespected, PreventsCircularInformationFlow, PreventsDirtyRead,
    PreventsDirtyWrite, PreventsLostUpdate, PreventsNonRepeatableRead, PreventsPhantomRead,
    PreventsReadSkew, PreventsSerializationAnomaly, PreventsWriteSkew, ReadCommittedIsolation,
    ReadUncommittedIsolation, RepeatableReadIsolation, RowExclusiveLockAcquired,
    RowShareLockAcquired, SavepointEstablished, SerializableIsolation, SessionIsolationLevelSet,
    ShareLockAcquired, ShareRowExclusiveLockAcquired, ShareUpdateExclusiveLockAcquired,
    SkipLockedApplied, SsiDangerousStructureAvoided, SsiPredicateLockHeld,
    SsiRwAntiDependencyTracked, StatementTimeoutRespected, TransactionIsolationLevelSet,
    TransactionReadOnly, TransactionReadWrite, TwoPhaseCommitFinalized, TwoPhaseCommitPrepared,
};

/// Alias: no dirty reads occurred — equivalent to [`PreventsDirtyRead`].
pub type NoDirtyReads = PreventsDirtyRead;

/// Alias: no phantom reads occurred — equivalent to [`PreventsPhantomRead`].
pub type NoPhantomReads = PreventsPhantomRead;

/// Alias: no lost updates occurred — equivalent to [`PreventsLostUpdate`].
pub type NoLostUpdates = PreventsLostUpdate;

/// Alias: no write skew occurred — equivalent to [`PreventsWriteSkew`].
pub type NoWriteSkew = PreventsWriteSkew;
