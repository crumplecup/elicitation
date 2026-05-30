//! Verified State Machines for the archive module.
//!
//! Four focused machines cover the full archive application lifecycle:
//!
//! | Machine | File | What it models |
//! |---|---|---|
//! | [`ArchiveConnectionMachine`] | `connection` | DB / KV connection lifecycle |
//! | [`ArchivePanelMachine`] | `panel` | Main content panel (18+ states, WCAG) |
//! | [`ArchiveNavMachine`] | `nav` | Nav tree browsing |
//! | [`ArchiveOverlayMachine`] | `overlay` | Modal overlays |
//!
//! ## WCAG contract
//!
//! Every [`ArchivePanelState`] variant that renders data carries the matching
//! `*Mode` from the `display` layer.  [`ArchivePanelConsistent`] is the
//! invariant that proves the panel is emitting a valid AccessKit node tree —
//! i.e. the UI is WCAG-compliant by construction.
//!
//! ## Machine chaining
//!
//! [`ArchiveConnectionMachine`] must be in a connected state before
//! [`ArchivePanelMachine`] and [`ArchiveNavMachine`] can leave their initial
//! states.  The connection proof token is threaded through to the inner
//! machines.

#[cfg(not(kani))]
mod connection;
#[cfg(kani)]
pub mod connection;
#[cfg(not(kani))]
mod nav;
#[cfg(kani)]
pub mod nav;
#[cfg(not(kani))]
mod overlay;
#[cfg(kani)]
pub mod overlay;
#[cfg(not(kani))]
mod panel;
#[cfg(kani)]
pub mod panel;

#[cfg(kani)]
pub use connection::archive_connection_consistent;
pub use connection::{
    ArchiveConnectionConsistent, ArchiveConnectionMachine, ArchiveConnectionState,
    begin_connect_kv, begin_connect_sql, connection_error, disconnect, finish_connect_kv,
    finish_connect_sql, reconnect,
};
#[cfg(kani)]
pub use nav::archive_nav_consistent;
pub use nav::{
    ArchiveNavConsistent, ArchiveNavMachine, ArchiveNavState, apply_filter, clear_filter,
    collapse_schema, expand_schema, load_nav, move_cursor_down, move_cursor_up, nav_loaded,
    nav_refresh,
};
#[cfg(kani)]
pub use overlay::archive_overlay_consistent;
pub use overlay::{
    ArchiveOverlayConsistent, ArchiveOverlayMachine, ArchiveOverlayState, close_overlay,
    open_export_picker, open_help, open_save_prompt, open_saved_browser, picker_move_down,
    picker_move_up, prompt_backspace, prompt_push, saved_browser_down, saved_browser_up,
};
#[cfg(kani)]
pub use panel::archive_panel_consistent;
pub use panel::{
    ArchivePanelConsistent, ArchivePanelMachine, ArchivePanelState, abort_edits, admin_ready,
    begin_edit, column_detail, commit_edits, constraints_ready, data_grid_ready, ddl_ready,
    erd_ready, explain_ready, export_ready, history_ready, indexes_ready, monitor_ready,
    open_connection_editor, open_export_panel, open_help_panel, open_saved_panel, open_sql_editor,
    panel_error, panel_loading, query_complete, saved_ready,
};
// BEGIN ELICITATION KANI REEXPORTS — DO NOT EDIT
pub use connection::begin_connect_kv_kani_contracted;
pub use connection::begin_connect_sql_kani_contracted;
pub use connection::connection_error_kani_contracted;
pub use connection::disconnect_kani_contracted;
pub use connection::finish_connect_kv_kani_contracted;
pub use connection::finish_connect_sql_kani_contracted;
pub use connection::reconnect_kani_contracted;
pub use nav::apply_filter_kani_contracted;
pub use nav::clear_filter_kani_contracted;
pub use nav::collapse_schema_kani_contracted;
pub use nav::expand_schema_kani_contracted;
pub use nav::load_nav_kani_contracted;
pub use nav::move_cursor_down_kani_contracted;
pub use nav::move_cursor_up_kani_contracted;
pub use nav::nav_loaded_kani_contracted;
pub use nav::nav_refresh_kani_contracted;
pub use overlay::close_overlay_kani_contracted;
pub use overlay::open_export_picker_kani_contracted;
pub use overlay::open_help_kani_contracted;
pub use overlay::open_save_prompt_kani_contracted;
pub use overlay::open_saved_browser_kani_contracted;
pub use overlay::picker_move_down_kani_contracted;
pub use overlay::picker_move_up_kani_contracted;
pub use overlay::prompt_backspace_kani_contracted;
pub use overlay::prompt_push_kani_contracted;
pub use overlay::saved_browser_down_kani_contracted;
pub use overlay::saved_browser_up_kani_contracted;
pub use panel::abort_edits_kani_contracted;
pub use panel::admin_ready_kani_contracted;
pub use panel::begin_edit_kani_contracted;
pub use panel::column_detail_kani_contracted;
pub use panel::commit_edits_kani_contracted;
pub use panel::constraints_ready_kani_contracted;
pub use panel::data_grid_ready_kani_contracted;
pub use panel::ddl_ready_kani_contracted;
pub use panel::erd_ready_kani_contracted;
pub use panel::explain_ready_kani_contracted;
pub use panel::export_ready_kani_contracted;
pub use panel::history_ready_kani_contracted;
pub use panel::indexes_ready_kani_contracted;
pub use panel::monitor_ready_kani_contracted;
pub use panel::open_connection_editor_kani_contracted;
pub use panel::open_export_panel_kani_contracted;
pub use panel::open_help_panel_kani_contracted;
pub use panel::open_saved_panel_kani_contracted;
pub use panel::open_sql_editor_kani_contracted;
pub use panel::panel_error_kani_contracted;
pub use panel::panel_loading_kani_contracted;
pub use panel::query_complete_kani_contracted;
pub use panel::saved_ready_kani_contracted;
// END ELICITATION KANI REEXPORTS
