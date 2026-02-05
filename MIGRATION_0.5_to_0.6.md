# Migration Guide: 0.5.0 → 0.6.0

## Overview

Version 0.6.0 introduces a **breaking change** in how MCP tools are generated and accessed. This change makes the API more idiomatic, enables automatic tool discovery, and follows Rust's `checked_*` pattern for verified operations.

## Breaking Changes

### Tool Access Pattern

**Before (0.5.0):** Standalone functions
```rust
use elicitation::Elicit;

#[derive(Elicit)]
struct Config { timeout: u32 }

// Generated: standalone function
let config = elicit_config(client).await?;
```

**After (0.6.0):** Methods on types
```rust
use elicitation::Elicit;
use std::sync::Arc;

#[derive(Elicit)]
struct Config { timeout: u32 }

// Generated: method on Config
let config = Config::elicit_checked(client).await?;
```

### Key Differences

| Aspect | 0.5.0 | 0.6.0 |
|--------|-------|-------|
| **Function/Method** | `elicit_config(client)` | `Config::elicit_checked(client)` |
| **Naming** | `elicit_` + snake_case | `elicit_checked` (constant) |
| **Location** | Module-level function | Method on type |
| **Discovery** | Manual import | Automatic via inventory |

## Migration Steps

### Step 1: Update Tool Calls

Find all calls to generated `elicit_*` functions and convert them to methods:

```rust
// Before
let config = elicit_config(client).await?;
let user = elicit_user(client).await?;
let settings = elicit_settings(client).await?;

// After
let config = Config::elicit_checked(client).await?;
let user = User::elicit_checked(client).await?;
let settings = Settings::elicit_checked(client).await?;
```

### Step 2: Remove Manual Tool Registrations

If you were manually registering tools with `rmcp::tool_router`:

```rust
// Before: Manual registration
use rmcp::handler::server::tool::ToolRouter;

let router = ToolRouter::new()
    .with_route((elicit_config_tool_attr(), elicit_config))
    .with_route((elicit_user_tool_attr(), elicit_user))
    .with_route((elicit_settings_tool_attr(), elicit_settings));
```

**After: Automatic discovery**
```rust
use elicitation::collect_all_elicit_tools;

// Collect all elicit tools automatically!
let tools = collect_all_elicit_tools();
println!("Found {} elicit tools", tools.len());

// Tools are auto-registered via #[rmcp::tool] attribute
// No manual registration needed!
```

### Step 3: Update Examples/Tests

Search your codebase for the pattern:

```bash
# Find old-style calls
rg "elicit_\w+\(.*client" --type rust

# Find old-style imports (if any)
rg "use.*::elicit_" --type rust
```

Convert each occurrence to the new method style.

## Why This Change?

### 1. **Idiomatic Rust**
Methods on types are more idiomatic than standalone functions. Compare:
- `Config::elicit_checked(client)` ✅ (clear ownership)
- `elicit_config(client)` ❌ (disconnected from type)

### 2. **Follows `checked_*` Pattern**
Rust's standard library uses `checked_*` for verified operations:
- `i32::checked_add()` - addition with overflow check
- `Config::elicit_checked()` - elicitation with MCP verification

### 3. **Automatic Discovery**
With inventory-based registration, you get:
- Zero manual tool registration
- Automatic discovery of 100+ tools
- No maintenance burden

### 4. **Better IDE Support**
- Type `Config::` and see `elicit_checked()` in autocomplete
- Jump-to-definition takes you to the type
- More discoverable API

## Verification

After migration, verify your changes compile:

```bash
# Check everything compiles
cargo check --workspace

# Run tests
cargo test --workspace

# Run all checks
just check-all
```

## Need Help?

If you encounter issues during migration:

1. **Check the examples:** See `crates/elicitation/examples/` for updated patterns
2. **Read the docs:** `cargo doc --open -p elicitation`
3. **Open an issue:** https://github.com/crumplecup/elicitation/issues

## New Features in 0.6.0

Beyond the breaking change, 0.6.0 adds:

### Automatic Tool Discovery
```rust
use elicitation::collect_all_elicit_tools;

let tools = collect_all_elicit_tools();
for tool in tools {
    println!("Found: {}", tool.qualified_name());
}
```

### Tool Metadata
```rust
let tools = collect_all_elicit_tools();
for tool in tools {
    println!("Type: {}", tool.type_name);
    println!("Module: {}", tool.module_path);
    println!("Qualified: {}", tool.qualified_name());
}
```

## Timeline

- **0.5.0:** Released with `Arc<Peer>` refactor
- **0.6.0:** Method generation + inventory (current)
- **0.7.0:** Planned enhancements to tool router integration

## Summary

This migration is mechanical and straightforward:
1. Replace `elicit_typename(client)` with `TypeName::elicit_checked(client)`
2. Remove manual tool registrations (now automatic)
3. Verify with `cargo check && cargo test`

The new API is more idiomatic, more discoverable, and enables automatic tool collection for large-scale applications like botticelli.
