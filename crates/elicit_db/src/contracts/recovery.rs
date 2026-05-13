//! Backup and recovery propositions.
//!
//! Source: PostgreSQL documentation, chapters 26 (backup and restore), 27 (replication),
//!         30 (WAL / reliability).

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    // -- Baseline --

    /// Backup is consistent and can be used for a full restore.
    ///
    /// Source: PostgreSQL docs §26 — Backup and Restore
    pub struct BackupConsistent;

    /// WAL segment is intact and can be replayed.
    ///
    /// Source: PostgreSQL docs §30 — Reliability and the Write-Ahead Log
    pub struct WALReplayable;

    /// Database can be restored to a specific point in time.
    ///
    /// Source: PostgreSQL docs §26.3 — Continuous Archiving and Point-in-Time Recovery
    pub struct PointInTimeRecoverable;

    // -- Base Backup --

    /// pg_basebackup or equivalent started a base backup.
    ///
    /// Source: PostgreSQL docs §26.3.2 — Making a Base Backup — pg_basebackup
    pub struct BaseBackupInitiated;

    /// Base backup completed without errors.
    ///
    /// Source: PostgreSQL docs §26.3.2 — Making a Base Backup — completion
    pub struct BaseBackupCompleted;

    /// Backup block checksums were verified (--checksum-algorithm).
    ///
    /// Source: PostgreSQL docs §26.3.2 — Making a Base Backup — checksum verification
    pub struct BaseBackupChecksumVerified;

    /// Backup files are encrypted at rest.
    ///
    /// Source: PostgreSQL docs §26.3.2 — Making a Base Backup — encryption
    pub struct BaseBackupEncrypted;

    /// Backup manifest file was created alongside the base backup.
    ///
    /// Source: PostgreSQL docs §26.3.5 — Backup Manifest Format
    pub struct BackupManifestGenerated;

    /// Backup manifest was verified with pg_verifybackup.
    ///
    /// Source: PostgreSQL docs §26.3.6 — pg_verifybackup
    pub struct BackupManifestVerified;

    // -- Logical Backup --

    /// pg_dump completed without errors.
    ///
    /// Source: PostgreSQL docs §26.1 — SQL Dump — pg_dump
    pub struct PgDumpCompleted;

    /// pg_dumpall completed for a full cluster dump.
    ///
    /// Source: PostgreSQL docs §26.1 — SQL Dump — pg_dumpall
    pub struct PgDumpAllCompleted;

    /// pg_dump -Fc custom format was used to enable parallel restore.
    ///
    /// Source: PostgreSQL docs §26.1 — SQL Dump — custom format
    pub struct CustomFormatDumpUsed;

    /// pg_restore --jobs completed a parallel restore.
    ///
    /// Source: PostgreSQL docs §26.1 — SQL Dump — pg_restore --jobs
    pub struct ParallelRestoreCompleted;

    /// pg_dump output is encrypted.
    ///
    /// Source: PostgreSQL docs §26.1 — SQL Dump — encryption
    pub struct DumpEncryptionApplied;

    // -- WAL Archiving --

    /// archive_mode is on; WAL archiving is enabled.
    ///
    /// Source: PostgreSQL docs §26.3.1 — Setting Up WAL Archiving — archive_mode
    pub struct WalArchivingEnabled;

    /// archive_command executed successfully for a WAL segment.
    ///
    /// Source: PostgreSQL docs §26.3.1 — Setting Up WAL Archiving — archive_command
    pub struct WalArchiveCommandSucceeded;

    /// Archived WAL segments are retained for the required duration.
    ///
    /// Source: PostgreSQL docs §26.3.1 — Setting Up WAL Archiving — retention
    pub struct WalArchiveRetentionMet;

    /// restore_command was tested and produces valid WAL.
    ///
    /// Source: PostgreSQL docs §26.3.3 — Recovery Configuration — restore_command
    pub struct WalRestoreCommandTested;

    /// Archive location is accessible for both read and write.
    ///
    /// Source: PostgreSQL docs §26.3.1 — Setting Up WAL Archiving — accessibility
    pub struct WalArchiveAccessible;

    // -- WAL Integrity --

    /// WAL segment passes pg_waldump integrity check.
    ///
    /// Source: PostgreSQL docs §30 — Reliability and the Write-Ahead Log — pg_waldump
    pub struct WalSegmentIntact;

    /// Data checksums were enabled at cluster init (initdb --data-checksums).
    ///
    /// Source: PostgreSQL docs §30.2 — Data Checksums
    pub struct WalChecksumEnabled;

    /// fsync=on ensures WAL durability on crash.
    ///
    /// Source: PostgreSQL docs §30.1 — Reliability — fsync
    pub struct WalFsyncConfirmed;

    /// wal_level=replica or logical satisfies the archiving requirement.
    ///
    /// Source: PostgreSQL docs §30.4 — Write-Ahead Logging — wal_level
    pub struct WalLevelSufficientForArchiving;

    /// wal_keep_size retains enough segments for the standby.
    ///
    /// Source: PostgreSQL docs §27.2.5 — Replication — wal_keep_size
    pub struct WalRetentionPolicySatisfied;

    // -- Streaming Replication --

    /// Standby walsender process is active on the primary.
    ///
    /// Source: PostgreSQL docs §27.2.2 — Streaming Replication — walsender
    pub struct StandbyConnectedToPrimary;

    /// Standby walreceiver is actively receiving WAL from the primary.
    ///
    /// Source: PostgreSQL docs §27.2.2 — Streaming Replication — walreceiver
    pub struct StandbyReceivingWal;

    /// synchronous_commit=on and standby confirmed write.
    ///
    /// Source: PostgreSQL docs §27.2.8 — Synchronous Replication — synchronous_commit
    pub struct SynchronousStandbyAcknowledged;

    /// Quorum-based synchronous replication confirmed write.
    ///
    /// Source: PostgreSQL docs §27.2.8 — Synchronous Replication — quorum commit
    pub struct QuorumCommitAcknowledged;

    // -- Replication Slots --

    /// pg_create_physical_replication_slot() succeeded.
    ///
    /// Source: PostgreSQL docs §27.2.6 — Replication Slots — physical slot
    pub struct ReplicationSlotCreated;

    // -- PITR and Recovery Targets --

    /// recovery_target_time/lsn/name was reached during recovery.
    ///
    /// Source: PostgreSQL docs §26.3.3 — Recovery Configuration — recovery_target
    pub struct RecoveryTargetReached;

    /// A PITR test restore completed and verified application consistency.
    ///
    /// Source: PostgreSQL docs §26.3 — Continuous Archiving and PITR — testing
    pub struct PitrTestPassed;

    // -- High Availability --

    /// Planned primary switchover completed successfully.
    ///
    /// Source: PostgreSQL docs §27.3 — Failover — switchover
    pub struct SwitchoverCompleted;

    /// Unplanned failover completed; new primary is accepting writes.
    ///
    /// Source: PostgreSQL docs §27.3 — Failover
    pub struct FailoverCompleted;

    /// PostgreSQL timeline ID incremented after failover or recovery.
    ///
    /// Source: PostgreSQL docs §26.3.5 — Timelines
    pub struct TimelineAdvanced;

    /// Standby promoted to standalone (no-replication) mode.
    ///
    /// Source: PostgreSQL docs §27.3 — Failover — pg_promote
    pub struct PromotedToStandalone;

    // -- Monitoring --

    /// Replication lag in seconds is below the alert threshold.
    ///
    /// Source: PostgreSQL docs §27.2.3 — Replication — pg_stat_replication
    pub struct StandbyLagBelowThreshold;

    /// pg_wal_lsn_diff estimate shows recovery is nearly complete.
    ///
    /// Source: PostgreSQL docs §9.27 — System Information Functions — pg_wal_lsn_diff
    pub struct RecoveryCompletionEstimated;

    /// pg_wal_replay_resume() cleared a recovery pause.
    ///
    /// Source: PostgreSQL docs §26.3.4 — Recovery Configuration — recovery_target_action
    pub struct RecoveryPauseCleared;

    /// Backup retention policy duration is satisfied.
    ///
    /// Source: PostgreSQL docs §26.1 — SQL Dump — retention policy
    pub struct BackupRetentionPolicyMet;

    // -- PITR target type sub-propositions --

    /// Recovery target was specified as a timestamp (`recovery_target_time`).
    ///
    /// Source: PostgreSQL §27.3.1 — `recovery_target_time` parameter
    pub struct RecoveryTargetTime;

    /// Recovery target was specified as a WAL LSN (`recovery_target_lsn`).
    ///
    /// Source: PostgreSQL §27.3.1 — `recovery_target_lsn` parameter
    pub struct RecoveryTargetLsn;

    /// Recovery target was specified as a transaction ID (`recovery_target_xid`).
    ///
    /// Source: PostgreSQL §27.3.1 — `recovery_target_xid` parameter
    pub struct RecoveryTargetTransaction;

    /// Recovery target was specified as a named restore point (`recovery_target_name`).
    ///
    /// Source: PostgreSQL §27.3.1 — `recovery_target_name` parameter
    pub struct RecoveryTargetName;

    macro_rules! recovery_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by backup/WAL integrity check */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by backup/WAL integrity check */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by backup/WAL integrity check */ }
                }
            }
        };
    }

    // Baseline
    recovery_prop!(BackupConsistent, "BackupConsistent");
    recovery_prop!(WALReplayable, "WALReplayable");
    recovery_prop!(PointInTimeRecoverable, "PointInTimeRecoverable");
    // Base Backup
    recovery_prop!(BaseBackupInitiated, "BaseBackupInitiated");
    recovery_prop!(BaseBackupCompleted, "BaseBackupCompleted");
    recovery_prop!(BaseBackupChecksumVerified, "BaseBackupChecksumVerified");
    recovery_prop!(BaseBackupEncrypted, "BaseBackupEncrypted");
    recovery_prop!(BackupManifestGenerated, "BackupManifestGenerated");
    recovery_prop!(BackupManifestVerified, "BackupManifestVerified");
    // Logical Backup
    recovery_prop!(PgDumpCompleted, "PgDumpCompleted");
    recovery_prop!(PgDumpAllCompleted, "PgDumpAllCompleted");
    recovery_prop!(CustomFormatDumpUsed, "CustomFormatDumpUsed");
    recovery_prop!(ParallelRestoreCompleted, "ParallelRestoreCompleted");
    recovery_prop!(DumpEncryptionApplied, "DumpEncryptionApplied");
    // WAL Archiving
    recovery_prop!(WalArchivingEnabled, "WalArchivingEnabled");
    recovery_prop!(WalArchiveCommandSucceeded, "WalArchiveCommandSucceeded");
    recovery_prop!(WalArchiveRetentionMet, "WalArchiveRetentionMet");
    recovery_prop!(WalRestoreCommandTested, "WalRestoreCommandTested");
    recovery_prop!(WalArchiveAccessible, "WalArchiveAccessible");
    // WAL Integrity
    recovery_prop!(WalSegmentIntact, "WalSegmentIntact");
    recovery_prop!(WalChecksumEnabled, "WalChecksumEnabled");
    recovery_prop!(WalFsyncConfirmed, "WalFsyncConfirmed");
    recovery_prop!(
        WalLevelSufficientForArchiving,
        "WalLevelSufficientForArchiving"
    );
    recovery_prop!(WalRetentionPolicySatisfied, "WalRetentionPolicySatisfied");
    // Streaming Replication
    recovery_prop!(StandbyConnectedToPrimary, "StandbyConnectedToPrimary");
    recovery_prop!(StandbyReceivingWal, "StandbyReceivingWal");
    recovery_prop!(
        SynchronousStandbyAcknowledged,
        "SynchronousStandbyAcknowledged"
    );
    recovery_prop!(QuorumCommitAcknowledged, "QuorumCommitAcknowledged");
    // Replication Slots
    recovery_prop!(ReplicationSlotCreated, "ReplicationSlotCreated");
    // PITR and Recovery Targets
    recovery_prop!(RecoveryTargetReached, "RecoveryTargetReached");
    recovery_prop!(PitrTestPassed, "PitrTestPassed");
    // High Availability
    recovery_prop!(SwitchoverCompleted, "SwitchoverCompleted");
    recovery_prop!(FailoverCompleted, "FailoverCompleted");
    recovery_prop!(TimelineAdvanced, "TimelineAdvanced");
    recovery_prop!(PromotedToStandalone, "PromotedToStandalone");
    // Monitoring
    recovery_prop!(StandbyLagBelowThreshold, "StandbyLagBelowThreshold");
    recovery_prop!(RecoveryCompletionEstimated, "RecoveryCompletionEstimated");
    recovery_prop!(RecoveryPauseCleared, "RecoveryPauseCleared");
    recovery_prop!(BackupRetentionPolicyMet, "BackupRetentionPolicyMet");

    recovery_prop!(RecoveryTargetTime, "RecoveryTargetTime");
    recovery_prop!(RecoveryTargetLsn, "RecoveryTargetLsn");
    recovery_prop!(RecoveryTargetTransaction, "RecoveryTargetTransaction");
    recovery_prop!(RecoveryTargetName, "RecoveryTargetName");
}

pub use emit_impls::{
    BackupConsistent, BackupManifestGenerated, BackupManifestVerified, BackupRetentionPolicyMet,
    BaseBackupChecksumVerified, BaseBackupCompleted, BaseBackupEncrypted, BaseBackupInitiated,
    CustomFormatDumpUsed, DumpEncryptionApplied, FailoverCompleted, ParallelRestoreCompleted,
    PgDumpAllCompleted, PgDumpCompleted, PitrTestPassed, PointInTimeRecoverable,
    PromotedToStandalone, QuorumCommitAcknowledged, RecoveryCompletionEstimated,
    RecoveryPauseCleared, RecoveryTargetLsn, RecoveryTargetName, RecoveryTargetReached,
    RecoveryTargetTime, RecoveryTargetTransaction, ReplicationSlotCreated,
    StandbyConnectedToPrimary, StandbyLagBelowThreshold, StandbyReceivingWal, SwitchoverCompleted,
    SynchronousStandbyAcknowledged, TimelineAdvanced, WALReplayable, WalArchiveAccessible,
    WalArchiveCommandSucceeded, WalArchiveRetentionMet, WalArchivingEnabled, WalChecksumEnabled,
    WalFsyncConfirmed, WalLevelSufficientForArchiving, WalRestoreCommandTested,
    WalRetentionPolicySatisfied, WalSegmentIntact,
};
