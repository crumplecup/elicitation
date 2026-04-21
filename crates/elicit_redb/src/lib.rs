//! Elicitation-enabled redb type wrappers.
//!
//! Provides MCP tools for [redb](https://docs.rs/redb) 4.x — a pure-Rust,
//! ACID, embedded key-value store using copy-on-write B-trees.
//!
//! - [`RedbDatabasePlugin`] — Database/Builder creation and management snippets
//! - [`RedbTablePlugin`] — TableDefinition, typed CRUD, iteration patterns
//! - [`RedbTransactionPlugin`] — stateful write-transaction builder
//! - [`RedbSavepointPlugin`] — savepoint create/restore/delete patterns
//! - [`RedbMultimapPlugin`] — MultimapTableDefinition and multimap CRUD
//! - [`RedbTypesPlugin`] — Key/Value/MutInPlaceValue trait implementation skeletons
//! - [`RedbBackendPlugin`] — StorageBackend implementation skeleton

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod plugins {
    pub mod backend_plugin;
    pub mod database_plugin;
    pub mod multimap_plugin;
    pub mod savepoint_plugin;
    pub mod table_plugin;
    pub mod txn_plugin;
    pub mod types_plugin;
}

pub use plugins::backend_plugin::RedbBackendPlugin;
pub use plugins::database_plugin::RedbDatabasePlugin;
pub use plugins::multimap_plugin::RedbMultimapPlugin;
pub use plugins::savepoint_plugin::RedbSavepointPlugin;
pub use plugins::table_plugin::RedbTablePlugin;
pub use plugins::txn_plugin::RedbTransactionPlugin;
pub use plugins::types_plugin::RedbTypesPlugin;
