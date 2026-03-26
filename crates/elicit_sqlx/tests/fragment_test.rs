//! Unit tests for `elicit_sqlx` fragment `EmitCode` implementations.
//!
//! No database required — these tests verify that each fragment produces
//! the correct token stream structure and declares the right crate deps.

use elicit_sqlx::fragments::{MigrateParams, QueryAsParams, QueryParams, QueryScalarParams};
use elicitation::emit_code::EmitCode;

// ── QueryParams ───────────────────────────────────────────────────────────────

#[test]
fn query_no_params_emits_macro_call() {
    let p = QueryParams {
        sql: "SELECT 1".to_string(),
        params: vec![],
    };
    let ts = p.emit_code().to_string();
    assert!(ts.contains("sqlx"), "expected sqlx in: {ts}");
    assert!(ts.contains("query"), "expected query in: {ts}");
    assert!(ts.contains("SELECT 1"), "expected sql literal in: {ts}");
}

#[test]
fn query_with_params_emits_bind_args() {
    let p = QueryParams {
        sql: "SELECT * FROM users WHERE id = $1".to_string(),
        params: vec!["user_id".to_string()],
    };
    let ts = p.emit_code().to_string();
    assert!(ts.contains("user_id"), "expected bind param in: {ts}");
}

#[test]
fn query_crate_dep_is_sqlx() {
    let p = QueryParams {
        sql: "SELECT 1".to_string(),
        params: vec![],
    };
    let deps = p.crate_deps();
    assert_eq!(deps.len(), 1);
    assert_eq!(deps[0].name, "sqlx");
    assert_eq!(deps[0].version, "0.8");
    assert!(deps[0].features.contains(&"runtime-tokio"));
}

// ── QueryAsParams ─────────────────────────────────────────────────────────────

#[test]
fn query_as_emits_type_and_sql() {
    let p = QueryAsParams {
        target_type: "UserRow".to_string(),
        sql: "SELECT id, name FROM users".to_string(),
        params: vec![],
    };
    let ts = p.emit_code().to_string();
    assert!(ts.contains("query_as"), "expected query_as in: {ts}");
    assert!(ts.contains("UserRow"), "expected type in: {ts}");
    assert!(
        ts.contains("SELECT id, name FROM users"),
        "expected sql in: {ts}"
    );
}

#[test]
fn query_as_with_params_emits_bindings() {
    let p = QueryAsParams {
        target_type: "Post".to_string(),
        sql: "SELECT * FROM posts WHERE author_id = $1".to_string(),
        params: vec!["author_id".to_string()],
    };
    let ts = p.emit_code().to_string();
    assert!(ts.contains("author_id"), "expected bind param in: {ts}");
}

// ── QueryScalarParams ─────────────────────────────────────────────────────────

#[test]
fn query_scalar_emits_correct_macro() {
    let p = QueryScalarParams {
        sql: "SELECT COUNT(*) FROM items".to_string(),
        params: vec![],
    };
    let ts = p.emit_code().to_string();
    assert!(
        ts.contains("query_scalar"),
        "expected query_scalar in: {ts}"
    );
    assert!(ts.contains("SELECT COUNT"), "expected sql in: {ts}");
}

// ── MigrateParams ─────────────────────────────────────────────────────────────

#[test]
fn migrate_emits_migrator_and_path() {
    let p = MigrateParams {
        migrations_path: "./migrations".to_string(),
        pool_var: "pool".to_string(),
    };
    let ts = p.emit_code().to_string();
    assert!(ts.contains("migrate"), "expected migrate in: {ts}");
    assert!(ts.contains("migrations"), "expected path in: {ts}");
    assert!(ts.contains("pool"), "expected pool var in: {ts}");
}

#[test]
fn migrate_crate_dep_is_sqlx() {
    let p = MigrateParams {
        migrations_path: "./migrations".to_string(),
        pool_var: "pool".to_string(),
    };
    let deps = p.crate_deps();
    assert_eq!(deps.len(), 1);
    assert_eq!(deps[0].name, "sqlx");
    assert!(deps[0].features.contains(&"migrate"));
}
