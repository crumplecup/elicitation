//! Canary test: Can we use Config::elicit_checked as a function pointer with rmcp?
//!
//! Tests that our Peer<RoleServer> signature is compatible with rmcp's CallToolHandler.

use elicitation::Elicit;
use rmcp::handler::server::router::tool::ToolRouter;

#[derive(Debug, Clone, Elicit)]
struct _CanaryConfig {
    _value: String,
}

struct _TestServer;

#[test]
fn canary_function_pointer_with_route() {
    // Test 1: Can we reference the function?
    let _fn_ptr = _CanaryConfig::elicit_checked;

    // Test 2: Can we get tool_attr?
    let tool = _CanaryConfig::elicit_checked_tool_attr();
    assert!(!tool.name.is_empty());
    assert_eq!(tool.name, "elicit_checked__CanaryConfig");

    // Test 3: Can we pass it to with_route?
    // This is the key test - does rmcp accept our Peer<RoleServer> signature?
    let router = ToolRouter::<_TestServer>::new().with_route((
        _CanaryConfig::elicit_checked_tool_attr(),
        _CanaryConfig::elicit_checked,
    ));

    // Success! The function signature is compatible with rmcp.
    drop(router);
}
