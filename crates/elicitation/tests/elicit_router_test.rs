//! Test the elicit_router! macro for aggregating MCP tools.

use elicitation::{Elicit, elicit_router};

#[derive(Debug, Clone, Elicit)]
struct _TestConfig {
    _name: String,
}

#[derive(Debug, Clone, Elicit)]
struct _TestUser {
    _id: u32,
}

// Generate the aggregator router
elicit_router! {
    pub TestElicitRouter: _TestConfig, _TestUser
}

#[test]
fn test_router_struct_exists() {
    // Verify the struct compiles
    let _router = TestElicitRouter;
}

#[test]
fn test_tool_router_method_exists() {
    // Verify the tool_router() method exists and returns a ToolRouter
    let _tool_router = TestElicitRouter::tool_router();
}
