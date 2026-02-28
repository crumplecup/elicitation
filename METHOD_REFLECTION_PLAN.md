# Method Reflection Implementation Plan

## ✅ Implementation Progress (2026-02-28)

### Milestone 1: Newtype Infrastructure - COMPLETE
- ✅ Implemented `elicit_newtype!` declarative macro in `elicitation/src/newtype_macro.rs`
- ✅ Implemented `elicit_newtypes!` bulk generation macro
- ✅ Type conversions via derive_more (Deref, DerefMut, From, AsRef)
- ✅ Bidirectional From implementations
- ✅ 6 integration tests passing
- ✅ Documentation with correct syntax examples (PathBuf, not Path)
- ✅ Committed: feat(macros) commit on method-reflection-macros branch

### Milestone 2: Method Discovery & Parameter Generation - COMPLETE
- ✅ Module structure in `elicitation_derive/src/method_reflection/`
  - `mod.rs` - Pipeline orchestration
  - `discovery.rs` - Method extraction from impl blocks
  - `params.rs` - Parameter struct generation with conversions
  - `wrapper.rs` - Wrapper method generation logic
- ✅ Method discovery from user-provided impl blocks
- ✅ Parameter struct generation with #[derive(Elicit, JsonSchema)]
- ✅ Type conversions: &str → String, &[T] → Vec<T>, &T → T
- ✅ Return type extraction (Result<T, E> → T)
- ✅ 13 unit tests passing (discovery + params + wrapper modules)
- ✅ 4 integration tests passing
- ✅ Wrapper generation logic implemented and tested
- ✅ Committed: feat(derive) commits

### Current State
The `#[reflect_methods]` macro currently:
1. Discovers public methods from impl blocks
2. Generates parameter structs with type conversions
3. Preserves original impl block (methods work via Deref)

Wrapper method generation logic is **implemented and tested** but **not yet integrated** into the pipeline. This allows us to:
- Keep original methods functional (via Deref)
- Generate parameter structs for type safety
- Defer MCP tool wrapper integration to next phase

### Next Steps
1. **Integrate wrapper generation** - Make it opt-in or default
2. **Reference conversion warnings** - Emit warnings for large clones
3. **Generic support** - Add JsonSchema bounds
4. **Documentation** - Update user guide with examples

## Executive Summary

Automatically generate MCP tool wrappers for third-party crate methods through **newtype-based method reflection**. This enables instant AI tool integration for any Rust library with minimal configuration, while maintaining formal verification guarantees.

**Key Innovation:** One-line newtype creation with automatic method discovery, smart parameter conversion, and MCP tool generation.

**Primary Use Case:** Companion crates (e.g., `elicit_reqwest`) that expose familiar type names (`Client`, `Request`) with MCP tool capabilities. Users get the ergonomics of the original API with AI agent integration built-in.

**Vision:** `cargo add elicit_reqwest` → instant MCP tools for all reqwest operations.

## Current State Analysis

### Existing Infrastructure (Strong Foundation)

#### elicitation_derive Crate
- `#[derive(Elicit)]` - Generates `Elicitation` trait impls for enums/structs
- `#[contract_type]` - Verification contract metadata
- `#[derive(Rand)]` - Contract-aware random generation
- Generates both trait implementations AND MCP tool functions

#### elicitation_macros Crate
- `#[instrumented_impl]` - Automatic tracing for impl blocks
- `#[elicit_tools(Type1, Type2)]` - Generates elicitation wrapper methods
  - Creates async methods calling `Type::elicit_checked(peer)`
  - Integrates with rmcp's `#[tool_router]`
- `#[elicit_trait_tools_router]` - Delegates trait methods as MCP tools

### Key Architectural Patterns

**Pattern 1: Type-Level Tool Generation**
```rust
// Existing pattern
#[elicit_tools(Config, User)]
#[tool_router]
impl MyServer { }

// Generates:
pub async fn elicit_config(peer: Peer<RoleServer>)
    -> Result<Json<ElicitToolOutput<Config>>, ErrorData>
{
    Config::elicit_checked(peer).await
        .map(Json)
        .map(ElicitToolOutput::new)
}
```

**Pattern 2: Trait Method Delegation**
```rust
// Existing pattern
#[elicit_trait_tools_router(Calculator, calc, [add, subtract])]
#[tool_router]
impl<C: Calculator> Server<C> { }

// Generates delegating wrapper methods
```

**Gap:** No automatic method reflection for third-party types - requires manual parameter struct creation and wrapper implementation.

## Proposed Solution Architecture

### Core Design Philosophy: Familiar Type Names

**Primary Use Case:** Create companion crates (e.g., `elicit_reqwest`) that shadow original types with same-name wrappers.

**Why Same-Name Wrappers?**
```rust
// In elicit_reqwest crate:
elicit_newtype!(reqwest::Client);  // Creates: pub struct Client(reqwest::Client)

// Users get familiar names:
use elicit_reqwest::Client;  // Not "HttpClient" or "ElicitClient"!

// Feels natural and ergonomic:
let client = Client::new();
let response = client.get("https://example.com").await?;

// The fact it's an MCP-tool-enabled wrapper is transparent
```

**Benefits:**
1. **Familiarity:** Users see `Client`, `Request`, `Response` - same as original
2. **No Name Collisions:** `elicit_reqwest::Client` vs `reqwest::Client` are distinct namespaces
3. **Drop-in Feel:** Wrapper is transparent via Deref/DerefMut
4. **Ecosystem Pattern:** Mirrors `tokio::fs::File` vs `std::fs::File`

**Optional Rename:** For edge cases where both types needed in same scope:
```rust
use reqwest::Client as HttpClient;
use elicit_reqwest::Client;  // Wrapper with original name

// Or if you prefer different naming:
elicit_newtype!(reqwest::Client, as ElicitClient);
```

### Phase 1: Newtype Wrapper Generation (`elicit_newtype!` macro)

**Location:** `elicitation_macros` crate (declarative macro)

**Purpose:** Generate a newtype wrapper around third-party types to satisfy orphan rule

**Primary Use Case:** Same-name wrapper (shadows original in elicit_* crate)
```rust
// User writes (in elicit_reqwest crate):
elicit_newtype!(reqwest::Client);

// Macro generates:
#[derive(Debug, Clone, derive_more::Deref, derive_more::DerefMut, derive_more::From)]
pub struct Client(pub reqwest::Client);  // Same name as original!

// derive_more handles:
// - Deref -> reqwest::Client
// - DerefMut -> reqwest::Client
// - From<reqwest::Client> for Client

// Manual From for unwrapping:
impl From<Client> for reqwest::Client {
    fn from(wrapper: Client) -> Self { wrapper.0 }
}

// Users import familiar names:
// use elicit_reqwest::Client;  // Not "HttpClient"!
```

**Secondary Use Case:** Custom rename (when both types needed in same scope)
```rust
// Optional rename for edge cases:
elicit_newtype!(reqwest::Client, as HttpClient);

// Generates: pub struct HttpClient(pub reqwest::Client);
```

**Macro Syntax:**
```rust
// Primary: Same-name wrapper (extracts "Client" from path)
elicit_newtype!(reqwest::Client);
// Generates:
// #[derive(Debug, Clone, derive_more::Deref, derive_more::DerefMut, derive_more::From)]
// pub struct Client(pub reqwest::Client);

// With visibility:
elicit_newtype!(pub(crate) tokio::fs::File);
// → pub(crate) struct File(pub tokio::fs::File);

// Optional rename (secondary use case):
elicit_newtype!(reqwest::Client, as HttpClient);
// → pub struct HttpClient(pub reqwest::Client);

// Bulk generation:
elicit_newtypes! {
    reqwest::Client,
    reqwest::Request,
    reqwest::Response,
}
```

**Generated Code Features:**
- Uses `derive_more::Deref` and `derive_more::DerefMut` (per CLAUDE.md standards)
- Uses `derive_more::From` for wrapping direction
- Optional: `derive_more::AsRef` and `derive_more::AsMut` for maximum transparency
- Manual `From` impl for unwrapping direction
- Simple declarative macro (no syn parsing needed)
- Extracts type name from path: `path::to::Type` → `Type`
- Optional visibility control
- Optional rename for edge cases

**Implementation Note:**
```rust
// Full derive set for transparent newtype:
#[derive(
    Debug, Clone,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::From,
    derive_more::AsRef,
    derive_more::AsMut,
)]
pub struct Client(pub reqwest::Client);

// Enables all access patterns:
let client = Client::from(reqwest_client);
let _: &reqwest::Client = &*client;        // Deref
let _: &reqwest::Client = client.as_ref(); // AsRef
```

### Phase 2: Method Reflection Attribute (`#[reflect_methods]`)

**Location:** `elicitation_derive` crate (proc macro)

**Purpose:** Analyze third-party type methods and generate MCP tool wrappers

```rust
// User writes (in elicit_reqwest crate):
elicit_newtype!(reqwest::Client);

#[reflect_methods]
impl Client {
    // Macro discovers methods on reqwest::Client and generates tools
}
```

**Implementation Steps:**

#### Step 2a: Method Discovery
- Use `syn` to parse target type path
- Introspect public methods via rustdoc JSON or trait bounds
- Classify methods by support level:
  - ✅ **Fully Supported:** Simple parameter types (primitives, String, owned types)
  - ✅ **Supported with Conversion:** Reference parameters where T: Clone (convert &T → T)
    - Warn if T is large (user can explicitly allow)
  - ✅ **Supported with Bounds:** Generic parameters where T: JsonSchema
  - ⚠️ **Warn but Generate:** Large clone conversions (user decides trade-off)
  - ❌ **Skip with Diagnostic:** Complex lifetimes (truly unsupported)
  - ❌ **Skip with Diagnostic:** Uncloneable borrowed types (no conversion possible)

**Philosophy:** Generate methods whenever possible, warn about potential issues, let users make informed decisions.

#### Step 2b: Parameter Struct Generation
For each supported method, generate parameter struct:

```rust
// Original method:
// impl reqwest::Client {
//     pub async fn get(&self, url: &str) -> Result<Response, Error>
// }

// Generated parameter struct:
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ClientGetParams {
    #[schemars(description = "URL to fetch")]
    pub url: String,  // Converted from &str
}
```

**Naming Convention:** `{TypeName}{MethodName}Params` in PascalCase
- Type name is the **wrapper** name (Client, not reqwest::Client)
- Keeps parameter struct names familiar and concise

#### Step 2c: Tool Wrapper Generation
```rust
#[tool(description = "HTTP GET request")]
pub async fn get(
    &self,
    params: Parameters<ClientGetParams>,
) -> Result<Json<Response>, ErrorData> {
    let params = params.into_inner();
    self.0.get(&params.url)
        .await
        .map(Json)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))
}
```

**Key Features:**
- Automatic &T → T conversion where T: Clone
- Error mapping to rmcp::ErrorData
- Integration with rmcp's Parameters wrapper
- Preserves async/sync nature of original method

### Phase 3: Smart Reference Conversion

**Location:** `elicitation_derive/src/method_reflection.rs` (new module)

**Strategy:** Convert with warnings, not restrictions

```rust
#[reflect_methods(
    clone_strategy = "smart",        // Default: convert all cloneable refs
    warn_clone_threshold = 1024,     // Warn on large clones (not skip!)
    allow_large_clones = true,       // Override warnings (explicit opt-in)
    exclude = ["specific_method"],   // Explicit method exclusion only
)]
impl Client { }
```

**Conversion Rules:**
- `&str` → `String` (always, no warning - common pattern)
- `&[T]` → `Vec<T>` (if T: Clone, warn if element is large)
- `&T` → `T` (if T: Clone, warn if T is large)
- Emit **compile-time warnings** for large clones, but **still generate the method**
- User can suppress warnings with `#[allow(large_clone)]` or `allow_large_clones = true`
- Only skip methods that are **truly unsupported** (complex lifetimes, non-Clone types)

**Implementation:**
```rust
fn should_convert_to_owned(ty: &syn::Type, config: &ConversionConfig) -> ConversionDecision {
    match ty {
        Type::Reference(r) => {
            if is_str_slice(&r.elem) {
                // Always convert &str -> String (common pattern, no warning)
                return ConversionDecision::Convert {
                    target_type: quote! { String },
                    warn: false,
                };
            }
            if is_byte_slice(&r.elem) {
                return ConversionDecision::Convert {
                    target_type: quote! { Vec<u8> },
                    warn: false,
                };
            }
            if is_cloneable(&r.elem) {
                let target = remove_reference(ty);
                let is_large = estimate_size(&r.elem) > config.warn_clone_threshold;

                // Convert regardless of size, but warn if large
                return ConversionDecision::Convert {
                    target_type: target,
                    warn: is_large && !config.allow_large_clones,
                };
            }
            // Only skip if NOT cloneable (truly unsupported)
            ConversionDecision::Skip {
                reason: format!("Type {:?} does not implement Clone", r.elem),
            }
        }
        _ => ConversionDecision::Keep,
    }
}

// Generated code includes warning attribute when needed:
#[cfg_attr(not(allow_large_clones), warn(large_clone))]
pub async fn process_data(&self, params: ProcessDataParams) -> Result<...> {
    // params.large_value was cloned from &LargeType
}

// Compile output example:
// warning: large clone in generated method
//   --> src/lib.rs:42:1
//    |
// 42 | pub async fn process_data(&self, params: ProcessDataParams)
//    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//    |
//    = note: Converting &LargeBuffer (2048 bytes) to owned value requires cloning
//    = help: This is common in I/O-bound code where clone cost is negligible
//    = help: To suppress this warning: #[reflect_methods(allow_large_clones = true)]
//    = help: To exclude this method: #[reflect_methods(exclude = ["process_data"])]
```

**Example User Workflow:**
1. Generate methods with `#[reflect_methods]`
2. See warning about large clone in `process_data`
3. User evaluates: "Is this method I/O-bound or CPU-bound?"
4. If I/O-bound: Suppress warning (clone cost irrelevant)
5. If CPU-bound: Either accept trade-off or manually optimize
6. User stays informed and in control

### Phase 4: JsonSchema-Bounded Generics

**Location:** `elicitation_derive/src/generic_support.rs` (new module)

**Purpose:** Support generic methods through JsonSchema bounds

```rust
// Original generic method:
// impl Serializer {
//     pub fn serialize<T: Serialize>(&self, value: &T) -> Result<String, Error>
// }

// Generated with JsonSchema bound:
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct SerializerSerializeParams<T>
where
    T: Serialize + JsonSchema + Clone,
{
    pub value: T,  // Converted from &T
}

#[tool(description = "Serialize value to JSON")]
pub async fn serialize<T>(
    &self,
    params: Parameters<SerializerSerializeParams<T>>,
) -> Result<Json<String>, ErrorData>
where
    T: Serialize + JsonSchema + Clone + Send + Sync,
{
    let params = params.into_inner();
    self.0.serialize(&params.value)
        .map(Json)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))
}
```

**Key Insight:** JsonSchema provides the "concreteness" needed for MCP tools while maintaining genericity

**Automatic Discovery:**
- Scan method signatures for generic type parameters
- Check existing trait bounds
- Add `+ JsonSchema` bound if not present
- Generate schema-compatible parameter structs

### Phase 5: Unified Macro Experience

**Location:** `elicitation_macros` crate

**User-Facing Macro:** Single-line integration

```rust
// Primary pattern - same-name wrapper (most common):
elicit_newtype!(reqwest::Client);

#[reflect_methods]
impl Client { }

// With configuration:
elicit_newtype!(tokio::fs::File);

#[reflect_methods(
    include = ["read", "write", "create"],  // Explicit method whitelist
    exclude = ["dangerous_method"],         // Exclusions
    clone_strategy = "smart",               // Reference conversion (default)
    warn_clone_threshold = 1024,            // Warn for clones > 1KB (default)
    allow_large_clones = false,             // Show warnings (default)
    generic_strategy = "jsonschema",        // Generic handling
    diagnostics = "verbose",                // Error reporting level
)]
impl File { }

// To suppress large clone warnings globally:
#[reflect_methods(allow_large_clones = true)]
impl File { }

// Methods still generated, warnings suppressed

// Advanced: Multiple types at once (same-name wrappers)
elicit_newtypes! {
    reqwest::Client,
    reqwest::Request,
    reqwest::Response,
}

// Optional: Custom rename for edge cases
elicit_newtype!(reqwest::Client, as HttpClient);
#[reflect_methods]
impl HttpClient { }
```

**Macro Expansion Flow:**
```text
elicit_newtype!(Type)
  ↓ Generates newtype wrapper

#[reflect_methods]
  ↓ Discovers methods on inner type
  ↓ Applies conversion rules
  ↓ Generates parameter structs (#[derive(Elicit)])
  ↓ Generates tool wrapper methods with #[tool]

#[tool_router]  (existing rmcp macro)
  ↓ Discovers generated #[tool] methods
  ↓ Registers in tool router
```

## Implementation Roadmap

### Milestone 1: Core Infrastructure (Week 1-2)
**Files to Create:**
- `crates/elicitation_derive/src/method_reflection/mod.rs`
- `crates/elicitation_derive/src/method_reflection/discovery.rs`
- `crates/elicitation_derive/src/method_reflection/params.rs`
- `crates/elicitation_derive/src/method_reflection/wrapper.rs`

**Deliverables:**
1. Basic method discovery (public methods only)
2. Simple parameter struct generation (primitives + String)
3. Wrapper method generation (non-generic, sync methods)
4. Add to `elicitation_derive/src/lib.rs`:
```rust
mod method_reflection;

#[proc_macro_attribute]
pub fn reflect_methods(attr: TokenStream, item: TokenStream) -> TokenStream {
    method_reflection::expand(attr, item)
}
```

**Tests:**
- Newtype wrapper generation with derive_more
- Deref/DerefMut/AsRef/AsMut trait verification
- Method discovery on std::fs::File
- Parameter struct generation
- Basic tool wrapper compilation

### Milestone 2: Reference Conversion (Week 3)
**Files to Modify:**
- `crates/elicitation_derive/src/method_reflection/conversion.rs` (new)

**Deliverables:**
1. &str → String conversion (no warnings)
2. &[T] → Vec<T> conversion (warn if large)
3. &T → T conversion with size detection
4. Configuration attribute parsing (`warn_clone_threshold`, `allow_large_clones`)
5. Compile-time warning emission for large clones
6. Diagnostic messages for truly unsupported methods (non-Clone types)

**Warning System:**
- Emit `#[warn(large_clone)]` on generated methods when applicable
- Provide clear warning message: "Converting &LargeType to LargeType requires cloning X bytes"
- User can suppress with `#[allow(large_clone)]` or global `allow_large_clones = true`
- Document performance implications in generated docs

**Tests:**
- reqwest::Client basic methods (get, post)
- Conversion correctness
- Warning emission for large clones
- Warning suppression via config
- Skip diagnostics for unsupported types only

### Milestone 3: Generic Support (Week 4)
**Files to Create:**
- `crates/elicitation_derive/src/method_reflection/generics.rs`

**Deliverables:**
1. Generic type parameter detection
2. JsonSchema bound injection
3. Generic parameter struct generation
4. Generic tool wrapper generation

**Tests:**
- serde_json serialization methods
- Generic collection methods
- Bound propagation correctness

### Milestone 4: Integration & Ergonomics (Week 5)
**Files to Create:**
- `crates/elicitation_macros/src/newtype_macro.rs` (declarative macro)

**Deliverables:**
1. `elicit_newtype!` macro
2. `elicit_newtypes!` bulk macro
3. Integration with `#[derive(Elicit)]` for parameters
4. Documentation and examples

**Tests:**
- End-to-end reqwest integration
- Multiple types bulk generation
- Real-world use case examples

### Milestone 5: Documentation & Examples (Week 6)
**Files to Create:**
- `examples/elicit_reqwest_companion_crate/` (full companion crate example)
- `examples/method_reflection_tokio_fs.rs`
- `docs/METHOD_REFLECTION_GUIDE.md`
- `docs/COMPANION_CRATE_PATTERN.md`

**Deliverables:**
1. Comprehensive usage guide
2. Companion crate creation guide (e.g., how to build `elicit_reqwest`)
3. Migration guide from manual wrappers
4. Performance benchmarks
5. Coverage metrics for popular crates

**Example Companion Crate Structure:**
```
elicit_reqwest/
├── Cargo.toml          # Depends on: elicitation, reqwest
├── src/
│   └── lib.rs          # Uses elicit_newtype! macros
├── examples/
│   └── basic_usage.rs  # Shows Client usage
└── README.md           # "reqwest with MCP tools"
```

```rust
// elicit_reqwest/src/lib.rs
use elicitation::elicit_newtype;

elicit_newtypes! {
    reqwest::Client,
    reqwest::Request,
    reqwest::Response,
}

#[reflect_methods]
impl Client { }

#[reflect_methods]
impl Request { }

#[reflect_methods]
impl Response { }
```

Users then: `cargo add elicit_reqwest` and use familiar types with MCP superpowers.

## Technical Considerations

### Orphan Rule Compliance
Newtype pattern satisfies orphan rule:
- We own `HttpClient` (defined in our crate)
- We can implement `Elicitation` for `HttpClient`
- We delegate to `reqwest::Client` methods via Deref

### MCP Schema Requirements
All parameter structs must be objects:
```rust
// ✅ GOOD: Object schema
#[derive(JsonSchema)]
struct GetParams {
    url: String,
}

// ❌ BAD: Primitive schema (MCP requires objects)
String  // type: "string" not allowed at tool level
```

Solution: Always generate wrapper structs, even for single parameters

### Verification Compatibility
Generated tools remain verifiable:
```rust
#[cfg_attr(kani, kani::requires(params.url.len() > 0))]
pub async fn get(&self, params: GetParams) -> Result<Response, Error> {
    // Verification contracts compose with generated code
}
```

Users can add contracts to generated methods via manual impl extension.

### Performance Impact
- **Clone overhead:**
  - Typically negligible for I/O-bound operations (network/disk dwarfs clone cost)
  - Large clones emit warnings, user decides trade-off
  - In hot paths, users can manually optimize specific methods
  - Philosophy: Warn and educate, don't restrict
- **Compile time:** Linear in number of methods (acceptable)
- **Binary size:** Only generated methods included (tree-shaken)
- **Runtime:** Zero-cost abstraction (inlines to direct calls)

## Success Metrics

### Quantitative Goals
- ✅ 70%+ automatic coverage for reqwest::Client methods
- ✅ 80%+ automatic coverage for serde_json::Value methods
- ✅ <100ms compile time overhead per reflected type
- ✅ Zero runtime overhead vs. manual wrappers

### Qualitative Goals
- ✅ One-line integration for any third-party crate
- ✅ Clear diagnostics for unsupported methods
- ✅ **Warnings, not restrictions:** Generate when possible, warn about trade-offs
- ✅ **User stays informed:** Helpful warning messages with actionable guidance
- ✅ Seamless integration with existing `#[derive(Elicit)]`
- ✅ Verification contracts remain composable

## Migration Strategy

### From Manual Wrappers
**Before (Manual):**
```rust
// In elicit_reqwest crate
pub struct Client(pub reqwest::Client);

#[derive(Elicit, JsonSchema)]
struct ClientGetParams {
    url: String,
}

impl Client {
    #[tool]
    pub async fn get(&self, params: Parameters<ClientGetParams>)
        -> Result<Json<Response>, ErrorData>
    {
        self.0.get(&params.url).await.map(Json).map_err(convert_error)
    }
}
```

**After (Generated):**
```rust
// In elicit_reqwest crate
elicit_newtype!(reqwest::Client);

#[reflect_methods(include = ["get", "post"])]
impl Client { }
// Parameter structs and tools generated automatically
```

**Benefits:**
- 90% reduction in boilerplate
- Automatic synchronization with upstream changes
- Consistent error handling
- Zero behavior changes (same generated code)

## Alternative Approaches Considered

### ❌ Blanket Trait Implementation
```rust
impl<T> Elicitation for T where T: SomeTrait { }
```
**Rejected:** Violates orphan rule for third-party types

### ❌ Derive Macro on Third-Party Types
```rust
#[derive(Elicit)]
struct reqwest::Client;  // Can't derive on external types
```
**Rejected:** Can't derive on types we don't own

### ✅ Newtype + Method Reflection (Chosen)
- Satisfies orphan rule
- Preserves type safety
- Enables automatic generation
- Maintains verification compatibility

## Future Enhancements (Post-MVP)

### Phase 6: Trait Method Reflection
Extend to trait methods (not just concrete types):
```rust
#[reflect_trait_methods(Serialize)]
impl<T: Serialize> JsonValue { }
```

### Phase 7: Custom Conversions
User-defined conversion rules:
```rust
#[reflect_methods(
    convert(Url => String, via = "to_string"),
    convert(PathBuf => String, via = "to_string_lossy"),
)]
impl HttpClient { }
```

### Phase 8: Batch Type Generation
Crate-level configuration:
```rust
// Cargo.toml
[package.metadata.elicitation]
reflect = [
    { crate = "reqwest", types = ["Client", "Request"] },
    { crate = "tokio::fs", types = ["File"] },
]
```

Auto-generates at build time via build.rs

## Risks and Mitigations

### Risk: Compilation Complexity
**Mitigation:**
- Incremental development with clear milestones
- Extensive unit tests for each component
- Feature flags for gradual rollout

### Risk: Breaking Changes in Third-Party Crates
**Mitigation:**
- Version pinning in examples
- Clear documentation on version compatibility
- Graceful degradation (skip changed methods)

### Risk: Performance Regression
**Mitigation:**
- Benchmark suite comparing to manual wrappers
- Warning system for potentially expensive clones (not restrictions)
- Clear performance documentation with trade-offs
- Users make informed decisions (we don't silently optimize away features)

**Philosophy on Large Clones:**
We emit warnings but still generate methods for large clones. Rationale:
1. User knows their use case better than we do
2. Many applications are I/O-bound (clone cost irrelevant)
3. Warnings educate users about potential costs
4. Users can strategically handle hot paths if needed
5. Better to have the tool available than silently missing

## Conclusion

This implementation plan provides a **pragmatic, incremental path** to achieving the method reflection vision:

1. **Companion Crate Ecosystem** - Enable `elicit_reqwest`, `elicit_tokio`, etc. with familiar type names
2. **Leverages existing infrastructure** (`#[derive(Elicit)]`, `#[elicit_tools]`)
3. **Respects Rust's orphan rule** (newtype pattern)
4. **Maintains verification compatibility** (contracts compose)
5. **Provides clear migration path** (drop-in replacement for manual wrappers)
6. **Delivers measurable value** (70%+ automatic coverage)

The phased approach allows for **early wins** (Milestone 1-2) while building toward the **complete vision** (Milestone 4-5), with clear success criteria at each stage.

**End Goal:** A thriving ecosystem of companion crates where `cargo add elicit_<crate>` instantly provides MCP tool integration for any Rust library, with users enjoying familiar type names and transparent AI agent capabilities.
