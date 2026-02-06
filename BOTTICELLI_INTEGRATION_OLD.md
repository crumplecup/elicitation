# Integrating Elicitation with Botticelli MCP Server

## The Problem

Botticelli has many parameter types with `#[derive(Elicit)]` but needs to expose them as MCP tools on the `BotticelliServer`.

The `elicit_router!` macro creates `ToolRouter<ElicitRouter>` which cannot be combined with `ToolRouter<BotticelliServer>` (type mismatch).

## The Solution: `elicit_tools!` macro

Use the `elicit_tools!` macro to generate elicitation methods **inside** your existing `#[tool_router]` impl block.

## Example

```rust
use elicitation::elicit_tools;
use rmcp::{tool, tool_router};

// Your types with #[derive(Elicit)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct CacheKeyNewParams {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct StorageNewParams {
    pub base_path: String,
}

// Your server
pub struct BotticelliServer {
    // fields...
}

#[tool_router]
impl BotticelliServer {
    // Your existing custom tools
    #[tool]
    async fn my_custom_tool(&self, params: Parameters<SomeType>) 
        -> Result<Json<SomeResult>, rmcp::ErrorData> 
    {
        // implementation...
    }
    
    // Add ALL your elicitable types here
    elicit_tools! {
        CacheKeyNewParams,
        StorageNewParams,
        // Add all other param types from your tool modules...
    }
}
```

## What Gets Generated

For each type, the macro generates:

```rust
#[tool]
async fn elicit_cache_key_new_params(
    &self,
    peer: rmcp::service::Peer<rmcp::service::RoleServer>,
) -> Result<CacheKeyNewParams, elicitation::ElicitError> {
    CacheKeyNewParams::elicit_checked(peer).await
}
```

## Benefits

1. **Type-safe**: All methods on `ToolRouter<BotticelliServer>`
2. **No wrappers**: Direct use of your `#[derive(Elicit)]` types
3. **Automatic naming**: `elicit_cache_key_new_params()` from `CacheKeyNewParams`
4. **Server-side elicitation**: Server prompts client/agent for structured data
5. **Validation included**: `elicit_checked()` validates the response

## Migration Path

### Before (manual wrappers)
```rust
#[tool]
pub async fn elicit_text(&self, Parameters(params): Parameters<ElicitTextParams>)
    -> Result<Json<ElicitTextResult>, rmcp::ErrorData> 
{
    // Manual implementation with protocol switching...
}
```

### After (macro-generated)
```rust
elicit_tools! {
    ElicitTextParams,  // Automatically generates elicit_elicit_text_params()
}
```

The client calls `elicit_elicit_text_params()`, server prompts, validates, and returns the typed object.

## Requirements

All types must have:
- `#[derive(Elicit)]` from `elicitation`
- `#[derive(JsonSchema)]` from `schemars` (for MCP schema generation)
- `#[derive(Serialize, Deserialize)]` from `serde`

## Collecting Types

To find all types that should be included:

```bash
# In botticelli repo
rg "derive.*Elicit" --type rust crates/botticelli_mcp/src/ | \
  grep -o "struct [A-Z][^ ]*" | \
  awk '{print $2","}' | \
  sort -u
```

Add all discovered types to the `elicit_tools!` macro invocation.
