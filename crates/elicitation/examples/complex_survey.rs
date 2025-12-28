//! Example demonstrating complex nested type elicitation.
//!
//! **Note**: This example requires an MCP client (Claude Desktop or Claude CLI).
//! Run with: `claude "Run the complex_survey example"`
//!
//! This example showcases:
//! - Nested structs
//! - Enums within structs
//! - Collections of custom types
//! - Optional custom types
//! - Composition of all paradigms

use elicitation::{Elicit, ElicitResult, Elicitation, Prompt, Select};
use pmcp::StdioTransport;

/// Priority level for tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Elicit)]
#[prompt("Select the priority level:")]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Elicit)]
#[prompt("What is the current status?")]
enum Status {
    Todo,
    InProgress,
    Blocked,
    Done,
}

/// Contact information
#[derive(Debug, Elicit)]
#[prompt("Let's add contact information:")]
struct Contact {
    #[prompt("Primary email address:")]
    email: String,

    #[prompt("Phone number (optional):")]
    phone: Option<String>,
}

/// Project member
#[derive(Debug, Elicit)]
struct Member {
    #[prompt("Member's full name:")]
    name: String,

    #[prompt("Member's role:")]
    role: String,

    contact: Contact,
}

/// Individual task
#[derive(Debug, Elicit)]
#[prompt("Let's create a task:")]
struct Task {
    #[prompt("Task title:")]
    title: String,

    #[prompt("Task description:")]
    description: String,

    priority: Priority,

    status: Status,

    #[prompt("Estimated hours:")]
    estimated_hours: Option<i32>,
}

/// Complete project structure
#[derive(Debug, Elicit)]
#[prompt("Let's set up your project:")]
struct Project {
    #[prompt("Project name:")]
    name: String,

    #[prompt("Project description:")]
    description: String,

    #[prompt("Add team members:")]
    team: Vec<Member>,

    #[prompt("Add project tasks:")]
    tasks: Vec<Task>,

    #[prompt("Project budget (optional):")]
    budget: Option<f64>,
}

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("complex_survey=debug,elicitation=debug")
        .init();

    tracing::info!("Starting complex survey example");

    // Create MCP client with stdio transport
    let transport = StdioTransport::new();
    let client = pmcp::Client::new(transport);

    // Elicit a complete project with all nested structures
    tracing::info!("=== Eliciting Project ===");
    let project = Project::elicit(&client).await?;

    // Display summary
    tracing::info!("=== Project Summary ===");
    tracing::info!("Name: {}", project.name);
    tracing::info!("Description: {}", project.description);
    tracing::info!("Team size: {}", project.team.len());
    tracing::info!("Total tasks: {}", project.tasks.len());
    tracing::info!("Budget: {:?}", project.budget);

    tracing::info!("=== Team Members ===");
    for (i, member) in project.team.iter().enumerate() {
        tracing::info!("{}. {} ({})", i + 1, member.name, member.role);
        tracing::info!("   Email: {}", member.contact.email);
        if let Some(phone) = &member.contact.phone {
            tracing::info!("   Phone: {}", phone);
        }
    }

    tracing::info!("=== Tasks ===");
    for (i, task) in project.tasks.iter().enumerate() {
        tracing::info!(
            "{}. {} [{:?}] - {:?}",
            i + 1,
            task.title,
            task.priority,
            task.status
        );
        tracing::info!("   {}", task.description);
        if let Some(hours) = task.estimated_hours {
            tracing::info!("   Estimated: {} hours", hours);
        }
    }

    tracing::info!("Project elicitation complete!");

    Ok(())
}
