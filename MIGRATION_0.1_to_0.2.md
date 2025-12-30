# Migration Guide: v0.1.0 to v0.2.0

This guide helps you migrate from elicitation v0.1.0 (using pmcp) to v0.2.0 (using rmcp, the official Rust MCP SDK).

## Overview

Version 0.2.0 introduces **breaking changes** due to the migration from the unofficial `pmcp` SDK to the official `rmcp` (Rust MCP) SDK. This migration provides better long-term support, improved API design, and official backing from the MCP ecosystem.

**Key Changes:**
- Client type: `pmcp::Client<T>` → `rmcp::service::Peer<RoleClient>`
- Transport setup: Simplified with `ServiceExt::serve()`
- Dependencies: `pmcp = "1.4"` → `rmcp = "0.12"`
- No behavioral changes to elicitation patterns

## Quick Migration Checklist

- [ ] Update `Cargo.toml` dependencies
- [ ] Update client creation code
- [ ] Update imports
- [ ] Test all elicitation code
- [ ] Update documentation/examples

## Step-by-Step Migration

### 1. Update Dependencies

**Before (v0.1.0):**
```toml
[dependencies]
elicitation = "0.1"
pmcp = "1.4"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

**After (v0.2.0):**
```toml
[dependencies]
elicitation = "0.2"
rmcp = "0.12"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Run `cargo update` after updating `Cargo.toml`.

### 2. Update Imports

**Before (v0.1.0):**
```rust
use elicitation::{Elicit, Elicitation, ElicitResult};
use pmcp::{Client, transport::StdioTransport};
```

**After (v0.2.0):**
```rust
use elicitation::{Elicit, Elicitation, ElicitResult};
use rmcp::ServiceExt;
```

### 3. Update Client Creation

**Before (v0.1.0):**
```rust
#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Create transport
    let transport = StdioTransport::new();

    // Create client
    let client = Client::new(transport);

    // Use client
    let value = String::elicit(&client).await?;
    Ok(())
}
```

**After (v0.2.0):**
```rust
#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Create client via ServiceExt::serve()
    let client = ()
        .serve(rmcp::transport::stdio())
        .await?;

    // Use client (same API!)
    let value = String::elicit(&client).await?;
    Ok(())
}
```

### 4. Update Elicitation Calls (No Changes Required!)

The elicitation API remains identical - only the client type changed:

```rust
// These work the same in both versions:
let age: i32 = i32::elicit(&client).await?;
let name: String = String::elicit(&client).await?;
let confirmed: bool = bool::elicit(&client).await?;
let items: Vec<String> = Vec::<String>::elicit(&client).await?;

// Derive macros work identically:
#[derive(Debug, Elicit)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Elicit)]
struct Task {
    #[prompt("What's the task title?")]
    title: String,
    priority: Priority,
}

let task = Task::elicit(&client).await?;
```

## Common Migration Issues

### Issue 1: Generic Type Parameters

**Error:**
```
error[E0107]: struct takes 1 generic argument but 0 were supplied
```

**Cause:** Trying to use old pmcp Client type

**Fix:** The new client type `Peer<RoleClient>` doesn't use generic parameters:

```rust
// ❌ Old - generic parameter
fn process<T: Transport>(client: &Client<T>) { }

// ✅ New - concrete type
fn process(client: &Peer<RoleClient>) { }
```

### Issue 2: Transport Creation

**Error:**
```
error[E0433]: failed to resolve: use of undeclared type 'StdioTransport'
```

**Cause:** pmcp's `StdioTransport` doesn't exist in rmcp

**Fix:** Use rmcp's stdio transport function:

```rust
// ❌ Old
let transport = StdioTransport::new();
let client = Client::new(transport);

// ✅ New
let client = ()
    .serve(rmcp::transport::stdio())
    .await?;
```

### Issue 3: Missing ServiceExt Import

**Error:**
```
error[E0599]: no method named 'serve' found for type '()'
```

**Cause:** Missing `ServiceExt` trait import

**Fix:** Import the trait:

```rust
use rmcp::ServiceExt;  // Required for .serve() method
```

### Issue 4: Client Lifetime in Structs

**Before (v0.1.0):**
```rust
struct App<T: Transport> {
    client: Client<T>,
}
```

**After (v0.2.0):**
```rust
use rmcp::service::{Peer, RoleClient};

struct App {
    client: Peer<RoleClient>,
}
```

## Testing Your Migration

After migrating, verify everything works:

### 1. Basic Compilation
```bash
cargo check
```

### 2. Run Examples
```bash
# Run through Claude CLI
claude mcp add elicitation-test --command "cargo run --example simple_types"
claude "Run the simple_types example"
```

### 3. Test Your Code
```bash
cargo test
```

## MCP Setup (No Changes Required)

The MCP client setup (Claude Desktop/CLI) remains identical between versions. If you had a working MCP configuration with v0.1.0, it will work with v0.2.0 without changes.

**Claude Desktop Configuration:**

```json
{
  "mcpServers": {
    "my-elicitation-app": {
      "command": "/path/to/your/binary",
      "args": [],
      "env": {}
    }
  }
}
```

This configuration works for both v0.1.0 and v0.2.0.

## Benefits of the Migration

### 1. Official SDK Support
- rmcp is the official Rust MCP SDK
- Long-term maintenance and updates
- Better integration with MCP ecosystem

### 2. Improved API Design
- Simpler client creation with `ServiceExt::serve()`
- Better type safety with concrete `Peer<RoleClient>` type
- More consistent with MCP protocol specifications

### 3. Better Documentation
- Official docs at [docs.rs/rmcp](https://docs.rs/rmcp)
- Active development and community support

## Rollback (If Needed)

If you need to temporarily revert to v0.1.0:

```toml
[dependencies]
elicitation = "0.1"
pmcp = "1.4"
```

Then revert your code changes using git:

```bash
git diff > migration_changes.patch  # Save your changes
git checkout -- .                    # Revert to previous version
```

## Getting Help

If you encounter issues during migration:

1. **Check the Examples**: See [crates/elicitation/examples/](crates/elicitation/examples/) for updated working examples
2. **Review CHANGELOG**: See [CHANGELOG.md](CHANGELOG.md) for complete list of changes
3. **Open an Issue**: Report problems at [GitHub Issues](https://github.com/crumplecup/elicitation/issues)
4. **Documentation**: Visit [docs.rs/elicitation](https://docs.rs/elicitation)

## Summary

The migration from v0.1.0 to v0.2.0 primarily affects:
- **Dependency declarations** in `Cargo.toml`
- **Client creation** code in your `main()` or setup functions
- **Import statements** for MCP SDK types

The **elicitation API itself is unchanged** - all your existing elicitation code, derive macros, and patterns work identically after updating the client.

Total migration time: **~5-15 minutes** for most projects.
