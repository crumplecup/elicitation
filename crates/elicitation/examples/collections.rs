//! Example demonstrating standard library collection elicitation.
//!
//! **Note**: This example requires an MCP client (Claude Desktop or Claude CLI).
//! Run with: `claude "Run the collections example"`
//!
//! This example shows how to elicit:
//! - HashMap<K, V> - Key-value pairs with duplicate key handling
//! - BTreeMap<K, V> - Ordered key-value pairs
//! - HashSet<T> - Unique items with automatic deduplication
//! - BTreeSet<T> - Ordered unique items

use elicitation::{Elicit, ElicitClient, ElicitResult, Elicitation, Prompt, Select};
use rmcp::ServiceExt;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

/// Configuration value that can be a string or number
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Elicit)]
enum ConfigValue {
    Text,
    Number,
}

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Initialize tracing for observability
    tracing_subscriber::fmt()
        .with_env_filter("collections=debug,elicitation=debug")
        .init();

    tracing::info!("Starting collections elicitation example");

    // Create MCP client with stdio transport

    let peer = ().serve(rmcp::transport::stdio()).await.expect("Failed to create MCP client");
    let client = ElicitClient::new(&peer);

    // Elicit a HashMap
    tracing::info!("=== Eliciting HashMap<String, i32> ===");
    let scores: HashMap<String, i32> = HashMap::elicit(&client).await?;
    tracing::info!(?scores, "Elicited HashMap");
    println!("Scores: {:?}", scores);

    // Elicit a BTreeMap
    tracing::info!("=== Eliciting BTreeMap<String, ConfigValue> ===");
    let config: BTreeMap<String, ConfigValue> = BTreeMap::elicit(&client).await?;
    tracing::info!(?config, "Elicited BTreeMap");
    println!("Config (ordered): {:?}", config);

    // Elicit a HashSet
    tracing::info!("=== Eliciting HashSet<String> ===");
    let tags: HashSet<String> = HashSet::elicit(&client).await?;
    tracing::info!(?tags, "Elicited HashSet");
    println!("Tags: {:?}", tags);

    // Elicit a BTreeSet
    tracing::info!("=== Eliciting BTreeSet<i32> ===");
    let priorities: BTreeSet<i32> = BTreeSet::elicit(&client).await?;
    tracing::info!(?priorities, "Elicited BTreeSet");
    println!("Priorities (ordered): {:?}", priorities);

    // Elicit a VecDeque
    tracing::info!("=== Eliciting VecDeque<String> ===");
    let queue: VecDeque<String> = VecDeque::elicit(&client).await?;
    tracing::info!(?queue, "Elicited VecDeque");
    println!("Queue: {:?}", queue);

    // Elicit a LinkedList
    tracing::info!("=== Eliciting LinkedList<i32> ===");
    let linked: LinkedList<i32> = LinkedList::elicit(&client).await?;
    tracing::info!(?linked, "Elicited LinkedList");
    println!("Linked list: {:?}", linked);

    tracing::info!("Example complete!");
    Ok(())
}
