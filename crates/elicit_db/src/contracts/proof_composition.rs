//! Database proof composition — `ProvableFrom` dependency chains.
//!
//! This module declares how higher-order propositions are provable from
//! lower-order evidence bundles.  None of this is runtime logic; it is
//! purely a type-level dependency graph that formal verification tools
//! (Kani, Creusot, Verus) can traverse.
//!
//! # Isolation level hierarchy
//!
//! Each isolation level implies prevention of a strict subset of read phenomena.
//! The implication lattice (weakest → strongest) is:
//!
//! ```text
//! READ UNCOMMITTED ⊂ READ COMMITTED ⊂ REPEATABLE READ ⊂ SERIALIZABLE
//!
//! SERIALIZABLE     → P0 P1 P2 P3 P4 A5B + serialization anomaly + CI-flow
//! REPEATABLE READ  → P0 P1 P2 P4
//! READ COMMITTED   → P0 P1
//! READ UNCOMMITTED → P0 only
//! ```
//!
//! Source: Berenson et al. "A Critique of ANSI SQL Isolation Levels" (1995);
//! ANSI X3.135-1992 §4.28; PostgreSQL SSI documentation.
//!
//! # ACID composite chain
//!
//! `AcidCompliant` requires all four ACID properties individually established.
//! Source: ISO/IEC 9075-2 §4.33.
//!
//! # Schema integrity chain
//!
//! `SchemaIntegrityEstablished` requires table creation plus satisfied constraints
//! and referential integrity.  Source: ISO/IEC 9075-2 §11.
//!
//! # Security chains
//!
//! `SecureAccessEstablished` — access control + audit + least privilege.
//! `DataProtectionEstablished` — encryption at rest + in transit + active TLS.
//! `AuthenticatedConnectionEstablished` — connection + authentication + TLS.
//!
//! Source: ISO/IEC 27001:2022; PostgreSQL protocol §55.
//!
//! # Recovery chain
//!
//! `RecoveryCapabilityEstablished` — consistent backup + WAL replayable + PITR.
//!
//! Source: PostgreSQL docs §26 — Backup and Restore; §30 — WAL.
//!
//! # 2PC finality chain
//!
//! `TwoPhaseCommitFinalized` requires the prepared state as a prerequisite.
//!
//! Source: ISO/IEC 9075-2 §17 — Transaction management; PostgreSQL PREPARE TRANSACTION.

use elicitation::{Established, contracts::ProvableFrom};

use crate::{
    // security
    AccessAuthorized,
    // iso_sql
    AcidCompliant,
    Atomic,
    AuditLogged,
    // recovery
    BackupConsistent,
    // transport
    ConnectionEstablished,
    Consistent,
    ConstraintSatisfied,
    Durable,
    EncryptedAtRest,
    EncryptedInTransit,
    // constraints
    ForeignKeyDefined,
    ForeignKeyEnforced,
    LeastPrivilegeEnforced,
    // replication
    LogicalReplicationConfigured,
    PointInTimeRecoverable,
    PreventsCircularInformationFlow,
    // isolation — phenomena props
    PreventsDirtyRead,
    PreventsDirtyWrite,
    PreventsLostUpdate,
    PreventsNonRepeatableRead,
    PreventsPhantomRead,
    PreventsSerializationAnomaly,
    PreventsWriteSkew,
    PrimaryKeyDefined,
    PrimaryKeyEnforced,
    PrimaryWalsenderActive,
    PublicationCreated,
    // isolation — level props
    ReadCommittedIsolation,
    ReadUncommittedIsolation,
    ReferentialIntegrityMaintained,
    RepeatableReadIsolation,
    SchemaIntegrityEstablished,
    SerializableIsolation,
    SnapshotIsolation,
    // isolation — SSI
    SsiDangerousStructureAvoided,
    SsiPredicateLockHeld,
    SsiRwAntiDependencyTracked,
    StandbyApplyingWal,
    StandbyWalreceiverActive,
    StreamingReplicationConfigured,
    SubscriptionActive,
    SubscriptionCreated,
    TableCreated,
    TransactionCommitted,
    // isolation — 2PC
    TwoPhaseCommitFinalized,
    TwoPhaseCommitPrepared,
    WALReplayable,
};

// ── READ UNCOMMITTED ──────────────────────────────────────────────────────────

/// Evidence bundle for READ UNCOMMITTED isolation.
///
/// READ UNCOMMITTED prevents dirty writes (P0) only.
///
/// Source: Berenson et al. §2.1 — P0 Dirty Write; ANSI SQL-92 Table 2
pub struct ReadUncommittedEvidence {
    /// The transaction ran at READ UNCOMMITTED level.
    pub isolation: Established<ReadUncommittedIsolation>,
}

impl ProvableFrom<ReadUncommittedEvidence> for PreventsDirtyWrite {}

// ── READ COMMITTED ────────────────────────────────────────────────────────────

/// Evidence bundle for READ COMMITTED isolation.
///
/// READ COMMITTED prevents dirty writes (P0) and dirty reads (P1).
///
/// Source: ANSI SQL-92 Table 2 — READ COMMITTED; Berenson et al. §2.1–§2.2
pub struct ReadCommittedEvidence {
    /// The transaction ran at READ COMMITTED level.
    pub isolation: Established<ReadCommittedIsolation>,
}

impl ProvableFrom<ReadCommittedEvidence> for PreventsDirtyWrite {}
impl ProvableFrom<ReadCommittedEvidence> for PreventsDirtyRead {}

// ── REPEATABLE READ ───────────────────────────────────────────────────────────

/// Evidence bundle for REPEATABLE READ isolation.
///
/// REPEATABLE READ prevents dirty writes (P0), dirty reads (P1),
/// non-repeatable reads (P2), and lost updates (P4).
///
/// Note: PostgreSQL REPEATABLE READ uses snapshot isolation semantics rather
/// than the strict ANSI definition, additionally preventing lost updates.
///
/// Source: ANSI SQL-92 Table 2; Berenson et al. §3 — Snapshot Isolation;
/// PostgreSQL docs §13.2.2 — Repeatable Read Isolation Level
pub struct RepeatableReadEvidence {
    /// The transaction ran at REPEATABLE READ level.
    pub isolation: Established<RepeatableReadIsolation>,
}

impl ProvableFrom<RepeatableReadEvidence> for PreventsDirtyWrite {}
impl ProvableFrom<RepeatableReadEvidence> for PreventsDirtyRead {}
impl ProvableFrom<RepeatableReadEvidence> for PreventsNonRepeatableRead {}
impl ProvableFrom<RepeatableReadEvidence> for PreventsLostUpdate {}

// ── SNAPSHOT ISOLATION ────────────────────────────────────────────────────────

/// Evidence bundle for snapshot isolation.
///
/// Snapshot isolation prevents dirty writes, dirty reads, non-repeatable reads,
/// and lost updates, but permits write skew unless SSI is also active.
///
/// Source: Berenson et al. §3 — Snapshot Isolation
pub struct SnapshotIsolationEvidence {
    /// The transaction ran under snapshot isolation.
    pub isolation: Established<SnapshotIsolation>,
}

impl ProvableFrom<SnapshotIsolationEvidence> for PreventsDirtyWrite {}
impl ProvableFrom<SnapshotIsolationEvidence> for PreventsDirtyRead {}
impl ProvableFrom<SnapshotIsolationEvidence> for PreventsNonRepeatableRead {}
impl ProvableFrom<SnapshotIsolationEvidence> for PreventsLostUpdate {}

// ── SERIALIZABLE ──────────────────────────────────────────────────────────────

/// Evidence bundle for SERIALIZABLE isolation.
///
/// SERIALIZABLE prevents all defined read phenomena and serialization anomalies.
/// In PostgreSQL, SERIALIZABLE uses SSI (Serializable Snapshot Isolation).
///
/// Source: ANSI SQL-92 Table 2 — SERIALIZABLE;
/// Ports & Grittner "Serializable Snapshot Isolation in PostgreSQL" (2012)
pub struct SerializableEvidence {
    /// The transaction ran at SERIALIZABLE level.
    pub isolation: Established<SerializableIsolation>,
}

impl ProvableFrom<SerializableEvidence> for PreventsDirtyWrite {}
impl ProvableFrom<SerializableEvidence> for PreventsDirtyRead {}
impl ProvableFrom<SerializableEvidence> for PreventsNonRepeatableRead {}
impl ProvableFrom<SerializableEvidence> for PreventsPhantomRead {}
impl ProvableFrom<SerializableEvidence> for PreventsLostUpdate {}
impl ProvableFrom<SerializableEvidence> for PreventsWriteSkew {}
impl ProvableFrom<SerializableEvidence> for PreventsSerializationAnomaly {}
impl ProvableFrom<SerializableEvidence> for PreventsCircularInformationFlow {}

// ── SSI COMPLETENESS ──────────────────────────────────────────────────────────

/// Evidence bundle for a completed, serializable SSI transaction.
///
/// When SSI tracks all rw-anti-dependencies and no dangerous structure
/// (pivot + cycle) was detected, the committed outcome is serializable.
///
/// Source: Ports & Grittner §2 — dangerous structures; §3 — SIRead locks
pub struct SsiCompletenessEvidence {
    /// SSI predicate locks tracked all reads.
    pub predicate_locks: Established<SsiPredicateLockHeld>,
    /// All rw-anti-dependency edges were tracked.
    pub anti_deps: Established<SsiRwAntiDependencyTracked>,
    /// No dangerous structure (pivot cycle) was found at commit time.
    pub no_danger: Established<SsiDangerousStructureAvoided>,
}

impl ProvableFrom<SsiCompletenessEvidence> for PreventsSerializationAnomaly {}
impl ProvableFrom<SsiCompletenessEvidence> for PreventsCircularInformationFlow {}

// ── ACID COMPOSITE ────────────────────────────────────────────────────────────

/// Evidence bundle for full ACID compliance.
///
/// A transaction is ACID-compliant when all four properties are individually
/// established: the operation is atomic, the DB remains consistent, the
/// committed state is durable, and a serializable isolation level was used.
///
/// Source: ISO/IEC 9075-2 §4.33 — Atomicity, Consistency, Isolation, Durability
pub struct AcidEvidence {
    /// Atomicity — the transaction either committed fully or was fully rolled back.
    pub atomic: Established<Atomic>,
    /// Consistency — integrity constraints remained satisfied throughout.
    pub consistent: Established<Consistent>,
    /// Isolation — the serializable level was in effect (strictest guarantee).
    pub isolation: Established<SerializableIsolation>,
    /// Durability — committed changes survived the transaction boundary.
    pub durable: Established<Durable>,
    /// Transaction reached `COMMIT` successfully.
    pub committed: Established<TransactionCommitted>,
}

impl ProvableFrom<AcidEvidence> for AcidCompliant {}

// ── SCHEMA INTEGRITY ──────────────────────────────────────────────────────────

/// Evidence bundle for schema integrity.
///
/// Schema integrity is established when the table exists, all constraints are
/// satisfied, and all foreign key references resolve to valid parent rows.
///
/// Source: ISO/IEC 9075-2 §11 — Schema definition; §11.6–§11.8 — Constraints
pub struct SchemaIntegrityEvidence {
    /// The table was successfully created (or already exists).
    pub table: Established<TableCreated>,
    /// All applicable table and column constraints are satisfied.
    pub constraints: Established<ConstraintSatisfied>,
    /// All foreign key values refer to existing parent rows.
    pub referential: Established<ReferentialIntegrityMaintained>,
}

impl ProvableFrom<SchemaIntegrityEvidence> for SchemaIntegrityEstablished {}

// ── SECURE ACCESS ─────────────────────────────────────────────────────────────

/// Evidence bundle for secure access.
///
/// Secure access is established when the request was authorised, the operation
/// was audit-logged, and the minimum required privileges were enforced.
///
/// Source: ISO/IEC 27001:2022 §A.5.15 — Access control; §A.8.15 — Logging
pub struct SecureAccessEvidence {
    /// The identity was authorised to perform this operation.
    pub authorized: Established<AccessAuthorized>,
    /// The operation was recorded in the audit log.
    pub logged: Established<AuditLogged>,
    /// Only the minimum necessary privileges were in effect.
    pub least_privilege: Established<LeastPrivilegeEnforced>,
}

impl ProvableFrom<SecureAccessEvidence> for AccessAuthorized {}

// ── DATA PROTECTION ───────────────────────────────────────────────────────────

/// Evidence bundle for data-at-rest and data-in-transit protection.
///
/// Both storage-layer encryption and transport-layer encryption must be
/// independently established.
///
/// Source: ISO/IEC 27001:2022 §A.8.24 — Use of cryptography
pub struct DataProtectionEvidence {
    /// Persistent storage is encrypted at rest.
    pub at_rest: Established<EncryptedAtRest>,
    /// All network traffic carrying this data is encrypted in transit.
    pub in_transit: Established<EncryptedInTransit>,
}

impl ProvableFrom<DataProtectionEvidence> for EncryptedAtRest {}
impl ProvableFrom<DataProtectionEvidence> for EncryptedInTransit {}

// ── AUTHENTICATED CONNECTION ──────────────────────────────────────────────────

/// Evidence bundle for an authenticated and encrypted database connection.
///
/// An authenticated connection requires that the TCP/IP or Unix-socket
/// connection was established and that the client completed the authentication
/// exchange required by `pg_hba.conf`.
///
/// Source: PostgreSQL docs §55.2 — Connection Setup; §21 — Client Authentication
pub struct AuthenticatedConnectionEvidence {
    /// The network connection to the server was successfully opened.
    pub connection: Established<ConnectionEstablished>,
    /// Access was authorised (the authentication method succeeded).
    pub authorized: Established<AccessAuthorized>,
    /// The session is recorded in the audit log.
    pub logged: Established<AuditLogged>,
}

impl ProvableFrom<AuthenticatedConnectionEvidence> for ConnectionEstablished {}
impl ProvableFrom<AuthenticatedConnectionEvidence> for AccessAuthorized {}

// ── RECOVERY CAPABILITY ───────────────────────────────────────────────────────

/// Evidence bundle for full recovery capability.
///
/// Recovery capability is established when a consistent base backup exists,
/// WAL segments are replayable from that base, and point-in-time recovery
/// is configured and operable.
///
/// Source: PostgreSQL docs §26 — Backup and Restore; §30 — WAL;
/// §26.3 — Continuous Archiving and PITR
pub struct RecoveryCapabilityEvidence {
    /// A consistent base backup is available.
    pub backup: Established<BackupConsistent>,
    /// WAL segments from the backup LSN onwards are intact and replayable.
    pub wal: Established<WALReplayable>,
    /// Point-in-time recovery is configured and has been tested.
    pub pitr: Established<PointInTimeRecoverable>,
}

impl ProvableFrom<RecoveryCapabilityEvidence> for BackupConsistent {}
impl ProvableFrom<RecoveryCapabilityEvidence> for WALReplayable {}
impl ProvableFrom<RecoveryCapabilityEvidence> for PointInTimeRecoverable {}

// ── TWO-PHASE COMMIT FINALITY ─────────────────────────────────────────────────

/// Evidence bundle for two-phase commit finality.
///
/// A distributed transaction can only be finalized (COMMIT PREPARED or
/// ROLLBACK PREPARED) after it has successfully completed the PREPARE phase.
///
/// Source: ISO/IEC 9075-2 §17 — Transaction management;
/// PostgreSQL — PREPARE TRANSACTION / COMMIT PREPARED
pub struct TwoPhaseCommitFinalityEvidence {
    /// The transaction has been prepared and is awaiting a second-phase decision.
    pub prepared: Established<TwoPhaseCommitPrepared>,
}

impl ProvableFrom<TwoPhaseCommitFinalityEvidence> for TwoPhaseCommitFinalized {}

// ── STREAMING REPLICATION ─────────────────────────────────────────────────────

/// Evidence bundle for an active streaming replication session.
///
/// Streaming replication is established when the primary has an active
/// walsender, the standby has an active walreceiver, and the standby is
/// actively applying WAL records.
///
/// Source: PostgreSQL docs §27.2 — Streaming Replication;
/// §54.55 — pg_stat_replication; §54.57 — pg_stat_wal_receiver
pub struct StreamingReplicationEvidence {
    /// The primary has at least one active walsender process.
    pub walsender: Established<PrimaryWalsenderActive>,
    /// The standby has an active walreceiver connected to the primary.
    pub walreceiver: Established<StandbyWalreceiverActive>,
    /// The standby is actively replaying and applying WAL records.
    pub applying: Established<StandbyApplyingWal>,
}

impl ProvableFrom<StreamingReplicationEvidence> for StreamingReplicationConfigured {}

// ── LOGICAL REPLICATION ───────────────────────────────────────────────────────

/// Evidence bundle for an active logical replication channel.
///
/// Logical replication requires a publication on the publisher side,
/// a subscription on the subscriber side, and the subscription must be
/// in the active (replicating) state.
///
/// Source: PostgreSQL docs §31 — Logical Replication;
/// §31.1 — Publication; §31.2 — Subscription
pub struct LogicalReplicationEvidence {
    /// A publication exists on the publisher for the replicated tables.
    pub publication: Established<PublicationCreated>,
    /// A matching subscription exists on the subscriber.
    pub subscription: Established<SubscriptionCreated>,
    /// The subscription is in the active (replicating) state.
    pub active: Established<SubscriptionActive>,
}

impl ProvableFrom<LogicalReplicationEvidence> for LogicalReplicationConfigured {}

// ── KEY CONSTRAINTS ───────────────────────────────────────────────────────────

/// Evidence bundle for primary key enforcement.
///
/// A primary key is enforced when the constraint is defined on the table
/// and the engine guarantees its uniqueness and non-nullability.
///
/// Source: ISO/IEC 9075-2 §11.7 — `<unique constraint definition>` PRIMARY KEY
pub struct PrimaryKeyEnforcementEvidence {
    /// The primary key constraint is defined on the table.
    pub defined: Established<PrimaryKeyDefined>,
}

impl ProvableFrom<PrimaryKeyEnforcementEvidence> for PrimaryKeyEnforced {}

/// Evidence bundle for foreign key enforcement.
///
/// A foreign key is enforced when the constraint is defined and referential
/// integrity is maintained — every FK value either matches a parent row or is null.
///
/// Source: ISO/IEC 9075-2 §11.8 — `<referential constraint definition>`
pub struct ForeignKeyEnforcementEvidence {
    /// The foreign key constraint is defined on the table.
    pub defined: Established<ForeignKeyDefined>,
    /// All FK values resolve to existing parent rows (or are null).
    pub referential: Established<ReferentialIntegrityMaintained>,
}

impl ProvableFrom<ForeignKeyEnforcementEvidence> for ForeignKeyEnforced {}
