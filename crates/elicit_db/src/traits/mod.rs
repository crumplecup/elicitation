//! Trait re-exports and the [`DbBackend`] supertrait.
//!
//! # Three-role taxonomy
//!
//! The database trait interface partitions operations into three orthogonal
//! roles, mirroring the design used in `elicit_gis` and `elicit_ui`:
//!
//! | Role | Description | Return type | Example traits |
//! |------|-------------|-------------|----------------|
//! | **1a** (leaf factory) | Takes a descriptor; asserts that the operation satisfies a specific database invariant. Produces fresh proof tokens. | `DbResult<(Descriptor, Established<P>)>` | `DbTableManager`, `DbRoutineFactory`, `DbConstraintFactory`, `DbIsolationFactory` |
//! | **1b** (section factory) | Takes an evidence bundle of upstream `Established<P>` tokens; mints an aggregate proof. Enforces sequential proof composition at the type level. | `DbResult<(Descriptor, Established<P>)>` | (see `contracts::proof_composition`) |
//! | **2** (reporter) | Queries backend state; no proof tokens consumed or produced. Independent of validity assertions. | `BoxFuture<'_, DbResult<T>>` | `DbMonitor`, `DbRoutineMeta`, `DbReplicationMeta`, `DbSecurityMeta` |
//!
//! # `DbBackend` supertrait
//!
//! [`DbBackend`] is the aggregate supertrait a fully-capable backend must satisfy.
//! It has a blanket impl: any type that implements all 20 sub-traits automatically
//! implements `DbBackend`.
//!
//! Use the individual object-safe sub-traits (`dyn DbTableManager`,
//! `dyn DbConstraintFactory`, etc.) for dynamic dispatch at architectural
//! boundaries.

mod backup;
mod constraint;
mod database;
mod index;
mod isolation;
mod monitor;
mod query;
mod replication;
mod role;
mod routine;
mod schema;
mod security;
mod server;
mod session;
mod table;
mod transaction;

pub use backup::DbBackupManager;
pub use constraint::{DbConstraintFactory, DbConstraintMeta};
pub use database::DbDatabaseManager;
pub use index::DbIndexManager;
pub use isolation::DbIsolationFactory;
pub use monitor::DbMonitor;
pub use query::DbQueryExecutor;
pub use replication::{DbReplicationFactory, DbReplicationMeta};
pub use role::DbRoleManager;
pub use routine::{DbRoutineFactory, DbRoutineMeta};
pub use schema::DbSchemaManager;
pub use security::{DbSecurityFactory, DbSecurityMeta};
pub use server::DbServerAdmin;
pub use session::DbSessionManager;
pub use table::DbTableManager;
pub use transaction::DbTransactor;

/// Complete database management backend — blanket supertrait.
///
/// Any type that implements all 20 sub-traits automatically implements
/// `DbBackend`. Use `dyn DbBackend` to accept any fully-capable implementation,
/// or constrain generics with `T: DbBackend`.
///
/// # Object safety
///
/// `DbBackend` is not itself object-safe (a supertrait of 20 traits), but each
/// individual sub-trait is object-safe and accepts `dyn SubTrait` directly.
///
/// # Proof composition
///
/// Factory traits return `Established<P>` tokens that can be composed into
/// aggregate proofs via the `ProvableFrom` chains in
/// `contracts::proof_composition`.
pub trait DbBackend:
    DbSessionManager
    + DbServerAdmin
    + DbDatabaseManager
    + DbSchemaManager
    + DbTableManager
    + DbQueryExecutor
    + DbTransactor
    + DbIndexManager
    + DbRoleManager
    + DbMonitor
    + DbBackupManager
    + DbRoutineFactory
    + DbRoutineMeta
    + DbReplicationFactory
    + DbReplicationMeta
    + DbSecurityFactory
    + DbSecurityMeta
    + DbConstraintFactory
    + DbConstraintMeta
    + DbIsolationFactory
    + Send
    + Sync
{
}

impl<T> DbBackend for T where
    T: DbSessionManager
        + DbServerAdmin
        + DbDatabaseManager
        + DbSchemaManager
        + DbTableManager
        + DbQueryExecutor
        + DbTransactor
        + DbIndexManager
        + DbRoleManager
        + DbMonitor
        + DbBackupManager
        + DbRoutineFactory
        + DbRoutineMeta
        + DbReplicationFactory
        + DbReplicationMeta
        + DbSecurityFactory
        + DbSecurityMeta
        + DbConstraintFactory
        + DbConstraintMeta
        + DbIsolationFactory
        + Send
        + Sync
{
}
