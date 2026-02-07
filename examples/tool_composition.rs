//! Example: Composing elicitation tools with regular tools
//!
//! Demonstrates how to combine:
//! - Elicitation tools (via #[elicit_tools])
//! - Regular tools (via #[tool])
//!
//! Run with: cargo run --example tool_composition

use elicitation::Elicit;
use elicitation_macros::elicit_tools;
use rmcp::handler::server::wrapper::Json;
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::service::{Peer, RoleServer};
use rmcp::{ErrorData, ServerHandler, tool, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Server configuration that can be elicited interactively
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: u32,
}

/// User information that can be elicited interactively
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct UserInfo {
    pub username: String,
    pub email: String,
}

/// Response type for status tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StatusResponse {
    pub status: String,
    pub uptime_seconds: u64,
    pub active_connections: u32,
}

/// Response type for restart tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RestartResponse {
    pub success: bool,
    pub message: String,
}

/// Example server that combines elicitation and regular tools
pub struct ExampleServer;

#[elicit_tools(ServerConfig, UserInfo)]
#[tool_router]
impl ExampleServer {
    /// Get current server status
    #[tool(description = "Get the current server status including uptime and connections")]
    pub async fn status(_peer: Peer<RoleServer>) -> Result<Json<StatusResponse>, ErrorData> {
        Ok(Json(StatusResponse {
            status: "running".to_string(),
            uptime_seconds: 42,
            active_connections: 3,
        }))
    }

    /// Restart server with new configuration
    #[tool(description = "Restart the server with a new configuration")]
    pub async fn restart(
        _peer: Peer<RoleServer>,
        config: ServerConfig,
    ) -> Result<Json<RestartResponse>, ErrorData> {
        println!(
            "Restarting server with host={}, port={}, max_connections={}",
            config.host, config.port, config.max_connections
        );

        Ok(Json(RestartResponse {
            success: true,
            message: format!("Server restarted on {}:{}", config.host, config.port),
        }))
    }

    /// Create a new user account
    #[tool(description = "Create a new user account")]
    pub async fn create_user(
        _peer: Peer<RoleServer>,
        user: UserInfo,
    ) -> Result<Json<RestartResponse>, ErrorData> {
        println!("Creating user: {} <{}>", user.username, user.email);

        Ok(Json(RestartResponse {
            success: true,
            message: format!("User {} created successfully", user.username),
        }))
    }
}

impl ServerHandler for ExampleServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            name: Some("example-server".into()),
            version: Some("1.0.0".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

fn main() {
    println!("Tool Composition Example");
    println!("========================\n");

    let router = ExampleServer::tool_router();
    let tools = router.list_all();

    println!("Registered {} tools:\n", tools.len());

    // Separate elicit tools from regular tools
    let (elicit_tools, regular_tools): (Vec<_>, Vec<_>) =
        tools.iter().partition(|t| t.name.starts_with("elicit_"));

    println!("Elicitation Tools ({}):", elicit_tools.len());
    for tool in &elicit_tools {
        println!(
            "  - {}: {}",
            tool.name,
            tool.description.as_deref().unwrap_or("")
        );
    }

    println!("\nRegular Tools ({}):", regular_tools.len());
    for tool in &regular_tools {
        println!(
            "  - {}: {}",
            tool.name,
            tool.description.as_deref().unwrap_or("")
        );
    }

    println!("\n\nExample Workflow:");
    println!("=================");
    println!("1. LLM calls elicit_server_config to gather config from user");
    println!("2. Library prompts user: host? port? max_connections?");
    println!("3. User provides: localhost, 8080, 100");
    println!("4. LLM receives validated ServerConfig");
    println!("5. LLM calls restart with the config");
    println!("6. Server restarts with new settings\n");

    println!("This demonstrates the 'last mile' of server-side elicitation:");
    println!("- Elicit tools gather interactive input");
    println!("- Regular tools use that input to take action");
    println!("- Both work seamlessly together in one server");
}
