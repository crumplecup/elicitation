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

pub mod actions;
mod backend;
pub mod display;
pub mod egui_frontend;
mod errors;
pub mod frontend_trait;
pub mod frontend_utils;
pub mod leptos_frontend;
pub mod nav_model;
pub mod nav_tree;
mod plugins;
pub mod ratatui_frontend;
pub mod types;

pub use backend::ArchiveDbBackend;

pub use actions::{ArchiveAction, ArchiveKey, ArchiveKeyMap, KeyCombo, KeyMapEntry, KeyMapMode};
pub use egui_frontend::run_egui;
pub use errors::{ArchiveError, ArchiveErrorKind, ArchiveResult};
pub use frontend_trait::ArchiveFrontend;
pub use frontend_utils::nav_tree_to_verified_tree;
pub use nav_model::ConnectionSet;
pub use nav_tree::{NavTree, SchemaEntry, build_nav_tree};
pub use plugins::{
    AkNodeEntry, ArchiveAdminPlugin, ArchiveBrowsePlugin, ArchiveConstraintPlugin,
    ArchiveDisplayPlugin, ArchiveMonitorPlugin, ArchiveQueryPlugin, ArchiveReplicationPlugin,
    ArchiveRoutinePlugin, ArchiveSecurityPlugin, ArchiveSpatialPlugin, HistoryStore, QueryExecuted,
    SavedQueryStore, SchemaExists, TableExists, explain_sql_direct, export_query_result,
    generate_ddl_direct, get_column_stats_direct, inspect_table_direct,
};
pub use types::{
    AdminSnapshot, AdminTab, BackendKind, ColumnDescriptor, ColumnStats, CompositeTypeAttribute,
    CompositeTypeDescriptor, ConnectionProfile, ConstraintDescriptor, ConstraintKind,
    DatabaseDescriptor, DdlDescriptor, DomainDescriptor, EnumDescriptor, ErdColumn, ErdDiagram,
    ErdEdge, ErdLayout, ErdNode, ExplainNode, ExportFormat, ExportResult, FkAction,
    ForeignKeyDescriptor, FunctionDescriptor, FunctionVolatility, IndexDescriptor, MonitorSnapshot,
    MonitorTab, QueryHistoryEntry, QueryResult, RowEditKind, RowEditState, SavedQuery,
    SchemaDescriptor, SequenceDescriptor, SslMode, StagedEdit, TableDescriptor, TableInspection,
    TableType, TriggerDescriptor, TriggerEvents,
};
