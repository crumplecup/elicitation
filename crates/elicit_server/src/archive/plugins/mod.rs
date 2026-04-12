//! Archive workflow plugins.
//!
//! Each plugin composes calls to the existing `elicit_sqlx`, `elicit_polars`,
//! and `elicit_geo` tools — no direct driver calls are made here.

pub mod browse;
pub mod query;
pub mod render;
pub mod spatial;

pub use browse::{ArchiveBrowsePlugin, SchemaExists, TableExists};
pub use query::{ArchiveQueryPlugin, QueryExecuted};
pub use render::{AkNodeEntry, ArchiveDisplayPlugin};
pub use spatial::ArchiveSpatialPlugin;
