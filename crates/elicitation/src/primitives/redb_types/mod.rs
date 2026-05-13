//! Trenchcoat wrappers for redb public value types.
//!
//! Each type adds [`schemars::JsonSchema`] so it can cross the MCP boundary.
//! Bidirectional or one-way `From` conversions to/from upstream `redb` types
//! are provided when the `redb-types` feature is enabled.
//!
//! # Types
//!
//! - [`TypeName`] — shadow of `redb::TypeName`
//! - [`Durability`] — shadow of `redb::Durability`
//! - [`DatabaseStats`] — shadow of `redb::DatabaseStats`
//! - [`TableStats`] — shadow of `redb::TableStats`
//! - [`CacheStats`] — shadow of `redb::CacheStats`
//!
//! # Enabled by the `redb-types` feature

mod cache_stats;
mod database_stats;
mod durability;
mod table_stats;
mod type_name;

pub use cache_stats::CacheStats;
pub use database_stats::DatabaseStats;
pub use durability::Durability;
pub use table_stats::TableStats;
pub use type_name::TypeName;
