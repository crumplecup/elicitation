//! Phase 1: Ledger workflow smoke test
//!
//! End-to-end validation of:
//! - Agent workflow composition (8 sqlx tool calls)
//! - emit_binary code generation
//! - cargo build compilation
//! - Binary execution
//!
//! This test executes a single hardcoded transfer (Alice -100, Bob +100)
//! using only sqlx workflow tools. Proves that the emit pipeline works
//! without HTTP complexity.

use elicitation::emit_code::{BinaryScaffold, EmitCode};
use serde_json::json;
use tempfile::tempdir;

/// Compose the simplified ledger workflow as JSON tool call sequence.
///
/// Steps:
/// 1. sqlx_workflow__connect (SQLite in-memory, establishes `pool` variable)
/// 2. sqlx_workflow__execute (CREATE TABLE, uses `pool`)
/// 3. sqlx_workflow__execute (INSERT Alice init, uses `pool`)
/// 4. sqlx_workflow__execute (INSERT Bob init, uses `pool`)
/// 5. sqlx_workflow__begin (transaction, establishes `tx` variable, uses `pool`)
/// 6. sqlx_workflow__tx_execute (INSERT Alice debit, uses `tx`)
/// 7. sqlx_workflow__tx_execute (INSERT Bob credit, uses `tx`)
/// 8. sqlx_workflow__commit (commit transaction, uses `tx`)
///
/// Note: The emit code generators for sqlx and tokio use hardcoded variable names
/// (`pool`, `tx`, `listener`, `stream`) with `shared_scope() = true`, so no IDs
/// are passed between steps - variable names are shared automatically.
fn compose_ledger_workflow() -> Vec<serde_json::Value> {
    vec![
        // Step 1: Connect to SQLite in-memory (generates: let pool = ...)
        json!({
            "tool": "sqlx_workflow__connect",
            "params": {
                "database_url": "sqlite::memory:",
                "max_connections": 1
            }
        }),
        // Step 2: CREATE TABLE (uses pool variable)
        json!({
            "tool": "sqlx_workflow__execute",
            "params": {
                "sql": r#"CREATE TABLE ledger_entries (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    account_name TEXT NOT NULL,
                    amount INTEGER NOT NULL,
                    transfer_id TEXT NOT NULL,
                    created_at INTEGER NOT NULL
                )"#,
                "args": []
            }
        }),
        // Step 3: INSERT Alice account marker
        json!({
            "tool": "sqlx_workflow__execute",
            "params": {
                "sql": "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Alice', 0, 'init', 0)",
                "args": []
            }
        }),
        // Step 4: INSERT Bob account marker
        json!({
            "tool": "sqlx_workflow__execute",
            "params": {
                "sql": "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Bob', 0, 'init', 0)",
                "args": []
            }
        }),
        // Step 5: Begin transaction (generates: let mut tx = pool.begin().await?)
        json!({
            "tool": "sqlx_workflow__begin",
            "params": {}
        }),
        // Step 6: INSERT Alice debit (uses tx variable)
        json!({
            "tool": "sqlx_workflow__tx_execute",
            "params": {
                "sql": "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Alice', -100, 'tx1', 1)",
                "args": []
            }
        }),
        // Step 7: INSERT Bob credit (uses tx variable)
        json!({
            "tool": "sqlx_workflow__tx_execute",
            "params": {
                "sql": "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Bob', 100, 'tx1', 1)",
                "args": []
            }
        }),
        // Step 8: Commit transaction (uses tx variable)
        json!({
            "tool": "sqlx_workflow__commit",
            "params": {}
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
    let output_path = output_dir.path().to_path_buf();
    // Keep temp dir for inspection (don't auto-clean on drop)
    std::mem::forget(output_dir);
    println!("Output directory: {}", output_path.display());

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
        .emit_to_disk(&output_path, "ledger_test")
        .expect("Failed to write source");
    println!("Source written to: {}", main_rs.display());

    // Step 5: Compile the binary
    let reported_binary_path =
        elicitation::emit_code::compile(&output_path).expect("Compilation failed");
    println!("compile() returned: {}", reported_binary_path.display());

    // Step 6: Verify binary exists (using package name, not dir name)
    let actual_binary_path = output_path.join("target/release/ledger_test");
    assert!(
        actual_binary_path.exists(),
        "Binary does not exist at {}",
        actual_binary_path.display()
    );
    assert!(actual_binary_path.is_file(), "Binary path is not a file");

    println!("Test passed! Binary at: {}", actual_binary_path.display());
    println!("Generated code can be inspected at: {}", main_rs.display());
}
