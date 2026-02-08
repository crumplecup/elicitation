# Tool Router Warnings Guide

This guide addresses common warnings when using elicitation with rmcp's `#[tool_router]` macro in projects with strict linting enabled (e.g., `#![warn(missing_docs)]`).

## Warning Categories

### 1. Missing Documentation on Generated Router Function

**Warning:**
```
warning: missing documentation for an associated function
  --> src/my_module.rs:21:1
   |
21 | #[tool_router(router = my_tool_router, vis = "pub")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

**Cause:** rmcp's `#[tool_router]` macro generates a public `tool_router()` function without documentation.

**Solution:** Add a doc comment before the `#[tool_router]` attribute:

```rust
/// Tool router for core operations.
/// 
/// Provides elicitation and management tools for the core module.
#[tool_router(router = core_tool_router, vis = "pub")]
impl MyServer {
    // ... tool methods ...
}
```

**Note:** The doc comment applies to the impl block, not the generated function, but it satisfies the missing docs warning.

### 2. Unused Parameters in Tool Methods

**Warning:**
```
warning: unused variable: `params`
   --> src/tools/cache.rs:221:20
    |
221 |         Parameters(params): Parameters<CacheCleanupParams>,
    |                    ^^^^^^ help: prefix with underscore: `_params`
```

**Cause:** Tool methods that don't use their parameter destructuring variable.

**Solution:** Prefix unused variables with underscore:

```rust
#[tool(description = "Clear cache")]
pub async fn cache_clear(
    peer: Peer<RoleServer>,
    Parameters(_params): Parameters<CacheClearParams>,  // ✅ Underscore prefix
) -> Result<Json<ClearResponse>, ErrorData> {
    // Implementation doesn't need params
    Ok(Json(ClearResponse { success: true }))
}
```

### 3. Unused `tool_router` Import

**Warning:**
```
warning: unused import: `rmcp::tool_router`
  --> src/tools/social.rs:10:5
   |
10 | use rmcp::tool_router;
   |     ^^^^^^^^^^^^^^^^^
```

**Cause:** The `#[tool_router]` attribute is used without importing the name.

**Solutions:**

**Option A:** Remove the unused import (recommended):
```rust
// No import needed - use fully qualified path in attribute
#[rmcp::tool_router]
impl MyServer { }
```

**Option B:** Use the import in the attribute:
```rust
use rmcp::tool_router;

#[tool_router]  // Uses imported name
impl MyServer { }
```

**Note:** Most projects use fully qualified paths in attributes and don't import the macro name.

## Quick Fixes

### Batch Fix with cargo fix

For unused variables and imports:
```bash
cargo fix --allow-dirty
```

### Module-Level Allow Directives

For generated code warnings you can't easily fix:

```rust
// At module level
#![allow(missing_docs)]  // Allow missing docs for generated code

#[tool_router(router = my_router)]
impl MyServer { }
```

**Caution:** This suppresses warnings for the entire module, not just generated code.

## Elicitation-Specific Notes

### Our `#[elicit_tools]` Macro

The `#[elicit_tools]` macro **already generates documentation** for tool methods:

```rust
#[elicit_tools(Config, User, Settings)]  // ✅ Methods have docs
#[tool_router]
impl MyServer {
    // Generated with doc comment:
    // /// Elicit `Config` via MCP.
    // pub async fn elicit_config(...)
}
```

So you only need to document:
1. The impl block (for the router function warning)
2. Your own `#[tool]` methods (elicit_tools handles its own)

### Recommended Pattern

```rust
/// Tool router providing configuration and user management tools.
///
/// This server offers:
/// - Elicitation tools for Config, User, Settings
/// - Custom validation and processing tools
#[elicit_tools(Config, User, Settings)]
#[tool_router(router = management_router, vis = "pub")]
impl ManagementServer {
    /// Validate configuration settings.
    #[tool(description = "Validate config")]
    pub async fn validate_config(
        peer: Peer<RoleServer>,
        Parameters(params): Parameters<ValidateParams>,
    ) -> Result<Json<ValidationResult>, ErrorData> {
        // ... implementation ...
    }
}
```

## Summary

| Warning Type | Quick Fix | Best Practice |
|--------------|-----------|---------------|
| Missing docs on router | Add doc comment to impl block | Document what tools the server provides |
| Unused `params` variable | Prefix with `_params` | Run `cargo fix` |
| Unused `tool_router` import | Remove import | Use `#[rmcp::tool_router]` |

## Related Issues

- **rmcp issue tracker**: Consider reporting missing docs on generated functions
- **Elicitation examples**: See `examples/` for documented patterns
- **Botticelli integration**: See `BOTTICELLI_INTEGRATION.md` for production patterns
