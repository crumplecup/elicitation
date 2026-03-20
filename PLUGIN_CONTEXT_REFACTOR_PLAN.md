# PluginContext Refactor Plan

## Problem

`PluginContext` is a monolithic concrete struct. Every shadow crate that needs
server-side state must add a feature-gated field to a central type in
`crates/elicitation/`. This creates O(N) coupling as N shadow crates grow.

```rust
// current — grows without bound
pub struct PluginContext {
    #[cfg(feature = "reqwest")]
    pub http: reqwest::Client,
    #[cfg(feature = "sqlx")]          // would have to add this
    pub pool: sqlx::any::AnyPool,     // and this
    // ... one field per crate forever
}
```

## Core insight

Object safety only matters at the registry boundary — `Arc<dyn ErasedElicitPlugin>`.
Everything above that is ordinary typed Rust. The design splits into three tiers:

1. **`Plugin`** — simple stateless trait; the easy path for the majority of plugins
2. **`StatefulPlugin`** — explicit opt-in for plugins that need server-side state;
   `type Context: PluginContext` carries the associated context type
3. **`ErasedElicitPlugin`** — object-safe; blanket impls from both tiers; registry only

Writing a normal stateless plugin requires no knowledge of contexts or generics.
Only plugins that actually hold state (`elicit_reqwest`, `elicit_sqlx`) implement
`StatefulPlugin`.

---

## Design

### `PluginContext` trait

```rust
/// Marker trait for plugin-specific server-side state.
///
/// Each stateful shadow crate defines its own context struct and implements
/// this trait. The context is held behind `Arc` and shared across all tool
/// calls on the same server instance.
pub trait PluginContext: Send + Sync + 'static {}
```

### `ToolDescriptor` — two flavors

Stateless plugins use the simple non-generic form. Stateful plugins use the
generic form. Both live in the same module.

```rust
// Stateless — no context parameter
pub struct ToolDescriptor {
    pub name: &'static str,
    pub(crate) tool: Tool,
    pub(crate) handler: Arc<
        dyn Fn(CallToolRequestParams)
            -> BoxFuture<'static, Result<CallToolResult, ErrorData>>
            + Send + Sync,
    >,
}

// Stateful — generic over context
pub struct StatefulToolDescriptor<Ctx: PluginContext> {
    pub name: &'static str,
    pub(crate) tool: Tool,
    pub(crate) handler: Arc<
        dyn Fn(Arc<Ctx>, CallToolRequestParams)
            -> BoxFuture<'static, Result<CallToolResult, ErrorData>>
            + Send + Sync,
    >,
}
```

Constructors:

```rust
pub fn make_descriptor<T, F>(name, description, handler: F) -> ToolDescriptor
pub fn make_descriptor_ctx<Ctx: PluginContext, T, F>(name, description, handler: F)
    -> StatefulToolDescriptor<Ctx>
```

### `Plugin` — stateless, simple to write

```rust
pub trait Plugin: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn descriptors(&self) -> &[ToolDescriptor];
}
```

This is the existing `DescriptorPlugin` pattern, kept exactly as easy to use.
No context, no generics, no boilerplate.

```rust
// Writing a stateless plugin — unchanged from today
pub struct MyPlugin;
impl Plugin for MyPlugin {
    fn name(&self) -> &'static str { "my_plugin" }
    fn descriptors(&self) -> &[ToolDescriptor] { &TOOLS }
}
```

### `StatefulPlugin` — explicit opt-in for server-side state

```rust
pub trait StatefulPlugin: Send + Sync + 'static {
    type Context: PluginContext;

    fn name(&self) -> &'static str;
    fn context(&self) -> Arc<Self::Context>;
    fn descriptors(&self) -> Vec<StatefulToolDescriptor<Self::Context>>;
}
```

```rust
// elicit_reqwest
pub struct HttpContext { pub http: reqwest::Client }
impl PluginContext for HttpContext {}

pub struct Plugin(pub Arc<HttpContext>);
impl StatefulPlugin for Plugin {
    type Context = HttpContext;
    fn context(&self) -> Arc<HttpContext> { self.0.clone() }
    fn descriptors(&self) -> Vec<StatefulToolDescriptor<HttpContext>> { ... }
}

// elicit_sqlx
pub struct SqlxContext {
    pub pool: sqlx::any::AnyPool,
    pub transactions: Mutex<HashMap<Uuid, Transaction<'static, Any>>>,
}
impl PluginContext for SqlxContext {}
```

### `ErasedElicitPlugin` — single type-erasure point

Both `Plugin` and `StatefulPlugin` erase here. The registry only ever
sees `Arc<dyn ErasedElicitPlugin>`.

```rust
pub trait ErasedElicitPlugin: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn list_tools(&self) -> Vec<Tool>;
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        ctx: RequestContext<RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>>;
}

// Stateless blanket impl
impl<P: Plugin> ErasedElicitPlugin for P {
    fn call_tool<'a>(&'a self, params: ...) -> BoxFuture<...> {
        let bare = strip_prefix(self.name(), &params.name);
        match self.descriptors().iter().find(|d| d.name == bare) {
            Some(d) => d.dispatch(params),    // no context needed
            None => Box::pin(async { Err(...) }),
        }
    }
}

// Stateful blanket impl — Ctx is concrete here, erased at the return type
impl<P: StatefulPlugin> ErasedElicitPlugin for P {
    fn call_tool<'a>(&'a self, params: ...) -> BoxFuture<...> {
        let ctx = self.context();             // Arc<P::Context> — concrete
        let bare = strip_prefix(self.name(), &params.name);
        match self.descriptors().into_iter().find(|d| d.name == bare) {
            Some(d) => d.dispatch(ctx, params),  // Ctx resolved, result erased
            None => Box::pin(async { Err(...) }),
        }
    }
}

pub type ArcPlugin = Arc<dyn ErasedElicitPlugin>;
```

---

## Touch points

| File | Change |
|---|---|
| `elicitation/src/plugin/context.rs` | Replace concrete struct with `pub trait PluginContext` |
| `elicitation/src/plugin/descriptor.rs` | Add `StatefulToolDescriptor<Ctx>`; `make_descriptor_ctx` becomes generic |
| `elicitation/src/plugin/mod.rs` | Add `StatefulPlugin`; rename `ElicitPlugin` → `ErasedElicitPlugin`; expose `Plugin` as the simple trait |
| `elicitation/src/plugin/descriptor_plugin.rs` | `DescriptorPlugin` becomes `Plugin`; remove now-unnecessary `type Context` |
| `elicitation/src/lib.rs` | Re-export `PluginContext`, `Plugin`, `StatefulPlugin`, `ErasedElicitPlugin`, `ArcPlugin` |
| `elicitation_derive/src/elicit_tool.rs` | `#[elicit_tool]` detects `Arc<*Context>` → emits `make_descriptor_ctx`; else `make_descriptor` |
| `elicitation_derive/src/derive_elicit_plugin.rs` | Detect `StatefulPlugin` newtype vs `Plugin` unit struct |
| `elicit_reqwest/src/plugins/http.rs` | `impl StatefulPlugin`; `HttpContext`; handlers `ctx: Arc<HttpContext>` |
| `elicit_reqwest/src/plugins/workflow.rs` | Same |
| All other `elicit_*` plugins | No change — already stateless `Plugin` impls |

---

## Phases

### Phase A — Core (`crates/elicitation/`)
- `PluginContext` → trait
- `StatefulToolDescriptor<Ctx>` alongside existing `ToolDescriptor`
- `StatefulPlugin` trait + `ErasedElicitPlugin` + both blanket impls
- `Plugin` replaces `DescriptorPlugin` (or keep as alias)
- `ArcPlugin = Arc<dyn ErasedElicitPlugin>`

### Phase B — `elicit_reqwest` migration
- `HttpContext { http: reqwest::Client }` implements `PluginContext`
- `Plugin(Arc<HttpContext>)` implements `StatefulPlugin`
- Handler signatures: `ctx: Arc<HttpContext>`

### Phase C — Derive macro updates
- `#[elicit_tool]`: detect `Arc<*Context>` first param → `make_descriptor_ctx`
- `#[derive(ElicitPlugin)]`: detect `StatefulPlugin` (newtype) vs `Plugin` (unit/descriptor)

### Phase D — Verify all other `elicit_*` crates
- Confirm they compile unchanged as stateless `Plugin` impls
- No handler changes expected

### Phase E — Update `ELICIT_SQLX_PLAN.md`
- Confirm `SqlxContext` / `StatefulPlugin` pattern matches

---

## Deferred

- Multiple contexts per plugin — compose: `struct CombinedCtx { http, db }` still one `type Context`
- Context sharing across plugins — pass same `Arc<Ctx>` at construction; no registry change needed
