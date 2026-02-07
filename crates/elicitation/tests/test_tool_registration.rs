use elicitation::Elicit;
use elicitation_macros::elicit_tools;
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::{ServerHandler, tool, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
struct TestConfig {
    name: String,
}

struct TestServer;

#[elicit_tools(TestConfig)]
#[tool_router]
impl TestServer {}

impl ServerHandler for TestServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[test]
fn test_tools_registered() {
    let router = TestServer::tool_router();
    let tools = router.list_all();
    println!("Tools registered: {}", tools.len());
    for tool in &tools {
        println!("  - {}", tool.name);
    }
    assert!(tools.iter().any(|t| t.name == "elicit_test_config"));
}
