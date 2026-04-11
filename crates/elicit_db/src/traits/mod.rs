//! Trait re-exports and the [`DbBackend`] supertrait.

pub mod backup;
pub mod database;
pub mod index;
pub mod monitor;
pub mod query;
pub mod role;
pub mod schema;
pub mod server;
pub mod session;
pub mod table;
pub mod transaction;

pub use backup::DbBackupManager;
pub use database::DbDatabaseManager;
pub use index::DbIndexManager;
pub use monitor::DbMonitor;
pub use query::DbQueryExecutor;
pub use role::DbRoleManager;
pub use schema::DbSchemaManager;
pub use server::DbServerAdmin;
pub use session::DbSessionManager;
pub use table::DbTableManager;
pub use transaction::DbTransactor;

/// Complete database management backend — blanket supertrait.
///
/// Any type that implements all 11 sub-traits automatically implements
/// `DbBackend`. Use `dyn DbBackend` to accept any fully-capable implementation.
///
/// # Object safety
///
/// `DbBackend` is not itself object-safe (a supertrait of 11 traits), but each
/// individual sub-trait is object-safe and accepts `dyn SubTrait` directly.
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
        + Send
        + Sync
{
}
