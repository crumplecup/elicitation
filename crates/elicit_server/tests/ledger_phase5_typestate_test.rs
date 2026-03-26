//! Phase 5: Typestate state machine integration
//!
//! Tests the Transfer<S> typestate API with a real database:
//! - Transfer<Pending> → validate() → Transfer<Validated>
//! - Transfer<Validated> → commit() → Transfer<Committed>
//! - Transfer<Committed> → verify_invariant()
//! - Validation failures → Transfer<Rejected>
//!
//! This demonstrates that:
//! - State transitions enforce compile-time guarantees
//! - Proofs compose correctly (AmountPositive ∧ SufficientFunds ∧ AccountsDistinct)
//! - Database integration works with typestate
//! - Double-entry invariant preserved across commits

use elicit_server::ledger::{
    AccountId, Amount, Pending, RejectionReason, Transfer, TransferId, ValidationError,
};
use sqlx::Row;

#[tokio::test]
#[ignore] // Requires database setup
async fn test_ledger_typestate_valid_transfer() {
    // Install Any drivers (required for AnyPool)
    sqlx::any::install_default_drivers();

    // Setup database
    let pool = sqlx::any::AnyPoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
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

    // === Test 1: Valid transfer (Alice -> Bob, 50) ===

    // Create pending transfer
    let transfer: Transfer<Pending> = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(50).expect("Valid amount"),
        TransferId::new("tx1"),
    );

    // Validate (queries database, checks balance >= amount)
    let validated = transfer
        .validate(&pool)
        .await
        .expect("Validation should succeed");

    // Verify balance captured
    assert_eq!(
        validated.from_balance(),
        100,
        "Alice's balance should be 100"
    );

    // Commit (writes debit + credit entries)
    let committed = validated
        .commit(&pool)
        .await
        .expect("Commit should succeed");

    // Verify balances
    let data = committed.committed_data();
    assert_eq!(data.from_balance_before, 100, "Alice started with 100");
    assert_eq!(data.from_balance_after, 50, "Alice should have 50 after");
    assert_eq!(data.to_balance_after, 50, "Bob should have 50 after");

    // Verify double-entry invariant
    assert!(
        committed.verify_invariant(),
        "Double-entry invariant should hold"
    );
    assert_eq!(
        committed.from_balance_delta(),
        -50,
        "Alice's delta should be -50"
    );

    pool.close().await;
}

#[tokio::test]
#[ignore]
async fn test_ledger_typestate_insufficient_funds() {
    // Install Any drivers (required for AnyPool)
    sqlx::any::install_default_drivers();

    // Setup database
    let pool = sqlx::any::AnyPoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
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

    // Initialize Alice with only 30
    sqlx::query(
        "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind("Alice")
    .bind(30i64)
    .bind("init")
    .bind(0i64)
    .execute(&pool)
    .await
    .expect("Failed to initialize Alice");

    // === Test: Insufficient funds (Alice tries to transfer 50, only has 30) ===

    let transfer: Transfer<Pending> = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(50).expect("Valid amount"),
        TransferId::new("tx1"),
    );

    // Validation should fail
    let result = transfer.validate(&pool).await;

    match result {
        Err(ValidationError::InsufficientFunds { balance, required }) => {
            assert_eq!(balance, 30, "Balance should be 30");
            assert_eq!(required, 50, "Required should be 50");
        }
        _ => panic!("Expected InsufficientFunds error"),
    }

    pool.close().await;
}

#[tokio::test]
#[ignore]
async fn test_ledger_typestate_same_account() {
    // Install Any drivers (required for AnyPool)
    sqlx::any::install_default_drivers();

    // Setup database
    let pool = sqlx::any::AnyPoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
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

    // === Test: Same account transfer (Alice -> Alice) ===

    let transfer: Transfer<Pending> = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Alice"),
        Amount::new(50).expect("Valid amount"),
        TransferId::new("tx1"),
    );

    // Validation should fail
    let result = transfer.validate(&pool).await;

    match result {
        Err(ValidationError::SameAccount) => {
            // Expected
        }
        _ => panic!("Expected SameAccount error"),
    }

    pool.close().await;
}

#[tokio::test]
#[ignore]
async fn test_ledger_typestate_manual_reject() {
    // Test manual rejection without database

    let transfer: Transfer<Pending> = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(50).expect("Valid amount"),
        TransferId::new("tx1"),
    );

    // Manually reject
    let rejected = transfer.reject(RejectionReason::ManualRollback);

    // Verify rejection reason
    match rejected.reason() {
        RejectionReason::ManualRollback => {
            // Expected
        }
        _ => panic!("Expected ManualRollback reason"),
    }
}

#[tokio::test]
#[ignore]
async fn test_ledger_typestate_rollback_after_validation() {
    // Install Any drivers (required for AnyPool)
    sqlx::any::install_default_drivers();

    // Setup database
    let pool = sqlx::any::AnyPoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
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

    // Create and validate transfer
    let transfer: Transfer<Pending> = Transfer::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(50).expect("Valid amount"),
        TransferId::new("tx1"),
    );

    let validated = transfer
        .validate(&pool)
        .await
        .expect("Validation should succeed");

    // Rollback instead of committing
    let rejected = validated.rollback(RejectionReason::ManualRollback);

    // Verify rejection reason
    match rejected.reason() {
        RejectionReason::ManualRollback => {
            // Expected
        }
        _ => panic!("Expected ManualRollback reason"),
    }

    // Verify Alice's balance unchanged (no commit happened)
    let row = sqlx::query("SELECT COALESCE(SUM(amount), 0) as balance FROM ledger_entries WHERE account_name = ?")
        .bind("Alice")
        .fetch_one(&pool)
        .await
        .expect("Failed to query balance");

    let balance: i64 = row.get("balance");
    assert_eq!(balance, 100, "Alice's balance should be unchanged");

    pool.close().await;
}

#[test]
fn test_ledger_typestate_negative_amount() {
    // Test that Amount::new rejects negative values

    let result = Amount::new(-50);

    match result {
        Err(e) => {
            assert_eq!(
                e.to_string(),
                "Amount must be positive, got -50",
                "Error message should match"
            );
        }
        Ok(_) => panic!("Expected error for negative amount"),
    }
}

#[test]
fn test_ledger_typestate_zero_amount() {
    // Test that Amount::new rejects zero

    let result = Amount::new(0);

    match result {
        Err(e) => {
            assert_eq!(
                e.to_string(),
                "Amount must be positive, got 0",
                "Error message should match"
            );
        }
        Ok(_) => panic!("Expected error for zero amount"),
    }
}
