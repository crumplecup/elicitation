# Server-Side Elicitation Usage Guide

This guide explains how to use elicitation in server-side MCP contexts.

## Overview

Elicitation now supports both client-side and server-side usage:

- **Client-side**: Traditional approach where your app uses MCP client to call tools
- **Server-side**: New approach where your app IS the MCP server providing tools

Both contexts use the same `Elicitation` trait implementations via the `ElicitCommunicator` abstraction.

## Quick Start

### 1. Define Your Type

```rust
use elicitation::Elicit;

#[derive(Debug, Clone, Elicit)]
struct Config {
    database_url: String,
    port: u16,
}
```

The `#[derive(Elicit)]` macro generates:
- `Elicitation` trait implementation
- `elicit_checked()` MCP tool function
- Tool metadata for discovery

### 2. Register Tools with Router

Use the `elicit_router!` macro to aggregate multiple types:

```rust
use elicitation::elicit_router;

// Aggregate all your elicitable types
elicit_router! {
    pub MyAppRouter: Config, DatabaseSettings, ApiKey
}
```

This generates a `MyAppRouter` struct with a `tool_router()` method.

### 3. Integrate with MCP Server

```rust
use rmcp::handler::server::ServerHandler;
use elicitation::elicit_router;

#[derive(Debug, Clone)]
struct MyServer;

// Your elicitable types
#[derive(Debug, Clone, Elicit)]
struct Config { /* ... */ }

// Generate router
elicit_router! {
    pub ConfigRouter: Config
}

// In your server setup:
async fn setup_server() -> Result<(), Box<dyn std::error::Error>> {
    let mut handler = ServerHandler::new("my-app-server");
    
    // Add elicitation tools
    handler.tool_router = ConfigRouter::tool_router();
    
    // ... rest of server setup
    Ok(())
}
```

## How It Works

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│ MCP Client (Claude, etc.)                                    │
│ - Calls elicit_checked_Config() tool                         │
│ - Receives structured prompts                                 │
│ - Returns responses                                           │
└─────────────────┬───────────────────────────────────────────┘
                  │ MCP Protocol
┌─────────────────▼───────────────────────────────────────────┐
│ Your MCP Server                                               │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │ elicit_checked_Config(peer: Peer<RoleServer>)           │ │
│ │   ↓                                                       │ │
│ │ ElicitServer::new(peer)                                  │ │
│ │   ↓                                                       │ │
│ │ Config::elicit(&server).await                            │ │
│ │   ↓                                                       │ │
│ │ server.send_prompt("Enter database URL:")                │ │
│ │   ↓                                                       │ │
│ │ peer.create_message(...)  ──────────────────────────────►│ │
│ └─────────────────────────────────────────────────────────┘ │
└───────────────────────────────────────────────────────────────┘
```

### Communication Flow

1. **Tool Call**: Client invokes `elicit_checked_Config()`
2. **Wrapper Creation**: Tool creates `ElicitServer` wrapper around `Peer<RoleServer>`
3. **Trait Delegation**: Calls `Config::elicit(&server).await`
4. **Prompt Sending**: Server sends prompts via `peer.create_message()`
5. **Response**: Client responds with structured data
6. **Validation**: Response parsed and validated
7. **Return**: Validated `Config` returned to client

## Advanced Usage

### Custom Styles

Define custom prompting styles for different contexts:

```rust
use elicitation::{Elicit, ElicitationStyle};

#[derive(Debug, Clone, Elicit)]
struct Config {
    #[prompt("Database connection string")]
    #[styled_prompt(terse = "DB URL")]
    #[styled_prompt(verbose = "Please enter the full database connection URL including credentials")]
    database_url: String,
}

// Style enum generated automatically:
// pub enum ConfigElicitStyle { Default, Terse, Verbose }
```

### Type Composition

Elicitable types compose naturally:

```rust
#[derive(Debug, Clone, Elicit)]
struct DatabaseConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Clone, Elicit)]
struct AppConfig {
    database: DatabaseConfig,  // Nested elicitation!
    api_key: String,
}
```

When `AppConfig` is elicited:
1. Prompts for `database` (which elicits `DatabaseConfig`)
   - Prompts for `host`
   - Prompts for `port`
2. Prompts for `api_key`

### Verification Types

Use verification wrappers for type-safe constraints:

```rust
use elicitation::verification::types::{NonEmpty, UrlHttp};

#[derive(Debug, Clone, Elicit)]
struct Config {
    // Guaranteed non-empty at compile time
    api_keys: NonEmpty<Vec<String>>,
    
    // Guaranteed valid HTTP/HTTPS URL
    webhook_url: UrlHttp,
}
```

## Client-Side vs Server-Side

### When to Use Server-Side

Use server-side elicitation when:
- Your app is an MCP server exposing tools
- You want clients (AI agents) to configure your app via conversation
- You need interactive configuration gathering
- You're building agentic tools that require structured input

### When to Use Client-Side

Use client-side elicitation when:
- Your app is an MCP client using tools
- You're building a CLI or TUI that gathers user input
- You want local, interactive data collection

### Both Work the Same

The beauty of the unified architecture: **the same `Elicitation` trait implementations work in both contexts**.

```rust
// This type works in BOTH client and server contexts
#[derive(Debug, Clone, Elicit)]
struct Config {
    database_url: String,
}

// Client-side usage:
let client = ElicitClient::new(peer_client);
let config = Config::elicit(&client).await?;

// Server-side usage (via tool):
// Auto-generated by #[derive(Elicit)]:
// async fn elicit_checked(peer: Peer<RoleServer>) -> Result<Config, ElicitError> {
//     let server = ElicitServer::new(peer);
//     Config::elicit(&server).await
// }
```

## Tool Discovery

Tools are automatically registered with the inventory system:

```rust
use elicitation::inventory;

// Query all elicitable types at runtime
for tool in inventory::iter::<ElicitToolDescriptor> {
    println!("Tool: {} in module {}", tool.type_name, tool.module_path);
}
```

This enables:
- Dynamic tool discovery
- Automatic documentation generation
- Runtime introspection
- IDE integration

## Error Handling

All elicitation operations return `ElicitResult<T>`:

```rust
use elicitation::{ElicitError, ElicitErrorKind, ElicitResult};

async fn example(server: &ElicitServer) -> ElicitResult<Config> {
    match Config::elicit(server).await {
        Ok(config) => {
            println!("Successfully elicited: {:?}", config);
            Ok(config)
        }
        Err(e) => {
            match &e.kind {
                ElicitErrorKind::ParseError(msg) => {
                    eprintln!("Invalid format: {}", msg);
                }
                ElicitErrorKind::Service(err) => {
                    eprintln!("MCP service error: {}", err);
                }
                _ => {
                    eprintln!("Elicitation failed: {}", e);
                }
            }
            Err(e)
        }
    }
}
```

## Testing

Test your elicitable types without MCP infrastructure:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use elicitation::ElicitCommunicator;
    
    // Mock communicator for testing
    struct MockCommunicator;
    
    impl ElicitCommunicator for MockCommunicator {
        async fn send_prompt(&self, _prompt: &str) -> ElicitResult<String> {
            Ok("mock response".to_string())
        }
        
        fn style_context(&self) -> &StyleContext {
            &StyleContext::default()
        }
        
        // ... other trait methods
    }
    
    #[tokio::test]
    async fn test_config_elicitation() {
        let mock = MockCommunicator;
        let result = Config::elicit(&mock).await;
        assert!(result.is_ok());
    }
}
```

## Best Practices

### 1. Keep Types Focused

```rust
// Good: Single responsibility
#[derive(Elicit)]
struct DatabaseConfig { /* ... */ }

#[derive(Elicit)]
struct ApiConfig { /* ... */ }

// Better than: Kitchen sink
#[derive(Elicit)]
struct AppConfig {
    database: DatabaseConfig,
    api: ApiConfig,
}
```

### 2. Use Verification Types

```rust
use elicitation::verification::types::*;

#[derive(Elicit)]
struct Config {
    // Don't: String (any value)
    // Do: EmailAddress (validated)
    admin_email: EmailAddress,
    
    // Don't: Vec<String> (could be empty)
    // Do: NonEmpty<Vec<String>> (guaranteed non-empty)
    api_keys: NonEmpty<Vec<String>>,
}
```

### 3. Provide Clear Prompts

```rust
#[derive(Elicit)]
struct Config {
    #[prompt("Enter the database connection URL (e.g., postgres://localhost:5432/mydb)")]
    database_url: String,
    
    #[prompt("API rate limit (requests per minute, 0 for unlimited)")]
    rate_limit: u32,
}
```

### 4. Document Tool Purpose

```rust
/// Configuration for the payment processing service.
///
/// This tool allows interactive configuration of payment gateway settings,
/// API credentials, and processing parameters.
#[derive(Elicit)]
struct PaymentConfig {
    /// Payment gateway URL (must be HTTPS)
    gateway_url: UrlHttp,
    
    /// API key for authentication
    api_key: String,
}
```

## Migration from Client-Side

If you have existing client-side elicitation code:

**Before (Client-Side Only)**:
```rust
async fn setup(client: &ElicitClient) -> Result<Config, ElicitError> {
    let config = Config::elicit(client).await?;
    Ok(config)
}
```

**After (Both Client and Server)**:
```rust
// Client-side (unchanged):
async fn setup_client(client: &ElicitClient) -> Result<Config, ElicitError> {
    let config = Config::elicit(client).await?;
    Ok(config)
}

// Server-side (new):
// Just use #[derive(Elicit)] and register with elicit_router!
// The elicit_checked() tool is auto-generated
```

No code changes needed - just add server registration!

## See Also

- [SERVER_SIDE_ELICITATION_PLAN.md](SERVER_SIDE_ELICITATION_PLAN.md) - Implementation plan
- [UNIFIED_ELICITATION_TRAIT_PLAN.md](UNIFIED_ELICITATION_TRAIT_PLAN.md) - Architecture design
- [MCP_INTEGRATION_SUMMARY.md](MCP_INTEGRATION_SUMMARY.md) - MCP integration overview
