//! Test that inventory-based tool discovery works.

use elicitation::{Elicit, collect_all_elicit_tools};

#[derive(Debug, Clone, Elicit)]
struct TestConfig {
    name: String,
}

#[derive(Debug, Clone, Elicit)]
struct TestUser {
    id: u32,
}

#[test]
fn test_tool_discovery() {
    // Collect all registered elicit tools
    let tools = collect_all_elicit_tools();
    
    // We should find at least our two test types
    let tool_names: Vec<&str> = tools.iter().map(|t| t.type_name).collect();
    
    println!("Found {} tools: {:?}", tools.len(), tool_names);
    
    // Check that our test types are registered
    assert!(tool_names.contains(&"TestConfig"), "TestConfig should be registered");
    assert!(tool_names.contains(&"TestUser"), "TestUser should be registered");
}

#[test]
fn test_qualified_names() {
    let tools = collect_all_elicit_tools();
    
    for tool in tools {
        let qualified = tool.qualified_name();
        println!("Tool: {} ({})", tool.type_name, qualified);
        
        // Qualified name should contain the type name
        assert!(qualified.contains(tool.type_name));
    }
}
