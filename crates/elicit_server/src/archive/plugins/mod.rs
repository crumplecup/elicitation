//! Archive workflow plugins.
//!
//! Each plugin composes calls to the existing `elicit_sqlx`, `elicit_polars`,
//! and `elicit_geo` tools — no direct driver calls are made here.

pub mod admin;
pub mod browse;
pub mod constraint;
#[cfg(not(kani))]
pub mod export;
pub mod history;
pub mod inspect;
pub mod monitor;
#[cfg(not(kani))]
pub mod query;
pub mod render;
pub mod replication;
pub mod routine;
pub mod saved;
pub mod security;
pub mod spatial;

pub use admin::ArchiveAdminPlugin;
pub use browse::{ArchiveBrowsePlugin, SchemaExists, TableExists};
pub use constraint::ArchiveConstraintPlugin;
#[cfg(not(kani))]
pub use export::export_query_result;
pub use history::HistoryStore;
pub use inspect::{
    explain_sql_direct, generate_ddl_direct, get_column_stats_direct, inspect_table_direct,
};
pub use monitor::ArchiveMonitorPlugin;
#[cfg(not(kani))]
pub use query::{ArchiveQueryPlugin, QueryExecuted};
pub use render::{AkNodeEntry, ArchiveDisplayPlugin};
pub use replication::ArchiveReplicationPlugin;
pub use routine::ArchiveRoutinePlugin;
pub use saved::SavedQueryStore;
pub use security::ArchiveSecurityPlugin;
pub use spatial::ArchiveSpatialPlugin;
