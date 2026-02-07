use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::{ErrorData, Json, Peer, RoleServer, ServerHandler, tool, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct TestType {
    value: String,
}

struct TestServer;

#[tool_router]
impl TestServer {
    #[tool(description = "Manual method matching macro output")]
    pub async fn manual_elicit(_peer: Peer<RoleServer>) -> Result<Json<TestType>, ErrorData> {
        Ok(Json(TestType {
            value: "test".to_string(),
        }))
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
fn test_manual_method() {
    let _router = TestServer::tool_router();
    let _attr = TestServer::manual_elicit_tool_attr();
    assert_eq!(_attr.name, "manual_elicit");
}
