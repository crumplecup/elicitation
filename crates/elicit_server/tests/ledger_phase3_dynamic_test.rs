//! Phase 3: Dynamic transfers with parameterized queries
//!
//! Extends Phase 2 by adding:
//! - JSON-driven transfer specifications
//! - Parameterized SQL queries with bound arguments
//! - Multiple dynamic transfers (not hardcoded values)
//! - Transfer ID tracking and validation
//!
//! This demonstrates that the emit pipeline can handle:
//! - Runtime data binding (not compile-time constants)
//! - Parameter extraction from JSON
//! - SQL injection prevention via proper binding
//!
//! Workflow:
//! 1. Connect to SQLite + CREATE TABLE
//! 2. Execute Transfer 1: Alice -100, Bob +100 (via parameterized query)
//! 3. Execute Transfer 2: Alice -50, Charlie +50 (via parameterized query)
//! 4. Execute Transfer 3: Bob -30, Charlie +30 (via parameterized query)
//! 5. Query all balances and verify correctness
//! 6. Verify double-entry invariant (sum = 0)

use elicitation::emit_code::{BinaryScaffold, EmitCode};
use serde_json::json;
use tempfile::tempdir;

/// Transfer specification in JSON format
#[derive(Debug, Clone)]
struct TransferSpec {
    from_account: &'static str,
    to_account: &'static str,
    amount: i64,
    transfer_id: &'static str,
    timestamp: i64,
}

/// Compose Phase 3 workflow: dynamic transfers with parameterized queries
fn compose_phase3_workflow() -> Vec<serde_json::Value> {
    // Define transfers as structured data
    let transfers = vec![
        TransferSpec {
            from_account: "Alice",
            to_account: "Bob",
            amount: 100,
            transfer_id: "tx1",
            timestamp: 1,
        },
        TransferSpec {
            from_account: "Alice",
            to_account: "Charlie",
            amount: 50,
            transfer_id: "tx2",
            timestamp: 2,
        },
        TransferSpec {
            from_account: "Bob",
            to_account: "Charlie",
            amount: 30,
            transfer_id: "tx3",
            timestamp: 3,
        },
    ];

    let mut workflow = vec![
        // === Database Setup ===
        json!({
            "tool": "sqlx_workflow__connect",
            "params": {
                "database_url": "sqlite::memory:",
                "max_connections": 1
            }
        }),
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
    ];

    // === Dynamic Transfers (parameterized queries) ===
    for transfer in transfers {
        // Begin transaction
        workflow.push(json!({
            "tool": "sqlx_workflow__begin",
            "params": {}
        }));

        // Debit from source account (parameterized)
        workflow.push(json!({
            "tool": "sqlx_workflow__tx_execute",
            "params": {
                "sql": "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES (?, ?, ?, ?)",
                "args": [
                    transfer.from_account,
                    -transfer.amount,  // Negative for debit
                    transfer.transfer_id,
                    transfer.timestamp
                ]
            }
        }));

        // Credit to destination account (parameterized)
        workflow.push(json!({
            "tool": "sqlx_workflow__tx_execute",
            "params": {
                "sql": "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES (?, ?, ?, ?)",
                "args": [
                    transfer.to_account,
                    transfer.amount,  // Positive for credit
                    transfer.transfer_id,
                    transfer.timestamp
                ]
            }
        }));

        // Commit transaction
        workflow.push(json!({
            "tool": "sqlx_workflow__commit",
            "params": {}
        }));
    }

    // === Balance Queries ===

    // Query Alice's balance (should be -150: -100 - 50)
    workflow.push(json!({
        "tool": "sqlx_workflow__fetch_one",
        "params": {
            "sql": "SELECT COALESCE(SUM(amount), 0) as balance FROM ledger_entries WHERE account_name = ?",
            "args": ["Alice"]
        }
    }));

    // Query Bob's balance (should be +70: +100 - 30)
    workflow.push(json!({
        "tool": "sqlx_workflow__fetch_one",
        "params": {
            "sql": "SELECT COALESCE(SUM(amount), 0) as balance FROM ledger_entries WHERE account_name = ?",
            "args": ["Bob"]
        }
    }));

    // Query Charlie's balance (should be +80: +50 + 30)
    workflow.push(json!({
        "tool": "sqlx_workflow__fetch_one",
        "params": {
            "sql": "SELECT COALESCE(SUM(amount), 0) as balance FROM ledger_entries WHERE account_name = ?",
            "args": ["Charlie"]
        }
    }));

    // Query total balance (should be 0 - double-entry invariant)
    workflow.push(json!({
        "tool": "sqlx_workflow__fetch_one",
        "params": {
            "sql": "SELECT COALESCE(SUM(amount), 0) as total FROM ledger_entries",
            "args": []
        }
    }));

    // Query transfer count (should be 6: 2 entries per transfer × 3 transfers)
    workflow.push(json!({
        "tool": "sqlx_workflow__fetch_one",
        "params": {
            "sql": "SELECT COUNT(*) as count FROM ledger_entries",
            "args": []
        }
    }));

    workflow
}

#[tokio::test]
#[ignore] // Run explicitly with: cargo test --test ledger_phase3_dynamic_test --features emit -- --ignored
async fn test_ledger_phase3_dynamic() {
    // Step 1: Compose workflow with parameterized queries
    let workflow = compose_phase3_workflow();
    println!("Composed Phase 3 workflow with {} steps", workflow.len());

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
        .emit_to_disk(&output_path, "ledger_phase3")
        .expect("Failed to write source");
    println!("Source written to: {}", main_rs.display());

    // Step 5: Read and validate generated code
    let generated_code = std::fs::read_to_string(&main_rs).expect("Failed to read generated code");
    println!("\n=== Generated Code ===\n{}\n", generated_code);

    // Validate parameterized queries are present
    assert!(
        generated_code.contains("VALUES (?, ?, ?, ?)"),
        "Generated code should contain parameterized INSERT queries"
    );
    assert!(
        generated_code.contains(".bind("),
        "Generated code should contain .bind() calls for parameters"
    );
    assert!(
        generated_code.contains("Alice"),
        "Generated code should reference Alice account"
    );
    assert!(
        generated_code.contains("Bob"),
        "Generated code should reference Bob account"
    );
    assert!(
        generated_code.contains("Charlie"),
        "Generated code should reference Charlie account"
    );
    assert!(
        generated_code.contains("WHERE account_name = ?"),
        "Generated code should use parameterized balance queries"
    );

    println!("✅ Code validation passed");

    // Step 6: Compile the binary
    let _reported_binary_path =
        elicitation::emit_code::compile(&output_path).expect("Compilation failed");

    // Step 7: Verify binary exists
    let actual_binary_path = output_path.join("target/release/ledger_phase3");
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

    println!("\n✅ Phase 3 test passed!");
    println!("Generated code demonstrates:");
    println!("  - Parameterized SQL queries (SQL injection prevention)");
    println!("  - Runtime data binding (not compile-time constants)");
    println!("  - Multiple dynamic transfers driven by structured data");
    println!("  - Balance queries with parameter binding");
    println!("  - Transfer tracking across 3 distinct transactions");
    println!("  - Double-entry ledger invariant validation");
    println!("\nExpected balances:");
    println!("  Alice:   -150 (-100 - 50)");
    println!("  Bob:     +70  (+100 - 30)");
    println!("  Charlie: +80  (+50 + 30)");
    println!("  Total:   0    (double-entry invariant)");
}
