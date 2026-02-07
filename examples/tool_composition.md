# Tool Composition Example

This example demonstrates how to combine elicitation tools with your own custom tools in a single MCP server.

## The Pattern

```rust
use elicitation::Elicit;
use elicitation_macros::elicit_tools;
use rmcp::{ServerHandler, tool, tool_router};
use rmcp::handler::server::wrapper::Json;
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::service::{Peer, RoleServer};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// 1. Define your domain types with #[derive(Elicit)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct User {
    pub username: String,
    pub email: String,
}

// 2. Define response types for your regular tools
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StatusResponse {
    pub status: String,
    pub uptime_seconds: u64,
}

// 3. Combine both in your server impl
pub struct MyServer;

#[elicit_tools(Config, User)]  // Generate elicitation tools
#[tool_router]                 // Register all tools with rmcp
impl MyServer {
    /// Regular tool - get server status
    #[tool(description = "Get current server status")]
    pub async fn status(
        _peer: Peer<RoleServer>
    ) -> Result<Json<StatusResponse>, ErrorData> {
        Ok(Json(StatusResponse {
            status: "running".to_string(),
            uptime_seconds: 42,
        }))
    }

    /// Regular tool - restart server with config
    #[tool(description = "Restart server with new configuration")]
    pub async fn restart(
        _peer: Peer<RoleServer>,
        config: Config,
    ) -> Result<Json<StatusResponse>, ErrorData> {
        // Use the elicited config to restart
        println!("Restarting with host={}, port={}", config.host, config.port);
        
        Ok(Json(StatusResponse {
            status: "restarted".to_string(),
            uptime_seconds: 0,
        }))
    }
}

impl ServerHandler for MyServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            ..Default::default()
        }
    }
}

fn main() {
    // Your server now exposes:
    // - elicit_config: Interactive config elicitation
    // - elicit_user: Interactive user elicitation
    // - status: Get server status
    // - restart: Restart with config
    
    let router = MyServer::tool_router();
    let tools = router.list_all();
    
    println!("Registered {} tools:", tools.len());
    for tool in &tools {
        println!("  - {}: {}", tool.name, tool.description.as_deref().unwrap_or(""));
    }
}
```

## Key Points

### 1. Return Types Must Be Objects

MCP requires tool output schemas to be objects, not primitives:

```rust
// ❌ BAD: Primitive return type
#[tool]
pub async fn get_name(_peer: Peer<RoleServer>) -> Result<Json<String>, ErrorData> {
    Ok(Json("Alice".to_string()))
}

// ✅ GOOD: Object return type
#[derive(Serialize, JsonSchema)]
pub struct NameResponse {
    pub name: String,
}

#[tool]
pub async fn get_name(_peer: Peer<RoleServer>) -> Result<Json<NameResponse>, ErrorData> {
    Ok(Json(NameResponse { name: "Alice".to_string() }))
}
```

### 2. Macro Ordering Matters

Always place `#[elicit_tools(...)]` BEFORE `#[tool_router]`:

```rust
#[elicit_tools(Type1, Type2)]  // First: generate elicitation tools
#[tool_router]                 // Second: register all tools
impl MyServer { }
```

### 3. All Types Get elicit_checked()

Every type implementing `Elicitation` automatically has an `elicit_checked()` method:

- Primitives: `String`, `i32`, `bool`, etc.
- Collections: `Vec<T>`, `HashMap<K,V>`, etc.
- Feature-gated: `url::Url`, `uuid::Uuid`, datetime types
- User types: via `#[derive(Elicit)]`

This means you can elicit ANY type server-side:

```rust
// In a tool implementation
let config = Config::elicit_checked(peer).await?;
let name = String::elicit_checked(peer).await?;
let count = i32::elicit_checked(peer).await?;
```

### 4. Workflow Example

```rust
// 1. LLM wants to configure server
// 2. Calls elicit_config tool
// 3. Elicitation library prompts user interactively
// 4. Returns validated Config to LLM
// 5. LLM calls restart tool with the Config
// 6. Server restarts with new configuration
```

## Running the Example

```bash
# Build
cargo build --example tool_composition

# Run
cargo run --example tool_composition
```

## See Also

- `BOTTICELLI_INTEGRATION.md` - Full integration guide
- `tests/composition_systematic_test.rs` - Complete test suite
- `tests/elicit_tools_proc_macro_test.rs` - Macro usage examples
