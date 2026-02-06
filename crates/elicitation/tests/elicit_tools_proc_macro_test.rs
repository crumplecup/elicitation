//! Test for the #[elicit_tools] proc macro attribute.
//!
//! Tests that the macro correctly generates methods that rmcp's #[tool_router] can discover.

use elicitation::Elicit;
use elicitation_macros::elicit_tools;
use rmcp::tool_router;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[allow(dead_code)]
struct TestConfig {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[allow(dead_code)]
struct TestUser {
    username: String,
}

struct TestServer;

// Order matters: elicit_tools must come BEFORE tool_router
#[elicit_tools(TestConfig, TestUser)]
#[tool_router]
impl TestServer {}

#[test]
fn test_proc_macro_compiles() {
    // If this compiles, the macro generated valid code
    let _server = TestServer;
}

#[test]
fn test_router_generated() {
    // Verify tool_router generated the router function
    let _router = TestServer::tool_router();
}

#[test]
fn test_tool_attr_functions_exist() {
    // Verify that #[tool] generated the _tool_attr functions
    let _config_tool = TestServer::elicit_test_config_tool_attr();
    let _user_tool = TestServer::elicit_test_user_tool_attr();
    
    // Check tool names
    assert_eq!(_config_tool.name, "elicit_test_config");
    assert_eq!(_user_tool.name, "elicit_test_user");
}
