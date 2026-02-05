# Add `#[rmcp::tool]` to Generated Elicit Functions

## Problem

The `#[derive(Elicit)]` macro generates `elicit_*()` functions but does **not** add the `#[rmcp::tool]` attribute. This means:

1. Generated functions exist but aren't registered as MCP tools
2. `#[tool_router]` macro can't find them (it only discovers functions with `#[tool]` or `#[rmcp::tool]`)
3. Users must manually wrap each generated function or can't use them in tool routers

## Current Behavior

```rust
// User writes:
#[derive(Elicit)]
struct Config {
    timeout: u32,
}

// Macro generates:
pub async fn elicit_config(
    client: &elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleClient>,
) -> Result<Config, elicitation::ElicitError> {
    Config::elicit(&ElicitClient::new(client)).await
}
```

**Missing**: `#[rmcp::tool]` attribute on generated function.

## Desired Behavior

```rust
// User writes:
#[derive(Elicit)]
struct Config {
    timeout: u32,
}

// Macro should generate:
#[rmcp::tool]
pub async fn elicit_config(
    client: &elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleClient>,
) -> Result<Config, elicitation::ElicitError> {
    Config::elicit(&ElicitClient::new(client)).await
}
```

## Proposed Fix

### File: `crates/elicitation_derive/src/tool_gen.rs`

**Current code** (lines 23-39):

```rust
quote! {
    /// Auto-generated MCP tool function for eliciting [`#type_name`].
    ///
    /// This function uses the derived `Elicitation` impl to
    /// interactively elicit a value from the user via MCP.
    ///
    /// # Usage
    ///
    /// Call this directly from an MCP server handler, or add `#[rmcp::tool]`
    /// attribute for automatic registration with tool routers.
    pub async fn #fn_name(
        client: &elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleClient>,
    ) -> Result<#type_name, elicitation::ElicitError> {
        use elicitation::{Elicitation, ElicitClient};
        #type_name::elicit(&ElicitClient::new(client)).await
    }
}
```

**Patched code**:

```rust
quote! {
    /// Auto-generated MCP tool function for eliciting [`#type_name`].
    ///
    /// This function uses the derived `Elicitation` impl to
    /// interactively elicit a value from the user via MCP.
    ///
    /// Automatically registered as an MCP tool via `#[rmcp::tool]`.
    #[elicitation::rmcp::tool]
    pub async fn #fn_name(
        client: &elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleClient>,
    ) -> Result<#type_name, elicitation::ElicitError> {
        use elicitation::{Elicitation, ElicitClient};
        #type_name::elicit(&ElicitClient::new(client)).await
    }
}
```

**Changes**:
1. Add `#[elicitation::rmcp::tool]` before function declaration (line 30)
2. Update doc comment to reflect automatic registration (line 31)

## Why `elicitation::rmcp::tool`?

The full path `elicitation::rmcp::tool` is needed because:
1. The generated code lives in the user's crate
2. User may not have `use rmcp::tool` in scope
3. Full path ensures it always resolves correctly
4. `elicitation` already re-exports `rmcp` for user convenience

## Impact

### Before (Current)
```rust
// botticelli_mcp usage
#[derive(Elicit)]
struct StoreParams { path: String }

// Generated function exists but not a tool
// Can't use #[tool_router] to discover it
// Must manually register:
impl BotticelliServer {
    #[tool]
    pub async fn wrapper_store(&self, ...) -> ... {
        elicit_store_params(...).await  // Call generated function
    }
}
```

### After (Patched)
```rust
// botticelli_mcp usage
#[derive(Elicit)]
struct StoreParams { path: String }

// Generated function IS a tool
// #[tool_router] automatically discovers it
// Can collect all elicit_* tools:
impl BotticelliServer {
    // Manually import generated tool functions
    pub fn elicit_tool_router() -> ToolRouter<Self> {
        use crate::tools::storage::elicit_store_params;
        
        ToolRouter::new()
            .with_route((Self::elicit_store_params_tool_attr(), Self::elicit_store_params))
    }
}
```

**Even better** - with a hypothetical `#[elicit_tool_router]` macro (future work):
```rust
#[elicit_tool_router(module = "crate::tools::storage")]
impl BotticelliServer {
    // Automatically discovers all elicit_* functions in module
}
```

## Testing

After applying patch:

1. **Verify compilation**:
   ```bash
   cd elicitation
   cargo check --all-features
   ```

2. **Test tool registration**:
   ```rust
   #[derive(Elicit)]
   struct TestType { value: i32 }
   
   // Should compile and register as MCP tool
   let router = ToolRouter::new()
       .with_route((elicit_test_type_tool_attr(), elicit_test_type));
   ```

3. **Integration test** in botticelli_mcp:
   ```bash
   cd ../botticelli
   cargo test -p botticelli_mcp --test test_elicit_tools
   ```

## Alternative Approaches Considered

### 1. Feature Flag
Add `#[cfg_attr(feature = "mcp-tools", rmcp::tool)]` to make it optional.

**Rejected**: Adds complexity. MCP integration is core to elicitation's purpose.

### 2. Separate Derive
Create `#[derive(ElicitTool)]` that adds the attribute.

**Rejected**: Redundant. All `elicit_*` functions should be tools.

### 3. Manual Registration Helper
Provide a macro that collects all `elicit_*` functions.

**Future work**: This would complement the tool attribute, not replace it.

## Migration

**Breaking change?** No.

- Functions that were plain async remain async
- Adding `#[rmcp::tool]` doesn't change function signature
- Existing direct calls to `elicit_*` functions still work
- Only benefit: automatic tool registration

**Version bump**: Patch (0.4.8 → 0.4.9) since it's additive.

## Implementation Checklist

- [ ] Modify `crates/elicitation_derive/src/tool_gen.rs` line 30
- [ ] Update doc comment at line 31
- [ ] Run `cargo check` in elicitation workspace
- [ ] Run `cargo test` in elicitation workspace
- [ ] Test in botticelli_mcp integration
- [ ] Update CHANGELOG.md
- [ ] Bump version to 0.4.9
- [ ] Commit: `feat(derive): Add #[rmcp::tool] to generated elicit functions`

## Related Issues

- botticelli: Tool router pattern needs elicitation-derived tools
- Current workaround: Manual wrappers for every derived type
- With patch: Automatic tool registration, no wrappers needed
