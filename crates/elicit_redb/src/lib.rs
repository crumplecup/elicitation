//! Elicitation shadow crate for [redb](https://docs.rs/redb) 4.x.
//!
//! Provides a complete MCP tool surface for the redb embedded key-value store.
//! Live objects (`Database`, `WriteTransaction`, `ReadTransaction`, `Savepoint`)
//! are held in [`RedbPlugin`]'s shared context and identified by UUID across
//! tool calls.  Generic `Table<K, V>` operations are surfaced through a
//! compile-time factory: each supported `(K, V)` pair produces a dedicated set
//! of typed insert / get / remove / len / iter tools.
//!
//! # Plugins
//!
//! - [`RedbPlugin`] — single stateful plugin covering the entire redb API surface

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod backend;
mod db_tools;
mod multimap_tools;
mod plugin;
mod table_tools;
mod txn_tools;

pub use backend::RedbBackend;
pub use plugin::{RedbCtx, RedbPlugin};
