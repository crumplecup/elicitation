//! AccessKit display layer for archive descriptor types.
//!
//! Provides the [`ArchiveDisplay`] trait and per-type `DisplayMode` enums.
//! Each mode maps to a different accesskit node tree structure suitable for
//! different UI contexts (data grids, tree navigators, summary cards, etc.).

mod column;
mod database;
mod query_result;
mod schema;
mod table;
mod trait_def;

pub use column::ColumnDescriptorMode;
pub use database::DatabaseDescriptorMode;
pub use query_result::QueryResultMode;
pub use schema::SchemaDescriptorMode;
pub use table::TableDescriptorMode;
pub use trait_def::ArchiveDisplay;
