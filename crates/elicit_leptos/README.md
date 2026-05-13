# elicit_leptos

MCP tools for the [Leptos](https://leptos.dev) 0.8 reactive web framework.

Provides two plugins covering reactive state management and code generation.

## Plugins

| Plugin | Namespace | Description |
|---|---|---|
| `LeptosReactivePlugin` | `leptos_reactive__*` | Signal/memo/action state management |
| `LeptosCodePlugin` | `leptos_code__*` | Component, view, routing, and app scaffolding |

## Quick Start

```rust,no_run
use elicit_leptos::{LeptosReactivePlugin, LeptosCodePlugin};

let reactive = LeptosReactivePlugin::new();
let code = LeptosCodePlugin::new();
```

## Reactive Tools (`leptos_reactive__*`)

### Signal tools

| Tool | Description |
|---|---|
| `leptos_reactive__signal_new` | Create an `RwSignal` with initial value, returns UUID |
| `leptos_reactive__signal_get` | Get current value by UUID |
| `leptos_reactive__signal_set` | Set new value by UUID |
| `leptos_reactive__signal_update` | Apply operation: `increment`, `decrement`, `append`, `clear`, `toggle`, `merge` |
| `leptos_reactive__signal_describe` | Get name, value, change count |
| `leptos_reactive__signal_list` | List all signals |
| `leptos_reactive__signal_delete` | Remove signal by UUID |
| `leptos_reactive__signal_track` | Get change count for dependency tracking |

### Memo tools

| Tool | Description |
|---|---|
| `leptos_reactive__memo_new` | Create a derived memo with ops: `upper`, `lower`, `negate`, `to_number`, `to_bool`, `length`, `abs`, `not` |
| `leptos_reactive__memo_get` | Compute current derived value |
| `leptos_reactive__memo_list` | List all memos with computed values |
| `leptos_reactive__memo_delete` | Remove memo by UUID |

### Context tools

| Tool | Description |
|---|---|
| `leptos_reactive__ctx_provide` | Store key/value in context |
| `leptos_reactive__ctx_get` | Retrieve value by key |
| `leptos_reactive__ctx_list` | List all context entries |
| `leptos_reactive__ctx_remove` | Remove entry by key |

### Action tools

| Tool | Description |
|---|---|
| `leptos_reactive__action_new` | Create a server action entry |
| `leptos_reactive__action_dispatch` | Dispatch input (marks pending) |
| `leptos_reactive__action_resolve` | Resolve with output |
| `leptos_reactive__action_list` | List all actions |

### Owner tools

| Tool | Description |
|---|---|
| `leptos_reactive__owner_reset` | Clear all reactive state |
| `leptos_reactive__owner_status` | Get counts of all primitives |

## Code Tools (`leptos_code__*`)

### Component tools

| Tool | Description |
|---|---|
| `leptos_code__component_new` | Create component descriptor |
| `leptos_code__component_add_prop` | Add a prop |
| `leptos_code__component_set_body` | Set function body |
| `leptos_code__component_set_island` | Mark as `#[island]` |
| `leptos_code__component_get` | Get descriptor |
| `leptos_code__component_delete` | Delete |
| `leptos_code__component_list` | List all |
| `leptos_code__component_emit` | Emit `#[component]` Rust source |
| `leptos_code__component_call_emit` | Emit call expression `<Name prop={val} />` |
| `leptos_code__component_file_emit` | Emit complete source file |

### View tools

| Tool | Description |
|---|---|
| `leptos_code__view_emit` | Emit `view! { ... }` |
| `leptos_code__element_emit` | Emit an HTML element |
| `leptos_code__show_emit` | Emit `<Show when=...>` |
| `leptos_code__for_emit` | Emit `<For each=... key=... let:x>` |
| `leptos_code__suspense_emit` | Emit `<Suspense fallback=...>` |
| `leptos_code__transition_emit` | Emit `<Transition>` |
| `leptos_code__error_boundary_emit` | Emit `<ErrorBoundary fallback=...>` |
| `leptos_code__reactive_binding_emit` | Emit `move \|\| signal.get()` expression |
| `leptos_code__event_handler_emit` | Emit `on:click={...}` |
| `leptos_code__class_binding_emit` | Emit `class:name={...}` |
| `leptos_code__attr_binding_emit` | Emit reactive attribute |
| `leptos_code__router_link_emit` | Emit `<A href="...">` |

### Server function tools

| Tool | Description |
|---|---|
| `leptos_code__server_fn_new` | Create server function descriptor |
| `leptos_code__server_fn_add_arg` | Add argument |
| `leptos_code__server_fn_get` | Get descriptor |
| `leptos_code__server_fn_delete` | Delete |
| `leptos_code__server_fn_list` | List all |
| `leptos_code__server_fn_emit` | Emit `#[server]` function source |
| `leptos_code__resource_emit` | Emit `Resource::new(...)` |
| `leptos_code__action_emit` | Emit `Action::new(...)` |
| `leptos_code__server_action_emit` | Emit `ServerAction::<Fn>::new()` |
| `leptos_code__action_form_emit` | Emit `<ActionForm action={...}>` |

### Routing tools

| Tool | Description |
|---|---|
| `leptos_code__route_new` | Create route descriptor |
| `leptos_code__route_add_nested` | Nest a route under another |
| `leptos_code__route_get` | Get route |
| `leptos_code__route_delete` | Delete route |
| `leptos_code__route_list` | List all routes |
| `leptos_code__router_emit` | Emit `<Router>` with stored routes |
| `leptos_code__route_emit` | Emit a single `<Route>` |
| `leptos_code__use_params_emit` | Emit `use_params_map()` access code |
| `leptos_code__use_navigate_emit` | Emit `use_navigate()` call |
| `leptos_code__redirect_emit` | Emit `<Redirect path="...">` |
| `leptos_code__outlet_emit` | Emit `<Outlet />` |

### Meta tools

| Tool | Description |
|---|---|
| `leptos_code__meta_title_emit` | Emit `<Title text="..."/>` |
| `leptos_code__meta_tag_emit` | Emit `<Meta name="..." content="..."/>` |
| `leptos_code__meta_link_emit` | Emit `<Link rel="..." href="..."/>` |
| `leptos_code__meta_stylesheet_emit` | Emit `<Stylesheet href="..."/>` |

### App scaffolding tools

| Tool | Description |
|---|---|
| `leptos_code__app_new` | Create app descriptor |
| `leptos_code__app_add_component` | Attach component to app |
| `leptos_code__app_add_route` | Attach route to app |
| `leptos_code__app_get` | Get app descriptor |
| `leptos_code__app_delete` | Delete app |
| `leptos_code__app_list` | List all apps |
| `leptos_code__app_emit_component` | Emit component from app |
| `leptos_code__app_emit_main_rs` | Emit `src/main.rs` |
| `leptos_code__app_emit_lib_rs` | Emit `src/lib.rs` with all components and router |
| `leptos_code__app_emit_cargo_toml` | Emit `Cargo.toml` with correct Leptos features |
| `leptos_code__app_emit_all` | Emit all scaffold files as JSON map |

### Catalog tools

| Tool | Description |
|---|---|
| `leptos_code__catalog_html_tags` | List all supported HTML5 tags |
| `leptos_code__catalog_leptos_components` | List built-in Leptos components |
| `leptos_code__catalog_events` | List DOM event names |
| `leptos_code__catalog_prop_attrs` | List HTML attributes with descriptions |
| `leptos_code__catalog_leptos_features` | List Leptos Cargo features |
| `leptos_code__catalog_template` | Starter templates: `counter`, `todo`, `blog` |

## License

Apache-2.0 OR MIT
