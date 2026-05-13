//! Tests for the archive Verified State Machines.
//!
//! Covers all four machines:
//! - [`ArchiveConnectionMachine`] — connection lifecycle
//! - [`ArchivePanelMachine`] — content panel + WCAG display modes
//! - [`ArchiveNavMachine`] — nav tree
//! - [`ArchiveOverlayMachine`] — modal overlays

use elicitation::{Established, VerifiedTransition};

use elicit_db::DbRows;
use elicit_server::archive::{
    BackendKind, DatabaseDescriptor, DdlDescriptor, QueryResult,
    display::{DdlDescriptorMode, QueryResultMode},
    nav_tree::NavTree,
    vsm::{
        // connection
        ArchiveConnectionConsistent,
        ArchiveConnectionMachine,
        ArchiveConnectionState,
        // nav
        ArchiveNavConsistent,
        ArchiveNavMachine,
        ArchiveNavState,
        // overlay
        ArchiveOverlayConsistent,
        ArchiveOverlayMachine,
        ArchiveOverlayState,
        // panel
        ArchivePanelConsistent,
        ArchivePanelMachine,
        ArchivePanelState,
        abort_edits,
        begin_connect_kv,
        begin_connect_sql,
        begin_edit,
        close_overlay,
        collapse_schema,
        commit_edits,
        data_grid_ready,
        ddl_ready,
        disconnect,
        expand_schema,
        finish_connect_kv,
        finish_connect_sql,
        load_nav,
        move_cursor_down,
        move_cursor_up,
        nav_loaded,
        open_export_picker,
        open_help,
        open_sql_editor,
        panel_error,
        panel_loading,
        query_complete,
        reconnect,
    },
};

// ── helpers ───────────────────────────────────────────────────────────────────

fn assert_conn_vt<T: VerifiedTransition<ArchiveConnectionMachine>>(_: &T) {}
fn assert_panel_vt<T: VerifiedTransition<ArchivePanelMachine>>(_: &T) {}
fn assert_nav_vt<T: VerifiedTransition<ArchiveNavMachine>>(_: &T) {}
fn assert_overlay_vt<T: VerifiedTransition<ArchiveOverlayMachine>>(_: &T) {}

fn conn_proof() -> Established<ArchiveConnectionConsistent> {
    Established::assert()
}

fn panel_proof() -> Established<ArchivePanelConsistent> {
    Established::assert()
}

fn nav_proof() -> Established<ArchiveNavConsistent> {
    Established::assert()
}

fn overlay_proof() -> Established<ArchiveOverlayConsistent> {
    Established::assert()
}

fn sample_db() -> DatabaseDescriptor {
    DatabaseDescriptor::new("postgres://localhost/testdb", "testdb", None)
}

fn empty_result() -> QueryResult {
    QueryResult {
        columns: vec![],
        rows: DbRows {
            rows: vec![],
            affected: 0,
        },
        row_count: 0,
        spatial_column_names: vec![],
    }
}

// ── BackendKind URL detection ─────────────────────────────────────────────────

#[test]
fn backend_kind_detects_redb_url() {
    assert_eq!(
        BackendKind::from_url("redb://data/archive.redb"),
        BackendKind::Redb
    );
    assert_eq!(BackendKind::from_url("redb:data.redb"), BackendKind::Redb);
}

#[test]
fn backend_kind_detects_existing_variants() {
    assert_eq!(
        BackendKind::from_url("postgres://localhost/db"),
        BackendKind::Postgres
    );
    assert_eq!(
        BackendKind::from_url("sqlite:./local.db"),
        BackendKind::Sqlite
    );
    assert_eq!(
        BackendKind::from_url("mysql://localhost/db"),
        BackendKind::Mysql
    );
    assert_eq!(
        BackendKind::from_url("http://unrecognised"),
        BackendKind::Unknown
    );
}

// ── ArchiveConnectionMachine ──────────────────────────────────────────────────

#[test]
fn connection_pure_transitions_satisfy_bound() {
    assert_conn_vt(&disconnect);
    assert_conn_vt(&reconnect);
}

#[test]
fn connection_sql_lifecycle() {
    let proof = conn_proof();
    let db = sample_db();

    let (state, proof) = begin_connect_sql(
        ArchiveConnectionState::Disconnected,
        proof,
        "prod".to_string(),
        BackendKind::Postgres,
    );
    assert!(matches!(state, ArchiveConnectionState::Connecting { .. }));

    let (state, proof) = finish_connect_sql(state, proof, db);
    assert!(matches!(state, ArchiveConnectionState::SqlConnected { .. }));

    let (state, _proof) = disconnect(state, proof);
    assert_eq!(state, ArchiveConnectionState::Disconnected);
}

#[test]
fn connection_kv_lifecycle() {
    let proof = conn_proof();

    let (state, proof) = begin_connect_kv(
        ArchiveConnectionState::Disconnected,
        proof,
        "redb://data/archive.redb".to_string(),
    );
    assert!(matches!(state, ArchiveConnectionState::Connecting { .. }));

    let (state, proof) = finish_connect_kv(state, proof, "redb://data/archive.redb".to_string());
    assert!(matches!(state, ArchiveConnectionState::KvConnected { .. }));

    let (state, _proof) = disconnect(state, proof);
    assert_eq!(state, ArchiveConnectionState::Disconnected);
}

// ── ArchivePanelMachine ───────────────────────────────────────────────────────

#[test]
fn panel_pure_transitions_satisfy_bound() {
    assert_panel_vt(&begin_edit);
    assert_panel_vt(&commit_edits);
    assert_panel_vt(&abort_edits);
}

#[test]
fn panel_sql_editor_lifecycle() {
    let proof = panel_proof();

    let (state, proof) = open_sql_editor(
        ArchivePanelState::ColumnDetail,
        proof,
        "SELECT 1".to_string(),
    );
    assert!(matches!(state, ArchivePanelState::SqlEditor { .. }));

    let (state, proof) = query_complete(state, proof, empty_result());
    assert!(
        matches!(
            state,
            ArchivePanelState::SqlEditor {
                result: Some(_),
                ..
            }
        ),
        "expected query result to be stored"
    );

    let (state, _proof) = panel_error(state, proof, "oops".to_string());
    assert!(matches!(state, ArchivePanelState::ErrorView { .. }));
}

#[test]
fn panel_data_grid_edit_lifecycle() {
    let proof = panel_proof();

    let (state, proof) = data_grid_ready(
        ArchivePanelState::ColumnDetail,
        proof,
        "public".to_string(),
        "users".to_string(),
        empty_result(),
        QueryResultMode::default(),
    );
    assert!(matches!(
        state,
        ArchivePanelState::DataGrid {
            edit_state: None,
            ..
        }
    ));

    let (state, proof) = begin_edit(state, proof);
    assert!(matches!(
        state,
        ArchivePanelState::DataGrid {
            edit_state: Some(_),
            ..
        }
    ));

    let (state, _proof) = commit_edits(state, proof);
    assert!(matches!(
        state,
        ArchivePanelState::DataGrid {
            edit_state: None,
            ..
        }
    ));
}

#[test]
fn panel_loading_then_ddl() {
    let proof = panel_proof();

    let (state, proof) = panel_loading(
        ArchivePanelState::ColumnDetail,
        proof,
        "public".to_string(),
        "users".to_string(),
    );
    assert!(matches!(state, ArchivePanelState::Loading { .. }));

    let ddl = DdlDescriptor {
        schema: "public".to_string(),
        object_name: "users".to_string(),
        ddl: "CREATE TABLE users (id SERIAL)".to_string(),
    };
    let (state, _proof) = ddl_ready(
        state,
        proof,
        "public".to_string(),
        "users".to_string(),
        ddl,
        DdlDescriptorMode::default(),
    );
    assert!(matches!(state, ArchivePanelState::DdlView { .. }));
}

// ── ArchiveNavMachine ─────────────────────────────────────────────────────────

#[test]
fn nav_pure_transitions_satisfy_bound() {
    assert_nav_vt(&move_cursor_up);
    // move_cursor_down takes max: usize, so test via closure
    assert_nav_vt(&(|s, p| move_cursor_down(s, p, 100)));
}

#[test]
fn nav_load_lifecycle() {
    let proof = nav_proof();

    let (state, proof) = load_nav(ArchiveNavState::NavUnloaded, proof);
    assert!(matches!(state, ArchiveNavState::NavLoading));

    let (state, proof) = nav_loaded(
        state,
        proof,
        NavTree {
            db_name: "testdb".to_string(),
            version: None,
            backend: BackendKind::Postgres,
            schemas: vec![],
        },
    );
    assert!(matches!(state, ArchiveNavState::NavReady { .. }));

    let (state, proof) = expand_schema(state, proof, 0, true);
    // still NavReady — index 0 with empty schemas is a no-op
    assert!(matches!(state, ArchiveNavState::NavReady { .. }));

    let (state, _proof) = collapse_schema(state, proof, 0);
    assert!(matches!(state, ArchiveNavState::NavReady { .. }));
}

// ── ArchiveOverlayMachine ─────────────────────────────────────────────────────

#[test]
fn overlay_pure_transitions_satisfy_bound() {
    assert_overlay_vt(&close_overlay);
    assert_overlay_vt(&open_help);
    // open_export_picker takes formats param — test via closure
    assert_overlay_vt(&(|s, p| open_export_picker(s, p, vec![])));
}

#[test]
fn overlay_help_lifecycle() {
    let proof = overlay_proof();

    let (state, proof) = open_help(ArchiveOverlayState::OverlayNone, proof);
    assert!(matches!(state, ArchiveOverlayState::HelpOpen));

    let (state, _proof) = close_overlay(state, proof);
    assert_eq!(state, ArchiveOverlayState::OverlayNone);
}

#[test]
fn overlay_export_picker_lifecycle() {
    let proof = overlay_proof();

    let (state, proof) = open_export_picker(ArchiveOverlayState::OverlayNone, proof, vec![]);
    assert!(matches!(
        state,
        ArchiveOverlayState::ExportPickerOpen { .. }
    ));

    let (state, _proof) = close_overlay(state, proof);
    assert_eq!(state, ArchiveOverlayState::OverlayNone);
}
