# Macro Ordering Issue & Solution

## Problem

The `elicit_tools!` declarative macro doesn't work because of Rust's macro expansion order:

1. **Attribute macros expand first** (`#[tool_router]`)
2. **Declarative macros expand later** (`elicit_tools!`)
3. Result: `#[tool_router]` processes the impl block before `elicit_tools!` generates methods
4. Generated methods never get registered

## Why Proc Macro Won't Work

We tried creating `#[elicit_tools(...)]` as a proc macro, but:
- Proc macros can't apply other proc macros (`#[tool]`)
- We'd need to generate tool registration code ourselves
- This duplicates rmcp's `#[tool_router]` logic

## The Real Solution: Separate Routers

Use `elicit_router!` to create a dedicated elicitation router:

```rust
// Create elicitation router (separate struct)
elicitation::elicit_router! {
    pub ElicitTools:
        CacheKeyNewParams,
        StorageNewParams,
        // ... all elicitable types
}

// Your server's tools
#[tool_router]
impl BotticelliServer {
    // Your custom tools...
}
```

## Combining Routers

The challenge: `ToolRouter<ElicitTools>` + `ToolRouter<BotticelliServer>` won't compile (different type parameters).

### Option 1: Manual Registration (Recommended)

Register both routers with the MCP server separately:

```rust
async fn setup_server() {
    let botticelli_router = BotticelliServer::tool_router();
    let elicit_router = ElicitTools::tool_router();
    
    // Register both with MCP server
    // (depends on how rmcp server setup works)
}
```

### Option 2: Wrapper Router

Create a wrapper that delegates to both:

```rust
struct CombinedRouter {
    botticelli: ToolRouter<BotticelliServer>,
    elicit: ToolRouter<ElicitTools>,
}

impl CombinedRouter {
    pub fn handle_tool(&self, name: &str, ...) -> ... {
        if name.starts_with("elicit_") {
            self.elicit.handle(name, ...)
        } else {
            self.botticelli.handle(name, ...)
        }
    }
}
```

### Option 3: Flatten at Server Level

If rmcp server accepts multiple routers, register them separately:

```rust
server
    .with_tools(BotticelliServer::tool_router())
    .with_tools(ElicitTools::tool_router())
    .start()
    .await?;
```

## Recommendation for Botticelli

1. Remove `elicit_tools!` invocation from impl block
2. Create `ElicitTools` router using `elicit_router!` at module level
3. Check rmcp server API for how to register multiple routers
4. If not supported, create issue/PR for rmcp to support multiple routers

## Long-Term Solution

Upstream change to rmcp: Allow combining routers with different type parameters, or support registering multiple routers with the server.

Alternatively, create a trait-based approach where both servers implement a common trait, allowing `ToolRouter<dyn CommonTrait>`.
