# Botticelli Integration Guide

This guide shows how to integrate elicitation into an rmcp-based MCP server like Botticelli.

## Overview

Elicitation provides strongly-typed data collection from MCP clients. For rmcp servers, use the `#[elicit_tools(...)]` proc macro attribute to add elicitation methods to your server impl.

**Key Pattern:** Generated methods follow rmcp's tool signature requirements:
- No `&self` parameter (standalone functions)
- Return `Result<Json<T>, ErrorData>` (structured data with standard errors)
- Use `Peer<RoleServer>` parameter (extracted via `FromContextPart`)

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
use rmcp::{tool, tool_router};

// Your existing server with tools
#[elicit_tools(CacheKeyNewParams, StorageNewParams)]
#[tool_router]
impl BotticelliServer {
    // Your existing tool methods
    #[tool(description = "Get server status")]
    async fn status(&self) -> String {
        "OK".to_string()
    }
    
    // elicit_tools macro generates methods like:
    //
    // #[tool(description = "Elicit CacheKeyNewParams via MCP")]
    // pub async fn elicit_cache_key_new_params(
    //     peer: Peer<RoleServer>,
    // ) -> Result<Json<CacheKeyNewParams>, ErrorData> {
    //     CacheKeyNewParams::elicit_checked(peer)
    //         .await
    //         .map(Json)
    //         .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    // }
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
1. `#[elicit_tools(...)]` runs first, adds methods with `#[tool]` markers to impl block
2. `#[tool_router]` runs second, discovers and registers all methods (yours + generated)
3. `#[tool]` markers get processed, transforming async methods to Pin<Box<Future>>

If reversed, `#[tool_router]` won't see the generated elicitation methods.

## How It Works

The `#[elicit_tools(...)]` proc macro generates one method per type:

### Generated Method Signature

```rust
#[tool(description = "Elicit CacheKeyNewParams via MCP")]
pub async fn elicit_cache_key_new_params(
    peer: rmcp::service::Peer<rmcp::service::RoleServer>,
) -> Result<rmcp::Json<CacheKeyNewParams>, rmcp::ErrorData> {
    CacheKeyNewParams::elicit_checked(peer)
        .await
        .map(rmcp::Json)
        .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
}
```

**Key Details:**
- **No `&self`**: Method is a "standalone function" for rmcp's parameter extraction
- **Return type**: `Result<Json<T>, ErrorData>` where:
  - `Json<T>` wraps the structured output (implements `IntoCallToolResult`)
  - `ErrorData` is rmcp's standard error type
- **Parameter**: `Peer<RoleServer>` extracted from tool call context via `FromContextPart`
- **`#[tool]` marker**: Processed by rmcp to generate metadata and async transformation

The `#[tool]` macro from rmcp:
1. Generates `*_tool_attr()` function with JSON schema metadata
2. Transforms async fn → sync returning Pin<Box<dyn Future>>
3. Enables `#[tool_router]` to discover and register the tool

## Usage in Tool Methods

Once registered, your other tool methods can access the peer and call elicitation:

```rust
use rmcp::{tool, tool_router, Peer, RoleServer, ErrorData, Json};
use elicitation::Elicitation;

#[tool_router]
impl BotticelliServer {
    #[tool(description = "Create a new cache key")]
    async fn cache_key_new(
        peer: Peer<RoleServer>,
    ) -> Result<Json<String>, ErrorData> {
        // Call elicit_checked directly on the type
        let params = CacheKeyNewParams::elicit_checked(peer.clone())
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        
        // Use the elicited data
        Ok(Json(format!("Created key with {} algorithm", params.hash_algorithm)))
    }
}
```

**Note:** You can either:
1. Call `Type::elicit_checked(peer)` directly in your tool methods
2. Call the generated `elicit_*` methods (they're registered as separate tools for direct client access)

## Generated Tool Names

The proc macro converts PascalCase type names to snake_case method names:

| Type Name | Generated Method | Registered Tool Name |
|-----------|-----------------|---------------------|
| `CacheKeyNewParams` | `elicit_cache_key_new_params` | `"elicit_cache_key_new_params"` |
| `StorageNewParams` | `elicit_storage_new_params` | `"elicit_storage_new_params"` |
| `ApiConfig` | `elicit_api_config` | `"elicit_api_config"` |

## Error Handling

Elicitation methods return `Result<Json<T>, ErrorData>`. The error conversion:

```rust
Type::elicit_checked(peer)
    .await
    .map(Json)  // Wrap success value in Json
    .map_err(|e| ErrorData::internal_error(e.to_string(), None))  // Convert ElicitError
```

Common `ElicitError` variants:
- `ValidationFailed`: User provided invalid data
- `ServerError`: rmcp communication error
- `PromptGenerationFailed`: Elicitation prompt creation failed
- `StyleNotFound`: Requested style variant doesn't exist

## Complete Example

```rust
use elicitation::Elicit;
use elicitation_macros::elicit_tools;
use rmcp::{tool, tool_router, ServerHandler, Peer, RoleServer, ErrorData, Json};
use rmcp::model::{ServerInfo, ServerCapabilities};
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
    async fn cache_key_new(
        peer: Peer<RoleServer>,
    ) -> Result<Json<String>, ErrorData> {
        let params = CacheKeyNewParams::elicit_checked(peer)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        
        Ok(Json(format!("Key: {}", params.hash_algorithm)))
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

## Testing the Integration

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tools_registered() {
        let router = BotticelliServer::tool_router();
        let tools = router.list_all();
        
        // Verify elicitation tool is registered
        assert!(tools.iter().any(|t| t.name == "elicit_cache_key_new_params"));
    }
}
```

## Troubleshooting

### "trait bound `IntoToolRoute` not satisfied"

This means the method signature doesn't match rmcp's requirements. The macro generates the correct signature automatically, but if manually writing methods, check:
- ✅ No `&self` parameter
- ✅ Return type is `Result<Json<T>, ErrorData>`
- ✅ Parameter is `Peer<RoleServer>` (no other parameters)
- ✅ Method has `#[tool]` attribute

### "the trait `IntoCallToolResult` is not implemented"

This means the return type isn't wrapped in `Json`:
```rust
// ❌ Wrong
Result<MyType, ErrorData>

// ✅ Correct
Result<Json<MyType>, ErrorData>
```

### "unresolved import `rmcp::Json`"

Make sure you import from the crate root:
```rust
use rmcp::{Json, ErrorData};  // ✅ Correct
// NOT: use rmcp::handler::server::Json;
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

## API Requirements Summary

For a type to work with `#[elicit_tools(...)]`:

1. **Derive Requirements:**
   ```rust
   #[derive(Clone, Serialize, Deserialize, Elicit, JsonSchema)]
   ```

2. **Import Requirements:**
   ```rust
   use elicitation::Elicit;
   use schemars::JsonSchema;
   use serde::{Serialize, Deserialize};
   ```

3. **Generated Method Signature:**
   - No `&self` parameter
   - Parameter: `Peer<RoleServer>`
   - Return: `Result<Json<T>, ErrorData>`
   - Marked with `#[tool]`

This ensures compatibility with rmcp's tool system and parameter extraction.

## Scaling to Many Types

The proc macro handles any number of types efficiently:

```rust
#[elicit_tools(
    CacheKeyNewParams,
    StorageNewParams,
    ApiConfig,
    DatabaseConfig,
    // ... add as many as needed
)]
#[tool_router]
impl BotticelliServer {
    // All elicitation methods generated automatically
}
```

Each type gets its own registered tool, accessible via MCP.

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

### Why the Specific Return Type?

The return type `Result<Json<T>, ErrorData>` is required by rmcp's trait bounds:

- **`Json<T>` wrapper:** Implements `IntoCallToolResult` for types with `Serialize + JsonSchema`, enabling structured MCP responses
- **`ErrorData`:** rmcp's standard error type, implements `IntoCallToolResult` for error responses
- **`Result<T, ErrorData>` pattern:** Generic impl in rmcp allows any `T: IntoCallToolResult` as success value

Without the `Json` wrapper, types would need to manually implement `IntoCallToolResult`, which isn't possible for external types.

### Why No `&self` Parameter?

rmcp's tool methods use parameter extraction via the `FromContextPart` trait:

```rust
// Parameters extracted from ToolCallContext:
fn tool(peer: Peer<RoleServer>) -> Result<T, ErrorData>
// NOT: fn tool(&self, peer: Peer<RoleServer>)
```

When methods have no `&self`, rmcp extracts parameters from `ToolCallContext` using `FromContextPart`. The `Peer<RoleServer>` provides access to MCP communication needed for elicitation.

This "standalone function" pattern is how rmcp's `#[tool]` macro expects methods to be written.

## See Also

- [Elicitation crate documentation](https://docs.rs/elicitation)
- [rmcp documentation](https://docs.rs/rmcp)
- [MCP protocol specification](https://modelcontextprotocol.io)
