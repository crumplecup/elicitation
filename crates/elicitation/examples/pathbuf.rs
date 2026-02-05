//! Example demonstrating filesystem path elicitation.
//!
//! **Note**: This example requires an MCP client (Claude Desktop or Claude CLI).
//! Run with: `claude "Run the pathbuf example"`
//!
//! This example shows how to elicit:
//! - PathBuf - Filesystem paths (files, directories, etc.)
use std::sync::Arc;

use elicitation::{ElicitClient, ElicitResult, Elicitation};
use rmcp::ServiceExt;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Initialize tracing for observability
    tracing_subscriber::fmt()
        .with_env_filter("pathbuf=debug,elicitation=debug")
        .init();

    tracing::info!("Starting PathBuf elicitation example");

    // Create MCP client with stdio transport
    let service = ().serve(rmcp::transport::stdio()).await.expect("Failed to create MCP client");
    let peer = service.peer();
    let client = ElicitClient::new(Arc::new(peer.clone()));

    // Elicit a file path
    tracing::info!("=== Eliciting file path ===");
    let file_path: PathBuf = PathBuf::elicit(&client).await?;
    tracing::info!(?file_path, "Elicited file path");
    println!("File path: {}", file_path.display());

    // Elicit a directory path
    tracing::info!("=== Eliciting directory path ===");
    let dir_path: PathBuf = PathBuf::elicit(&client).await?;
    tracing::info!(?dir_path, "Elicited directory path");
    println!("Directory path: {}", dir_path.display());

    // Elicit an optional path
    tracing::info!("=== Eliciting optional config path ===");
    let config_path: Option<PathBuf> = Option::<PathBuf>::elicit(&client).await?;
    tracing::info!(?config_path, "Elicited optional path");
    match config_path {
        Some(path) => println!("Config path: {}", path.display()),
        None => println!("No config path provided"),
    }

    tracing::info!("Example complete!");
    Ok(())
}
