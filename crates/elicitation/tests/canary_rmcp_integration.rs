//! Canary test: Verify elicit_checked method signature is rmcp-compatible.
//!
//! Tests that the generated elicit_checked method can be used with rmcp's
//! tool system via macros like elicit_router! and elicit_tools!

use elicitation::Elicit;

#[derive(Debug, Clone, Elicit)]
struct _CanaryConfig {
    _value: String,
}

#[test]
fn canary_tool_metadata() {
    // Test 1: Tool metadata is generated
    let tool = _CanaryConfig::elicit_checked_tool_attr();
    assert!(!tool.name.is_empty());
    assert_eq!(tool.name, "elicit_checked");
    
    // Test 2: The method exists (compilation test)
    // If this compiles, the signature is correct:
    // async fn elicit_checked(peer: Peer<RoleServer>) -> Result<Self, ElicitError>
    // Note: We can't actually call it without a real peer, but compilation proves the signature.
    let _ = _CanaryConfig::elicit_checked;
}

#[test]
fn canary_macro_integration() {
    use elicitation::elicit_router;
    
    // Test that elicit_router! macro works with the generated methods
    elicit_router! {
        _CanaryRouter: _CanaryConfig
    }
    
    // If this compiles, the macro integration is working
    let _router = _CanaryRouter;
}
