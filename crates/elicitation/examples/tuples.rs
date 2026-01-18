//! Example demonstrating tuple elicitation.
//!
//! **Note**: This example requires an MCP client (Claude Desktop or Claude CLI).
//! Run with: `claude "Run the tuples example"`
//!
//! This example shows how to elicit:
//! - Tuples of various sizes (arity 2-12)
//! - Tuples with mixed types
//! - Tuples with complex nested types

use elicitation::{ElicitResult, Elicitation};
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Initialize tracing for observability
    tracing_subscriber::fmt()
        .with_env_filter("tuples=debug,elicitation=debug")
        .init();

    tracing::info!("Starting tuple elicitation example");

    // Create MCP client with stdio transport

    let client = ().serve(rmcp::transport::stdio()).await.expect("Failed to create MCP client");

    // Elicit a simple 2-tuple (pair)
    tracing::info!("=== Eliciting (String, i32) ===");
    let pair: (String, i32) = <(String, i32)>::elicit(&client).await?;
    tracing::info!(?pair, "Elicited pair");
    println!("Name and age: {} is {} years old", pair.0, pair.1);

    // Elicit a 3-tuple
    tracing::info!("=== Eliciting (String, String, bool) ===");
    let triple: (String, String, bool) = <(String, String, bool)>::elicit(&client).await?;
    tracing::info!(?triple, "Elicited triple");
    println!("User: {} <{}> (verified: {})", triple.0, triple.1, triple.2);

    // Elicit a tuple with mixed numeric types
    tracing::info!("=== Eliciting (f64, f64, f64) ===");
    let coordinates: (f64, f64, f64) = <(f64, f64, f64)>::elicit(&client).await?;
    tracing::info!(?coordinates, "Elicited coordinates");
    println!(
        "3D Point: ({}, {}, {})",
        coordinates.0, coordinates.1, coordinates.2
    );

    // Elicit a tuple with complex types
    tracing::info!("=== Eliciting (Vec<i32>, Option<String>) ===");
    let complex: (Vec<i32>, Option<String>) = <(Vec<i32>, Option<String>)>::elicit(&client).await?;
    tracing::info!(?complex, "Elicited complex tuple");
    println!("Scores: {:?}", complex.0);
    match &complex.1 {
        Some(note) => println!("Note: {}", note),
        None => println!("No note provided"),
    }

    // Elicit a larger tuple (arity 5)
    tracing::info!("=== Eliciting (String, i32, i32, i32, String) ===");
    let score_record: (String, i32, i32, i32, String) =
        <(String, i32, i32, i32, String)>::elicit(&client).await?;
    tracing::info!(?score_record, "Elicited score record");
    println!(
        "Player: {}, Score: {}, Kills: {}, Deaths: {}, Rank: {}",
        score_record.0, score_record.1, score_record.2, score_record.3, score_record.4
    );

    tracing::info!("Example complete!");
    Ok(())
}
