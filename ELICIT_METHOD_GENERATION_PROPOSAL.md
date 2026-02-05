# Elicit Method Generation & Aggregation Architecture

## Problem

The current `#[derive(Elicit)]` macro generates **standalone functions** that cannot be discovered by `#[tool_router]` or similar aggregation macros:

```rust
#[derive(Elicit)]
pub struct Config {
    timeout: u32,
}

// Current: Generates standalone function
#[cfg_attr(not(test), elicitation::rmcp::tool)]
pub async fn elicit_config(
    client: Arc<Peer<RoleClient>>,
) -> Result<Config, ElicitError> {
    Config::elicit(&ElicitClient::new(client)).await
}
```

**Issues:**
1. Functions live at module level, not discoverable by macro introspection
2. No way to automatically aggregate all elicit tools in a module
3. Users must manually import and register each function
4. Doesn't compose with tool router patterns

## Proposed Architecture

### Part 1: Generate Methods on Types

Change the derive macro to generate methods **on the type itself**:

```rust
#[derive(Elicit)]
pub struct Config {
    timeout: u32,
}

// Proposed: Generate method on impl block
impl Config {
    /// Auto-generated MCP tool for eliciting [`Config`].
    ///
    /// Automatically registered as an MCP tool via `#[rmcp::tool]` in non-test builds.
    #[cfg_attr(not(test), elicitation::rmcp::tool)]
    pub async fn elicit_checked(
        client: Arc<elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleClient>>,
    ) -> Result<Self, elicitation::ElicitError> {
        use elicitation::{Elicitation, ElicitClient};
        Self::elicit(&ElicitClient::new(client)).await
    }
}
```

**Benefits:**
1. Methods are attached to types (better organization)
2. Consistent naming: `TypeName::elicit_checked()`
3. Enables macro introspection of types in scope
4. More idiomatic Rust (methods > module functions)

**Method name**: Use `elicit_checked()` instead of `elicit()` to avoid conflicts with the `Elicitation` trait's `elicit()` method.

### Part 2: Aggregation Macro

Provide a `#[collect_elicit_checkeds]` macro that discovers all types with `elicit_checked()` methods in a module:

```rust
use elicitation::collect_elicit_checkeds;

// User's module with many Elicit types
pub mod storage {
    use elicitation::Elicit;
    
    #[derive(Elicit)]
    pub struct StoreParams { data: Vec<u8> }
    
    #[derive(Elicit)]
    pub struct StoreResult { reference: String }
    
    #[derive(Elicit)]
    pub struct RetrieveParams { reference: String }
    
    // ... many more types
}

// Aggregation in a separate location
use rmcp::handler::server::tool::ToolRouter;

pub struct ElicitToolRegistry;

#[collect_elicit_checkeds(module = "crate::storage")]
impl ElicitToolRegistry {
    // Macro generates:
    // pub fn tool_router() -> ToolRouter<Self> {
    //     ToolRouter::new()
    //         .with_route((StoreParams::elicit_checked_tool_attr(), StoreParams::elicit_checked))
    //         .with_route((StoreResult::elicit_checked_tool_attr(), StoreResult::elicit_checked))
    //         .with_route((RetrieveParams::elicit_checked_tool_attr(), RetrieveParams::elicit_checked))
    // }
}
```

## Implementation Details

### File: `crates/elicitation_derive/src/tool_gen.rs`

**Current code** (lines 19-40):

```rust
pub fn generate_tool_function(input: &DeriveInput) -> TokenStream {
    let type_name = &input.ident;
    let fn_name = format_ident!("elicit_{}", to_snake_case(&type_name.to_string()));

    quote! {
        /// Auto-generated MCP tool function for eliciting [`#type_name`].
        ///
        /// This function uses the derived `Elicitation` impl to
        /// interactively elicit a value from the user via MCP.
        ///
        /// Automatically registered as an MCP tool via `#[rmcp::tool]` in non-test builds.
        #[cfg_attr(not(test), elicitation::rmcp::tool)]
        pub async fn #fn_name(
            client: std::sync::Arc<elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleClient>>,
        ) -> Result<#type_name, elicitation::ElicitError> {
            use elicitation::{Elicitation, ElicitClient};
            #type_name::elicit(&ElicitClient::new(client)).await
        }
    }
}
```

**Proposed code**:

```rust
pub fn generate_tool_function(input: &DeriveInput) -> TokenStream {
    let type_name = &input.ident;

    quote! {
        impl #type_name {
            /// Auto-generated MCP tool for eliciting [`#type_name`].
            ///
            /// This method uses the derived `Elicitation` impl to
            /// interactively elicit a value from the user via MCP.
            ///
            /// Automatically registered as an MCP tool via `#[rmcp::tool]` in non-test builds.
            #[cfg_attr(not(test), elicitation::rmcp::tool)]
            pub async fn elicit_checked(
                client: std::sync::Arc<elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleClient>>,
            ) -> Result<Self, elicitation::ElicitError> {
                use elicitation::{Elicitation, ElicitClient};
                Self::elicit(&ElicitClient::new(client)).await
            }
        }
    }
}
```

**Changes:**
1. Wrap in `impl #type_name { }` block
2. Change function name from `elicit_typename` to `elicit_checked`
3. Remove `pub async fn` prefix (already inside impl block)
4. Use `Self` instead of concrete type name
5. Return type: `Result<Self, ...>` instead of `Result<#type_name, ...>`

### New File: `crates/elicitation_macros/src/collect_tools.rs`

Create a new procedural macro for tool aggregation:

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemImpl, Lit, Meta, NestedMeta};

/// Collect all elicit_checked() methods from types in a module into a tool router.
///
/// # Usage
///
/// ```ignore
/// #[collect_elicit_checkeds(module = "crate::storage")]
/// impl ElicitToolRegistry {
///     // Macro generates tool_router() method here
/// }
/// ```
///
/// # How it Works
///
/// 1. Parses the `module` attribute to get the module path
/// 2. Uses syn to discover all types in that module
/// 3. Checks each type for a public `elicit_checked()` method
/// 4. Generates a `tool_router()` method that registers all found tools
///
/// # Generated Code
///
/// ```ignore
/// impl ElicitToolRegistry {
///     pub fn tool_router() -> rmcp::handler::server::tool::ToolRouter<Self> {
///         rmcp::handler::server::tool::ToolRouter::new()
///             .with_route((TypeA::elicit_checked_tool_attr(), TypeA::elicit_checked))
///             .with_route((TypeB::elicit_checked_tool_attr(), TypeB::elicit_checked))
///             // ... for each type with elicit_checked()
///     }
/// }
/// ```
pub fn collect_elicit_checkeds(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let impl_block = parse_macro_input!(input as ItemImpl);

    // Extract module path from attribute
    let module_path = extract_module_path(&args);

    // TODO: Actual implementation would:
    // 1. Parse the module at compile time (challenging - may need runtime discovery)
    // 2. Find all types with elicit_checked() methods
    // 3. Generate tool_router() that registers them all

    // For now, generate a placeholder that shows the pattern
    quote! {
        #impl_block

        // Generated tool router method would go here
        impl ElicitToolRegistry {
            pub fn tool_router() -> rmcp::handler::server::tool::ToolRouter<Self> {
                // Would generate registration for each discovered type
                rmcp::handler::server::tool::ToolRouter::new()
            }
        }
    }
    .into()
}

fn extract_module_path(args: &AttributeArgs) -> String {
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("module") {
                if let Lit::Str(lit) = &nv.lit {
                    return lit.value();
                }
            }
        }
    }
    panic!("Missing required `module` attribute");
}
```

### Export from `crates/elicitation_macros/src/lib.rs`

```rust
mod collect_tools;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn collect_elicit_checkeds(args: TokenStream, input: TokenStream) -> TokenStream {
    collect_tools::collect_elicit_checkeds(args, input)
}
```

### Export from `crates/elicitation/src/lib.rs`

```rust
// Re-export the collection macro
pub use elicitation_macros::collect_elicit_checkeds;
```

## Alternative: Inventory-Based Discovery

Since macro introspection of modules is difficult at compile time, consider using the [`inventory`](https://docs.rs/inventory) crate for runtime registration:

```rust
use elicitation::Elicit;
use inventory;

#[derive(Elicit)]
pub struct Config { timeout: u32 }

// Derive macro also generates:
impl Config {
    #[cfg_attr(not(test), elicitation::rmcp::tool)]
    pub async fn elicit_checked(...) -> Result<Self, ElicitError> { ... }
}

// AND submits to inventory:
inventory::submit! {
    elicitation::ElicitToolDescriptor {
        name: "Config",
        tool_fn: Config::elicit_checked,
    }
}

// Users can then collect all tools:
pub fn all_elicit_checkeds() -> Vec<elicitation::ElicitToolDescriptor> {
    inventory::iter::<elicitation::ElicitToolDescriptor>()
        .cloned()
        .collect()
}
```

**Pros:**
- Works at runtime (no compile-time module introspection needed)
- Automatic registration when type is defined
- Simple user API

**Cons:**
- Runtime overhead (small)
- Requires `inventory` dependency
- Slightly more complex derive macro

## Usage Examples

### Before (Current - 0.5.0)

```rust
// Type definition
#[derive(Elicit)]
pub struct Config { timeout: u32 }

// Manual registration required
use rmcp::handler::server::tool::ToolRouter;

let router = ToolRouter::new()
    .with_route((elicit_config_tool_attr(), elicit_config))
    .with_route((elicit_other_tool_attr(), elicit_other))
    // ... manual registration for every type
```

### After (Proposed)

**Option 1: With aggregation macro**

```rust
// Type definition (no changes from user perspective)
#[derive(Elicit)]
pub struct Config { timeout: u32 }

#[derive(Elicit)]
pub struct OtherType { value: String }

// Automatic aggregation
use elicitation::collect_elicit_checkeds;

pub struct ElicitTools;

#[collect_elicit_checkeds(module = "crate")]
impl ElicitTools {
    // Macro generates tool_router() automatically
}

// Use it
let router = ElicitTools::tool_router();
```

**Option 2: With inventory-based discovery**

```rust
// Type definition (no changes)
#[derive(Elicit)]
pub struct Config { timeout: u32 }

// Automatic registration happens in derive macro

// Collection at runtime
use elicitation::collect_all_elicit_checkeds;

let router = collect_all_elicit_checkeds();
```

## Migration Path

### Phase 1: Generate Methods (Breaking Change)

**Version:** 0.6.0 (major bump due to breaking change)

1. Update `tool_gen.rs` to generate methods instead of functions
2. Update all documentation and examples
3. Provide migration guide for users

**Migration:**
```rust
// Before: 0.5.0
elicit_config(client).await

// After: 0.6.0
Config::elicit_checked(client).await
```

### Phase 2: Add Aggregation (Additive)

**Version:** 0.6.1 (minor bump - new feature)

Choose either:
- **Option A**: Compile-time macro aggregation (cleaner, harder to implement)
- **Option B**: Runtime inventory (easier, small overhead)

Provide the aggregation mechanism and update docs with examples.

### Phase 3: Botticelli Integration

Update `botticelli_mcp` to use new elicit tools:

```rust
// Automatically collect all elicit tools from our modules
#[collect_elicit_checkeds(module = "crate::rmcp_server::tools")]
impl BotticelliElicitTools {}

// Combine with manual tool routers
let router = BotticelliServer::core_tool_router()
    + BotticelliServer::cache_tool_router()
    + BotticelliServer::storage_tool_router()
    + BotticelliElicitTools::tool_router();  // All elicit tools automatically
```

## Testing Strategy

### Unit Tests

Test method generation:

```rust
#[test]
fn test_method_generation() {
    #[derive(Elicit)]
    struct TestType { value: i32 }
    
    // Should compile - method exists
    let _method = TestType::elicit_checked;
}
```

### Integration Tests

Test aggregation:

```rust
mod test_module {
    #[derive(Elicit)]
    pub struct TypeA { a: i32 }
    
    #[derive(Elicit)]
    pub struct TypeB { b: String }
}

#[collect_elicit_checkeds(module = "test_module")]
impl TestRegistry {}

#[test]
fn test_tool_collection() {
    let router = TestRegistry::tool_router();
    let tools = router.list_tools();
    
    assert!(tools.iter().any(|t| t.name == "TypeA"));
    assert!(tools.iter().any(|t| t.name == "TypeB"));
}
```

### Botticelli Integration Tests

Test in real usage:

```bash
cd botticelli
cargo test -p botticelli_mcp --test test_elicit_checkeds
```

Expected: 100+ tools registered (22 manual + 80+ elicit)

## Open Questions

1. **Method naming**: `elicit_checked()` vs `elicit()` vs other?
   - Current: `elicit()` (conflicts with trait method)
   - Proposed: `elicit_checked()` (clear, no conflicts)
   - Alternative: `mcp_tool()`, `tool()`, `elicit_mcp()`

2. **Aggregation approach**: Macro vs inventory vs other?
   - Macro: Clean, compile-time, harder to implement
   - Inventory: Simple, runtime, small overhead
   - Other: Proc-macro hackery, export_name tricks?

3. **Breaking change timing**: When to release 0.6.0?
   - Now: Get architecture right early
   - Later: Wait for more user feedback
   - Consider: Provide both patterns temporarily?

4. **Tool naming convention**: How to name the generated tools?
   - `TypeName` (simple, matches type)
   - `elicit_type_name` (descriptive, but redundant)
   - `TypeName::elicit_checked` (full path, clearest)

## Recommendation

**Phase 1 (Immediate - 0.6.0):**
- Implement method generation (lines 19-40 in tool_gen.rs)
- Use inventory-based aggregation (simpler to implement correctly)
- Release as 0.6.0 with migration guide

**Phase 2 (Future - 0.7.0):**
- If inventory proves problematic, explore macro-based aggregation
- Consider compile-time alternatives if performance becomes an issue

**Reasoning:**
- Method generation is architecturally superior (no debate here)
- Inventory is proven, simple, and reliable
- Can always optimize later if needed
- Gets botticelli unblocked quickly with 100+ elicit tools

## Implementation Checklist

- [ ] Update `tool_gen.rs` to generate impl blocks with methods
- [ ] Add `inventory` dependency to elicitation workspace
- [ ] Create `ElicitToolDescriptor` type for inventory
- [ ] Update derive macro to submit to inventory
- [ ] Create `collect_all_elicit_checkeds()` helper function
- [ ] Write unit tests for method generation
- [ ] Write integration tests for tool collection
- [ ] Update all documentation and examples
- [ ] Write migration guide (0.5.0 → 0.6.0)
- [ ] Update CHANGELOG.md
- [ ] Bump version to 0.6.0
- [ ] Test in botticelli integration
- [ ] Commit: `feat!: Generate elicit methods on types (BREAKING)`
