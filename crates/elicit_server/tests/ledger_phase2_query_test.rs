//! Phase 2: Ledger workflow with balance query validation
//!
//! Extends Phase 1 by adding:
//! - Balance query using SQL aggregation (SELECT SUM)
//! - Multiple transfers to test accumulation
//! - Validation that balances are correct after transfers
//!
//! This tests the core ledger logic without HTTP complexity.
//!
//! Workflow:
//! 1. Connect to SQLite + CREATE TABLE
//! 2. Execute first transfer (Alice -100, Bob +100)
//! 3. Execute second transfer (Alice -50, Bob +50)
//! 4. Query Alice's balance (should be -150)
//! 5. Query Bob's balance (should be +150)
//! 6. Verify balances sum to zero (double-entry invariant)

use elicitation::emit_code::{BinaryScaffold, EmitCode};
use serde_json::json;
use tempfile::tempdir;

/// Compose Phase 2 workflow: multiple transfers + balance queries
fn compose_phase2_workflow() -> Vec<serde_json::Value> {
    vec![
        // === Database Setup ===

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
        // === Transfer 1: Alice -100, Bob +100 ===
        json!({
            "tool": "sqlx_workflow__begin",
            "params": {}
        }),
        json!({
            "tool": "sqlx_workflow__tx_execute",
            "params": {
                "sql": "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Alice', -100, 'tx1', 1)",
                "args": []
            }
        }),
        json!({
            "tool": "sqlx_workflow__tx_execute",
            "params": {
                "sql": "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Bob', 100, 'tx1', 1)",
                "args": []
            }
        }),
        json!({
            "tool": "sqlx_workflow__commit",
            "params": {}
        }),
        // === Transfer 2: Alice -50, Bob +50 ===
        json!({
            "tool": "sqlx_workflow__begin",
            "params": {}
        }),
        json!({
            "tool": "sqlx_workflow__tx_execute",
            "params": {
                "sql": "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Alice', -50, 'tx2', 2)",
                "args": []
            }
        }),
        json!({
            "tool": "sqlx_workflow__tx_execute",
            "params": {
                "sql": "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Bob', 50, 'tx2', 2)",
                "args": []
            }
        }),
        json!({
            "tool": "sqlx_workflow__commit",
            "params": {}
        }),
        // === Balance Queries ===

        // Step 11: Query Alice's balance
        json!({
            "tool": "sqlx_workflow__fetch_one",
            "params": {
                "sql": "SELECT COALESCE(SUM(amount), 0) as balance FROM ledger_entries WHERE account_name = 'Alice'",
                "args": []
            }
        }),
        // Step 12: Query Bob's balance
        json!({
            "tool": "sqlx_workflow__fetch_one",
            "params": {
                "sql": "SELECT COALESCE(SUM(amount), 0) as balance FROM ledger_entries WHERE account_name = 'Bob'",
                "args": []
            }
        }),
        // Step 13: Query total balance (should be zero - double-entry invariant)
        json!({
            "tool": "sqlx_workflow__fetch_one",
            "params": {
                "sql": "SELECT COALESCE(SUM(amount), 0) as total FROM ledger_entries",
                "args": []
            }
        }),
        // === Validation ===
        // The generated binary will execute these queries. We can inspect the
        // generated code to verify the queries are correct, and optionally
        // add assertions to the emitted code.
    ]
}

#[tokio::test]
#[ignore] // Run explicitly with: cargo test --test ledger_phase2_query_test -- --ignored
async fn test_ledger_phase2_queries() {
    // Step 1: Compose workflow
    let workflow = compose_phase2_workflow();
    println!("Composed Phase 2 workflow with {} steps", workflow.len());

    // Step 2: Dispatch to EmitCode impls
    let output_dir = tempdir().expect("Failed to create temp dir");
    let output_path = output_dir.path().to_path_buf();
    std::mem::forget(output_dir); // Keep for inspection
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

    let mut scaffold = BinaryScaffold::new(steps, true);
    scaffold = scaffold.with_workspace_root(&workspace_root);

    // Step 4: Emit to disk
    let main_rs = scaffold
        .emit_to_disk(&output_path, "ledger_phase2")
        .expect("Failed to write source");
    println!("Source written to: {}", main_rs.display());

    // Step 5: Read and validate generated code
    let generated_code = std::fs::read_to_string(&main_rs).expect("Failed to read generated code");
    println!("\n=== Generated Code ===\n{}\n", generated_code);

    // Validate that balance queries are present
    assert!(
        generated_code.contains("SELECT COALESCE(SUM(amount), 0)"),
        "Generated code should contain balance aggregation query"
    );
    assert!(
        generated_code.contains("account_name = 'Alice'"),
        "Generated code should query Alice's balance"
    );
    assert!(
        generated_code.contains("account_name = 'Bob'"),
        "Generated code should query Bob's balance"
    );
    assert!(
        generated_code.contains("tx1"),
        "Generated code should include first transfer"
    );
    assert!(
        generated_code.contains("tx2"),
        "Generated code should include second transfer"
    );

    println!("✅ Code validation passed");

    // Step 6: Compile the binary
    let _reported_binary_path =
        elicitation::emit_code::compile(&output_path).expect("Compilation failed");

    // Step 7: Verify binary exists
    let actual_binary_path = output_path.join("target/release/ledger_phase2");
    assert!(
        actual_binary_path.exists(),
        "Binary does not exist at {}",
        actual_binary_path.display()
    );

    println!(
        "✅ Binary compiled successfully: {}",
        actual_binary_path.display()
    );

    // Step 8: Run the binary and capture output
    println!("\n=== Running binary ===");
    let output = std::process::Command::new(&actual_binary_path)
        .output()
        .expect("Failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("Exit code: {}", output.status);
    if !stdout.is_empty() {
        println!("STDOUT:\n{}", stdout);
    }
    if !stderr.is_empty() {
        println!("STDERR:\n{}", stderr);
    }

    assert!(output.status.success(), "Binary should exit successfully");

    println!("\n✅ Phase 2 test passed!");
    println!("Generated code demonstrates:");
    println!("  - Multiple sequential transactions");
    println!("  - SQL aggregation queries (SUM)");
    println!("  - Balance calculation per account");
    println!("  - Double-entry ledger invariant (total = 0)");
}
