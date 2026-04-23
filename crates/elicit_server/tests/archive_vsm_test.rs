//! Tests for the archive [`VerifiedStateMachine`].
//!
//! Verifies that:
//! - [`ArchiveState`] satisfies `ElicitComplete`.
//! - The pure 2-param transitions satisfy [`VerifiedTransition<ArchiveMachine>`].
//! - Parameterised transitions wrapped in closures also satisfy the bound.
//! - Proof tokens carry the [`ArchiveConsistent`] invariant through a multi-step
//!   lifecycle: `Disconnected → Connecting → SqlConnected → Browsing → RunningQuery
//!   → ViewingResults → EditingRows → ViewingResults → Exporting → Browsing → Disconnected`.
//! - `BackendKind::from_url` recognises `redb://` paths.

use elicitation::{Established, VerifiedTransition};

use elicit_server::archive::{
    ArchiveConnectionCredential, ArchiveConsistent, ArchiveMachine, ArchiveState, BackendKind,
    DatabaseDescriptor, ExportFormat,
    vsm::{
        begin_browse, begin_connect_sql, begin_edit, begin_export, commit_edits, disconnect,
        execute_query, finish_connect_kv, finish_connect_sql, finish_export, query_complete,
    },
};

// ── helpers ───────────────────────────────────────────────────────────────────

fn assert_vt<T: VerifiedTransition<ArchiveMachine>>(_: &T) {}

fn bootstrap() -> Established<ArchiveConsistent> {
    // SAFETY (audit trail): initial state is Disconnected — trivially consistent.
    Established::assert()
}

fn sample_db() -> DatabaseDescriptor {
    DatabaseDescriptor::new("postgres://localhost/testdb", "testdb", None)
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

// ── Pure transitions satisfy VerifiedTransition ───────────────────────────────

#[test]
fn pure_transitions_satisfy_verified_transition_bound() {
    assert_vt(&disconnect);
    assert_vt(&query_complete);
    assert_vt(&begin_edit);
    assert_vt(&commit_edits);
    assert_vt(&finish_export);
}

// ── Closures over parameterised transitions satisfy the bound ──────────────────

#[test]
fn closure_transitions_satisfy_verified_transition_bound() {
    let db = sample_db();
    let t_begin = |s, p| begin_connect_sql(s, p, "my-profile".to_string(), BackendKind::Postgres);
    let t_finish_sql = |s, p| finish_connect_sql(s, p, db.clone());
    let t_finish_kv = |s, p| finish_connect_kv(s, p, "redb://data/archive.redb".to_string());
    let t_browse = |s, p| begin_browse(s, p, Some("public".to_string()));
    let t_query = |s, p| execute_query(s, p, "SELECT 1".to_string());
    let t_export = |s, p| begin_export(s, p, ExportFormat::Csv);

    assert_vt(&t_begin);
    assert_vt(&t_finish_sql);
    assert_vt(&t_finish_kv);
    assert_vt(&t_browse);
    assert_vt(&t_query);
    assert_vt(&t_export);
}

// ── Full lifecycle round-trip ─────────────────────────────────────────────────

#[test]
fn sql_lifecycle_round_trip_carries_invariant() {
    let proof = bootstrap();
    let db = sample_db();

    // Disconnected → Connecting
    let (state, proof) = begin_connect_sql(
        ArchiveState::Disconnected,
        proof,
        "prod".to_string(),
        BackendKind::Postgres,
    );
    assert!(matches!(state, ArchiveState::Connecting { .. }));

    // Connecting → SqlConnected
    let (state, proof) = finish_connect_sql(state, proof, db.clone());
    assert!(matches!(state, ArchiveState::SqlConnected { .. }));

    // SqlConnected → Browsing
    let (state, proof) = begin_browse(state, proof, Some("public".to_string()));
    assert!(matches!(state, ArchiveState::Browsing { .. }));

    // Browsing → RunningQuery
    let (state, proof) = execute_query(state, proof, "SELECT * FROM users".to_string());
    assert!(matches!(state, ArchiveState::RunningQuery { .. }));

    // RunningQuery → ViewingResults
    let (state, proof) = query_complete(state, proof);
    assert!(matches!(state, ArchiveState::ViewingResults { .. }));

    // ViewingResults → EditingRows
    let (state, proof) = begin_edit(state, proof);
    assert!(matches!(state, ArchiveState::EditingRows { .. }));

    // EditingRows → ViewingResults
    let (state, proof) = commit_edits(state, proof);
    assert!(matches!(state, ArchiveState::ViewingResults { .. }));

    // ViewingResults → Exporting
    let (state, proof) = begin_export(state, proof, ExportFormat::Csv);
    assert!(matches!(state, ArchiveState::Exporting { .. }));

    // Exporting → Browsing
    let (state, proof) = finish_export(state, proof);
    assert!(matches!(state, ArchiveState::Browsing { .. }));

    // Browsing → Disconnected
    let (state, _proof) = disconnect(state, proof);
    assert_eq!(state, ArchiveState::Disconnected);
}

#[test]
fn kv_lifecycle_carries_invariant() {
    let cred = ArchiveConnectionCredential;
    let proof = Established::prove(&cred);

    // Disconnected → KvConnected
    let (state, proof) = finish_connect_kv(
        ArchiveState::Disconnected,
        proof,
        "redb://data/archive.redb".to_string(),
    );
    assert!(matches!(state, ArchiveState::KvConnected { .. }));

    // KvConnected → Disconnected
    let (state, _proof) = disconnect(state, proof);
    assert_eq!(state, ArchiveState::Disconnected);
}

// ── Invalid transitions are no-ops ────────────────────────────────────────────

#[test]
fn begin_edit_on_non_results_is_noop() {
    let proof = bootstrap();
    let browsing = ArchiveState::Browsing {
        db: sample_db(),
        selected_schema: None,
    };
    // begin_edit only moves ViewingResults → EditingRows; all others are no-ops
    let (state, _) = begin_edit(browsing.clone(), proof);
    assert_eq!(state, browsing);
}

#[test]
fn query_complete_on_non_running_is_noop() {
    let proof = bootstrap();
    let disconnected = ArchiveState::Disconnected;
    let (state, _) = query_complete(disconnected, proof);
    assert_eq!(state, ArchiveState::Disconnected);
}
