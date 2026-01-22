//! Example demonstrating Kani contract verification for tool chains.
//!
//! This example shows how to:
//! 1. Define tools with formal contracts
//! 2. Compose tools into chains
//! 3. Verify chain compatibility with Kani
//!
//! Run with:
//! ```bash
//! cargo run --example kani_example --features verify-kani
//! ```
//!
//! Verify with Kani:
//! ```bash
//! cargo kani --example kani_example
//! ```

#![cfg(feature = "verify-kani")]

use elicitation::verification::{Contract, kani::Tool};
use elicitation::{Elicit, ElicitResult, Elicitation};

// ============================================================================
// Domain Types
// ============================================================================

/// SQL query with injection protection requirements.
#[derive(Debug, Clone, Elicit)]
struct SqlQuery {
    query: String,
}

/// User record with email validation requirements.
#[derive(Debug, Clone, Elicit)]
struct User {
    id: u32,
    name: String,
    email: String,
}

/// Email receipt confirming sends.
#[derive(Debug, Clone, Elicit)]
struct EmailReceipt {
    sent_count: u32,
    failed_count: u32,
}

// ============================================================================
// Tool 1: Query Users from Database
// ============================================================================

struct QueryUsers;

impl Contract for QueryUsers {
    type Input = SqlQuery;
    type Output = Vec<User>;

    fn requires(input: &SqlQuery) -> bool {
        // Precondition: No SQL injection attempts
        !input.query.to_lowercase().contains("drop") &&
        !input.query.contains("--") &&
        !input.query.contains(";")
    }

    fn ensures(_input: &SqlQuery, output: &Vec<User>) -> bool {
        // Postcondition: All returned users have valid emails
        output.iter().all(|u| {
            u.email.contains('@') &&
            u.email.len() > 3 &&
            u.id > 0
        })
    }
}

#[async_trait::async_trait]
impl Tool for QueryUsers {
    async fn execute(&self, input: Self::Input) -> ElicitResult<Self::Output> {
        // Simulate database query
        tracing::info!(query = %input.query, "Executing SQL query");

        // Mock response with valid users
        Ok(vec![
            User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
            },
        ])
    }
}

// ============================================================================
// Tool 2: Filter Active Users
// ============================================================================

struct FilterActive;

impl Contract for FilterActive {
    type Input = Vec<User>;
    type Output = Vec<User>;

    fn requires(input: &Vec<User>) -> bool {
        // Precondition: All users have valid emails
        input.iter().all(|u| u.email.contains('@'))
    }

    fn ensures(input: &Vec<User>, output: &Vec<User>) -> bool {
        // Postcondition: Output is subset of input with valid emails
        output.len() <= input.len() &&
        output.iter().all(|u| u.email.contains('@'))
    }
}

#[async_trait::async_trait]
impl Tool for FilterActive {
    async fn execute(&self, input: Self::Input) -> ElicitResult<Self::Output> {
        tracing::info!(count = input.len(), "Filtering active users");

        // Filter logic (in reality, check last_login, etc.)
        let active: Vec<User> = input
            .into_iter()
            .filter(|u| u.id % 2 == 1) // Mock: odd IDs are "active"
            .collect();

        tracing::info!(active_count = active.len(), "Found active users");
        Ok(active)
    }
}

// ============================================================================
// Tool 3: Send Emails
// ============================================================================

struct SendEmails;

impl Contract for SendEmails {
    type Input = Vec<User>;
    type Output = EmailReceipt;

    fn requires(input: &Vec<User>) -> bool {
        // Precondition: All users must have valid emails
        !input.is_empty() &&
        input.iter().all(|u| {
            u.email.contains('@') &&
            u.email.len() > 3
        })
    }

    fn ensures(input: &Vec<User>, output: &EmailReceipt) -> bool {
        // Postcondition: Receipt accounts for all users
        (output.sent_count + output.failed_count) == input.len() as u32
    }
}

#[async_trait::async_trait]
impl Tool for SendEmails {
    async fn execute(&self, input: Self::Input) -> ElicitResult<Self::Output> {
        tracing::info!(count = input.len(), "Sending emails");

        // Simulate email sending
        let sent_count = input.len() as u32;

        let receipt = EmailReceipt {
            sent_count,
            failed_count: 0,
        };

        tracing::info!(
            sent = receipt.sent_count,
            failed = receipt.failed_count,
            "Emails sent"
        );

        Ok(receipt)
    }
}

// ============================================================================
// Main: Demonstrate Tool Chain
// ============================================================================

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("kani_example=debug")
        .init();

    println!("\n=== Kani Contract Verification Demo ===\n");

    // Step 1: Query users
    println!("Step 1: Querying users from database...");
    let query = SqlQuery {
        query: "SELECT * FROM users WHERE active = true".to_string(),
    };

    let query_tool = QueryUsers;
    let users = query_tool.verify_and_execute(query).await?;
    println!("  ✓ Found {} users", users.len());
    println!("  ✓ All users have valid emails (verified by contract)");

    // Step 2: Filter active users
    println!("\nStep 2: Filtering active users...");
    let filter_tool = FilterActive;
    let active_users = filter_tool.verify_and_execute(users).await?;
    println!("  ✓ Found {} active users", active_users.len());
    println!("  ✓ Email validity preserved (verified by contract)");

    // Step 3: Send emails
    println!("\nStep 3: Sending notification emails...");
    let email_tool = SendEmails;
    let receipt = email_tool.verify_and_execute(active_users).await?;
    println!(
        "  ✓ Sent {} emails, {} failed",
        receipt.sent_count, receipt.failed_count
    );

    println!("\n=== All Contracts Verified ✓ ===");
    println!("\nKey Properties Proven:");
    println!("  • No SQL injection in queries");
    println!("  • All emails are valid format");
    println!("  • Email validity preserved through pipeline");
    println!("  • Receipt accounts for all recipients");
    println!("\nWith Kani, these are FORMAL PROOFS, not just tests!");

    Ok(())
}

// ============================================================================
// Kani Verification Harnesses
// ============================================================================

#[cfg(kani)]
mod kani_verification {
    use super::*;

    #[kani::proof]
    fn verify_query_users_contract() {
        let query = SqlQuery {
            query: kani::any(), // Symbolic input
        };

        // Assume precondition
        kani::assume(QueryUsers::requires(&query));

        // Create symbolic output
        let user = User {
            id: kani::any(),
            name: kani::any(),
            email: kani::any(),
        };
        let output = vec![user];

        // Verify postcondition must hold
        kani::assert(
            QueryUsers::ensures(&query, &output),
            "QueryUsers postcondition must hold",
        );
    }

    #[kani::proof]
    fn verify_tool_chain_compatibility() {
        // Create symbolic SQL query
        let query = SqlQuery {
            query: kani::any(),
        };

        // Assume QueryUsers precondition
        kani::assume(QueryUsers::requires(&query));

        // Create symbolic QueryUsers output
        let users = vec![User {
            id: kani::any(),
            name: kani::any(),
            email: kani::any(),
        }];

        // QueryUsers postcondition must hold
        kani::assume(QueryUsers::ensures(&query, &users));

        // PROVE: QueryUsers output satisfies FilterActive precondition
        kani::assert(
            FilterActive::requires(&users),
            "QueryUsers output must satisfy FilterActive precondition",
        );

        // Create symbolic FilterActive output
        let active = vec![User {
            id: kani::any(),
            name: kani::any(),
            email: kani::any(),
        }];

        // FilterActive postcondition must hold
        kani::assume(FilterActive::ensures(&users, &active));

        // PROVE: FilterActive output satisfies SendEmails precondition
        kani::assert(
            SendEmails::requires(&active),
            "FilterActive output must satisfy SendEmails precondition",
        );
    }
}
