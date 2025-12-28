//! Example demonstrating Result<T, E> elicitation.
//!
//! **Note**: This example requires an MCP client (Claude Desktop or Claude CLI).
//! Run with: `claude "Run the result example"`
//!
//! This example shows how to elicit:
//! - Result<T, E> - Success or failure outcomes with values

use elicitation::{Elicit, ElicitResult, Elicitation, Prompt, Select};
use pmcp::StdioTransport;

/// Simple error type for demonstration
#[derive(Debug, Clone, Elicit)]
enum ApiError {
    NotFound,
    Unauthorized,
    ServerError,
}

/// Status code type
#[derive(Debug, Clone, Elicit)]
enum StatusCode {
    Success,
    Redirect,
    ClientError,
    ServerError,
}

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Initialize tracing for observability
    tracing_subscriber::fmt()
        .with_env_filter("result=debug,elicitation=debug")
        .init();

    tracing::info!("Starting Result elicitation example");

    // Create MCP client
    let transport = StdioTransport::new();
    let client = pmcp::Client::new(transport);

    // Elicit a simple Result<String, String>
    tracing::info!("=== Eliciting Result<String, String> ===");
    let operation_result: Result<String, String> = Result::elicit(&client).await?;
    tracing::info!(?operation_result, "Elicited result");
    match operation_result {
        Ok(value) => println!("Operation succeeded: {}", value),
        Err(error) => println!("Operation failed: {}", error),
    }

    // Elicit Result with custom error enum
    tracing::info!("=== Eliciting Result<i32, ApiError> ===");
    let api_result: Result<i32, ApiError> = Result::elicit(&client).await?;
    tracing::info!(?api_result, "Elicited API result");
    match api_result {
        Ok(status_code) => println!("API returned status code: {}", status_code),
        Err(error) => println!("API error: {:?}", error),
    }

    // Elicit Result with enum types
    tracing::info!("=== Eliciting Result<StatusCode, ApiError> ===");
    let status_result: Result<StatusCode, ApiError> = Result::elicit(&client).await?;
    tracing::info!(?status_result, "Elicited status result");
    match status_result {
        Ok(status) => println!("Status: {:?}", status),
        Err(error) => println!("Error: {:?}", error),
    }

    // Elicit optional Result
    tracing::info!("=== Eliciting Option<Result<String, i32>> ===");
    let optional_result: Option<Result<String, i32>> =
        Option::<Result<String, i32>>::elicit(&client).await?;
    tracing::info!(?optional_result, "Elicited optional result");
    match optional_result {
        Some(Ok(value)) => println!("Got successful value: {}", value),
        Some(Err(code)) => println!("Got error code: {}", code),
        None => println!("No result provided"),
    }

    // Elicit collection of Results
    tracing::info!("=== Eliciting Vec<Result<i32, String>> ===");
    let results: Vec<Result<i32, String>> = Vec::<Result<i32, String>>::elicit(&client).await?;
    tracing::info!(?results, count = results.len(), "Elicited results");
    println!("Batch operation results:");
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(value) => println!("  {}. Success: {}", i + 1, value),
            Err(error) => println!("  {}. Error: {}", i + 1, error),
        }
    }

    tracing::info!("Example complete!");
    Ok(())
}
