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

mod connection;
mod nav;
mod overlay;
mod panel;

pub use connection::{
    ArchiveConnectionConsistent, ArchiveConnectionMachine, ArchiveConnectionState,
    begin_connect_kv, begin_connect_sql, connection_error, disconnect, finish_connect_kv,
    finish_connect_sql, reconnect,
};
#[cfg(kani)]
pub use connection::archive_connection_consistent;
pub use nav::{
    ArchiveNavConsistent, ArchiveNavMachine, ArchiveNavState, apply_filter, clear_filter,
    collapse_schema, expand_schema, load_nav, move_cursor_down, move_cursor_up, nav_loaded,
    nav_refresh,
};
#[cfg(kani)]
pub use nav::archive_nav_consistent;
pub use overlay::{
    ArchiveOverlayConsistent, ArchiveOverlayMachine, ArchiveOverlayState, close_overlay,
    open_export_picker, open_help, open_save_prompt, open_saved_browser, picker_move_down,
    picker_move_up, prompt_backspace, prompt_push, saved_browser_down, saved_browser_up,
};
#[cfg(kani)]
pub use overlay::archive_overlay_consistent;
pub use panel::{
    ArchivePanelConsistent, ArchivePanelMachine, ArchivePanelState, abort_edits, admin_ready,
    begin_edit, column_detail, commit_edits, constraints_ready, data_grid_ready, ddl_ready,
    erd_ready, explain_ready, export_ready, history_ready, indexes_ready, monitor_ready,
    open_connection_editor, open_export_panel, open_help_panel, open_saved_panel, open_sql_editor,
    panel_error, panel_loading, query_complete, saved_ready,
};
#[cfg(kani)]
pub use panel::archive_panel_consistent;
