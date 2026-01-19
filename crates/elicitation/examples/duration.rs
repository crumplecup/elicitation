//! Example demonstrating Duration elicitation.
//!
//! **Note**: This example requires an MCP client (Claude Desktop or Claude CLI).
//! Run with: `claude "Run the duration example"`
//!
//! This example shows how to elicit:
//! - Duration - Time durations in seconds (supports decimals)

use elicitation::{ElicitClient, ElicitResult, Elicitation};
use rmcp::ServiceExt;
use std::time::Duration;

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Initialize tracing for observability
    tracing_subscriber::fmt()
        .with_env_filter("duration=debug,elicitation=debug")
        .init();

    tracing::info!("Starting Duration elicitation example");

    // Create MCP client with stdio transport

    let peer = ().serve(rmcp::transport::stdio()).await.expect("Failed to create MCP client");
    let client = ElicitClient::new(&peer);

    // Elicit a timeout duration
    tracing::info!("=== Eliciting timeout duration ===");
    let timeout: Duration = Duration::elicit(&client).await?;
    tracing::info!(?timeout, "Elicited timeout");
    println!("Timeout: {:?} ({} seconds)", timeout, timeout.as_secs_f64());

    // Elicit a retry delay
    tracing::info!("=== Eliciting retry delay ===");
    let retry_delay: Duration = Duration::elicit(&client).await?;
    tracing::info!(?retry_delay, "Elicited retry delay");
    println!(
        "Retry delay: {:?} ({} ms)",
        retry_delay,
        retry_delay.as_millis()
    );

    // Elicit an optional duration
    tracing::info!("=== Eliciting optional cache TTL ===");
    let cache_ttl: Option<Duration> = Option::<Duration>::elicit(&client).await?;
    tracing::info!(?cache_ttl, "Elicited optional TTL");
    match cache_ttl {
        Some(ttl) => println!("Cache TTL: {:?}", ttl),
        None => println!("No cache TTL (cache disabled)"),
    }

    // Elicit multiple durations for intervals
    tracing::info!("=== Eliciting polling intervals ===");
    let intervals: Vec<Duration> = Vec::<Duration>::elicit(&client).await?;
    tracing::info!(?intervals, count = intervals.len(), "Elicited intervals");
    println!("Polling intervals:");
    for (i, interval) in intervals.iter().enumerate() {
        println!("  {}. {:?}", i + 1, interval);
    }

    tracing::info!("Example complete!");
    Ok(())
}
