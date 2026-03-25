# elicitation_derive

Procedural macros for the `elicitation` framework, providing automatic trait implementations, MCP tool generation, method reflection, and contract-aware random value generation.

---

## Table of Contents

- [Derive Macros](#derive-macros)
  - [`#[derive(Elicit)]`](#deriveelicit)
  - [`#[derive(Rand)]`](#deriverand)
  - [`#[derive(ElicitPlugin)]`](#deriveelicitplugin)
  - [`#[derive(ElicitProxy)]`](#deriveelicitproxy)
- [Attribute Macros](#attribute-macros)
  - [`#[contract_type]`](#contract_type)
  - [`#[reflect_methods]`](#reflect_methods)
  - [`#[elicit_tool]`](#elicit_tool)
- [Complete Examples](#complete-examples)

---

## Derive Macros

### `#[derive(Elicit)]`

Automatically implements the `Elicitation` trait for enums and structs, enabling interactive value elicitation through the MCP protocol.

#### Requirements

**Must also derive `JsonSchema`** for MCP tool compatibility:

```rust
use elicitation::Elicit;
use schemars::JsonSchema;

#[derive(Debug, Clone, Elicit, JsonSchema)]
struct Config {
    host: String,
    port: u16,
}
```

#### What Gets Generated

1. **`Elicitation` trait implementation** - Async elicitation logic
2. **`elicit_checked()` MCP tool** - Decorated with `#[rmcp::tool]`
3. **Supporting traits**:
   - `Prompt` - Provides prompt text
   - `Select` (for enums) - Finite options pattern
   - `Survey` (for structs) - Multi-field elicitation

#### Enum Derivation (Select Pattern)

Supports three variant types:

**Unit Variants:**
```rust
#[derive(Elicit, JsonSchema)]
enum Role {
    System,
    User,
    Assistant,
}
```

**Tuple Variants:**
```rust
#[derive(Elicit, JsonSchema)]
enum MediaSource {
    Url(String),
    Base64(String),
    Binary(Vec<u8>),
}
```

**Struct Variants:**
```rust
#[derive(Elicit, JsonSchema)]
enum Input {
    Text(String),
    Image {
        mime: Option<String>,
        source: MediaSource,
    },
}
```

All three can coexist in the same enum.

#### Struct Derivation (Survey Pattern)

Sequential field-by-field elicitation:

```rust
#[derive(Elicit, JsonSchema)]
struct DeployConfig {
    #[prompt("Which environment?")]
    environment: Environment,

    #[prompt("Number of replicas (1–16):")]
    #[spec_requires("replicas >= 1 && replicas <= 16")]
    replicas: u8,

    #[skip]  // Omitted from elicitation
    _internal_id: u64,
}
```

#### Supported Attributes

- **`#[prompt("...")]`** - Custom prompt text (type or field level)
- **`#[skip]`** - Skip struct field during elicitation
- **`#[spec_summary("...")]`** - Type summary for documentation
- **`#[spec_requires("...")]`** - Field-level precondition

---

### `#[derive(Rand)]`

Generates contract-aware random value generation implementations.

#### Basic Usage

```rust
use elicitation_rand::Rand;

#[derive(Rand)]
#[rand(bounded(1, 100))]
struct Score(u32);

// Usage:
let generator = Score::random_generator(42);  // Seeded
let score = generator.generate();
```

#### Supported Contracts

**Range Contracts:**
```rust
#[rand(bounded(1, 100))]     // Values in [1, 100)
#[rand(bounded(-50, 50))]    // Negative ranges supported
```

**Simple Contracts:**
```rust
#[rand(positive)]  // Positive values only
#[rand(nonzero)]   // Non-zero values
#[rand(even)]      // Even numbers only
#[rand(odd)]       // Odd numbers only
```

**Composite Contracts:**
```rust
#[rand(and(positive, even))]           // Positive even numbers
#[rand(or(positive, negative))]        // Either positive or negative
#[rand(and(bounded(1, 100), even))]    // Even numbers in [1, 100)
```

**Nested Contracts:**
```rust
#[rand(or(
    and(positive, bounded(1, 50)),
    and(negative, bounded(-100, -1))
))]
```

#### Field-Level Contracts

```rust
#[derive(Rand)]
struct GameState {
    #[rand(bounded(1, 6))]
    dice_roll: u32,

    #[rand(positive)]
    score: i32,

    #[rand(and(bounded(0, 100), even))]
    health: u8,
}
```

#### Generated API

```rust
impl TypeName {
    // Seeded generator for reproducible output
    pub fn random_generator(seed: u64) -> impl Generator<Target = Self>;
}

impl Generator for TypeName {
    type Target = Self;
    fn generate(&self) -> Self;
}
```

---

### `#[derive(ElicitPlugin)]`

Automatically implements the `ElicitPlugin` trait using the `inventory` crate for compile-time tool registration.

#### Basic Usage

```rust
use elicitation_derive::ElicitPlugin;

#[derive(ElicitPlugin)]
#[plugin(name = "my_plugin")]
pub struct MyPlugin;
```

#### Struct Shapes

**Unit Structs** (ephemeral context):
```rust
#[derive(ElicitPlugin)]
#[plugin(name = "my_plugin")]
pub struct MyPlugin;
// Creates new PluginContext for each call
```

**Newtype Pattern** (shared context):
```rust
#[derive(ElicitPlugin)]
#[plugin(name = "database")]
pub struct DatabasePlugin(Arc<PluginContext>);
// Reuses shared context (connection pools, caches)
```

#### Integration with `#[elicit_tool]`

```rust
#[derive(ElicitPlugin)]
#[plugin(name = "network")]
pub struct NetworkPlugin;

#[elicit_tool(
    plugin = "network",
    name = "ping",
    description = "Ping a host"
)]
async fn ping(params: PingParams) -> Result<CallToolResult, ErrorData> {
    // Implementation
}
```

Tools are automatically discovered and registered via `inventory`.

---

### `#[derive(ElicitProxy)]`

Generates the identity `ElicitProxy` implementation where `type Proxy = Self`.

Use this on types that already satisfy `Serialize + DeserializeOwned + JsonSchema`.

```rust
use elicitation_derive::ElicitProxy;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit, ElicitProxy)]
pub struct MyConfig {
    pub name: String,
    pub value: i32,
}
```

Generates:
```rust
impl ElicitProxy for MyConfig {
    type Proxy = MyConfig;
    fn into_proxy(self) -> MyConfig { self }
    fn from_proxy(proxy: MyConfig) -> MyConfig { proxy }
}
```

---

## Attribute Macros

### `#[contract_type]`

Annotates types with formal verification contract metadata (preconditions and postconditions).

#### Usage

```rust
use elicitation_derive::contract_type;

#[contract_type(
    requires = "value > 0",
    ensures = "result.get() > 0"
)]
pub struct I8Positive(i8);
```

#### Generated Code

```rust
impl I8Positive {
    #[doc(hidden)]
    pub const fn __contract_requires() -> &'static str {
        "value > 0"
    }

    #[doc(hidden)]
    pub const fn __contract_ensures() -> &'static str {
        "result.get() > 0"
    }
}
```

#### Integration with `#[derive(Elicit)]`

Contract metadata is queried at compile time to compose verification conditions:

```rust
#[derive(Elicit)]
pub struct User {
    name: StringNonEmpty,  // has contract metadata
    age: I8Positive,       // has contract metadata
}

// May generate verification conditions combining field contracts
```

#### Use Cases

**Newtype Validation:**
```rust
#[contract_type(requires = "value >= 1024 && value <= 65535")]
pub struct PortNumber(u16);
```

**Domain Constraints:**
```rust
#[contract_type(
    requires = "email.contains('@')",
    ensures = "result.is_normalized()"
)]
pub struct Email(String);
```

---

### `#[reflect_methods]`

Automatically discovers public methods in impl blocks and generates MCP tool wrappers with parameter structs.

#### Basic Usage

```rust
use elicitation_derive::reflect_methods;

#[reflect_methods]
impl Client {
    pub async fn get(&self, url: &str) -> Result<Response, Error> {
        self.0.get(url).await
    }
}
```

#### What Gets Generated

**1. Parameter Struct:**
```rust
#[derive(Debug, Clone, Elicit, JsonSchema)]
pub struct GetParams {
    pub url: String,  // &str converted to String
}
```

**2. Wrapper Method:**
```rust
impl Client {
    #[tool(description = "Get resource from URL")]
    pub async fn get_tool(
        &self,
        params: Parameters<GetParams>,
    ) -> Result<Json<Response>, ErrorData> {
        self.0.get(params.url.as_str())
            .await
            .map(Json)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }
}
```

#### Type Conversions

Automatic conversions for MCP compatibility:

- `&str` → `String` (owned string for serialization)
- `&[T]` → `Vec<T>` (owned vector)
- `&T` → `T` (requires Clone, warns for large types)

#### Generic Methods

Full support with proper bound enforcement:

```rust
#[reflect_methods]
impl Container {
    pub fn fetch<T>(&self, id: &str) -> Result<T, Error>
    where
        T: Elicitation + JsonSchema,
    {
        // Implementation
    }
}

// Generates:
#[derive(Elicit, JsonSchema)]
pub struct FetchParams<T: Elicitation + JsonSchema> {
    pub id: String,
}
```

#### Configuration

```rust
#[reflect_methods(
    warn_clone_threshold = 1024,  // Warn for clones > 1KB
    allow_large_clones = false,   // Show warnings (default)
)]
impl Client { }
```

---

### `#[elicit_tool]`

Generates `ToolDescriptor` companion functions for async tool handlers and optionally registers them with plugins.

#### Basic Usage

```rust
use elicitation_derive::elicit_tool;

#[derive(Deserialize, JsonSchema)]
struct PingParams { message: String }

#[elicit_tool(name = "ping", description = "Echo a message")]
async fn ping(params: PingParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(params.message)]))
}

// Generates: pub fn ping_descriptor() -> ToolDescriptor { ... }
```

#### Plugin Registration

```rust
#[elicit_tool(
    plugin = "network",
    name = "fetch",
    description = "Fetch URL content"
)]
async fn fetch_url(params: FetchParams) -> Result<CallToolResult, ErrorData> {
    // Implementation
}
```

Automatically registers with the plugin via `inventory::submit!`.

#### Context-Aware Tools

```rust
#[elicit_tool(
    name = "secure_fetch",
    description = "Fetch with authentication",
    plugin = "network"
)]
async fn secure_fetch(
    ctx: Arc<PluginContext>,
    params: SecureFetchParams
) -> Result<CallToolResult, ErrorData> {
    let client = ctx.http_client();
    client.get(&params.url).send().await
}
```

#### Code Emission (Advanced)

Generate executable Rust code from tool handlers:

```rust
#[elicit_tool(
    name = "query",
    description = "Execute SQL query",
    emit_ctx("ctx.db_url" => r#"std::env::var("DATABASE_URL").unwrap()"#)
)]
async fn execute_query(
    ctx: Arc<PluginContext>,
    params: QueryParams
) -> Result<CallToolResult, ErrorData> {
    // Implementation emits sqlx::query!() code
}
```

With `feature = "emit"`, generates:

```rust
#[cfg(feature = "emit")]
impl EmitCode for QueryParams {
    fn emit_code(&self) -> TokenStream { /* ... */ }
    fn crate_deps(&self) -> Vec<CrateDep> { /* ... */ }
}
```

#### Custom Emit Implementation

```rust
struct CustomEmit;

#[elicit_tool(
    name = "fetch",
    description = "Custom fetch",
    emit = CustomEmit
)]
async fn fetch(params: FetchParams) -> Result<...> { ... }
```

---

## Complete Examples

### Shadow Crate Pattern

```rust
use elicitation::{Elicit, elicit_newtype};
use elicitation_derive::{ElicitPlugin, reflect_methods, elicit_tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Newtype wrapper for third-party type
elicit_newtype!(::reqwest::Client, as Client);

// Auto-generate MCP tools for methods
#[reflect_methods]
impl Client {
    pub async fn get(&self, url: &str) -> Result<Response, Error> {
        self.0.get(url).send().await
    }

    pub async fn post(&self, url: &str, body: Vec<u8>) -> Result<Response, Error> {
        self.0.post(url).send(body).await
    }
}

// Plugin with custom tools
#[derive(ElicitPlugin)]
#[plugin(name = "http")]
pub struct HttpPlugin;

#[derive(Deserialize, JsonSchema)]
struct FetchParams {
    url: String,
}

#[elicit_tool(
    plugin = "http",
    name = "fetch_json",
    description = "Fetch JSON from URL"
)]
async fn fetch_json(params: FetchParams) -> Result<CallToolResult, ErrorData> {
    // Custom implementation
}
```

### Contract-Verified Configuration

```rust
use elicitation_derive::{contract_type, Elicit};
use schemars::JsonSchema;

#[contract_type(requires = "value >= 1024 && value <= 65535")]
pub struct PortNumber(u16);

#[contract_type(requires = "value.len() > 0")]
pub struct StringNonEmpty(String);

#[derive(Elicit, JsonSchema)]
pub struct ServerConfig {
    #[prompt("Server hostname:")]
    host: StringNonEmpty,

    #[prompt("Port number (1024-65535):")]
    port: PortNumber,

    #[prompt("Enable TLS?")]
    tls_enabled: bool,
}
```

### Random Generation with Contracts

```rust
use elicitation_rand::{Rand, Generator};

#[derive(Rand)]
struct GameConfig {
    #[rand(bounded(1, 6))]
    difficulty: u8,

    #[rand(and(bounded(10, 100), even))]
    starting_health: u32,

    #[rand(positive)]
    score_multiplier: i32,
}

#[test]
fn test_random_config() {
    let generator = GameConfig::random_generator(42);
    let config = generator.generate();

    assert!(config.difficulty >= 1 && config.difficulty <= 6);
    assert!(config.starting_health >= 10 && config.starting_health <= 100);
    assert!(config.starting_health % 2 == 0);
    assert!(config.score_multiplier > 0);
}
```

### Full Plugin Example

```rust
use elicitation_derive::{ElicitPlugin, elicit_tool};
use std::sync::Arc;

// Plugin definition
#[derive(ElicitPlugin)]
#[plugin(name = "math")]
pub struct MathPlugin(Arc<PluginContext>);

// Tool 1: Context-free
#[elicit_tool(
    plugin = "math",
    name = "add",
    description = "Add two numbers"
)]
async fn add(params: AddParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text((params.a + params.b).to_string())
    ]))
}

// Tool 2: Context-aware
#[elicit_tool(
    plugin = "math",
    name = "compute",
    description = "Complex computation"
)]
async fn compute(
    ctx: Arc<PluginContext>,
    params: ComputeParams
) -> Result<CallToolResult, ErrorData> {
    // Use ctx for shared resources
}

#[derive(Deserialize, JsonSchema)]
struct AddParams {
    a: i32,
    b: i32,
}

#[derive(Deserialize, JsonSchema)]
struct ComputeParams {
    expression: String,
}
```

---

## Feature Flags

- **`emit`** - Enable code emission functionality in `#[elicit_tool]`
- **`proofs`** - Enable proof generation in `#[derive(Elicit)]`

---

## Related Crates

- **`elicitation`** - Core library with traits and runtime
- **`elicitation_macros`** - Additional procedural macros (`#[reflect_trait]`)
- **`elicitation_rand`** - Random generation framework
- **`elicit_*`** - Shadow crates for third-party libraries

---

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
