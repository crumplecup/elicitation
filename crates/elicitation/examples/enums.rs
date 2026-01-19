//! Example demonstrating enum elicitation with the Select pattern.
//!
//! **Note**: This example requires an MCP client (Claude Desktop or Claude CLI).
//! Run with: `claude "Run the enums example"`
//!
//! This example shows how enums automatically use the Select paradigm,
//! allowing users to choose from a finite set of options.

use elicitation::{ElicitClient, Elicit, ElicitResult, Elicitation, Prompt, Select};
use rmcp::ServiceExt;

/// Simple enum with default prompt
#[derive(Debug, Clone, Copy, PartialEq, Eq, Elicit)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Enum with custom prompt
#[derive(Debug, Clone, Copy, PartialEq, Eq, Elicit)]
#[prompt("Choose your preferred programming language:")]
enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
}

/// Enum for status tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Elicit)]
#[prompt("What is the current status?")]
enum Status {
    Pending,
    InProgress,
    Blocked,
    Completed,
    Cancelled,
}

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("enums=debug,elicitation=debug")
        .init();

    tracing::info!("Starting enum elicitation example");

    // Create MCP client with stdio transport
    let peer = ().serve(rmcp::transport::stdio()).await.expect("Failed to create MCP client");
    let client = ElicitClient::new(&peer);

    // Elicit priority level
    tracing::info!("=== Eliciting priority ===");
    let priority = Priority::elicit(&client).await?;
    tracing::info!(priority = ?priority, "Selected priority");

    // Elicit programming language
    tracing::info!("=== Eliciting language ===");
    let language = Language::elicit(&client).await?;
    tracing::info!(language = ?language, "Selected language");

    // Elicit status
    tracing::info!("=== Eliciting status ===");
    let status = Status::elicit(&client).await?;
    tracing::info!(status = ?status, "Selected status");

    // Elicit optional enum
    tracing::info!("=== Eliciting optional priority ===");
    let optional_priority: Option<Priority> = Option::<Priority>::elicit(&client).await?;
    tracing::info!(optional_priority = ?optional_priority, "Optional priority");

    // Elicit collection of enums
    tracing::info!("=== Eliciting multiple languages ===");
    let languages: Vec<Language> = Vec::<Language>::elicit(&client).await?;
    tracing::info!(languages = ?languages, "Selected languages");

    tracing::info!("=== Summary ===");
    tracing::info!("Priority: {:?}", priority);
    tracing::info!("Language: {:?}", language);
    tracing::info!("Status: {:?}", status);
    tracing::info!("Optional Priority: {:?}", optional_priority);
    tracing::info!("Languages: {:?}", languages);

    Ok(())
}
