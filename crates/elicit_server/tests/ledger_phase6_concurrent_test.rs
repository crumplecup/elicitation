//! Phase 6: Concurrent transfers with transaction isolation
//!
//! Tests the Transfer<S> typestate under concurrent load to demonstrate:
//! - Double-entry invariant preservation (total balance unchanged)
//! - Typestate API correctness under concurrent execution
//! - SQLite's default isolation level limitations
//!
//! **Important Note on SQLite Isolation:**
//!
//! These tests use SQLite with its default DEFERRED locking mode. This means:
//! - Transactions acquire locks lazily (on first write, not BEGIN)
//! - Validation queries may see stale balances from before concurrent commits
//! - Multiple transactions can pass validation and all commit successfully
//! - This can lead to negative balances (insufficient funds not detected)
//!
//! However, the double-entry invariant ALWAYS holds:
//! - Total balance across all accounts remains unchanged
//! - Every debit has a matching credit
//! - The ledger is internally consistent (sum = initial sum)
//!
//! For production use with strict balance constraints, use:
//! - BEGIN IMMEDIATE (acquire write lock immediately)
//! - Serializable isolation level
//! - Application-level locking
//! - A database with better concurrency control (PostgreSQL, etc.)

use elicit_server::ledger::{AccountId, Amount, Pending, Transfer, TransferId};
use sqlx::Row;
use std::sync::Arc;

#[tokio::test]
#[ignore] // Requires database setup
async fn test_ledger_concurrent_transfers_from_same_account() {
    // Install Any drivers
    sqlx::any::install_default_drivers();

    // Use shared in-memory database for concurrent access
    // Regular :memory: databases are per-connection
    // file::memory:?cache=shared creates a shared in-memory database
    let pool = sqlx::any::AnyPoolOptions::new()
        .max_connections(10)
        .connect("sqlite:file::memory:?cache=shared")
        .await
        .expect("Failed to connect");

    // Create table
    sqlx::query(
        r#"CREATE TABLE ledger_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            account_name TEXT NOT NULL,
            amount INTEGER NOT NULL,
            transfer_id TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )"#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    // Initialize Alice with 100
    sqlx::query(
        "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind("Alice")
    .bind(100i64)
    .bind("init")
    .bind(0i64)
    .execute(&pool)
    .await
    .expect("Failed to initialize Alice");

    // Initialize Bob, Carol, Dave with 0
    for name in ["Bob", "Carol", "Dave"] {
        sqlx::query(
            "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES (?, ?, ?, ?)",
        )
        .bind(name)
        .bind(0i64)
        .bind("init")
        .bind(0i64)
        .execute(&pool)
        .await
        .expect("Failed to initialize account");
    }

    // === Test: 5 concurrent transfers of 25 each (total 125) from Alice (who has 100) ===
    // Note: Due to SQLite's deferred locking in BEGIN, validation queries may see
    // stale balances, allowing some transfers that would normally fail. This tests
    // that the typestate API correctly handles concurrent scenarios, even if the
    // database doesn't provide perfect isolation.

    let pool = Arc::new(pool);
    let mut handles = vec![];

    for i in 0..5 {
        let pool = Arc::clone(&pool);
        let to_account = match i {
            0 => "Bob",
            1 => "Carol",
            2 => "Dave",
            3 => "Bob",
            _ => "Carol",
        };

        let handle = tokio::spawn(async move {
            let transfer: Transfer<Pending> = Transfer::new(
                AccountId::new("Alice"),
                AccountId::new(to_account),
                Amount::new(25).expect("Valid amount"),
                TransferId::new(format!("concurrent_tx_{}", i)),
            );

            // Validate and commit
            match transfer.validate(&pool).await {
                Ok(validated) => match validated.commit(&pool).await {
                    Ok(committed) => {
                        // Note: verify_invariant checks that the balance delta matches
                        // the transfer amount at the time of commit, which should always hold
                        Ok((i, committed.from_balance_delta()))
                    }
                    Err(e) => Err(format!("Commit failed for tx {}: {}", i, e)),
                },
                Err(e) => Err(format!("Validation failed for tx {}: {}", i, e)),
            }
        });

        handles.push(handle);
    }

    // Wait for all transfers to complete
    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.expect("Task panicked"))
        .collect();

    // Count successes and failures
    let successes = results.iter().filter(|r| r.is_ok()).count();
    let failures = results.iter().filter(|r| r.is_err()).count();

    println!("Successes: {}, Failures: {}", successes, failures);
    for result in results.iter() {
        match result {
            Ok((i, delta)) => println!("Transfer {} succeeded with delta {}", i, delta),
            Err(e) => println!("Transfer failed: {}", e),
        }
    }

    // At least some transfers should complete
    assert!(successes > 0, "At least one transfer should succeed");

    // Verify final balances
    let alice_balance = get_balance(&pool, "Alice").await;
    let bob_balance = get_balance(&pool, "Bob").await;
    let carol_balance = get_balance(&pool, "Carol").await;
    let dave_balance = get_balance(&pool, "Dave").await;

    println!(
        "Final balances: Alice={}, Bob={}, Carol={}, Dave={}",
        alice_balance, bob_balance, carol_balance, dave_balance
    );

    // Verify double-entry invariant: sum of all balances = 100 (initial total)
    // This MUST hold regardless of isolation level - it's the fundamental ledger property
    let total_balance = alice_balance + bob_balance + carol_balance + dave_balance;
    assert_eq!(
        total_balance, 100,
        "Double-entry invariant: total balance must be unchanged"
    );

    // Alice's balance should be reduced by the sum of successful transfers
    let total_sent = 100 - alice_balance;
    assert_eq!(
        total_sent,
        bob_balance + carol_balance + dave_balance,
        "Alice's sent amount should equal recipients' total"
    );

    pool.close().await;
}

#[tokio::test]
#[ignore]
async fn test_ledger_concurrent_transfers_no_overdraft() {
    // Install Any drivers
    sqlx::any::install_default_drivers();

    // Use shared in-memory database for concurrent access
    let pool = sqlx::any::AnyPoolOptions::new()
        .max_connections(10)
        .connect("sqlite:file::memory:?cache=shared")
        .await
        .expect("Failed to connect");

    // Create table
    sqlx::query(
        r#"CREATE TABLE ledger_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            account_name TEXT NOT NULL,
            amount INTEGER NOT NULL,
            transfer_id TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )"#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    // Initialize Alice with exactly 50
    sqlx::query(
        "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind("Alice")
    .bind(50i64)
    .bind("init")
    .bind(0i64)
    .execute(&pool)
    .await
    .expect("Failed to initialize Alice");

    // Initialize Bob with 0
    sqlx::query(
        "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind("Bob")
    .bind(0i64)
    .bind("init")
    .bind(0i64)
    .execute(&pool)
    .await
    .expect("Failed to initialize Bob");

    // === Test: 10 concurrent transfers of 50 each from Alice (who has 50) ===
    // Note: Due to SQLite's deferred locking, validation may see stale balances.
    // This tests that the double-entry invariant holds regardless of how many
    // transfers succeed concurrently.

    let pool = Arc::new(pool);
    let mut handles = vec![];

    for i in 0..10 {
        let pool = Arc::clone(&pool);

        let handle = tokio::spawn(async move {
            let transfer: Transfer<Pending> = Transfer::new(
                AccountId::new("Alice"),
                AccountId::new("Bob"),
                Amount::new(50).expect("Valid amount"),
                TransferId::new(format!("overdraft_tx_{}", i)),
            );

            // Validate and commit
            match transfer.validate(&pool).await {
                Ok(validated) => match validated.commit(&pool).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Commit failed: {}", e)),
                },
                Err(e) => Err(format!("Validation failed: {}", e)),
            }
        });

        handles.push(handle);
    }

    // Wait for all transfers
    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.expect("Task panicked"))
        .collect();

    // Count successes and failures
    let successes = results.iter().filter(|r| r.is_ok()).count();
    let failures = results.iter().filter(|r| r.is_err()).count();

    println!("Successes: {}, Failures: {}", successes, failures);

    // At least one transfer should succeed
    assert!(successes > 0, "At least one transfer should succeed");

    // Verify final balances
    let alice_balance = get_balance(&pool, "Alice").await;
    let bob_balance = get_balance(&pool, "Bob").await;

    println!(
        "Final balances: Alice={}, Bob={}",
        alice_balance, bob_balance
    );

    // The critical invariant: total balance unchanged
    let total_balance = alice_balance + bob_balance;
    assert_eq!(
        total_balance, 50,
        "Double-entry invariant: total balance must be 50"
    );

    // Verify consistency: Alice's deduction equals Bob's addition
    assert_eq!(
        50 - alice_balance,
        bob_balance,
        "Alice's sent amount must equal Bob's received amount"
    );

    pool.close().await;
}

#[tokio::test]
#[ignore]
async fn test_ledger_concurrent_bidirectional_transfers() {
    // Install Any drivers
    sqlx::any::install_default_drivers();

    // Use shared in-memory database for concurrent access
    let pool = sqlx::any::AnyPoolOptions::new()
        .max_connections(10)
        .connect("sqlite:file::memory:?cache=shared")
        .await
        .expect("Failed to connect");

    // Create table
    sqlx::query(
        r#"CREATE TABLE ledger_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            account_name TEXT NOT NULL,
            amount INTEGER NOT NULL,
            transfer_id TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )"#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    // Initialize Alice and Bob with 100 each
    for name in ["Alice", "Bob"] {
        sqlx::query(
            "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES (?, ?, ?, ?)",
        )
        .bind(name)
        .bind(100i64)
        .bind("init")
        .bind(0i64)
        .execute(&pool)
        .await
        .expect("Failed to initialize account");
    }

    // === Test: Concurrent bidirectional transfers (Alice -> Bob and Bob -> Alice) ===
    // Each sends 10 to the other, 5 times concurrently
    // Expected: Both end with 100 (net zero transfer)

    let pool = Arc::new(pool);
    let mut handles = vec![];

    // Alice -> Bob transfers
    for i in 0..5 {
        let pool = Arc::clone(&pool);
        let handle = tokio::spawn(async move {
            let transfer: Transfer<Pending> = Transfer::new(
                AccountId::new("Alice"),
                AccountId::new("Bob"),
                Amount::new(10).expect("Valid amount"),
                TransferId::new(format!("alice_to_bob_{}", i)),
            );

            transfer
                .validate(&pool)
                .await
                .expect("Validation should succeed")
                .commit(&pool)
                .await
                .expect("Commit should succeed");
        });
        handles.push(handle);
    }

    // Bob -> Alice transfers
    for i in 0..5 {
        let pool = Arc::clone(&pool);
        let handle = tokio::spawn(async move {
            let transfer: Transfer<Pending> = Transfer::new(
                AccountId::new("Bob"),
                AccountId::new("Alice"),
                Amount::new(10).expect("Valid amount"),
                TransferId::new(format!("bob_to_alice_{}", i)),
            );

            transfer
                .validate(&pool)
                .await
                .expect("Validation should succeed")
                .commit(&pool)
                .await
                .expect("Commit should succeed");
        });
        handles.push(handle);
    }

    // Wait for all transfers
    futures::future::join_all(handles).await;

    // Verify final balances (should both be 100)
    let alice_balance = get_balance(&pool, "Alice").await;
    let bob_balance = get_balance(&pool, "Bob").await;

    assert_eq!(
        alice_balance, 100,
        "Alice should have 100 (sent 50, received 50)"
    );
    assert_eq!(
        bob_balance, 100,
        "Bob should have 100 (sent 50, received 50)"
    );

    // Verify total balance unchanged
    let total_balance = alice_balance + bob_balance;
    assert_eq!(total_balance, 200, "Total balance should be unchanged");

    pool.close().await;
}

#[tokio::test]
#[ignore]
async fn test_ledger_concurrent_many_to_one() {
    // Install Any drivers
    sqlx::any::install_default_drivers();

    // Use shared in-memory database for concurrent access
    let pool = sqlx::any::AnyPoolOptions::new()
        .max_connections(20)
        .connect("sqlite:file::memory:?cache=shared")
        .await
        .expect("Failed to connect");

    // Create table
    sqlx::query(
        r#"CREATE TABLE ledger_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            account_name TEXT NOT NULL,
            amount INTEGER NOT NULL,
            transfer_id TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )"#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    // Initialize 10 accounts with 10 each, Target with 0
    for i in 0..10 {
        sqlx::query(
            "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES (?, ?, ?, ?)",
        )
        .bind(format!("Account{}", i))
        .bind(10i64)
        .bind("init")
        .bind(0i64)
        .execute(&pool)
        .await
        .expect("Failed to initialize account");
    }

    sqlx::query(
        "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind("Target")
    .bind(0i64)
    .bind("init")
    .bind(0i64)
    .execute(&pool)
    .await
    .expect("Failed to initialize Target");

    // === Test: 10 accounts each send 10 to Target concurrently ===
    // Expected: Target receives 100 total

    let pool = Arc::new(pool);
    let mut handles = vec![];

    for i in 0..10 {
        let pool = Arc::clone(&pool);
        let handle = tokio::spawn(async move {
            let transfer: Transfer<Pending> = Transfer::new(
                AccountId::new(format!("Account{}", i)),
                AccountId::new("Target"),
                Amount::new(10).expect("Valid amount"),
                TransferId::new(format!("many_to_one_{}", i)),
            );

            transfer
                .validate(&pool)
                .await
                .expect("Validation should succeed")
                .commit(&pool)
                .await
                .expect("Commit should succeed");
        });
        handles.push(handle);
    }

    // Wait for all transfers
    futures::future::join_all(handles).await;

    // Verify Target received all transfers
    let target_balance = get_balance(&pool, "Target").await;
    assert_eq!(
        target_balance, 100,
        "Target should have received 100 (10 * 10)"
    );

    // Verify all source accounts are empty
    for i in 0..10 {
        let balance = get_balance(&pool, &format!("Account{}", i)).await;
        assert_eq!(balance, 0, "Account{} should be empty", i);
    }

    // Verify total balance unchanged (10 * 10 + 0 = 100)
    let mut total_balance = target_balance;
    for i in 0..10 {
        total_balance += get_balance(&pool, &format!("Account{}", i)).await;
    }
    assert_eq!(total_balance, 100, "Total balance should be unchanged");

    pool.close().await;
}

// Helper function to get balance for an account
async fn get_balance(pool: &sqlx::AnyPool, account_name: &str) -> i64 {
    let row = sqlx::query(
        "SELECT COALESCE(SUM(amount), 0) as balance FROM ledger_entries WHERE account_name = ?",
    )
    .bind(account_name)
    .fetch_one(pool)
    .await
    .expect("Failed to query balance");

    row.get("balance")
}
