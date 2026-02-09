//! Tests for serde_json::Value elicitation.

#![cfg(feature = "serde_json")]

use elicitation::{ElicitResult, Elicitation};
use serde_json::Value;

// Note: These tests require a real MCP client connection.
// They are designed to be run manually with an MCP client.
// Mark as ignored to prevent running in CI.

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_null() -> ElicitResult<()> {
    // This test requires manual verification:
    // 1. Start an MCP server
    // 2. Connect a client
    // 3. Run this test
    // 4. When prompted, select "null"

    // Mock client setup would go here
    // For now, this is a placeholder showing the intended usage

    Ok(())
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_bool() -> ElicitResult<()> {
    Ok(())
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_string() -> ElicitResult<()> {
    Ok(())
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_number() -> ElicitResult<()> {
    Ok(())
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_array() -> ElicitResult<()> {
    Ok(())
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_object() -> ElicitResult<()> {
    Ok(())
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_nested_array() -> ElicitResult<()> {
    // Test: [[1, 2], [3, 4]]
    Ok(())
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_nested_object() -> ElicitResult<()> {
    // Test: {"user": {"name": "Alice", "age": 30}}
    Ok(())
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_depth_limit() -> ElicitResult<()> {
    // Test: 11 levels deep should fail
    Ok(())
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_mixed_nesting() -> ElicitResult<()> {
    // Test: {"items": [1, "two", {"nested": true}]}
    Ok(())
}

// Unit tests for type validation
#[test]
fn test_value_implements_elicitation() {
    // Compile-time check that Value implements Elicitation
    fn assert_elicitation<T: Elicitation>() {}
    assert_elicitation::<Value>();
}
