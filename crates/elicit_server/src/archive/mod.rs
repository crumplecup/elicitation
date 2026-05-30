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
//! | `errors` | `ArchiveError` / `ArchiveErrorKind` |
//! | [`display`] | `ArchiveDisplay` trait + `DisplayMode` enums |
//! | `plugins` | Verified workflow plugins (`browse`, `query`, `spatial`, `render`) |

pub mod actions;
mod backend;
pub mod display;
#[cfg(not(kani))]
pub mod egui_frontend;
mod errors;
pub mod frontend_trait;
pub mod frontend_utils;
#[cfg(not(kani))]
pub mod leptos_frontend;
pub mod nav_model;
pub mod nav_tree;
mod plugins;
#[cfg(not(kani))]
pub mod ratatui_frontend;
pub mod types;
pub mod vsm;

pub use backend::{ArchiveDbBackend, ArchiveKvBackend};

pub use actions::{ArchiveAction, ArchiveKey, ArchiveKeyMap, KeyCombo, KeyMapEntry, KeyMapMode};
#[cfg(not(kani))]
pub use egui_frontend::run_egui;
pub use errors::{ArchiveError, ArchiveErrorKind, ArchiveResult};
pub use frontend_trait::ArchiveFrontend;
pub use frontend_utils::nav_tree_to_verified_tree;
pub use nav_model::ConnectionSet;
pub use nav_tree::{NavTree, SchemaEntry, build_nav_tree};
pub use plugins::{
    AkNodeEntry, ArchiveAdminPlugin, ArchiveBrowsePlugin, ArchiveConstraintPlugin,
    ArchiveDisplayPlugin, ArchiveKvPlugin, ArchiveMonitorPlugin, ArchiveReplicationPlugin,
    ArchiveRoutinePlugin, ArchiveSecurityPlugin, ArchiveSpatialPlugin, HistoryStore,
    SavedQueryStore, SchemaExists, TableExists, explain_sql_direct, generate_ddl_direct,
    get_column_stats_direct, inspect_table_direct,
};
#[cfg(not(kani))]
pub use plugins::{ArchiveQueryPlugin, QueryExecuted, export_query_result};
pub use types::{
    AdminSnapshot, AdminTab, BackendKind, ColumnDescriptor, ColumnStats, CompositeTypeAttribute,
    CompositeTypeDescriptor, ConnectionProfile, ConstraintDescriptor, ConstraintKind,
    DatabaseDescriptor, DdlDescriptor, DomainDescriptor, EnumDescriptor, ErdColumn, ErdDiagram,
    ErdEdge, ErdLayout, ErdNode, ExplainComparison, ExplainNode, ExplainPlan, ExportFormat,
    ExportResult, FkAction, ForeignKeyDescriptor, FunctionDescriptor, FunctionVolatility,
    IndexDescriptor, KvEntryDescriptor, KvScanResult, KvSnapshotDescriptor, KvStatsDescriptor,
    KvTableDescriptor, MonitorSnapshot, MonitorTab, QueryHistoryEntry, QueryResult, RowEditKind,
    RowEditState, SavedQuery, SchemaDescriptor, SequenceDescriptor, SslMode, StagedEdit,
    TableDescriptor, TableInspection, TableType, TriggerDescriptor, TriggerEvents,
};
pub use vsm::{
    ArchiveConnectionConsistent,
    // Connection machine
    ArchiveConnectionMachine,
    ArchiveConnectionState,
    // Nav machine
    ArchiveNavConsistent,
    ArchiveNavMachine,
    ArchiveNavState,
    // Overlay machine
    ArchiveOverlayConsistent,
    ArchiveOverlayMachine,
    ArchiveOverlayState,
    // Panel machine
    ArchivePanelConsistent,
    ArchivePanelMachine,
    ArchivePanelState,
    abort_edits,
    admin_ready,
    apply_filter,
    begin_connect_kv,
    begin_connect_sql,
    begin_edit,
    clear_filter,
    close_overlay,
    collapse_schema,
    column_detail,
    commit_edits,
    connection_error,
    constraints_ready,
    data_grid_ready,
    ddl_ready,
    disconnect,
    erd_ready,
    expand_schema,
    explain_ready,
    export_ready,
    finish_connect_kv,
    finish_connect_sql,
    history_ready,
    indexes_ready,
    load_nav,
    monitor_ready,
    move_cursor_down,
    move_cursor_up,
    nav_loaded,
    nav_refresh,
    open_connection_editor,
    open_export_panel,
    open_export_picker,
    open_help,
    open_help_panel,
    open_save_prompt,
    open_saved_browser,
    open_saved_panel,
    open_sql_editor,
    panel_error,
    panel_loading,
    picker_move_down,
    picker_move_up,
    prompt_backspace,
    prompt_push,
    query_complete,
    reconnect,
    saved_browser_down,
    saved_browser_up,
    saved_ready,
};
// BEGIN ELICITATION KANI REEXPORTS — DO NOT EDIT
pub use vsm::abort_edits_kani_contracted;
pub use vsm::admin_ready_kani_contracted;
pub use vsm::apply_filter_kani_contracted;
pub use vsm::archive_connection_consistent;
pub use vsm::archive_nav_consistent;
pub use vsm::archive_overlay_consistent;
pub use vsm::archive_panel_consistent;
pub use vsm::begin_connect_kv_kani_contracted;
pub use vsm::begin_connect_sql_kani_contracted;
pub use vsm::begin_edit_kani_contracted;
pub use vsm::clear_filter_kani_contracted;
pub use vsm::close_overlay_kani_contracted;
pub use vsm::collapse_schema_kani_contracted;
pub use vsm::column_detail_kani_contracted;
pub use vsm::commit_edits_kani_contracted;
pub use vsm::connection_error_kani_contracted;
pub use vsm::constraints_ready_kani_contracted;
pub use vsm::data_grid_ready_kani_contracted;
pub use vsm::ddl_ready_kani_contracted;
pub use vsm::disconnect_kani_contracted;
pub use vsm::erd_ready_kani_contracted;
pub use vsm::expand_schema_kani_contracted;
pub use vsm::explain_ready_kani_contracted;
pub use vsm::export_ready_kani_contracted;
pub use vsm::finish_connect_kv_kani_contracted;
pub use vsm::finish_connect_sql_kani_contracted;
pub use vsm::history_ready_kani_contracted;
pub use vsm::indexes_ready_kani_contracted;
pub use vsm::load_nav_kani_contracted;
pub use vsm::monitor_ready_kani_contracted;
pub use vsm::move_cursor_down_kani_contracted;
pub use vsm::move_cursor_up_kani_contracted;
pub use vsm::nav_loaded_kani_contracted;
pub use vsm::nav_refresh_kani_contracted;
pub use vsm::open_connection_editor_kani_contracted;
pub use vsm::open_export_panel_kani_contracted;
pub use vsm::open_export_picker_kani_contracted;
pub use vsm::open_help_kani_contracted;
pub use vsm::open_help_panel_kani_contracted;
pub use vsm::open_save_prompt_kani_contracted;
pub use vsm::open_saved_browser_kani_contracted;
pub use vsm::open_saved_panel_kani_contracted;
pub use vsm::open_sql_editor_kani_contracted;
pub use vsm::panel_error_kani_contracted;
pub use vsm::panel_loading_kani_contracted;
pub use vsm::picker_move_down_kani_contracted;
pub use vsm::picker_move_up_kani_contracted;
pub use vsm::prompt_backspace_kani_contracted;
pub use vsm::prompt_push_kani_contracted;
pub use vsm::query_complete_kani_contracted;
pub use vsm::reconnect_kani_contracted;
pub use vsm::saved_browser_down_kani_contracted;
pub use vsm::saved_browser_up_kani_contracted;
pub use vsm::saved_ready_kani_contracted;
// END ELICITATION KANI REEXPORTS
