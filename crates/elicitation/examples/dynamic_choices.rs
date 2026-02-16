//! Example demonstrating dynamic choice set elicitation with ChoiceSet.
//!
//! **Note**: This example requires an MCP client (Claude Desktop or Claude CLI).
//! Run with: `claude "Run the dynamic_choices example"`
//!
//! This example shows how to use ChoiceSet for selecting from runtime-generated
//! options, such as available moves in a game or dynamic menu items.

use std::sync::Arc;

use elicitation::{ChoiceSet, ElicitClient, ElicitResult};
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Initialize tracing for observability
    tracing_subscriber::fmt()
        .with_env_filter("dynamic_choices=debug,elicitation=debug")
        .init();

    tracing::info!("Starting dynamic choices elicitation example");

    // Create MCP client with stdio transport
    let service = ().serve(rmcp::transport::stdio()).await.expect("Failed to create MCP client");
    let peer = service.peer();
    let client = ElicitClient::new(Arc::new(peer.clone()));

    // Example 1: Tic-tac-toe available moves
    // In a real game, this would be computed from the board state
    tracing::info!("=== Example 1: Tic-Tac-Toe Moves ===");
    let available_moves = vec![1, 3, 5, 7, 9]; // Center and corners available

    let selected_move = ChoiceSet::new(available_moves)
        .with_prompt("Pick your move (available positions):")
        .elicit(&client)
        .await?;

    tracing::info!(move_selected = %selected_move, "Player selected move");
    println!("You selected position: {}", selected_move);

    // Example 2: Menu items from inventory
    tracing::info!("=== Example 2: Dynamic Menu ===");
    let menu_items = vec![
        "New Game".to_string(),
        "Load Save".to_string(),
        "Settings".to_string(),
        "Quit".to_string(),
    ];

    let menu_choice = ChoiceSet::new(menu_items)
        .with_prompt("Main Menu:")
        .elicit(&client)
        .await?;

    tracing::info!(choice = %menu_choice, "Menu item selected");
    println!("Menu selection: {}", menu_choice);

    // Example 3: Available actions based on game state
    tracing::info!("=== Example 3: Conditional Actions ===");
    let has_key = true;
    let has_torch = false;

    // Build action list based on current state
    let mut actions = vec!["Look around".to_string(), "Go back".to_string()];

    if has_key {
        actions.push("Unlock door".to_string());
    }
    if has_torch {
        actions.push("Light torch".to_string());
    }

    let action = ChoiceSet::new(actions)
        .with_prompt("What do you want to do?")
        .elicit(&client)
        .await?;

    tracing::info!(action = %action, "Action selected");
    println!("Action: {}", action);

    // Example 4: Selecting from filtered results
    tracing::info!("=== Example 4: Filtered Options ===");
    let all_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let even_numbers: Vec<i32> = all_numbers.into_iter().filter(|n| n % 2 == 0).collect();

    let chosen_even = ChoiceSet::new(even_numbers)
        .with_prompt("Pick an even number:")
        .elicit(&client)
        .await?;

    tracing::info!(number = %chosen_even, "Even number selected");
    println!("You chose: {}", chosen_even);

    tracing::info!("Example complete!");
    Ok(())
}
