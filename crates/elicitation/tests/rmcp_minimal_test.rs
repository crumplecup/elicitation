//! Minimal rmcp test to verify tool registration works

use rmcp::{handler::server::wrapper::Parameters, tool, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct TestRequest {
    value: String,
}

struct TestServer;

#[tool_router]
impl TestServer {
    #[tool(description = "Manual test method")]
    fn manual_test(&self, Parameters(req): Parameters<TestRequest>) -> String {
        req.value
    }
}

impl rmcp::ServerHandler for TestServer {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo {
            capabilities: rmcp::model::ServerCapabilities::builder()
                .enable_tools()
                .build(),
            ..Default::default()
        }
    }
}

#[test]
fn test_manual_method_works() {
    let _router = TestServer::tool_router();
    let _attr = TestServer::manual_test_tool_attr();
    assert_eq!(_attr.name, "manual_test");
}
