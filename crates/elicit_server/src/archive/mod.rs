//! The `archive` module — a pgAdmin-style database manager built from the
//! elicit_* ecosystem.
//!
//! # Design
//!
//! Every data-retrieval operation is expressed as a **verified workflow
//! composition** using the existing `elicit_sqlx`, `elicit_polars`, and
//! `elicit_geo` plugins.  No direct calls to sqlx/polars/geo_types are made
//! here; the tool call chains are the implementation.  When native performance
//! is needed, the chains collapse to their Rust equivalents and the formal
//! proofs travel along for free.
//!
//! # Modules
//!
//! | Module | Purpose |
//! |--------|---------|
//! | [`types`] | `ElicitComplete` descriptor types for DB objects |
//! | [`errors`] | `ArchiveError` / `ArchiveErrorKind` |
//! | [`display`] | `ArchiveDisplay` trait + `DisplayMode` enums |
//! | [`plugins`] | Verified workflow plugins (`browse`, `query`, `spatial`, `render`) |

pub mod display;
mod errors;
pub mod plugins;
pub mod types;

pub use errors::{ArchiveError, ArchiveErrorKind, ArchiveResult};
pub use types::{
    BackendKind, ColumnDescriptor, DatabaseDescriptor, IndexDescriptor, QueryResult,
    SchemaDescriptor, TableDescriptor, TableType,
};
