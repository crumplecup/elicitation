//! Example demonstrating elicitation of primitive types.
//!
//! **Note**: This example requires an MCP client (Claude Desktop or Claude CLI).
//! Run with: `claude "Run the simple_types example"`
//!
//! This example shows how to elicit basic Rust types:
//! - Integers (i32, u8, etc.)
//! - Floats (f32, f64)
//! - Booleans
//! - Strings
//! - Optional values (Option<T>)
//! - Collections (Vec<T>)

use elicitation::{ElicitClient, ElicitResult, Elicitation};
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Initialize tracing for observability
    tracing_subscriber::fmt()
        .with_env_filter("simple_types=debug,elicitation=debug")
        .init();

    tracing::info!("Starting simple types example");

    // Create MCP client with stdio transport
    let peer = ().serve(rmcp::transport::stdio()).await.expect("Failed to create MCP client");
    let client = ElicitClient::new(&peer);

    // Elicit an integer
    tracing::info!("=== Eliciting integer ===");
    let age: i32 = i32::elicit(&client).await?;
    tracing::info!(age = %age, "Received integer");

    // Elicit a float
    tracing::info!("=== Eliciting float ===");
    let temperature: f64 = f64::elicit(&client).await?;
    tracing::info!(temperature = %temperature, "Received float");

    // Elicit a boolean
    tracing::info!("=== Eliciting boolean ===");
    let confirmed: bool = bool::elicit(&client).await?;
    tracing::info!(confirmed = %confirmed, "Received boolean");

    // Elicit a string
    tracing::info!("=== Eliciting string ===");
    let name: String = String::elicit(&client).await?;
    tracing::info!(name = %name, "Received string");

    // Elicit an optional value
    tracing::info!("=== Eliciting optional value ===");
    let nickname: Option<String> = Option::<String>::elicit(&client).await?;
    tracing::info!(nickname = ?nickname, "Received optional");

    // Elicit a collection
    tracing::info!("=== Eliciting collection ===");
    let scores: Vec<i32> = Vec::<i32>::elicit(&client).await?;
    tracing::info!(scores = ?scores, "Received collection");

    tracing::info!("=== Summary ===");
    tracing::info!("Age: {}", age);
    tracing::info!("Temperature: {:.2}°C", temperature);
    tracing::info!("Confirmed: {}", confirmed);
    tracing::info!("Name: {}", name);
    tracing::info!("Nickname: {:?}", nickname);
    tracing::info!("Scores: {:?}", scores);

    Ok(())
}
