//! Phase 1: Ledger workflow smoke test
//!
//! End-to-end validation of:
//! - Agent workflow composition (13 tool calls)
//! - emit_binary code generation
//! - cargo build compilation
//! - Binary execution
//! - HTTP validation with reqwest
//!
//! This test executes a single hardcoded transfer (Alice -100, Bob +100)
//! and validates the HTTP response. No JSON parsing, no routing - just
//! proof that the emit pipeline works.

use std::process::Command;
use std::time::Duration;

use elicitation::emit_code::{BinaryScaffold, EmitCode};
use serde_json::json;
use tempfile::tempdir;

/// Compose the 13-step ledger workflow as JSON tool call sequence.
///
/// Steps:
/// 1. sqlx_workflow__connect (SQLite in-memory)
/// 2. sqlx_workflow__execute (CREATE TABLE)
/// 3. sqlx_workflow__execute (INSERT Alice init)
/// 4. sqlx_workflow__execute (INSERT Bob init)
/// 5. tokio_net__tcp_listener_bind (localhost:8080)
/// 6. tokio_net__tcp_listener_accept
/// 7. tokio_net__tcp_stream_read (ignore request)
/// 8. sqlx_workflow__begin (transaction)
/// 9. sqlx_workflow__tx_execute (INSERT Alice debit)
/// 10. sqlx_workflow__tx_execute (INSERT Bob credit)
/// 11. sqlx_workflow__commit
/// 12. tokio_net__tcp_stream_write (HTTP response)
/// 13. tokio_net__tcp_stream_close
fn compose_ledger_workflow() -> Vec<serde_json::Value> {
    vec![
        // Step 1: Connect to SQLite in-memory
        json!({
            "tool": "sqlx_workflow__connect",
            "params": {
                "database_url": "sqlite::memory:",
                "max_connections": 1
            }
        }),
        // Step 2: CREATE TABLE
        json!({
            "tool": "sqlx_workflow__execute",
            "params": {
                "pool_id": "$step1.pool_id",
                "sql": r#"CREATE TABLE ledger_entries (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    account_name TEXT NOT NULL,
                    amount INTEGER NOT NULL,
                    transfer_id TEXT NOT NULL,
                    created_at INTEGER NOT NULL
                )"#
            }
        }),
        // Step 3: INSERT Alice account marker
        json!({
            "tool": "sqlx_workflow__execute",
            "params": {
                "pool_id": "$step1.pool_id",
                "sql": "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Alice', 0, 'init', 0)"
            }
        }),
        // Step 4: INSERT Bob account marker
        json!({
            "tool": "sqlx_workflow__execute",
            "params": {
                "pool_id": "$step1.pool_id",
                "sql": "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Bob', 0, 'init', 0)"
            }
        }),
        // Step 5: Bind TCP listener
        json!({
            "tool": "tokio_net__tcp_listener_bind",
            "params": {
                "addr": "127.0.0.1:8080"
            }
        }),
        // Step 6: Accept connection
        json!({
            "tool": "tokio_net__tcp_listener_accept",
            "params": {
                "listener_id": "$step5.listener_id"
            }
        }),
        // Step 7: Read request (we ignore the content in Phase 1)
        json!({
            "tool": "tokio_net__tcp_stream_read",
            "params": {
                "stream_id": "$step6.stream_id",
                "max_bytes": 1024
            }
        }),
        // Step 8: Begin transaction
        json!({
            "tool": "sqlx_workflow__begin",
            "params": {
                "pool_id": "$step1.pool_id"
            }
        }),
        // Step 9: INSERT Alice debit
        json!({
            "tool": "sqlx_workflow__tx_execute",
            "params": {
                "tx_id": "$step8.tx_id",
                "sql": "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Alice', -100, 'tx1', 1)"
            }
        }),
        // Step 10: INSERT Bob credit
        json!({
            "tool": "sqlx_workflow__tx_execute",
            "params": {
                "tx_id": "$step8.tx_id",
                "sql": "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Bob', 100, 'tx1', 1)"
            }
        }),
        // Step 11: Commit transaction
        json!({
            "tool": "sqlx_workflow__commit",
            "params": {
                "tx_id": "$step8.tx_id"
            }
        }),
        // Step 12: Write HTTP response
        json!({
            "tool": "tokio_net__tcp_stream_write",
            "params": {
                "stream_id": "$step6.stream_id",
                "data": "HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\n{\"status\":\"ok\"}"
            }
        }),
        // Step 13: Close stream
        json!({
            "tool": "tokio_net__tcp_stream_close",
            "params": {
                "stream_id": "$step6.stream_id"
            }
        }),
    ]
}

#[tokio::test]
#[ignore] // Run explicitly with: cargo test --test ledger_smoke_test -- --ignored
async fn test_ledger_workflow_smoke() {
    // Step 1: Compose workflow
    let workflow = compose_ledger_workflow();
    println!("Composed workflow with {} steps", workflow.len());

    // Step 2: Dispatch each step to EmitCode impl
    let output_dir = tempdir().expect("Failed to create temp dir");
    println!("Output directory: {}", output_dir.path().display());

    let mut steps: Vec<Box<dyn EmitCode>> = Vec::new();
    for (i, step_json) in workflow.iter().enumerate() {
        let tool = step_json["tool"].as_str().expect("Missing tool field");
        let params = step_json["params"].clone();

        match elicit_server::emit_dispatch(tool, params) {
            Ok(boxed) => steps.push(boxed),
            Err(e) => {
                panic!("Step {} ('{}') failed to deserialize: {}", i, tool, e);
            }
        }
    }
    println!("Dispatched {} steps to EmitCode impls", steps.len());

    // Step 3: Assemble into binary scaffold
    let workspace_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let mut scaffold = BinaryScaffold::new(steps, true); // with_tracing = true
    scaffold = scaffold.with_workspace_root(&workspace_root);

    // Step 4: Emit to disk
    let main_rs = scaffold
        .emit_to_disk(output_dir.path(), "ledger_test")
        .expect("Failed to write source");
    println!("Source written to: {}", main_rs.display());

    // Step 5: Compile the binary
    let binary_path =
        elicitation::emit_code::compile(output_dir.path()).expect("Compilation failed");
    println!("Binary compiled: {}", binary_path.display());

    // Step 6: Run binary in background
    let mut child = Command::new(&binary_path)
        .spawn()
        .expect("Failed to start binary");
    println!("Binary started with PID: {}", child.id());

    // Wait for server to be ready
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Step 7: Validate with HTTP client
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:8080/")
        .send()
        .await
        .expect("Request failed");

    assert_eq!(response.status(), 200, "Expected 200 OK");
    let body = response.text().await.expect("Failed to read response body");
    assert_eq!(body, r#"{"status":"ok"}"#, "Unexpected response body");
    println!("HTTP validation successful: {}", body);

    // Step 8: Cleanup
    child.kill().ok();
    println!("Test passed!");
}
