//! Example demonstrating smart pointer elicitation.
//!
//! **Note**: This example requires an MCP client (Claude Desktop or Claude CLI).
//! Run with: `claude "Run the smart_pointers example"`
//!
//! This example shows how to elicit:
//! - Box<T> - Heap-allocated values
//! - Rc<T> - Reference-counted values
//! - Arc<T> - Thread-safe reference-counted values

use elicitation::{ElicitResult, Elicitation};
use pmcp::StdioTransport;
use std::rc::Rc;
use std::sync::Arc;

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Initialize tracing for observability
    tracing_subscriber::fmt()
        .with_env_filter("smart_pointers=debug,elicitation=debug")
        .init();

    tracing::info!("Starting smart pointer elicitation example");

    // Create MCP client
    let transport = StdioTransport::new();
    let client = pmcp::Client::new(transport);

    // Elicit a Box<String>
    tracing::info!("=== Eliciting Box<String> ===");
    let boxed_string: Box<String> = Box::<String>::elicit(&client).await?;
    tracing::info!(?boxed_string, "Elicited Box");
    println!("Boxed string: {}", boxed_string);

    // Elicit a Box with a complex type
    tracing::info!("=== Eliciting Box<Vec<i32>> ===");
    let boxed_vec: Box<Vec<i32>> = Box::<Vec<i32>>::elicit(&client).await?;
    tracing::info!(?boxed_vec, count = boxed_vec.len(), "Elicited Box<Vec>");
    println!("Boxed vector: {:?}", boxed_vec);

    // Elicit an Rc<String>
    tracing::info!("=== Eliciting Rc<String> ===");
    let rc_string: Rc<String> = Rc::<String>::elicit(&client).await?;
    tracing::info!(?rc_string, "Elicited Rc");
    println!("Rc string: {}", rc_string);
    println!("Strong count: {}", Rc::strong_count(&rc_string));

    // Elicit an Arc<i32>
    tracing::info!("=== Eliciting Arc<i32> ===");
    let arc_number: Arc<i32> = Arc::<i32>::elicit(&client).await?;
    tracing::info!(?arc_number, "Elicited Arc");
    println!("Arc number: {}", arc_number);
    println!("Strong count: {}", Arc::strong_count(&arc_number));

    // Elicit an Arc with complex nested type
    tracing::info!("=== Eliciting Arc<Option<String>> ===");
    let arc_option: Arc<Option<String>> = Arc::<Option<String>>::elicit(&client).await?;
    tracing::info!(?arc_option, "Elicited Arc<Option>");
    match arc_option.as_ref() {
        Some(value) => println!("Arc<Option>: Some({})", value),
        None => println!("Arc<Option>: None"),
    }

    // Demonstrate shared ownership with Rc
    tracing::info!("=== Demonstrating Rc shared ownership ===");
    let rc_data: Rc<String> = Rc::<String>::elicit(&client).await?;
    let rc_clone = Rc::clone(&rc_data);
    println!("Original: {}", rc_data);
    println!("Clone: {}", rc_clone);
    println!(
        "Both point to same data: {}",
        Rc::ptr_eq(&rc_data, &rc_clone)
    );
    println!("Strong count: {}", Rc::strong_count(&rc_data));

    tracing::info!("Example complete!");
    Ok(())
}
