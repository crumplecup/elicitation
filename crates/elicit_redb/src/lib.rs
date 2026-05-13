//! MCP-compatible shadow of the [`redb`] embedded key-value store.
//!
//! All public types mirror their `redb` counterparts with identical names and
//! method/tool names.  UUID handles are used for types that cannot be cloned
//! (`Database`, `WriteTransaction`, `ReadTransaction`, `Savepoint`).
//!
//! # Shadow types
//!
//! | `redb` type              | `elicit_redb` shadow      | Strategy          |
//! |--------------------------|---------------------------|-------------------|
//! | `Database`               | [`Database`]              | UUID handle       |
//! | `WriteTransaction`       | [`WriteTransaction`]      | UUID handle       |
//! | `ReadTransaction`        | [`ReadTransaction`]       | UUID handle       |
//! | `Savepoint`              | [`Savepoint`]             | UUID handle       |
//! | `DatabaseStats`          | [`DatabaseStats`]         | Trenchcoat        |
//! | `TableStats`             | [`TableStats`]            | Trenchcoat        |
//! | `CacheStats`             | [`CacheStats`]            | Trenchcoat        |
//! | `StorageError`           | [`StorageError`]          | Trenchcoat        |
//! | `TableError`             | [`TableError`]            | Trenchcoat        |
//! | `DatabaseError`          | [`DatabaseError`]         | Trenchcoat        |
//! | `SavepointError`         | [`SavepointError`]        | Trenchcoat        |
//! | `CompactionError`        | [`CompactionError`]       | Trenchcoat        |
//! | `SetDurabilityError`     | [`SetDurabilityError`]    | Trenchcoat        |
//! | `TransactionError`       | [`TransactionError`]      | Trenchcoat        |
//! | `CommitError`            | [`CommitError`]           | Trenchcoat        |
//! | `Table<u64,u64>`         | [`TableU64U64`]           | UUID handle       |
//! | `Table<&str,&str>`       | [`TableStrStr`]           | UUID handle       |
//! | `ReadOnlyTable<u64,u64>` | [`ReadTableU64U64`]       | UUID handle       |
//! | `ReadOnlyTable<&str,&str>`| [`ReadTableStrStr`]      | UUID handle       |
//!
//! # Plugin
//!
//! Register [`RedbPlugin`] with your MCP server.  All `redb__*` tools share a
//! single [`RedbCtx`] context.
//!
//! # Typical MCP session
//!
//! ```text
//! database__create("my.db")             → Database
//! database__begin_write(db)             → WriteTransaction
//! write_txn__open_table_u64_u64(txn, "scores") → TableU64U64
//! table_u64_u64__insert(table, 1, 42)
//! write_txn__commit(txn)
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod backend;
mod database;
mod error;
mod plugin;
mod savepoint;
mod stats;
mod table_def;
mod transactions;

pub use backend::RedbBackend;
pub use database::Database;
pub use error::{
    CommitError, CompactionError, DatabaseError, SavepointError, SetDurabilityError, StorageError,
    TableError, TransactionError,
};
pub use plugin::{RedbCtx, RedbPlugin};
pub use savepoint::Savepoint;
pub use stats::{CacheStats, DatabaseStats, TableStats};
pub use table_def::{ReadTableStrStr, ReadTableU64U64, TableStrStr, TableU64U64};
pub use transactions::{ReadTransaction, WriteTransaction};
