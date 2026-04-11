# elicit_leptos — Reactive Web Framework MCP Tools

> **Target:** Leptos 0.8 (latest: 0.8.17)
> **Strategy:** StatefulPlugin for server-side reactive primitives + DescriptorPlugin for code generation.
> **Key insight:** Macros (`#[component]`, `view!`, `#[server]`) are code-generation targets — wrap as emit tools.
> Leptos's reactive primitives (`RwSignal`, `Memo`, `Effect`) work server-side inside an `Owner` scope.
> DOM/view rendering is browser-only — not attempted at runtime.

---

## Executive Summary

**Scope:** Leptos 0.8 reactive primitives (server-side) + complete code generation for all framework patterns
**Plugins:** 2 — `LeptosReactivePlugin` (StatefulPlugin) + `LeptosCodePlugin` (DescriptorPlugin)
**Estimated tools:** ~75
**Architecture decision:** DOM/view rendering is browser-only — not attempted at runtime. Reactive primitives
(`RwSignal`, `Memo`, `Effect`, `Owner`) run fine in a tokio/server context. Code generation covers 100%
of the macro surface.

### What runs at runtime (server-side, in `Owner` scope)
- `RwSignal<serde_json::Value>` — typed values via JSON, UUID-keyed
- `Memo` — derived computed values
- `Effect` — side-effect tracking
- `provide_context` / `use_context` — context propagation within an `Owner` tree
- `Action` — async operations via tokio

### What is code-generation only
- `#[component]` — emit tools generate correct `fn Foo(props) -> impl IntoView` source
- `view!` macro — emit tools generate RSX syntax blocks
- `#[server]` — emit tools generate server function stubs
- `#[island]` — emit tools generate island component source
- Routing, SSR, app scaffolding — all pure string/code generation

### Pattern mapping

| Pattern | Plugin | Guide section |
|---|---|---|
| StatefulPlugin (runtime state) | `LeptosReactivePlugin` | Phase 3 + StatefulPlugin |
| Descriptor-registry (code gen) | `LeptosCodePlugin` | Phase 3E |
| Macros as emit tools | `LeptosCodePlugin` | Phase 3E.5 + macro pattern |
| Closures as emit tools | `LeptosCodePlugin` | Phase 3E.5 (event handlers, effects) |

---


## Architecture: 2 Plugins, ~75 Tools

```
crates/elicit_leptos/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs               # mod + pub use only
    ├── error.rs             # LeptosError / LeptosErrorKind
    ├── reactive.rs          # LeptosReactivePlugin (StatefulPlugin)
    └── code.rs              # LeptosCodePlugin (DescriptorPlugin)
```

---

## Plugin 1: `LeptosReactivePlugin` (StatefulPlugin, ~22 tools)

### Context

```rust
use leptos::reactive_graph::owner::Owner;
use leptos::prelude::*;

pub struct LeptosReactiveContext {
    owner: Owner,
    signals: Mutex<HashMap<Uuid, RwSignal<serde_json::Value>>>,
    memos:   Mutex<HashMap<Uuid, (Memo<serde_json::Value>, String)>>, // (memo, op_name)
    actions: Mutex<HashMap<Uuid, ActionEntry>>,
    ctx_map: Mutex<HashMap<String, serde_json::Value>>,
}

pub struct ActionEntry {
    name: String,
    pending: bool,
    last_input:  Option<serde_json::Value>,
    last_output: Option<serde_json::Value>,
}

impl PluginContext for LeptosReactiveContext {}
```

The `owner` is created once at context construction with `Owner::new()`. All signals and
memos are created within that owner's scope (using `owner.with(|| ...)`) and live as long
as the context lives. Because `RwSignal` is `Send + Sync + 'static` in Leptos 0.7+, they
can be stored in a `Mutex<HashMap>` and accessed from any thread.

```rust
#[derive(ElicitPlugin)]
#[plugin(name = "leptos_reactive")]
pub struct LeptosReactivePlugin(pub Arc<LeptosReactiveContext>);
```

### Signal tools (8)

| Tool | Params | Description |
|---|---|---|
| `leptos_reactive__signal_new` | `name: String, value: Value` | Create `RwSignal<Value>`, return UUID |
| `leptos_reactive__signal_get` | `id` | Current value of the signal |
| `leptos_reactive__signal_set` | `id, value: Value` | Set the signal's value |
| `leptos_reactive__signal_update` | `id, op: String, operand?: Value` | Mutate: `"increment"`, `"decrement"`, `"append"`, `"clear"`, `"toggle"`, `"merge"` |
| `leptos_reactive__signal_describe` | `id` | Name, current value, creation time |
| `leptos_reactive__signal_list` | — | All stored signals with name + current value |
| `leptos_reactive__signal_delete` | `id` | Dispose signal, remove from registry |
| `leptos_reactive__signal_track` | `id` | Return change log (via attached Effect, buffered in context) |

### Memo tools (4)

Memos are derived values. Since we can't eval arbitrary expressions from strings, each
memo is defined by a **named transform** applied to a source signal.

| Tool | Params | Description |
|---|---|---|
| `leptos_reactive__memo_new` | `source_id, op: String` | Derive memo from signal: `"upper"`, `"lower"`, `"negate"`, `"to_number"`, `"to_bool"`, `"length"`, `"abs"`, `"not"` |
| `leptos_reactive__memo_get` | `id` | Current computed value |
| `leptos_reactive__memo_list` | — | All memos with source signal and op |
| `leptos_reactive__memo_delete` | `id` | Dispose and remove |

### Context tools (4)

`provide_context` / `use_context` equivalent — a key/value store scoped to the Owner.

| Tool | Params | Description |
|---|---|---|
| `leptos_reactive__ctx_provide` | `key: String, value: Value` | Store a value in reactive context |
| `leptos_reactive__ctx_get` | `key: String` | Retrieve a context value |
| `leptos_reactive__ctx_list` | — | All provided context values |
| `leptos_reactive__ctx_remove` | `key: String` | Remove a context value |

### Action tools (4)

Actions are imperatively-triggered async operations. In the MCP model, `action_dispatch`
stores the input, resolves immediately (synchronously), and records the output. Actual async
dispatch (calling server functions) is handled by the code plugin.

| Tool | Params | Description |
|---|---|---|
| `leptos_reactive__action_new` | `name: String` | Register a named action slot |
| `leptos_reactive__action_dispatch` | `id, input: Value` | Record input, mark pending; returns action entry |
| `leptos_reactive__action_resolve` | `id, output: Value` | Mark resolved, store output |
| `leptos_reactive__action_list` | — | All actions with pending/value state |

### Owner tools (2)

| Tool | Params | Description |
|---|---|---|
| `leptos_reactive__owner_reset` | — | Dispose all signals/memos/actions, create fresh Owner |
| `leptos_reactive__owner_status` | — | Count of signals, memos, actions |

---

## Plugin 2: `LeptosCodePlugin` (DescriptorPlugin, ~53 tools)

Pure code generation — no leptos runtime required, no WASM. All tools emit idiomatic
Leptos 0.8 source code as strings. Macros (`#[component]`, `view!`, `#[server]`,
`#[island]`) and closures (`on:click=move |_| { ... }`) are handled as emit targets
per the macro/closure patterns in the Third Party Support Guide.

### elicitation Primitives (feature: `leptos-types`)

```
crates/elicitation/src/primitives/leptos_types/
├── mod.rs
├── enums.rs        # LeptosMode, LeptosHtmlTag, LeptosBuiltinComponent
└── descriptors.rs  # LeptosComponentDescriptor, LeptosViewNode,
                    # LeptosPropDescriptor, LeptosRouteDescriptor,
                    # LeptosServerFnDescriptor, LeptosAppDescriptor
```

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema,
         strum::EnumIter, derive_more::Display, ToCodeLiteral)]
pub enum LeptosMode { Csr, Ssr, Hydrate, Islands }

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct LeptosPropDescriptor {
    pub name: String,
    pub ty: String,
    pub optional: bool,
    pub default_value: Option<String>,
    pub into: bool,             // #[prop(into)]
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct LeptosComponentDescriptor {
    pub name: String,
    pub props: Vec<LeptosPropDescriptor>,
    pub has_children: bool,
    pub island: bool,
    pub body: String,           // view! body (or arbitrary expression)
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct LeptosViewNode {
    pub tag: String,            // "div", "Show", "For", "button", etc.
    pub attrs: Vec<(String, String)>,
    pub on_events: Vec<(String, String)>,  // ("click", "move |_| { ... }")
    pub children: Vec<LeptosViewNode>,
    pub text: Option<String>,   // text node
    pub reactive_expr: Option<String>, // {move || signal.get()}
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct LeptosRouteDescriptor {
    pub path: String,
    pub view: String,           // component name
    pub nested: Vec<LeptosRouteDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct LeptosAppDescriptor {
    pub package_name: String,
    pub mode: LeptosMode,
    pub components: Vec<LeptosComponentDescriptor>,
    pub routes: Vec<LeptosRouteDescriptor>,
}
```

### Component tools (8)

| Tool | Params | Description |
|---|---|---|
| `leptos_code__component_new` | `name: String` | Create a `LeptosComponentDescriptor`, return UUID |
| `leptos_code__component_add_prop` | `id, prop: LeptosPropDescriptor` | Add a prop to descriptor |
| `leptos_code__component_set_body` | `id, body: String` | Set view! body |
| `leptos_code__component_set_island` | `id, island: bool` | Toggle `#[island]` |
| `leptos_code__component_emit` | `id` | Emit complete `#[component]` or `#[island]` source |
| `leptos_code__component_call_emit` | `name, props: Vec<(String,String)>` | Emit `<ComponentName prop=val/>` call |
| `leptos_code__component_file_emit` | `id` | Emit a complete `.rs` file with use statements |
| `leptos_code__component_list` | — | All stored component descriptors |

### View tools (12)

| Tool | Params | Description |
|---|---|---|
| `leptos_code__view_emit` | `content: String` | Wrap content in `view! { ... }` |
| `leptos_code__element_emit` | `tag, attrs?, on_events?, children?` | `<tag attrs>children</tag>` — handles any HTML5 tag |
| `leptos_code__show_emit` | `when_expr, children, fallback?` | `<Show when=move \|\| expr fallback=...>` |
| `leptos_code__for_emit` | `each_expr, key_expr, let_var, children` | `<For each=move \|\| items key=\|i\| i.id let:item>` |
| `leptos_code__suspense_emit` | `children, fallback?` | `<Suspense fallback=move \|\| view! { ... }>` |
| `leptos_code__transition_emit` | `children` | `<Transition>` |
| `leptos_code__error_boundary_emit` | `children, fallback_var, fallback_body` | `<ErrorBoundary fallback=\|err\| view! { ... }>` |
| `leptos_code__reactive_binding_emit` | `expr` | `{move \|\| expr}` (reactive interpolation) |
| `leptos_code__event_handler_emit` | `event, body` | `on:click=move \|ev\| { body }` (closure pattern) |
| `leptos_code__class_binding_emit` | `class, condition_expr` | `class:active=move \|\| condition` |
| `leptos_code__attr_binding_emit` | `attr, value_expr` | `attr:href=url_signal` |
| `leptos_code__router_link_emit` | `href, children` | `<A href="..." attr:aria-current="page">children</A>` |

Note: `element_emit` handles all HTML5 tags with a single parametric tool. The 140
per-element tools in the original plan are replaced by this one flexible tool.

### Server function tools (7)

| Tool | Params | Description |
|---|---|---|
| `leptos_code__server_fn_emit` | `name, args: Vec<(name,ty)>, return_type, body` | Full `#[server] pub async fn` |
| `leptos_code__resource_emit` | `name, source_expr, fetcher_expr` | `Resource::new(\|\| source, \|s\| async { ... })` |
| `leptos_code__action_emit` | `name, input_type, body` | `Action::new(\|input: T\| async { body })` |
| `leptos_code__server_action_emit` | `server_fn_name` | `ServerAction::<ServerFnName>::new()` |
| `leptos_code__action_form_emit` | `action_var, children` | `<ActionForm action=action_var>children</ActionForm>` |
| `leptos_code__server_error_emit` | `variants: Vec<(name,msg)>` | Custom `ServerFnError` enum |
| `leptos_code__server_fn_file_emit` | `fns: Vec<Uuid>` | Complete server_fns.rs file |

### Routing tools (8)

| Tool | Params | Description |
|---|---|---|
| `leptos_code__route_new` | `path, view` | Create `LeptosRouteDescriptor` |
| `leptos_code__route_add_nested` | `route_id, child_id` | Add nested route |
| `leptos_code__router_emit` | `route_ids` | Full `<Router><Routes>...</Routes></Router>` |
| `leptos_code__route_emit` | `route_id` | Single `<Route path="..." view=.../>` |
| `leptos_code__use_params_emit` | `params: Vec<(name,ty)>` | `use_params_map()` or typed `use_params!` |
| `leptos_code__use_navigate_emit` | — | `let navigate = use_navigate();` |
| `leptos_code__redirect_emit` | `path` | `<Redirect path="..."/>` |
| `leptos_code__protected_route_emit` | `condition, redirect, view` | Auth-gated route wrapper |

### Meta tools (4)

Wraps `leptos_meta` crate components.

| Tool | Params | Description |
|---|---|---|
| `leptos_code__meta_title_emit` | `title` | `<Title text="..."/>` |
| `leptos_code__meta_tag_emit` | `name, content` | `<Meta name="..." content="..."/>` |
| `leptos_code__meta_link_emit` | `rel, href` | `<Link rel="..." href="..."/>` |
| `leptos_code__meta_stylesheet_emit` | `href` | `<Stylesheet href="..."/>` |

### App scaffolding tools (8)

| Tool | Params | Description |
|---|---|---|
| `leptos_code__app_new` | `package_name, mode` | Create `LeptosAppDescriptor`, return UUID |
| `leptos_code__app_add_component` | `app_id, component_id` | Add component descriptor to app |
| `leptos_code__app_add_route` | `app_id, route_id` | Add route descriptor to app |
| `leptos_code__app_emit_component` | `app_id` | Emit root `App` component with router |
| `leptos_code__app_emit_main_rs` | `app_id` | Dual CSR/SSR `main.rs` (feature-gated) |
| `leptos_code__app_emit_lib_rs` | `app_id` | `lib.rs` with mod declarations |
| `leptos_code__app_emit_cargo_toml` | `app_id` | Complete `Cargo.toml` with leptos deps |
| `leptos_code__app_emit_all` | `app_id` | JSON map of all files → complete project |

`emit_cargo_toml` generates:
```toml
[dependencies]
leptos = { version = "0.8", features = ["csr"] }  # or ssr/hydrate/islands
leptos_router = { version = "0.8" }
leptos_meta = { version = "0.8" }
# + leptos_axum if ssr mode
```

### Catalog tools (6)

| Tool | Params | Description |
|---|---|---|
| `leptos_code__catalog_html_tags` | — | All valid HTML5 void/normal/SVG tag names |
| `leptos_code__catalog_leptos_components` | — | Show, For, Suspense, Transition, ErrorBoundary, Outlet, A, Form, ActionForm, Router, Routes, Route, Redirect |
| `leptos_code__catalog_events` | — | All DOM event names (click, input, change, keydown, …) |
| `leptos_code__catalog_prop_attrs` | — | Common HTML attributes by tag |
| `leptos_code__catalog_leptos_features` | — | csr, ssr, hydrate, islands, nonce — what each enables |
| `leptos_code__catalog_template` | `name` | Named starter apps: `"counter"`, `"todo"`, `"blog"`, `"auth"` |

---

## Leptos 0.8 API Reference (for implementation)

### Reactive primitives (server-safe)

```rust
use leptos::prelude::*;

// Signals
let sig: RwSignal<i32> = RwSignal::new(0);
sig.set(1);
sig.get();                          // → 1
sig.update(|v| *v += 1);

// Arc variants (non-Copy, reference-counted)
let (read, write) = arc_signal(0i32);

// Memos
let doubled = Memo::new(move |_prev| sig.get() * 2);
doubled.get();                      // → 2

// Effects (run on dependency change)
Effect::new(move |_| println!("{}", sig.get()));

// Owner scope
let owner = Owner::new();
let sig = owner.with(|| RwSignal::new(42));
```

### Macros as code-generation targets

```rust
// #[component] — emit tool generates:
#[component]
pub fn Counter(initial: i32) -> impl IntoView {
    let count = RwSignal::new(initial);
    view! {
        <button on:click=move |_| count.update(|n| *n += 1)>
            {move || count.get()}
        </button>
    }
}

// #[server] — emit tool generates:
#[server]
pub async fn add_todo(text: String) -> Result<u64, ServerFnError> {
    db::insert_todo(&text).await.map_err(|e| ServerFnError::from(e))
}

// #[island] — emit tool generates:
#[island]
pub fn LikeButton(post_id: u32) -> impl IntoView {
    let likes = RwSignal::new(0u32);
    view! { <button on:click=move |_| likes.update(|n| *n += 1)>{move || likes.get()}</button> }
}
```

### Closures in view! — emit pattern

Event handlers and reactive bindings are closures. The emit tools generate them as strings:

```
on:click=move |_| { body }         ← event_handler_emit
{move || signal_expr}              ← reactive_binding_emit
class:active=move || condition     ← class_binding_emit
```

These follow the same closure-as-fragment pattern used in `elicit_clap` and `elicit_tokio`.

---

## Cargo.toml

```toml
[package]
name = "elicit_leptos"
# ... workspace fields ...
description = "Elicitation-enabled leptos shadow crate — MCP tools for reactive state and code generation"
keywords = ["mcp", "leptos", "reactive", "webassembly", "elicitation"]
categories = ["web-programming", "wasm"]

[dependencies]
elicitation = { workspace = true, features = ["leptos-types", "emit"] }
leptos = { version = "0.8", default-features = false, features = ["ssr"] }
# ssr feature: reactive_graph + server-rendering, no WASM/DOM
reactive_graph = { version = "0.2" }
rmcp = { workspace = true }
schemars = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
tokio = { workspace = true }
tracing = { workspace = true }
uuid = { workspace = true, features = ["v4"] }
derive_more = { workspace = true }
strum = { workspace = true }

[features]
default = []
emit = ["elicitation/emit"]
```

Note: `features = ["ssr"]` gives us the reactive system and SSR rendering without
pulling in browser/WASM dependencies. The `ssr` feature includes `reactive_graph`.

---

## Tool Count Summary

| Category | Plugin | Count |
|---|---|---|
| Signals | `LeptosReactivePlugin` | 8 |
| Memos | `LeptosReactivePlugin` | 4 |
| Context | `LeptosReactivePlugin` | 4 |
| Actions | `LeptosReactivePlugin` | 4 |
| Owner | `LeptosReactivePlugin` | 2 |
| **Reactive subtotal** | | **22** |
| Components | `LeptosCodePlugin` | 8 |
| View / elements | `LeptosCodePlugin` | 12 |
| Server functions | `LeptosCodePlugin` | 7 |
| Routing | `LeptosCodePlugin` | 8 |
| Meta | `LeptosCodePlugin` | 4 |
| App scaffolding | `LeptosCodePlugin` | 8 |
| Catalog | `LeptosCodePlugin` | 6 |
| **Code subtotal** | | **53** |
| **Total** | | **~75** |

---

## What Changed from the Original Plan

| Original | Revised | Reason |
|---|---|---|
| 520–900 tools | ~75 tools | One parametric `element_emit` replaces 140 per-element tools |
| "Dual-mode" runtime view | Code-gen only | DOM rendering is browser-only |
| `create_signal()` API | `RwSignal::new()` / `signal()` | Leptos 0.7+ deprecated the old API |
| Three "patterns" | StatefulPlugin + DescriptorPlugin | Maps to actual codebase patterns |
| Week 1–10 timeline | Single-crate implementation | Scoped correctly |
| 140 HTML element tools | 1 `element_emit(tag, ...)` tool | Parametric is more useful |
| No API for `#[island]` | `component_set_island(true)` | Handled in component descriptor |
| Missing `leptos_meta` | 4 meta tools | Covers `<Title>`, `<Meta>`, `<Link>`, `<Stylesheet>` |

