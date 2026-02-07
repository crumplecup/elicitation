//! Test rmcp's pattern: async fn with Peer parameter (no &self)

use rmcp::model::{Meta, ServerCapabilities, ServerInfo};
use rmcp::{Peer, RoleServer, ServerHandler, tool, tool_router};

struct TestServer;

#[tool_router]
impl TestServer {
    #[tool(description = "Test with peer only")]
    pub async fn test_method(_peer: Peer<RoleServer>) -> Result<String, rmcp::ErrorData> {
        Ok("test".to_string())
    }

    #[tool(description = "Test with meta and peer")]
    pub async fn test_with_meta(
        _meta: Meta,
        _peer: Peer<RoleServer>,
    ) -> Result<String, rmcp::ErrorData> {
        Ok("test".to_string())
    }
}

impl ServerHandler for TestServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[test]
fn test_compiles() {
    let _router = TestServer::tool_router();
}
