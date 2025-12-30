//! Example demonstrating fixed-size array elicitation.
//!
//! **Note**: This example requires an MCP client (Claude Desktop or Claude CLI).
//! Run with: `claude "Run the arrays example"`
//!
//! This example shows how to elicit:
//! - [T; N] - Fixed-size arrays with const generics

use elicitation::{ElicitResult, Elicitation};
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Initialize tracing for observability
    tracing_subscriber::fmt()
        .with_env_filter("arrays=debug,elicitation=debug")
        .init();

    tracing::info!("Starting fixed-size array elicitation example");

    // Create MCP client with stdio transport

    let client = ()

        .serve(rmcp::transport::stdio())

        .await.expect("Failed to create MCP client");

    // Elicit a small array
    tracing::info!("=== Eliciting [i32; 3] ===");
    let coords: [i32; 3] = <[i32; 3]>::elicit(&client).await?;
    tracing::info!(?coords, "Elicited array");
    println!(
        "3D Coordinates: [{}, {}, {}]",
        coords[0], coords[1], coords[2]
    );

    // Elicit an array of strings
    tracing::info!("=== Eliciting [String; 4] ===");
    let names: [String; 4] = <[String; 4]>::elicit(&client).await?;
    tracing::info!(?names, "Elicited string array");
    println!("Team members:");
    for (i, name) in names.iter().enumerate() {
        println!("  {}. {}", i + 1, name);
    }

    // Elicit an array of floats (RGB color)
    tracing::info!("=== Eliciting [f32; 3] (RGB color) ===");
    let rgb: [f32; 3] = <[f32; 3]>::elicit(&client).await?;
    tracing::info!(?rgb, "Elicited RGB color");
    println!("RGB Color: ({:.2}, {:.2}, {:.2})", rgb[0], rgb[1], rgb[2]);

    // Elicit an array with complex type
    tracing::info!("=== Eliciting [(String, i32); 2] ===");
    let pairs: [(String, i32); 2] = <[(String, i32); 2]>::elicit(&client).await?;
    tracing::info!(?pairs, "Elicited tuple array");
    println!("Key-value pairs:");
    for (key, value) in &pairs {
        println!("  {}: {}", key, value);
    }

    // Elicit a larger array
    tracing::info!("=== Eliciting [i32; 5] (test scores) ===");
    let scores: [i32; 5] = <[i32; 5]>::elicit(&client).await?;
    tracing::info!(?scores, "Elicited scores");
    println!("Test scores: {:?}", scores);
    let average = scores.iter().sum::<i32>() as f64 / scores.len() as f64;
    println!("Average: {:.1}", average);

    tracing::info!("Example complete!");
    Ok(())
}
