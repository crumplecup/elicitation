//! Example demonstrating struct elicitation with the Survey pattern.
//!
//! This example shows how structs automatically use the Survey paradigm,
//! eliciting each field sequentially to build the complete structure.

use elicitation::{DeriveElicit, Elicit, ElicitResult, Prompt, Survey};
use pmcp::StdioTransport;

/// Simple struct with default prompts
#[derive(Debug, DeriveElicit)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

/// Struct with custom field prompts
#[derive(Debug, DeriveElicit)]
#[prompt("Let's configure your account:")]
struct Account {
    #[prompt("What username would you like?")]
    username: String,

    #[prompt("Enter your email address:")]
    email: String,

    #[prompt("How old are you?")]
    age: u8,

    #[prompt("Would you like to receive notifications?")]
    notifications_enabled: bool,
}

/// Struct with optional fields
#[derive(Debug, DeriveElicit)]
struct Profile {
    #[prompt("What's your full name?")]
    name: String,

    #[prompt("What's your preferred nickname?")]
    nickname: Option<String>,

    #[prompt("What's your bio?")]
    bio: Option<String>,

    #[prompt("How old are you?")]
    age: u8,
}

/// Struct with skipped fields (use Default::default())
#[derive(Debug, Default, DeriveElicit)]
struct Task {
    #[prompt("What's the task title?")]
    title: String,

    #[prompt("Describe the task:")]
    description: String,

    // Internal fields that aren't elicited
    #[skip]
    created_at: String,

    #[skip]
    id: u64,
}

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("structs=debug,elicitation=debug")
        .init();

    tracing::info!("Starting struct elicitation example");

    // Create MCP client with stdio transport
    let transport = StdioTransport::new();
    let client = pmcp::Client::new(transport);

    // Elicit a simple person
    tracing::info!("=== Eliciting Person ===");
    let person = Person::elicit(&client).await?;
    tracing::info!(person = ?person, "Created person");

    // Elicit account with custom prompts
    tracing::info!("=== Eliciting Account ===");
    let account = Account::elicit(&client).await?;
    tracing::info!(account = ?account, "Created account");

    // Elicit profile with optional fields
    tracing::info!("=== Eliciting Profile ===");
    let profile = Profile::elicit(&client).await?;
    tracing::info!(profile = ?profile, "Created profile");

    // Elicit task with skipped fields
    tracing::info!("=== Eliciting Task ===");
    let task = Task::elicit(&client).await?;
    tracing::info!(task = ?task, "Created task");

    tracing::info!("=== Summary ===");
    tracing::info!("Person: {:?}", person);
    tracing::info!("Account: {:?}", account);
    tracing::info!("Profile: {:?}", profile);
    tracing::info!("Task: {:?}", task);

    Ok(())
}
