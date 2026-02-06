# Botticelli Integration Guide

This guide shows how to integrate elicitation into an rmcp-based MCP server like Botticelli.

## Overview

Elicitation provides strongly-typed data collection from MCP clients. For rmcp servers, use the `#[elicit_tools(...)]` proc macro attribute to add elicitation methods to your server impl.

## Prerequisites

Your types must derive both `Elicit` and `JsonSchema`:

```rust
use elicitation::Elicit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Elicit, JsonSchema)]
pub struct CacheKeyNewParams {
    pub hash_algorithm: String,
    pub ttl_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Elicit, JsonSchema)]
pub struct StorageNewParams {
    pub backend: String,
    pub path: String,
}
```

## Integration Pattern

### Using the Proc Macro Attribute (Recommended)

The `#[elicit_tools(...)]` proc macro attribute adds elicitation methods to your existing server impl:

```rust
use elicitation_macros::elicit_tools;
use rmcp::tool_router;

// Your existing server with tools
#[elicit_tools(CacheKeyNewParams, StorageNewParams)]
#[tool_router]
impl BotticelliServer {
    // Your existing tool methods
    #[tool(description = "Get server status")]
    async fn status(&self) -> String {
        "OK".to_string()
    }
    
    // elicit_tools macro generates:
    // - pub fn elicit_cache_key_new_params(&self, peer) -> Pin<Box<...>>
    // - pub fn elicit_cache_key_new_params_tool_attr() -> Tool
    // - pub fn elicit_storage_new_params(&self, peer) -> Pin<Box<...>>  
    // - pub fn elicit_storage_new_params_tool_attr() -> Tool
}
```

### Important: Macro Ordering

**The order matters!** Place `#[elicit_tools(...)]` BEFORE `#[tool_router]`:

```rust
#[elicit_tools(Type1, Type2)]  // ← FIRST
#[tool_router]                  // ← SECOND
impl MyServer { }
```

**Why?** Rust processes attribute macros outer-to-inner (stack-like):
1. `#[elicit_tools(...)]` runs first, adds methods to impl block
2. `#[tool_router]` runs second, discovers and registers all methods (yours + generated)

If reversed, `#[tool_router]` won't see the generated elicitation methods.

## How It Works

The `#[elicit_tools(...)]` proc macro generates two items per type:

### 1. The Tool Method

```rust
pub fn elicit_cache_key_new_params(
    &self,
    peer: rmcp::service::Peer<rmcp::service::RoleServer>,
) -> std::pin::Pin<Box<
    dyn std::future::Future<
        Output = Result<CacheKeyNewParams, elicitation::ElicitError>
    > + Send + '_
>> {
    Box::pin(async move {
        CacheKeyNewParams::elicit_checked(peer).await
    })
}
```

This method:
- Accepts `Peer<RoleServer>` (rmcp's server peer)
- Returns a pinned future (compatible with rmcp's async handlers)
- Calls `elicit_checked()` which handles the elicitation flow
- Is discovered by `#[tool_router]` and registered as an MCP tool

### 2. The Tool Metadata Function

```rust
pub fn elicit_cache_key_new_params_tool_attr() -> rmcp::model::Tool {
    rmcp::model::Tool {
        name: "elicit_cache_key_new_params".into(),
        description: Some("Elicit CacheKeyNewParams via MCP".into()),
        input_schema: Arc::new(/* empty object schema */),
        output_schema: Some(schema_for_type::<CacheKeyNewParams>()),
        // ... other metadata
    }
}
```

This function:
- Returns tool metadata for rmcp
- Includes JSON schema for the output type
- Is called by `#[tool_router]` during registration

## Usage in Tool Methods

Once registered, your other tool methods can call elicitation methods:

```rust
#[tool_router]
impl BotticelliServer {
    #[tool(description = "Create a new cache key")]
    async fn cache_key_new(&self, peer: Peer<RoleServer>) -> Result<String, ErrorData> {
        // Call generated elicitation method
        let params = self.elicit_cache_key_new_params(peer.clone())
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string()))?;
        
        // Use the elicited data
        Ok(format!("Created key with {} algorithm", params.hash_algorithm))
    }
}
```

## Generated Tool Names

The proc macro converts PascalCase type names to snake_case method names:

| Type Name | Generated Method | Generated Tool Attr |
|-----------|-----------------|---------------------|
| `CacheKeyNewParams` | `elicit_cache_key_new_params` | `elicit_cache_key_new_params_tool_attr` |
| `StorageNewParams` | `elicit_storage_new_params` | `elicit_storage_new_params_tool_attr` |
| `ApiConfig` | `elicit_api_config` | `elicit_api_config_tool_attr` |

## Error Handling

Elicitation methods return `Result<T, ElicitError>`. Common error scenarios:

```rust
match self.elicit_cache_key_new_params(peer).await {
    Ok(params) => {
        // Use params
    }
    Err(ElicitError::ValidationFailed { message, .. }) => {
        // User provided invalid data
    }
    Err(ElicitError::ServerError { message, .. }) => {
        // rmcp communication error
    }
    // ... other error variants
}
```

## Complete Example

```rust
use elicitation::Elicit;
use elicitation_macros::elicit_tools;
use rmcp::{tool, tool_router, ServerHandler};
use rmcp::service::{Peer, RoleServer};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// 1. Define types with Elicit + JsonSchema
#[derive(Debug, Clone, Serialize, Deserialize, Elicit, JsonSchema)]
pub struct CacheKeyNewParams {
    pub hash_algorithm: String,
    pub ttl_seconds: u64,
}

// 2. Server struct
pub struct BotticelliServer {
    // ... fields
}

// 3. Add elicitation to server
#[elicit_tools(CacheKeyNewParams)]
#[tool_router]
impl BotticelliServer {
    #[tool(description = "Create a cache key")]
    async fn cache_key_new(&self, peer: Peer<RoleServer>) -> Result<String, ErrorData> {
        let params = self.elicit_cache_key_new_params(peer)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string()))?;
        
        Ok(format!("Key: {}", params.hash_algorithm))
    }
}

// 4. Implement ServerHandler
impl ServerHandler for BotticelliServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            ..Default::default()
        }
    }
}
```

## Troubleshooting

### "trait bound not satisfied" errors

**Symptom:** Compilation error about `IntoToolRoute` or `CallToolHandler` not satisfied.

**Cause:** Missing `JsonSchema` derive on types.

**Fix:** Add `#[derive(JsonSchema)]`:
```rust
#[derive(Debug, Clone, Elicit, JsonSchema)]  // ← JsonSchema required
pub struct MyType { }
```

### "cannot find function" errors

**Symptom:** `tool_router()` not generated or methods missing.

**Cause:** Wrong macro order - `#[tool_router]` before `#[elicit_tools(...)]`.

**Fix:** Reverse the order:
```rust
#[elicit_tools(Type1)]  // ← FIRST
#[tool_router]          // ← SECOND
impl MyServer { }
```

### Type not in scope

**Symptom:** Generated code can't find your type.

**Cause:** Type not imported where macro is used.

**Fix:** Import types in same module as server impl:
```rust
use crate::types::{CacheKeyNewParams, StorageNewParams};

#[elicit_tools(CacheKeyNewParams, StorageNewParams)]
#[tool_router]
impl BotticelliServer { }
```

## Implementation Details

### Why a Proc Macro Attribute?

We initially tried a declarative `elicit_tools!` macro but hit Rust's macro expansion ordering:

```rust
// ❌ Declarative macro - expands AFTER attribute macros
#[tool_router]
impl Server {
    elicit_tools! { Type1 }  // Expands last - tool_router can't see it
}
```

Declarative macros expand AFTER all attribute macros, so `#[tool_router]` never sees the generated methods.

The proc macro attribute solves this:

```rust
// ✅ Proc macro attribute - runs BEFORE tool_router
#[elicit_tools(Type1)]  // Adds methods first
#[tool_router]          // Discovers methods second
impl Server { }
```

### Why Not Apply #[tool] to Generated Methods?

We initially tried generating methods with `#[::rmcp::tool]` markers, but **proc macros cannot apply other proc macros**. The generated `#[::rmcp::tool]` is just text that never expands.

Instead, we studied rmcp's source (`rmcp-macros-0.14.0/src/tool.rs`) and directly implement what `#[tool]` does:
1. Generate `*_tool_attr()` metadata functions
2. Transform async methods to sync returning `Pin<Box<dyn Future>>`
3. Wrap bodies in `Box::pin(async move { ... })`

This ensures compatibility with rmcp's trait bounds (`CallToolHandler`, `IntoToolRoute`) without requiring `#[tool]` to process our generated code.

## See Also

- [Elicitation crate documentation](https://docs.rs/elicitation)
- [rmcp documentation](https://docs.rs/rmcp)
- [MCP protocol specification](https://modelcontextprotocol.io)
