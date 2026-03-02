# elicitation_macros

Procedural macros that power the [`elicitation`] framework's tool-generation,
tracing instrumentation, and newtype-wrapping infrastructure.

This crate is re-exported by the main `elicitation` crate — users import the
macros from there, not directly from this crate.

## Macros at a glance

| Macro | Kind | Purpose |
|---|---|---|
| `#[instrumented_impl]` | attribute on `impl` | Auto-adds `#[tracing::instrument]` to every method |
| `#[elicit_tools(T1, T2, …)]` | attribute on `impl` | Generates MCP elicitation tool methods for listed types |
| `#[elicit_trait_tools_router]` | attribute on `impl` | Wraps trait methods as individually discoverable MCP tools |
| `elicit_safe!` | declarative | Marks a type as safe to surface in MCP schemas |
| `elicit_newtype!` | declarative | Creates an Arc-wrapped newtype over an existing type |
| `reflect_methods!` | declarative | Generates MCP tool boilerplate from method signatures |
| `contract_type` | attribute on type | Attaches `requires`/`ensures` verification metadata |

---

## `#[instrumented_impl]`

Adds `#[tracing::instrument]` to every public method in an `impl` block,
choosing the right instrumentation level automatically:

| Method name pattern | Instrumentation |
|---|---|
| `new`, `from_*`, `try_*`, `default` | `#[instrument(ret, err)]` — logs return value and errors |
| `get`, `as_*`, `to_*`, `into_inner` | `#[instrument(level = "trace", ret)]` — low-noise accessor tracing |
| everything else | `#[instrument(skip(self))]` — spans without logging `self` |

```rust
use elicitation::instrumented_impl;

#[instrumented_impl]
impl Config {
    pub fn new(host: String, port: u16) -> Self { … }
    pub fn host(&self) -> &str { … }
    pub fn validate(&self) -> Result<(), ConfigError> { … }
}
// Expands to:
// new          → #[instrument(ret, err)]
// host         → #[instrument(level = "trace", ret)]
// validate     → #[instrument(skip(self))]
```

Becomes a no-op under `#[cfg(kani)]` so formal verification harnesses are
not disrupted by tracing instrumentation.

---

## `#[elicit_tools(T1, T2, …)]`

Generates a standalone MCP elicitation endpoint for each listed type. Place
it above `#[tool_router]` on a server `impl` block:

```rust
use elicitation::{elicit_tools, Elicit};
use rmcp::{tool_router, ServerHandler};

#[elicit_tools(ServerConfig, UserCredentials)]
#[tool_router]
impl MyServer {
    // Your other #[tool] methods here
}
```

For each type `T`, this generates:

```rust
#[tool(description = "Elicit ServerConfig via MCP")]
pub async fn elicit_server_config(
    &self,
    peer: Peer<RoleServer>,
) -> Result<Json<ElicitToolOutput<ServerConfig>>, ErrorData> {
    // drives the sampling conversation then returns the result
}
```

The `ElicitToolOutput<T>` wrapper ensures the MCP schema is always an object
(never a bare enum or primitive), which rmcp requires.

---

## `#[elicit_trait_tools_router(TraitName, field, [method1, method2, …])]`

Delegates trait methods to an inner field and registers each as an MCP tool.
Useful when you implement a trait on a server struct and want every method
individually callable:

```rust
use elicitation::elicit_trait_tools_router;

#[elicit_trait_tools_router(HttpClient, client, [get, post, put, delete])]
impl MyServer {
    // client: Box<dyn HttpClient>
}
```

For each method `foo`, the macro generates:
- A `FooParams` struct (deriving `Elicit` and `JsonSchema`)
- A `foo_tool` method decorated with `#[tool]`
- Delegation: `self.client.foo(params.into())`

---

## `elicit_safe!`

Marks a type as safe to surface in MCP schemas — used by the framework to
confirm that a type can appear as a tool input or output without violating
MCP schema constraints.

```rust
use elicitation::elicit_safe;

elicit_safe!(MyNewtype);
```

Generates a marker trait implementation that the framework checks at
compile time before registering a type with the MCP tool registry.

---

## `elicit_newtype!`

Creates a newtype wrapper that places the inner value behind `Arc`, making
consuming builder types (like `reqwest::RequestBuilder`) `Clone`-able across
async and MCP tool boundaries:

```rust
use elicitation::elicit_newtype;

elicit_newtype!(pub struct RequestBuilder(reqwest::RequestBuilder));
elicit_newtype!(pub struct Response(reqwest::Response));
```

Generated code:
- `struct RequestBuilder(Arc<reqwest::RequestBuilder>)`
- `impl Clone for RequestBuilder` — clones the `Arc`
- `impl Deref / DerefMut` — transparent access to the inner type
- `impl From<reqwest::RequestBuilder> for RequestBuilder`
- `elicit_safe!(RequestBuilder)` — MCP schema safety marker

This is the foundation of the [`elicit_reqwest`] crate, where all nine
reqwest/http/url types are wrapped this way.

---

## `reflect_methods!`

Reads the method signatures in an `impl` block and generates the MCP tool
boilerplate for each one — parameter structs with JSON schemas, tool wrapper
methods, and type conversions — without any hand-written duplication:

```rust
use elicitation::reflect_methods;

elicit_newtype!(pub struct Client(reqwest::Client));

#[reflect_methods]
impl Client {
    pub async fn get(&self, url: Url) -> RequestBuilder { … }
    pub async fn post(&self, url: Url) -> RequestBuilder { … }
}
```

For each method `foo(param: ParamType) -> ReturnType`, the macro generates:

```rust
// 1. Parameter struct
#[derive(Elicit, JsonSchema)]
pub struct FooParams { pub param: ParamType }

// 2. Tool wrapper
#[tool(description = "foo")]
pub async fn foo_tool(
    &self,
    params: Parameters<FooParams>,
) -> Result<Json<ReturnType>, ErrorData> {
    self.foo(params.param).await.map(Json).map_err(Into::into)
}
```

Type conversions applied automatically:
- `&str` → `String`
- `&[T]` → `Vec<T>`
- `&T` → `T` (requires `Clone`)

Methods returning `Self` (consuming builders) are skipped — they cannot be
wrapped meaningfully as stateless tool calls.

---

## `contract_type`

An attribute macro that attaches `requires` and `ensures` expressions to a
type as const-fn methods, making the verification metadata queryable at
compile time and readable by the `TypeSpec` explorer at runtime:

```rust
use elicitation::contract_type;

#[contract_type(requires = "value > 0", ensures = "result.get() > 0")]
pub struct I32Positive(i32);
```

Generates:
```rust
impl I32Positive {
    pub const fn __contract_requires() -> &'static str { "value > 0" }
    pub const fn __contract_ensures()  -> &'static str { "result.get() > 0" }
}
```

These are consumed by the `ElicitSpec` composition in `#[derive(Elicit)]`
to build the TypeSpec inventory entry for the type automatically.

---

## How the macros compose

A typical newtype MCP plugin is built layer by layer:

```
elicit_newtype!(Client)          ← 1. Arc-wrap, Clone, Deref
    ↓
#[reflect_methods] impl Client   ← 2. Generate param structs + tool wrappers
    ↓
#[instrumented_impl] impl Client ← 3. Add tracing spans to every method
    ↓
PluginRegistry::new()
    .register("http", ClientPlugin::new())  ← 4. Register with rmcp
```

For user-defined types the flow is simpler:

```
#[derive(Elicit, JsonSchema)]    ← 1. Generate Prompt/Survey/Select + elicit_checked()
    ↓
#[elicit_tools(MyType)]          ← 2. (optional) standalone tool endpoint
#[tool_router]
impl MyServer { … }              ← 3. rmcp discovers all #[tool] methods
```

## License

Licensed under either of [Apache License 2.0](../../LICENSE-APACHE) or
[MIT License](../../LICENSE-MIT) at your option.

[`elicitation`]: https://crates.io/crates/elicitation
[`elicit_reqwest`]: https://crates.io/crates/elicit_reqwest
