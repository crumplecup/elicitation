# Elicitation RMCP Migration Plan - Phase 2

**Date:** 2025-12-29
**Status:** Implementation Plan
**Related:** ELICITATION_RMCP_IMPLEMENTATION_VISION.md
**Version:** 0.1.0 → 0.2.0

## Executive Summary

This document provides a detailed implementation plan for migrating the `elicitation` crate from `pmcp` to `rmcp` 0.12.0 as outlined in Phase 2 of the vision document.

**Approach**: Big-bang migration on dedicated branch (no dual support)
**Breaking Change**: Yes - v0.2.0 release
**Development Stage**: Early development allows aggressive refactoring

## API Verification Results

### RMCP 0.12.0 API Analysis

Examined `/tmp/rmcp-verification` (tag: `rmcp-v0.12.0`)

#### Key Findings

1. **No `Client<T>` type exists**
   - Vision document assumption was incorrect
   - Actual pattern: `RunningService<RoleClient, S>` which derefs to `Peer<RoleClient>`
   - `Peer<RoleClient>` provides client methods (call_tool, list_tools, etc.)

2. **Transport trait is different**
   - **Vision**: `Transport` with generic type parameter `<T: Transport>`
   - **Reality**: `Transport<R: ServiceRole>` parameterized by role, not transport type

3. **Client creation pattern**
   ```rust
   // Actual rmcp 0.12.0 API:
   let client = ()
       .serve(TokioChildProcess::new(Command::new("server")))
       .await?;

   // `client` is RunningService<RoleClient, ()>
   // Derefs to Peer<RoleClient>
   // Methods: client.call_tool(), client.list_tools(), etc.
   ```

4. **Built-in elicitation feature**
   - RMCP has its own `elicitation` feature for MCP 2025-06-18 spec
   - Different purpose: Server→Client elicitation requests
   - Our elicitation crate: Client-side type-safe value elicitation
   - **No conflict**: Different layers of abstraction

#### Correct API Patterns

**What the vision document said:**
```rust
// INCORRECT (from vision):
impl Elicitation for bool {
    async fn elicit<T: rmcp::transport::Transport>(
        client: &rmcp::Client<T>,  // ❌ rmcp::Client doesn't exist
    ) -> ElicitResult<Self> {
        // ...
    }
}
```

**What we actually need:**
```rust
// CORRECT (verified from rmcp 0.12.0):
use rmcp::service::{Peer, RoleClient};

impl Elicitation for bool {
    async fn elicit(
        client: &Peer<RoleClient>,  // ✅ Actual rmcp type
    ) -> ElicitResult<Self> {
        // ...
    }
}
```

### Compatibility Assessment

| Vision Assumption | Reality | Migration Impact |
|-------------------|---------|------------------|
| `rmcp::Client<T>` | `Peer<RoleClient>` | Update all trait signatures |
| `Transport` trait generic | `Transport<R: ServiceRole>` | No direct impact (internal) |
| Simple type param | Service role param | Simplifies our API |
| Tool calls via `client.call_tool()` | Same ✅ | No changes needed |
| Async methods | Same ✅ | No changes needed |

**Good news**: The core interaction pattern (calling MCP tools) is compatible. We just need to update type signatures.

## Migration Strategy

### Approach: Big Bang (Option A)

**Rationale**:
- Early development stage
- Dedicated branch isolates risk
- No backward compatibility burden
- Faster iteration

**Risk Mitigation**:
- Comprehensive testing after migration
- Document all breaking changes
- Clear upgrade path in CHANGELOG

### Breaking Changes

1. **Trait signature changes**
   - `Elicitation::elicit<T: pmcp::shared::transport::Transport>` → `Elicitation::elicit`
   - Client parameter: `&pmcp::Client<T>` → `&Peer<RoleClient>`

2. **Error types**
   - `PmcpError` → `RmcpError`
   - `ElicitErrorKind::Mcp(PmcpError)` → `ElicitErrorKind::Rmcp(RmcpError)`

3. **Re-exports**
   - `pub use pmcp;` → `pub use rmcp;`

4. **Dependency changes**
   - Remove `pmcp = "1.4"`
   - Add `rmcp = "0.12"`

## Detailed Implementation Steps

### Step 1: Create Branch and Update Dependencies

**Files**:
- `Cargo.toml` (workspace)
- `crates/elicitation/Cargo.toml`
- `crates/elicitation_derive/Cargo.toml`

**Changes**:

```toml
# Cargo.toml (workspace)
[workspace.dependencies]
# Remove pmcp
# pmcp = "1.4"

# Add rmcp
rmcp = { version = "0.12", features = ["client"] }

# Keep existing
derive_more = { version = "1", features = ["display", "error", "from"] }
derive-getters = "0.5"
tracing = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```toml
# crates/elicitation/Cargo.toml
[dependencies]
rmcp = { workspace = true }  # Changed from pmcp
derive_more = { workspace = true }
derive-getters = { workspace = true }
tracing = { workspace = true }
tokio = { workspace = true }
serde_json = { workspace = true }
elicitation_derive = { path = "../elicitation_derive", version = "0.1.0" }
```

**Commands**:
```bash
cd /home/erik/repos/elicitation
git checkout -b rmcp-migration
# Edit Cargo.toml files
cargo update
cargo check  # Will fail, expected
```

### Step 2: Update Error Types

**File**: `crates/elicitation/src/error.rs`

**Changes**:

1. **Replace PmcpError with RmcpError**:
```rust
/// RMCP error wrapper.
#[derive(Debug, Clone, Display, derive_getters::Getters)]
#[display("RMCP error: {}", source)]
pub struct RmcpError {
    /// The underlying rmcp error.
    source: String,
    /// Line number where the error occurred.
    line: u32,
    /// File where the error occurred.
    file: &'static str,
}

impl std::error::Error for RmcpError {}

impl RmcpError {
    /// Creates a new RMCP error with caller location.
    #[track_caller]
    pub fn new(source: rmcp::error::ErrorData) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source: source.to_string(),
            line: loc.line(),
            file: loc.file(),
        }
    }
}

impl From<rmcp::error::ErrorData> for RmcpError {
    #[track_caller]
    fn from(source: rmcp::error::ErrorData) -> Self {
        Self::new(source)
    }
}
```

2. **Update ElicitErrorKind**:
```rust
#[derive(Debug, Clone, Display, From)]
pub enum ElicitErrorKind {
    /// RMCP error.
    #[display("{}", _0)]
    #[from]
    Rmcp(RmcpError),  // Changed from Mcp(PmcpError)

    // ... rest unchanged
}
```

3. **Update bridge_error! and error_from! macros**:
```rust
// Bridge From implementations
bridge_error!(rmcp::error::ErrorData => RmcpError);
bridge_error!(rmcp::service::ServiceError => RmcpServiceError);  // May need this
bridge_error!(serde_json::Error => JsonError);

// Complete conversion chains
error_from!(rmcp::error::ErrorData);
error_from!(rmcp::service::ServiceError);
error_from!(serde_json::Error);
```

4. **Remove PmcpError completely**

### Step 3: Update Core Traits

**File**: `crates/elicitation/src/traits.rs`

**Changes**:

```rust
use crate::ElicitResult;
use rmcp::service::{Peer, RoleClient};

/// Main elicitation trait - entry point for value elicitation.
pub trait Elicitation: Sized + Prompt {
    /// Elicit a value of this type from the user via RMCP client.
    ///
    /// # Arguments
    ///
    /// * `client` - The RMCP peer (client interface) to use for interaction
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` if elicitation succeeds, or `Err(ElicitError)` if:
    /// - The user provides invalid input
    /// - The MCP tool call fails
    /// - The user cancels the operation
    fn elicit(
        client: &Peer<RoleClient>,  // Changed from &pmcp::Client<T>
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send;
}
```

**Note**: Removed generic type parameter - much simpler!

### Step 4: Update Primitive Implementations

**Files**: All files in `crates/elicitation/src/primitives/*.rs`

**Pattern** (using `boolean.rs` as example):

```rust
// BEFORE:
impl Elicitation for bool {
    #[tracing::instrument(skip(client))]
    async fn elicit<T: pmcp::shared::transport::Transport>(
        client: &pmcp::Client<T>,
    ) -> ElicitResult<Self> {
        let prompt = Self::prompt().unwrap();
        tracing::debug!("Eliciting boolean");

        let params = mcp::bool_params(prompt);
        let result = client
            .call_tool(mcp::tool_names::elicit_bool(), params)
            .await?;

        let value = mcp::extract_value(result)?;
        mcp::parse_bool(value)
    }
}

// AFTER:
use rmcp::service::{Peer, RoleClient};

impl Elicitation for bool {
    #[tracing::instrument(skip(client))]
    async fn elicit(
        client: &Peer<RoleClient>,  // Changed type
    ) -> ElicitResult<Self> {
        let prompt = Self::prompt().unwrap();
        tracing::debug!("Eliciting boolean");

        let params = mcp::bool_params(prompt);

        // rmcp API: call_tool returns CallToolResult directly
        let result = client
            .call_tool(rmcp::model::CallToolRequestParam {
                name: mcp::tool_names::elicit_bool().into(),
                arguments: Some(params),
            })
            .await?;

        let value = mcp::extract_value(result)?;
        mcp::parse_bool(value)
    }
}
```

**Files to update**:
- `primitives/boolean.rs`
- `primitives/duration.rs`
- `primitives/floats.rs`
- `primitives/integers.rs`
- `primitives/pathbuf.rs`
- `primitives/string.rs`
- `primitives/tuples.rs`
- `primitives/network.rs`

**Key changes**:
1. Remove generic type parameter
2. Change client type to `&Peer<RoleClient>`
3. Update `call_tool` API to use `CallToolRequestParam`

### Step 5: Update Container Implementations

**Files**: All files in `crates/elicitation/src/containers/*.rs`

**Pattern** (using `option.rs` as example):

```rust
// BEFORE:
impl<T: Elicitation> Elicitation for Option<T> {
    async fn elicit<U: pmcp::shared::transport::Transport>(
        client: &pmcp::Client<U>,
    ) -> ElicitResult<Self> {
        // ... implementation
    }
}

// AFTER:
use rmcp::service::{Peer, RoleClient};

impl<T: Elicitation> Elicitation for Option<T> {
    async fn elicit(
        client: &Peer<RoleClient>,
    ) -> ElicitResult<Self> {
        // ... implementation (logic unchanged, just type)
    }
}
```

**Files to update**:
- `containers/option.rs`
- `containers/vec.rs`
- `containers/result.rs`
- `containers/array.rs`
- `containers/smart_pointers.rs`

**Key changes**:
1. Remove generic transport parameter
2. Update client type
3. Logic remains identical (transport-agnostic)

### Step 6: Update MCP Tool Builders

**File**: `crates/elicitation/src/mcp/tools.rs`

**Current**: JSON builders for tool parameters
**Keep**: Same pattern, but note rmcp expects `CallToolRequestParam`

**Changes**:
```rust
// Update documentation to reflect rmcp usage
/// Build parameters for elicit_number tool.
///
/// Returns a JSON object compatible with rmcp's CallToolRequestParam.
pub fn number_params(prompt: &str, min: i64, max: i64) -> serde_json::Value {
    json!({
        "prompt": prompt,
        "min": min,
        "max": max,
    })
}

// Same for all other param builders
```

**No functional changes needed** - JSON structure is the same.

### Step 7: Update Derive Macro

**Files**: `crates/elicitation_derive/src/*.rs`

**Changes**:

1. **Update imports**:
```rust
// In generated code, change:
// pmcp::Client<T> → rmcp::service::Peer<rmcp::service::RoleClient>
```

2. **Update generated code** (`enum_impl.rs` example):
```rust
// BEFORE (generated):
impl #impl_generics elicitation::Elicitation for #name #ty_generics #where_clause {
    async fn elicit<__ElicitTransport: pmcp::shared::transport::Transport>(
        client: &pmcp::Client<__ElicitTransport>,
    ) -> elicitation::ElicitResult<Self> {
        // ...
    }
}

// AFTER (generated):
impl #impl_generics elicitation::Elicitation for #name #ty_generics #where_clause {
    async fn elicit(
        client: &elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleClient>,
    ) -> elicitation::ElicitResult<Self> {
        // ...
    }
}
```

3. **Files to update**:
   - `derive_elicit.rs` - Main derive logic
   - `enum_impl.rs` - Enum implementation generator
   - `struct_impl.rs` - Struct implementation generator (future)

### Step 8: Update lib.rs Re-exports

**File**: `crates/elicitation/src/lib.rs`

**Changes**:
```rust
// BEFORE:
pub use pmcp;

// AFTER:
pub use rmcp;
```

**Documentation updates**:
```rust
//! # MCP Setup Required
//!
//! This library runs as an **MCP server** and requires an **MCP client**
//! (like Claude Desktop or Claude CLI) to provide the elicitation tools.
//! Your application won't work standalone - it must be invoked by an MCP client.
//!
//! # Integration
//!
//! The library uses the [rmcp](https://crates.io/crates/rmcp) crate for
//! MCP client integration. All elicitation happens through
//! asynchronous MCP tool calls.
//!
//! # Example
//!
//! ```rust,ignore
//! use elicitation::{Elicitation, ElicitResult};
//! use rmcp::service::{Peer, RoleClient};
//!
//! async fn example(client: &Peer<RoleClient>) -> ElicitResult<()> {
//!     // Elicit a simple integer
//!     let age: i32 = i32::elicit(client).await?;
//!
//!     // Elicit an optional value
//!     let nickname: Option<String> = Option::<String>::elicit(client).await?;
//!
//!     // Elicit a collection
//!     let scores: Vec<i32> = Vec::<i32>::elicit(client).await?;
//!     Ok(())
//! }
//! ```
```

### Step 9: Update Tests

**Note**: No tests directory exists yet (CLAUDE.md requires tests in `tests/` not `#[cfg(test)]`)

**Action**: Create test infrastructure in future phase

**For now**: Verify compilation and manual testing

### Step 10: Version Bump and Documentation

**Files to update**:
1. `Cargo.toml` - Version 0.1.0 → 0.2.0
2. `CHANGELOG.md` - Add v0.2.0 entry
3. `README.md` - Update examples to use rmcp
4. This plan document - Mark as completed

## Testing Strategy

### Compilation Verification

```bash
cd /home/erik/repos/elicitation

# Check workspace
cargo check

# Check individual crates
cargo check -p elicitation
cargo check -p elicitation_derive

# Run clippy
cargo clippy --all-targets

# Format check
cargo fmt --check
```

### Manual Testing

Create a simple test binary:

```rust
// tests/manual_rmcp_test.rs
use elicitation::{Elicitation, ElicitResult};
use rmcp::{ServiceExt, service::{Peer, RoleClient}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test client
    let client = ()
        .serve(rmcp::transport::stdio(
            tokio::io::stdin(),
            tokio::io::stdout(),
        ))
        .await?;

    // Test boolean elicitation
    let answer = bool::elicit(&client).await?;
    println!("Got boolean: {}", answer);

    // Test integer elicitation
    let number = i32::elicit(&client).await?;
    println!("Got number: {}", number);

    // Test string elicitation
    let text = String::elicit(&client).await?;
    println!("Got string: {}", text);

    Ok(())
}
```

### Integration Testing

Test with actual MCP server:
1. Create simple MCP server with elicitation tools
2. Connect elicitation client
3. Verify all primitive types work
4. Verify containers work
5. Verify derive macro works

## Acceptance Criteria

- [ ] All compilation errors resolved
- [ ] Zero clippy warnings
- [ ] All primitive types compile and have correct signatures
- [ ] All container types compile and have correct signatures
- [ ] Derive macro generates correct code
- [ ] Manual testing confirms functionality
- [ ] Documentation updated
- [ ] CHANGELOG updated
- [ ] Version bumped to 0.2.0

## Risk Assessment

### High Risk
None (isolated branch, early development)

### Medium Risk
1. **rmcp API changes**: Mitigated by version pinning
2. **Tool call compatibility**: Mitigated by JSON parameter builders

### Low Risk
1. **Type inference issues**: Simpler API reduces risk
2. **Macro generation**: Straightforward substitution

## Timeline

**Estimated**: 4-6 hours of focused work

| Step | Estimated Time | Priority |
|------|----------------|----------|
| 1. Dependencies | 15 min | P0 |
| 2. Error types | 30 min | P0 |
| 3. Core traits | 15 min | P0 |
| 4. Primitives | 60 min | P0 |
| 5. Containers | 45 min | P0 |
| 6. MCP tools | 15 min | P1 |
| 7. Derive macro | 90 min | P0 |
| 8. lib.rs | 10 min | P1 |
| 9. Tests | 30 min | P1 |
| 10. Docs | 30 min | P1 |

**Total**: ~5.5 hours

## Next Steps After Migration

1. Merge to main branch
2. Publish elicitation v0.2.0 to crates.io
3. Update botticelli to use elicitation v0.2.0
4. Implement Phase 3: Unify Tool Definitions (rmcp-based tool types)
5. Implement Phase 4: Dual-Derive Pattern
6. Implement Phase 5: Advanced Patterns (Survey, Authorize)

## References

- **RMCP Repository**: `/tmp/rmcp-verification` (tag: rmcp-v0.12.0)
- **RMCP Docs**: https://docs.rs/rmcp/0.12.0/rmcp/
- **Vision Document**: ELICITATION_RMCP_IMPLEMENTATION_VISION.md
- **CLAUDE.md**: Project coding standards
- **Examples**: `/tmp/rmcp-verification/examples/clients/`

---

**Status**: Ready for implementation
**Created**: 2025-12-29
**Last Updated**: 2025-12-29
