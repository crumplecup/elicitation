//! `DbReplicationFactory`      — replication topology factory (Role 1a).
//! `DbReplicationMeta`         — replication status reporter (Role 2).
//!
//! # Three-role taxonomy
//!
//! | Role | Trait | Purpose |
//! |------|-------|---------|
//! | 1a (leaf factory) | [`DbReplicationFactory`] | Creates slots, publications, subscriptions; returns proof tokens |
//! | 2 (reporter) | [`DbReplicationMeta`] | Reads pg_replication_slots, pg_stat_replication; no proof tokens |
//!
//! Source: PostgreSQL docs §27 — High Availability, Load Balancing, and Replication;
//!         §29 — Logical Replication.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{
    AuditLogged, DbPublicationDescriptor, DbReplicationSlotDescriptor, DbResult,
    DbSubscriptionDescriptor, LogicalReplicationConfigured, LogicalReplicationSlotCreated,
    PhysicalReplicationSlotCreated, PublicationCreated, ReplicationSlotDropped,
    StreamingReplicationConfigured, SubscriptionCreated, WalLevelLogical, WalLevelReplica,
};

// ── Role 1a: replication topology factory ─────────────────────────────────────

/// Creates and manages replication topology: slots, publications, and subscriptions.
///
/// Each method returns proof tokens that can be composed via the `ProvableFrom`
/// chains in `contracts::proof_composition`.
///
/// Source: PostgreSQL docs §27 — High Availability;
///         §29 — Logical Replication.
pub trait DbReplicationFactory: Send + Sync {
    /// Create a logical replication publication on this server.
    ///
    /// Source: PostgreSQL docs §29.6 — `CREATE PUBLICATION`
    fn create_publication(
        &self,
        descriptor: DbPublicationDescriptor,
    ) -> BoxFuture<
        '_,
        DbResult<(
            DbPublicationDescriptor,
            Established<PublicationCreated>,
            Established<AuditLogged>,
        )>,
    >;

    /// Create a logical replication subscription to a remote publisher.
    ///
    /// Source: PostgreSQL docs §29.7 — `CREATE SUBSCRIPTION`
    fn create_subscription(
        &self,
        descriptor: DbSubscriptionDescriptor,
    ) -> BoxFuture<
        '_,
        DbResult<(
            DbSubscriptionDescriptor,
            Established<SubscriptionCreated>,
            Established<AuditLogged>,
        )>,
    >;

    /// Create a physical replication slot.
    ///
    /// Source: PostgreSQL docs §27.2.6 — `pg_create_physical_replication_slot()`
    fn create_physical_slot(
        &self,
        name: &str,
        immediately_reserve: bool,
    ) -> BoxFuture<
        '_,
        DbResult<(
            DbReplicationSlotDescriptor,
            Established<PhysicalReplicationSlotCreated>,
            Established<AuditLogged>,
        )>,
    >;

    /// Create a logical replication slot for a given output plugin.
    ///
    /// Source: PostgreSQL docs §27.2.6 — `pg_create_logical_replication_slot()`
    fn create_logical_slot(
        &self,
        name: &str,
        plugin: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            DbReplicationSlotDescriptor,
            Established<LogicalReplicationSlotCreated>,
            Established<AuditLogged>,
        )>,
    >;

    /// Drop a replication slot.
    ///
    /// Source: PostgreSQL docs §27.2.6 — `pg_drop_replication_slot()`
    fn drop_slot(
        &self,
        name: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<ReplicationSlotDropped>,
            Established<AuditLogged>,
        )>,
    >;

    /// Configure streaming replication parameters (`wal_level = replica`,
    /// `max_wal_senders`).
    ///
    /// Returns `WalLevelReplica` and `StreamingReplicationConfigured`.
    ///
    /// Source: PostgreSQL docs §27.2 — Log-Shipping Standby Servers
    fn configure_streaming_replication(
        &self,
        max_wal_senders: u32,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<WalLevelReplica>,
            Established<StreamingReplicationConfigured>,
            Established<AuditLogged>,
        )>,
    >;

    /// Configure logical replication parameters (`wal_level = logical`,
    /// `max_replication_slots`).
    ///
    /// Returns `WalLevelLogical` and `LogicalReplicationConfigured`.
    ///
    /// Source: PostgreSQL docs §29.2 — Logical Replication Configuration
    fn configure_logical_replication(
        &self,
        max_replication_slots: u32,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<WalLevelLogical>,
            Established<LogicalReplicationConfigured>,
            Established<AuditLogged>,
        )>,
    >;
}

// ── Role 2: replication status reporter ──────────────────────────────────────

/// Orthogonal reporter for replication topology and health.
///
/// Queries `pg_replication_slots`, `pg_stat_replication`,
/// `pg_publication`, `pg_subscription`. No proof tokens are produced.
///
/// Source: PostgreSQL docs §54.48 — pg_stat_replication.
pub trait DbReplicationMeta: Send + Sync {
    /// Return all replication slots and their current lag in bytes.
    ///
    /// Source: PostgreSQL docs §54.46 — pg_replication_slots
    fn replication_slot_lag(
        &self,
    ) -> BoxFuture<'_, DbResult<Vec<(DbReplicationSlotDescriptor, u64)>>>;

    /// Return all publications defined on this server.
    ///
    /// Source: PostgreSQL docs §54.40 — pg_publication
    fn list_publications(&self) -> BoxFuture<'_, DbResult<Vec<DbPublicationDescriptor>>>;

    /// Return all subscriptions on this server.
    ///
    /// Source: PostgreSQL docs §54.50 — pg_subscription
    fn list_subscriptions(&self) -> BoxFuture<'_, DbResult<Vec<DbSubscriptionDescriptor>>>;

    /// Return streaming replication status rows from `pg_stat_replication`.
    ///
    /// Returns `(application_name, state)` pairs.
    ///
    /// Source: PostgreSQL docs §28.2 — pg_stat_replication
    fn streaming_replication_status(&self) -> BoxFuture<'_, DbResult<Vec<(String, String)>>>;
}
