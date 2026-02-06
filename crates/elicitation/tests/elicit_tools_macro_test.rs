//! Test for the elicit_tools! macro inside an existing impl block.

use elicitation::{Elicit, elicit_tools};
use rmcp::handler::server::wrapper::{Json, Parameters};
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
struct TestConfig {
    name: String,
    port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
struct TestUser {
    username: String,
    email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct CustomResult {
    message: String,
}

struct TestServer;

#[tool_router]
impl TestServer {
    /// A custom tool method.
    #[tool]
    async fn custom_tool(
        &self,
        _name: Parameters<String>,
    ) -> Result<Json<CustomResult>, rmcp::ErrorData> {
        Ok(Json(CustomResult {
            message: "custom".to_string(),
        }))
    }

    // Add elicitation tools using the macro
    elicit_tools! {
        TestConfig,
        TestUser,
    }
}

#[test]
fn test_macro_compiles() {
    // If this compiles, the macro works
    let _server = TestServer;
}

#[test]
fn test_router_exists() {
    // Verify the tool_router function was generated
    let _router = TestServer::tool_router();
}
