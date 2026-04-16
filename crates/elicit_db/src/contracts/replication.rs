//! Replication and high-availability propositions.
//!
//! Source: PostgreSQL §27 — High Availability, Load Balancing, and Replication; §49 — Logical Decoding.

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

    // -- Streaming replication (§27.2) --

    /// primary_conninfo / primary_slot_name set on standby.
    ///
    /// Source: PostgreSQL §27.2 — Standby server setup: primary_conninfo
    pub struct StreamingReplicationConfigured;
    structural_prop!(
        StreamingReplicationConfigured,
        "StreamingReplicationConfigured"
    );

    /// Walsender process is active on the primary for this standby.
    ///
    /// Source: PostgreSQL §27.2 — Primary: pg_stat_replication walsender
    pub struct PrimaryWalsenderActive;
    structural_prop!(PrimaryWalsenderActive, "PrimaryWalsenderActive");

    /// Walreceiver process is active on the standby.
    ///
    /// Source: PostgreSQL §27.2 — Standby: pg_stat_wal_receiver walreceiver
    pub struct StandbyWalreceiverActive;
    structural_prop!(StandbyWalreceiverActive, "StandbyWalreceiverActive");

    /// Startup/apply process is replaying received WAL.
    ///
    /// Source: PostgreSQL §27.2 — Standby WAL apply process
    pub struct StandbyApplyingWal;
    structural_prop!(StandbyApplyingWal, "StandbyApplyingWal");

    /// hot_standby=on and standby accepts read-only queries.
    ///
    /// Source: PostgreSQL §27.4 — Hot standby: hot_standby parameter
    pub struct HotStandbyEnabled;
    structural_prop!(HotStandbyEnabled, "HotStandbyEnabled");

    /// A read query executed successfully on a hot standby.
    ///
    /// Source: PostgreSQL §27.4 — Hot standby query execution
    pub struct HotStandbyQueryExecuted;
    structural_prop!(HotStandbyQueryExecuted, "HotStandbyQueryExecuted");

    /// Streaming replication lag is within the SLA threshold.
    ///
    /// Source: PostgreSQL §27.2 — pg_stat_replication: write_lag / replay_lag
    pub struct ReplicationLagWithinSla;
    structural_prop!(ReplicationLagWithinSla, "ReplicationLagWithinSla");

    /// synchronous_standby_names is set.
    ///
    /// Source: PostgreSQL §27.2 — Synchronous replication: synchronous_standby_names
    pub struct SynchronousReplicationConfigured;
    structural_prop!(
        SynchronousReplicationConfigured,
        "SynchronousReplicationConfigured"
    );

    /// synchronous_commit=local (local durable, async to standby).
    ///
    /// Source: PostgreSQL §27.2 — synchronous_commit=local
    pub struct SynchronousReplicationModeLocal;
    structural_prop!(
        SynchronousReplicationModeLocal,
        "SynchronousReplicationModeLocal"
    );

    /// synchronous_commit=remote_write.
    ///
    /// Source: PostgreSQL §27.2 — synchronous_commit=remote_write
    pub struct SynchronousReplicationModeRemoteWrite;
    structural_prop!(
        SynchronousReplicationModeRemoteWrite,
        "SynchronousReplicationModeRemoteWrite"
    );

    /// synchronous_commit=remote_apply.
    ///
    /// Source: PostgreSQL §27.2 — synchronous_commit=remote_apply
    pub struct SynchronousReplicationModeRemoteApply;
    structural_prop!(
        SynchronousReplicationModeRemoteApply,
        "SynchronousReplicationModeRemoteApply"
    );

    /// Synchronous standby acknowledged WAL write.
    ///
    /// Source: PostgreSQL §27.2 — Synchronous standby: write acknowledgement
    pub struct SynchronousStandbyAcknowledgedWrite;
    structural_prop!(
        SynchronousStandbyAcknowledgedWrite,
        "SynchronousStandbyAcknowledgedWrite"
    );

    /// Synchronous standby acknowledged WAL apply.
    ///
    /// Source: PostgreSQL §27.2 — Synchronous standby: apply acknowledgement
    pub struct SynchronousStandbyAcknowledgedApply;
    structural_prop!(
        SynchronousStandbyAcknowledgedApply,
        "SynchronousStandbyAcknowledgedApply"
    );

    /// Quorum-based synchronous commit satisfied.
    ///
    /// Source: PostgreSQL §27.2 — Quorum synchronous commit: ANY N (standbies)
    pub struct QuorumSynchronousCommitAcknowledged;
    structural_prop!(
        QuorumSynchronousCommitAcknowledged,
        "QuorumSynchronousCommitAcknowledged"
    );

    // -- Replication slots (§27.2.6) --

    /// pg_create_physical_replication_slot() succeeded.
    ///
    /// Source: PostgreSQL §27.2.6 — Physical replication slots
    pub struct PhysicalReplicationSlotCreated;
    structural_prop!(
        PhysicalReplicationSlotCreated,
        "PhysicalReplicationSlotCreated"
    );

    /// pg_create_logical_replication_slot() succeeded.
    ///
    /// Source: PostgreSQL §27.2.6 — Logical replication slots
    pub struct LogicalReplicationSlotCreated;
    structural_prop!(
        LogicalReplicationSlotCreated,
        "LogicalReplicationSlotCreated"
    );

    /// Slot is currently being consumed (slot.active=true).
    ///
    /// Source: PostgreSQL §27.2.6 — pg_replication_slots: active column
    pub struct ReplicationSlotActive;
    structural_prop!(ReplicationSlotActive, "ReplicationSlotActive");

    /// Retained WAL size from slot is within limits.
    ///
    /// Source: PostgreSQL §27.2.6 — Replication slot lag: retained_wal_size
    pub struct ReplicationSlotLagAcceptable;
    structural_prop!(ReplicationSlotLagAcceptable, "ReplicationSlotLagAcceptable");

    /// pg_drop_replication_slot() succeeded.
    ///
    /// Source: PostgreSQL §27.2.6 — pg_drop_replication_slot()
    pub struct ReplicationSlotDropped;
    structural_prop!(ReplicationSlotDropped, "ReplicationSlotDropped");

    /// Failover replication slot created for HA.
    ///
    /// Source: PostgreSQL §27.2.6 — Failover slots
    pub struct FailoverSlotCreated;
    structural_prop!(FailoverSlotCreated, "FailoverSlotCreated");

    // -- Logical replication (§29) --

    /// wal_level=logical is set.
    ///
    /// Source: PostgreSQL §29 — Logical replication: wal_level=logical requirement
    pub struct LogicalReplicationConfigured;
    structural_prop!(LogicalReplicationConfigured, "LogicalReplicationConfigured");

    /// CREATE PUBLICATION succeeded.
    ///
    /// Source: PostgreSQL §29.2 — CREATE PUBLICATION statement
    pub struct PublicationCreated;
    structural_prop!(PublicationCreated, "PublicationCreated");

    /// Target table is part of the publication.
    ///
    /// Source: PostgreSQL §29.2 — Publication table membership
    pub struct PublicationIncludesTable;
    structural_prop!(PublicationIncludesTable, "PublicationIncludesTable");

    /// FOR ALL TABLES publication scope used.
    ///
    /// Source: PostgreSQL §29.2 — CREATE PUBLICATION ... FOR ALL TABLES
    pub struct PublicationAllTablesScope;
    structural_prop!(PublicationAllTablesScope, "PublicationAllTablesScope");

    /// Publication has a row filter expression.
    ///
    /// Source: PostgreSQL §29.4 — Publication row filters
    pub struct PublicationRowFilterDefined;
    structural_prop!(PublicationRowFilterDefined, "PublicationRowFilterDefined");

    /// Publication has a column list restriction.
    ///
    /// Source: PostgreSQL §29.4 — Publication column lists
    pub struct PublicationColumnListDefined;
    structural_prop!(PublicationColumnListDefined, "PublicationColumnListDefined");

    /// CREATE SUBSCRIPTION succeeded.
    ///
    /// Source: PostgreSQL §29.5 — CREATE SUBSCRIPTION statement
    pub struct SubscriptionCreated;
    structural_prop!(SubscriptionCreated, "SubscriptionCreated");

    /// Subscription is in the 'ready'/'streaming'/'catching up' state.
    ///
    /// Source: PostgreSQL §29.5 — pg_subscription_rel: srsubstate
    pub struct SubscriptionActive;
    structural_prop!(SubscriptionActive, "SubscriptionActive");

    /// Initial table copy (tablesync) completed for a table.
    ///
    /// Source: PostgreSQL §29.5 — Logical replication initial table synchronization
    pub struct SubscriptionTableCopied;
    structural_prop!(SubscriptionTableCopied, "SubscriptionTableCopied");

    /// A replication conflict was resolved per conflict_action.
    ///
    /// Source: PostgreSQL §29.5 — Logical replication conflict resolution
    pub struct SubscriptionConflictResolved;
    structural_prop!(SubscriptionConflictResolved, "SubscriptionConflictResolved");

    // -- Logical decoding (§49) --

    /// pg_logical_slot_get_changes() returns changes.
    ///
    /// Source: PostgreSQL §49 — Logical decoding: pg_logical_slot_get_changes()
    pub struct LogicalDecodingActive;
    structural_prop!(LogicalDecodingActive, "LogicalDecodingActive");

    /// Output plugin (pgoutput, wal2json, etc.) is registered.
    ///
    /// Source: PostgreSQL §49.6 — Logical decoding output plugins
    pub struct LogicalDecodingPluginRegistered;
    structural_prop!(
        LogicalDecodingPluginRegistered,
        "LogicalDecodingPluginRegistered"
    );

    /// A CDC (INSERT/UPDATE/DELETE) change event was decoded.
    ///
    /// Source: PostgreSQL §49 — Change Data Capture via logical decoding
    pub struct CdcChangeEventProduced;
    structural_prop!(CdcChangeEventProduced, "CdcChangeEventProduced");

    /// pg_replication_origin_create() registered an origin.
    ///
    /// Source: PostgreSQL §49.3 — Replication origins: pg_replication_origin_create()
    pub struct ReplicationOriginCreated;
    structural_prop!(ReplicationOriginCreated, "ReplicationOriginCreated");

    /// pg_replication_origin_progress() reports correct LSN.
    ///
    /// Source: PostgreSQL §49.3 — Replication origin progress tracking
    pub struct ReplicationOriginProgressTracked;
    structural_prop!(
        ReplicationOriginProgressTracked,
        "ReplicationOriginProgressTracked"
    );

    // -- HA lifecycle --

    /// pg_promote() or trigger file caused promotion.
    ///
    /// Source: PostgreSQL §27.3 — Standby server promotion: pg_promote()
    pub struct StandbyPromotedToPrimary;
    structural_prop!(StandbyPromotedToPrimary, "StandbyPromotedToPrimary");

    /// Timeline ID incremented after a promotion/recovery.
    ///
    /// Source: PostgreSQL §27.3 — Timeline history after promotion
    pub struct TimelineIdAdvanced;
    structural_prop!(TimelineIdAdvanced, "TimelineIdAdvanced");

    /// Database exited recovery mode and is accepting writes.
    ///
    /// Source: PostgreSQL §27.3 — End of recovery: pg_is_in_recovery() = false
    pub struct RecoveryModeExited;
    structural_prop!(RecoveryModeExited, "RecoveryModeExited");

    /// Planned switchover to standby completed.
    ///
    /// Source: PostgreSQL §27.3 — Planned failover / switchover
    pub struct StandbySwitchoverCompleted;
    structural_prop!(StandbySwitchoverCompleted, "StandbySwitchoverCompleted");

    /// Patroni/Pacemaker/etc. triggered automatic failover.
    ///
    /// Source: PostgreSQL §27 — High availability: automatic failover via HA manager
    pub struct AutomaticFailoverTriggered;
    structural_prop!(AutomaticFailoverTriggered, "AutomaticFailoverTriggered");

    // -- WAL level configuration --

    /// `wal_level = logical` is configured, enabling logical decoding.
    ///
    /// Source: PostgreSQL §19.6.3 — `wal_level` parameter: logical
    pub struct WalLevelLogical;
    structural_prop!(WalLevelLogical, "WalLevelLogical");

    /// `wal_level = replica` is configured, enabling streaming replication.
    ///
    /// Source: PostgreSQL §19.6.3 — `wal_level` parameter: replica
    pub struct WalLevelReplica;
    structural_prop!(WalLevelReplica, "WalLevelReplica");

    /// `wal_level = minimal` is configured (only crash recovery, no replication).
    ///
    /// Source: PostgreSQL §19.6.3 — `wal_level` parameter: minimal
    pub struct WalLevelMinimal;
    structural_prop!(WalLevelMinimal, "WalLevelMinimal");

    /// `max_wal_senders` is set to a non-zero value, permitting standby connections.
    ///
    /// Source: PostgreSQL §19.6.4 — `max_wal_senders` parameter
    pub struct MaxWalSendersConfigured;
    structural_prop!(MaxWalSendersConfigured, "MaxWalSendersConfigured");

    /// `max_replication_slots` is set to a non-zero value, permitting replication slots.
    ///
    /// Source: PostgreSQL §19.6.4 — `max_replication_slots` parameter
    pub struct MaxReplicationSlotsConfigured;
    structural_prop!(
        MaxReplicationSlotsConfigured,
        "MaxReplicationSlotsConfigured"
    );
}

pub use emit_impls::{
    AutomaticFailoverTriggered, CdcChangeEventProduced, FailoverSlotCreated, HotStandbyEnabled,
    HotStandbyQueryExecuted, LogicalDecodingActive, LogicalDecodingPluginRegistered,
    LogicalReplicationConfigured, LogicalReplicationSlotCreated, MaxReplicationSlotsConfigured,
    MaxWalSendersConfigured, PhysicalReplicationSlotCreated, PrimaryWalsenderActive,
    PublicationAllTablesScope, PublicationColumnListDefined, PublicationCreated,
    PublicationIncludesTable, PublicationRowFilterDefined, QuorumSynchronousCommitAcknowledged,
    RecoveryModeExited, ReplicationLagWithinSla, ReplicationOriginCreated,
    ReplicationOriginProgressTracked, ReplicationSlotActive, ReplicationSlotDropped,
    ReplicationSlotLagAcceptable, StandbyApplyingWal, StandbyPromotedToPrimary,
    StandbySwitchoverCompleted, StandbyWalreceiverActive, StreamingReplicationConfigured,
    SubscriptionActive, SubscriptionConflictResolved, SubscriptionCreated, SubscriptionTableCopied,
    SynchronousReplicationConfigured, SynchronousReplicationModeLocal,
    SynchronousReplicationModeRemoteApply, SynchronousReplicationModeRemoteWrite,
    SynchronousStandbyAcknowledgedApply, SynchronousStandbyAcknowledgedWrite, TimelineIdAdvanced,
    WalLevelLogical, WalLevelMinimal, WalLevelReplica,
};
