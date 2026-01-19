//! Example demonstrating struct elicitation with the Survey pattern.
//!
//! **Note**: This example requires an MCP client (Claude Desktop or Claude CLI).
//! Run with: `claude "Run the structs example"`
//!
//! This example shows how structs automatically use the Survey paradigm,
//! eliciting each field sequentially to build the complete structure.

use elicitation::{Elicit, ElicitResult, Elicitation};
use rmcp::ServiceExt;

/// Simple struct with default prompts
#[derive(Debug, Elicit)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

/// Struct with custom field prompts
#[derive(Debug, Elicit)]
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
#[derive(Debug, Elicit)]
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
#[derive(Debug, Default, Elicit)]
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
    let peer = ().serve(rmcp::transport::stdio()).await.expect("Failed to create MCP client");
    let client = ElicitClient::new(&peer);

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

    // Demonstrate field access
    println!("\n=== Field Access Demo ===");
    println!(
        "Person's name: {}, age: {}, email: {}",
        person.name, person.age, person.email
    );
    println!(
        "Account username: {}, email: {}, age: {}, notifications: {}",
        account.username, account.email, account.age, account.notifications_enabled
    );
    println!(
        "Profile name: {}, nickname: {:?}, bio: {:?}, age: {}",
        profile.name, profile.nickname, profile.bio, profile.age
    );
    println!(
        "Task title: {}, description: {}, created_at: {}, id: {}",
        task.title, task.description, task.created_at, task.id
    );

    Ok(())
}
