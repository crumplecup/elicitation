//! Archive workflow plugins.
//!
//! Each plugin composes calls to the existing `elicit_sqlx`, `elicit_polars`,
//! and `elicit_geo` tools — no direct driver calls are made here.

pub mod browse;
pub mod export;
pub mod history;
pub mod inspect;
pub mod query;
pub mod render;
pub mod saved;
pub mod spatial;

pub use browse::{ArchiveBrowsePlugin, SchemaExists, TableExists};
pub use export::export_query_result;
pub use history::HistoryStore;
pub use inspect::{
    explain_sql_direct, generate_ddl_direct, get_column_stats_direct, inspect_table_direct,
};
pub use query::{ArchiveQueryPlugin, QueryExecuted};
pub use render::{AkNodeEntry, ArchiveDisplayPlugin};
pub use saved::SavedQueryStore;
pub use spatial::ArchiveSpatialPlugin;
