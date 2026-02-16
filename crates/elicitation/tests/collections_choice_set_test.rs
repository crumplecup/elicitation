//! Tests for ChoiceSet with dynamic choice elicitation.

use anyhow::Result;
use elicitation::{ChoiceSet, ElicitClient};
use rmcp::{
    model::{CallToolResult, Content, ToolStatus},
    service::{Peer, RoleClient},
};

/// Mock peer for testing ChoiceSet elicitation.
fn mock_peer_with_selection(selection: &str) -> Peer<RoleClient> {
    // Create a mock peer that returns the specified selection
    let result = CallToolResult {
        content: vec![Content::Text {
            text: selection.to_string(),
        }],
        is_error: None,
        tool_status: Some(ToolStatus::Complete),
    };

    // In a real test, we'd set up the mock to return this result
    // For now, this is a placeholder showing the intended structure
    todo!("Mock peer creation requires rmcp mock support")
}

#[tokio::test]
#[ignore] // Requires mock support
async fn test_choice_set_tic_tac_toe_moves() -> Result<()> {
    // Available moves in a tic-tac-toe game (positions 1, 3, 5, 7, 9)
    let available_moves = vec![1, 3, 5, 7, 9];

    let choice_set = ChoiceSet::new(available_moves.clone())
        .with_prompt("Pick your move (available positions):");

    // Mock would return "3" as the selection
    let peer = mock_peer_with_selection("3");
    let client = ElicitClient::new(peer);

    let selected_move = choice_set.elicit(&client).await?;

    // The result should be the selected item (i32), not a ChoiceSet
    assert_eq!(selected_move, 3);
    Ok(())
}

#[tokio::test]
#[ignore] // Requires mock support
async fn test_choice_set_string_choices() -> Result<()> {
    // String choices (chess pieces)
    let pieces = vec![
        "Pawn".to_string(),
        "Knight".to_string(),
        "Bishop".to_string(),
        "Rook".to_string(),
        "Queen".to_string(),
        "King".to_string(),
    ];

    let choice_set = ChoiceSet::new(pieces).with_prompt("Choose a piece:");

    let peer = mock_peer_with_selection("Queen");
    let client = ElicitClient::new(peer);

    let selected_piece = choice_set.elicit(&client).await?;

    assert_eq!(selected_piece, "Queen");
    Ok(())
}

#[tokio::test]
#[ignore] // Requires mock support
async fn test_choice_set_default_prompt() -> Result<()> {
    // Test without custom prompt (uses default)
    let options = vec!["A", "B", "C"];
    let choice_set = ChoiceSet::new(options);

    let peer = mock_peer_with_selection("B");
    let client = ElicitClient::new(peer);

    let selected_option = choice_set.elicit(&client).await?;

    assert_eq!(selected_option, "B");
    Ok(())
}

#[test]
fn test_choice_set_construction() {
    // Test that ChoiceSet can be constructed
    let moves = vec![1, 2, 3];
    let choice_set = ChoiceSet::new(moves.clone());

    assert_eq!(choice_set.items(), &[1, 2, 3]);

    let with_prompt = choice_set.with_prompt("Pick one:");
    assert_eq!(with_prompt.items(), &[1, 2, 3]);
}
