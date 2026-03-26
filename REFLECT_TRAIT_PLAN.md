# `#[reflect_trait]` — Trait Wrapping and Dynamic Tool Factories

## What We Are Actually Building

Third-party crates define traits like `diesel::Insertable`, `serde::Serialize`,
or `clap::ValueEnum`. Types implementing those traits get the trait's methods
for free — but those methods are invisible to MCP because they live in the
upstream crate's impl, not in any `impl` block we own.

**Goal:** capture a third-party trait's methods as MCP tools, available to
agents for any concrete type that implements the trait, without requiring the
user to write per-type boilerplate.

This is fundamentally different from `#[reflect_methods]`, which wraps methods
the user writes. Here we wrap methods that already exist upstream.

---

## The Three-Time Model

```text
COMPILE TIME    inventory::submit! → ToolFactoryRegistration
                    (macro generates factory struct + submission)

STARTUP TIME    server.register_type::<diesel::User>("user")
                    (user tells registry which concrete types exist)
                    (monomorphization happens here — closures capture T)

REQUEST TIME    agent calls "instantiate_insertable" { prefix: "user" }
                    (factory creates DynamicToolDescriptors for T="user")
                    (notify_tool_list_changed fires)
                    (agent re-calls list_tools, sees user_insert etc.)
```

The agent is oblivious to which time any tool was created. From its perspective:
it sees a factory meta-tool, calls it, gets new tools. No redeployment needed.

---

## MCP Protocol Support — Already There

rmcp 0.15 already has everything we need:

```rust
// In rmcp::service::server.rs — already exists:
method!(peer_not notify_tool_list_changed ToolListChangedNotification);

// In our capabilities builder — already exists:
ServerCapabilitiesBuilder::default()
    .enable_tool_list_changed()  // sets listChanged: true in capabilities
```

Clients that see `listChanged: true` in server capabilities know to refresh
their tool list when `notifications/tools/list_changed` arrives. Claude Desktop,
Cursor, and the official MCP Inspector all support this.

---

## Core Types

### `DynamicToolDescriptor`

Like a static `ToolDescriptor` but with a runtime dispatch handler:

```rust
/// A tool whose handler was created at runtime via a factory.
pub struct DynamicToolDescriptor {
    /// MCP tool name (e.g. "user__insert")
    pub name: String,
    /// Description shown to the agent
    pub description: String,
    /// JSON Schema for the tool's parameters, generated at registration time
    pub schema: schemars::Schema,
    /// Type-erased handler — all type safety is enforced at registration via bounds
    pub handler: Arc<dyn Fn(serde_json::Value) -> BoxFuture<'static, Result<CallToolResult, ErrorData>> + Send + Sync>,
}
```

The bounds (`T: Serialize + Deserialize + JsonSchema + Elicit + Send + Sync + 'static`)
are enforced when the closure is *created* inside `register_type::<T>()`.
By the time it reaches `DynamicToolDescriptor`, it's already type-safe — no
`Box<dyn Any>` anywhere in the hot path.

### `AnyToolSlot`

A type-erased but bounds-checked container for a concrete type `T`.
Created at startup inside `register_type::<T>()`, holds the vtable for
calling trait methods on `T`.

```rust
/// Object-safe wrapper holding the dispatch vtable for one registered type.
///
/// Created once per `register_type::<T>()` call. The factory uses it to
/// generate DynamicToolDescriptors without knowing T at factory-definition time.
pub trait AnyToolSlot: Send + Sync + 'static {
    /// Type name as provided by the user (e.g. "user", "post")
    fn prefix(&self) -> &str;
    /// Rust type name for diagnostics
    fn type_name(&self) -> &'static str;
    /// JSON schema for the type's fields (from JsonSchema bound)
    fn schema(&self) -> schemars::Schema;
}

/// Concrete slot for a known T — only constructable when bounds are satisfied
pub struct TypedSlot<T>
where
    T: Serialize + DeserializeOwned + JsonSchema + Elicit + Send + Sync + 'static,
{
    prefix: String,
    _phantom: PhantomData<T>,
}
```

### `AnyToolFactory`

Object-safe trait submitted to inventory. Each factory knows the method
shapes of one third-party trait and can produce tools for any `AnyToolSlot`.

```rust
/// Object-safe factory — no generics, works as a trait object in inventory.
pub trait AnyToolFactory: Send + Sync + 'static {
    /// The third-party trait this factory wraps (for display and meta-tool naming)
    fn trait_name(&self) -> &'static str;

    /// Human-readable description for the factory meta-tool
    fn factory_description(&self) -> &'static str;

    /// Names of methods this factory can produce tools for
    fn method_names(&self) -> &'static [&'static str];

    /// Produce DynamicToolDescriptors for one registered slot.
    ///
    /// Called when an agent requests tool instantiation.
    /// The factory downcasts the slot to its expected concrete type.
    fn instantiate(
        &self,
        slot: &dyn AnyToolSlot,
    ) -> Result<Vec<DynamicToolDescriptor>, ErrorData>;
}

/// Inventory key for factory discovery at runtime
pub struct ToolFactoryRegistration {
    pub trait_name: &'static str,
    pub factory: &'static dyn AnyToolFactory,
}
inventory::collect!(ToolFactoryRegistration);
```

### `DynamicToolRegistry`

The middleware layer. Sits between inventory and `PluginRegistry`.
Implements `ElicitPlugin` so it drops into the existing plugin system.

```rust
pub struct DynamicToolRegistry {
    /// Factories discovered from inventory at construction
    factories: Vec<&'static dyn AnyToolFactory>,

    /// Type slots registered at startup — keyed by prefix
    /// Arc<RwLock> because register_type can be called concurrently at startup
    slots: Arc<RwLock<HashMap<String, Box<dyn AnyToolSlot>>>>,

    /// Live dynamic tools — populated when agent calls a factory meta-tool
    dynamic_tools: Arc<RwLock<Vec<DynamicToolDescriptor>>>,

    /// rmcp peer handle for sending notify_tool_list_changed
    /// Set when the registry is attached to a server
    peer: Arc<OnceLock<rmcp::service::Peer<RoleServer>>>,
}

impl DynamicToolRegistry {
    /// Collect all factories from inventory
    pub fn new() -> Self { ... }

    /// Called at server startup — registers a concrete type T with a prefix.
    /// Monomorphization happens here: closures are generated capturing T's vtable.
    pub fn register_type<T>(&self, prefix: impl Into<String>) -> &Self
    where
        T: Serialize + DeserializeOwned + JsonSchema + Elicit + Send + Sync + 'static,
    { ... }

    /// Called by a factory meta-tool at request time.
    /// Creates DynamicToolDescriptors for the given prefix and fires list_changed.
    async fn instantiate(&self, trait_name: &str, prefix: &str) -> Result<CallToolResult, ErrorData>
    { ... }
}

impl ElicitPlugin for DynamicToolRegistry {
    fn name(&self) -> &'static str { "dynamic" }

    fn list_tools(&self) -> Vec<Tool> {
        // factory meta-tools (always present) + instantiated dynamic tools
        self.factory_meta_tools()
            .chain(self.dynamic_tools.read().iter().map(|d| d.as_tool()))
            .collect()
    }

    fn call_tool(&self, params, ctx) -> BoxFuture<...> {
        // Route to: factory meta-tool handler, or dynamic tool handler
    }
}
```

---

## The Factory Meta-Tool

For each factory submitted to inventory, one meta-tool is automatically
registered in `DynamicToolRegistry::list_tools()`. Example for an
`InsertableToolFactory`:

```text
Tool name:    "dynamic__instantiate_insertable"
Description:  "Create insert/batch_insert tools for a registered type.
               Call register_type::<T>(prefix) at startup to make T available."
Parameters:   { "prefix": "string — the name used in register_type::<T>(prefix)" }
```

When the agent calls this tool:

1. `instantiate("diesel::Insertable", "user")` runs
2. Finds the `TypedSlot<User>` registered under prefix `"user"`
3. Calls `InsertableToolFactory::instantiate(slot)` → Vec<DynamicToolDescriptor>
4. Adds descriptors to `dynamic_tools`
5. Calls `peer.notify_tool_list_changed()`
6. Agent re-calls `list_tools`, sees `user__insert`, `user__batch_insert`, etc.

---

## The `#[reflect_trait]` Macro

Lives in `elicitation_macros` (attribute macro, per project convention).
Applied to an impl block that names the third-party trait and lists the
method signatures to capture:

```rust
#[reflect_trait(diesel::Insertable)]
impl InsertableTools {
    fn insert(&self) -> QueryResult<usize>;
    fn batch_insert(items: Vec<Self>) -> QueryResult<usize>;
}
```

**What the macro generates:**

```rust
// 1. The AnyToolFactory implementation
pub struct InsertableToolFactory;

impl AnyToolFactory for InsertableToolFactory {
    fn trait_name(&self) -> &'static str { "diesel::Insertable" }
    fn factory_description(&self) -> &'static str {
        "Tools for types implementing diesel::Insertable"
    }
    fn method_names(&self) -> &'static [&'static str] {
        &["insert", "batch_insert"]
    }

    fn instantiate(&self, slot: &dyn AnyToolSlot) -> Result<Vec<DynamicToolDescriptor>, ErrorData> {
        // Downcast to get the concrete type's dispatch — via a registered vtable
        let vtable = slot.vtable::<InsertableVTable>()
            .ok_or_else(|| ErrorData::invalid_params("type not registered for Insertable", None))?;

        let prefix = slot.prefix().to_string();

        Ok(vec![
            DynamicToolDescriptor {
                name: format!("{prefix}__insert"),
                description: "Insert one record".to_string(),
                schema: vtable.insert_schema.clone(),
                handler: Arc::new({
                    let dispatch = vtable.insert.clone();
                    move |params| {
                        let dispatch = dispatch.clone();
                        Box::pin(async move { dispatch(params).await })
                    }
                }),
            },
            // ... one per method
        ])
    }
}

// 2. A vtable struct that captures T's method impls as boxed closures
struct InsertableVTable {
    insert_schema: schemars::Schema,
    insert: Arc<dyn Fn(serde_json::Value) -> BoxFuture<...> + Send + Sync>,
    batch_insert_schema: schemars::Schema,
    batch_insert: Arc<dyn Fn(serde_json::Value) -> BoxFuture<...> + Send + Sync>,
}

impl InsertableVTable {
    // Called inside register_type::<T>() — monomorphization happens here
    fn for_type<T>() -> Self
    where
        T: diesel::Insertable + Serialize + DeserializeOwned + JsonSchema + Send + Sync + 'static,
    {
        Self {
            insert_schema: schema_for::<InsertParams>(),
            insert: Arc::new(|params| Box::pin(async move {
                // T is captured in monomorphized code
                let result = T::insert(/* deserialize params */)?;
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string(&result)?
                )]))
            })),
            // ...
        }
    }
}

// 3. inventory submission
inventory::submit!(ToolFactoryRegistration {
    trait_name: "diesel::Insertable",
    factory: &InsertableToolFactory,
});
```

---

## Integration with PluginRegistry

`DynamicToolRegistry` implements `ElicitPlugin`, so it registers like any other plugin:

```rust
// Server setup (user code)
let dynamic = DynamicToolRegistry::new()
    .register_type::<diesel::User>("user")
    .register_type::<diesel::Post>("post");

let registry = PluginRegistry::new()
    .register("http",    elicit_reqwest::Plugin::new(client))
    .register("db",      elicit_diesel::Plugin::new(pool))
    .register("dynamic", dynamic);  // ← drops in here

registry.serve(rmcp::transport::stdio()).await?;
```

The agent sees `dynamic__instantiate_insertable` in `list_tools` immediately.
After calling it with `{ "prefix": "user" }`, it sees `dynamic__user__insert`.

---

## Module Layout

### New in `crates/elicitation/src/`

```text
dynamic/
├── mod.rs                  ← DynamicToolRegistry, DynamicToolDescriptor
├── slot.rs                 ← AnyToolSlot, TypedSlot<T>
├── factory.rs              ← AnyToolFactory, ToolFactoryRegistration
└── meta_tool.rs            ← factory meta-tool generation
```

Exported from `lib.rs`:

```rust
pub use dynamic::{
    AnyToolFactory, AnyToolSlot, DynamicToolDescriptor,
    DynamicToolRegistry, ToolFactoryRegistration,
};
```

### New in `crates/elicitation_macros/src/`

```text
trait_reflection/
├── mod.rs          ← expand() entry point, parses impl block
├── vtable.rs       ← VTable struct generation per method
├── factory.rs      ← AnyToolFactory impl generation
├── naming.rs       ← type path → snake_case tool prefix
└── params.rs       ← param struct generation (adapt from method_reflection)
```

---

## Milestones

### Milestone 1 — `DynamicToolRegistry` (no macro)

Build the runtime layer independently. Manually construct a
`DynamicToolDescriptor` in a test to prove the wiring works end-to-end:
`register_type::<T>()` → `instantiate()` → `notify_tool_list_changed` → tools visible.

Deliverables:

- `crates/elicitation/src/dynamic/` module (4 files)
- `DynamicToolRegistry` implements `ElicitPlugin`
- `PluginRegistry::register` accepts it
- `enable_tool_list_changed()` set in server capabilities
- Integration test: manually register a fake type, call meta-tool, verify new tools appear

### Milestone 2 — `#[reflect_trait]` macro

Deliverables:

- `crates/elicitation_macros/src/trait_reflection/` module
- `pub fn reflect_trait(attr, item)` in `lib.rs`
- Unit tests for generated factory struct and vtable
- Integration test: `#[reflect_trait(SomeTrait)]` on a local trait, tools appear correctly

### Milestone 3 — Apply to elicit_clap Select types

Replace the 5 `elicit_clap` newtype files for Select enums. Each becomes:

```rust
#[reflect_trait(elicitation::Select)]
impl SelectTools for clap::ColorChoice {
    fn labels() -> Vec<String>;
    fn from_label(label: String) -> Option<clap::ColorChoice>;
    fn options() -> Vec<clap::ColorChoice>;
}
```

Verify: `just check-all elicit_clap` passes, tools still appear in list_tools.

### Milestone 4 — Update guides

- `THIRD_PARTY_SUPPORT_GUIDE.md`: add `#[reflect_trait]` as Phase 3b (trait wrapping)
- `REFLECT_TRAIT_PLAN.md` (this file): mark milestones complete as work progresses

---

## Checklist

### Milestone 1 — DynamicToolRegistry

- [ ] `crates/elicitation/src/dynamic/mod.rs` — `DynamicToolRegistry`, `DynamicToolDescriptor`
- [ ] `crates/elicitation/src/dynamic/slot.rs` — `AnyToolSlot`, `TypedSlot<T>`
- [ ] `crates/elicitation/src/dynamic/factory.rs` — `AnyToolFactory`, `ToolFactoryRegistration`
- [ ] `crates/elicitation/src/dynamic/meta_tool.rs` — factory meta-tool generation
- [ ] `lib.rs` exports wired
- [ ] `enable_tool_list_changed()` in server capabilities
- [ ] Integration test: manual factory → instantiate → list_tools shows new tools
- [ ] `just check-all elicitation` passes

### Milestone 2 — `#[reflect_trait]` macro

- [ ] `crates/elicitation_macros/src/trait_reflection/mod.rs`
- [ ] `crates/elicitation_macros/src/trait_reflection/vtable.rs`
- [ ] `crates/elicitation_macros/src/trait_reflection/factory.rs`
- [ ] `crates/elicitation_macros/src/trait_reflection/naming.rs`
- [ ] `crates/elicitation_macros/src/trait_reflection/params.rs`
- [ ] `lib.rs`: `pub fn reflect_trait()`
- [ ] Unit tests for generated code
- [ ] Integration test
- [ ] `just check-all elicitation_macros` passes

### Milestone 3 — elicit_clap

- [ ] Replace 5 Select enum files with `#[reflect_trait]` impls
- [ ] `just check-all elicit_clap` passes
- [ ] Tools still appear correctly in list_tools

### Milestone 4 — Docs

- [ ] `THIRD_PARTY_SUPPORT_GUIDE.md` updated
- [ ] This plan marked complete
