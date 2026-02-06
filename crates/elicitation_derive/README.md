# elicitation_derive

Derive macros for the `elicitation` crate.

## Usage

```rust
use elicitation::Elicit;
use schemars::JsonSchema;

#[derive(Debug, Clone, Elicit, JsonSchema)]
struct Config {
    host: String,
    port: u16,
}
```

**Important:** You must include `#[derive(schemars::JsonSchema)]` along with `#[derive(Elicit)]`. This is required because the generated `elicit_checked()` function returns `Self`, which must implement `JsonSchema` for rmcp's `#[tool]` attribute to work properly.

## What Gets Generated

The `#[derive(Elicit)]` macro generates:

1. **`Elicitation` trait implementation** - Async elicitation logic
2. **`elicit_checked()` function** - MCP tool decorated with `#[rmcp::tool]`
3. **Supporting traits** - `Prompt`, `Select` (for enums), `Survey` (for structs)

## Why JsonSchema is Required

The generated tool function looks like:

```rust
#[rmcp::tool]
pub async fn elicit_checked(peer: Peer<RoleServer>) -> Result<Self, ElicitError> {
    // ...
}
```

The `#[rmcp::tool]` macro requires the return type (`Self`) to implement `JsonSchema` for OpenAPI/JSON Schema generation.

## Example with Tool Router

```rust
use elicitation::{Elicit, elicit_router};
use schemars::JsonSchema;

#[derive(Debug, Clone, Elicit, JsonSchema)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Clone, Elicit, JsonSchema)]
struct UserProfile {
    username: String,
    email: String,
}

// Generate router with all tools
elicit_router! {
    pub MyTools: ServerConfig, UserProfile
}
```

This generates MCP tools:
- `elicit_checked_ServerConfig(peer)` 
- `elicit_checked_UserProfile(peer)`

All automatically registered and ready to use!
