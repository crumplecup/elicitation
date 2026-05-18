# elicit_axum

`elicit_axum` is the [elicitation] shadow crate for [axum]. It exposes 22 MCP
tools across 4 plugins that let an AI agent describe and emit a complete axum
web service — routers, handlers, responses, and the top-level serve
configuration — without any live axum instances crossing the MCP boundary.

## Shadow crate concept

A shadow crate mirrors the API surface of its source library using names that
are familiar to any developer who knows axum, but replaces the live types with
MCP-compatible descriptors. When an agent calls `axum_router__new` it gets back
a UUID handle, not a real `Router<S>`. All descriptors live in server-side
registries keyed by those UUIDs. When the agent is satisfied with the
configuration it calls the `emit` tool to recover idiomatic Rust source that can
be pasted directly into a project.

## Plugins

| Plugin | Namespace | Description |
|---|---|---|
| `AxumRouterPlugin` | `axum_router__*` | Router descriptor: routes, layers, fallback, state |
| `AxumHandlerPlugin` | `axum_handler__*` | Handler descriptor: extractors, body |
| `AxumResponsePlugin` | `axum_response__*` | Response descriptor: JSON, HTML, redirect, status |
| `AxumServePlugin` | `axum_serve__*` | Serve descriptor: bind address, graceful shutdown |

## Tool reference

### `axum_router__*`

| Tool | Params | Returns | Establishes |
|---|---|---|---|
| `new` | `state_type` | `{ router_id }` | `AxumRouterCreated` |
| `add_route` | `router_id, method, path, handler` | `{ route_count }` | `AxumRouteAdded` |
| `add_layer` | `router_id, layer_expr` | `{ layer_count }` | — |
| `set_fallback` | `router_id, handler` | — | — |
| `set_db_slot` | `router_id, pool_type, var_name, provide_leptos_context` | — | — |
| `set_custom_state` | `router_id, state_type, state_expr` | — | — |
| `describe` | `router_id` | JSON descriptor | — |
| `emit` | `router_id` | Rust source string | — |

### `axum_handler__*`

| Tool | Params | Returns | Establishes |
|---|---|---|---|
| `new` | `name, return_type` | `{ handler_id }` | `AxumHandlerDefined` |
| `add_extractor` | `handler_id, var_name, kind, type_name` | `{ extractor_count }` | `AxumExtractorAdded` |
| `set_body` | `handler_id, body` | — | — |
| `describe` | `handler_id` | JSON descriptor | — |
| `emit` | `handler_id` | Rust `async fn` source | — |

Supported extractor kinds: `Path`, `Query`, `Json`, `State`, `Extension`,
`Form`, `Headers`, `RawBody`, `RawQuery`, `OriginalUri`, `MatchedPath`,
`ConnectInfo`.

### `axum_response__*`

| Tool | Params | Returns | Establishes |
|---|---|---|---|
| `json` | `status_code, body_expr` | `{ response_id }` | `AxumResponseDefined` |
| `html` | `status_code, body_expr` | `{ response_id }` | `AxumResponseDefined` |
| `redirect_permanent` | `uri` | `{ response_id }` | `AxumResponseDefined` |
| `redirect_temporary` | `uri` | `{ response_id }` | `AxumResponseDefined` |
| `no_content` | — | `{ response_id }` | `AxumResponseDefined` |
| `status` | `status_code, body_expr?` | `{ response_id }` | `AxumResponseDefined` |
| `describe` | `response_id` | JSON descriptor | — |
| `emit` | `response_id` | Rust expression string | — |

### `axum_serve__*`

| Tool | Params | Returns | Establishes |
|---|---|---|---|
| `new` | `router_id, addr` | `{ serve_id }` | `AxumServerConfigured` |
| `with_graceful_shutdown` | `serve_id, signal_expr` | — | — |
| `describe` | `serve_id` | JSON descriptor | — |
| `emit_main` | `serve_id` | Complete `main.rs` source | — |

## Propositions

Each state-changing tool establishes a typed proposition:

| Proposition | Established by |
|---|---|
| `AxumRouterCreated` | `axum_router__new` |
| `AxumRouteAdded` | `axum_router__add_route` |
| `AxumHandlerDefined` | `axum_handler__new` |
| `AxumExtractorAdded` | `axum_handler__add_extractor` |
| `AxumResponseDefined` | all `axum_response__*` creation tools |
| `AxumServerConfigured` | `axum_serve__new` |

Propositions implement [`Prop`] and [`VerifiedWorkflow`], and carry Kani, Verus,
and Creusot proof skeletons that can be discharged by the elicitation prove
pipeline.

## Typical workflow

```text
1. axum_router__new           → router_id
2. axum_handler__new          → handler_id
3. axum_handler__add_extractor (repeat as needed)
4. axum_handler__set_body
5. axum_router__add_route     → links handler name into router
6. axum_response__json        → response_id  (optional, for documenting responses)
7. axum_serve__new            → serve_id
8. axum_serve__with_graceful_shutdown  (optional)
9. axum_router__emit          → paste Router construction into project
10. axum_handler__emit         → paste async fn into project
11. axum_serve__emit_main      → paste main() into project
```

## Feature flags

| Feature | Description |
|---|---|
| `emit` | Enable `EmitCode` code-recovery support (re-exports `elicitation/emit`) |

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
elicit_axum = "0.11"
```

Register the plugins with your MCP server:

```rust
use elicit_axum::{AxumHandlerPlugin, AxumResponsePlugin, AxumRouterPlugin, AxumServePlugin};

let server = server
    .register_plugin(AxumRouterPlugin::new())
    .register_plugin(AxumHandlerPlugin::new())
    .register_plugin(AxumResponsePlugin::new())
    .register_plugin(AxumServePlugin::new());
```

[elicitation]: https://crates.io/crates/elicitation
[axum]: https://crates.io/crates/axum
[`Prop`]: https://docs.rs/elicitation/latest/elicitation/contracts/trait.Prop.html
[`VerifiedWorkflow`]: https://docs.rs/elicitation/latest/elicitation/trait.VerifiedWorkflow.html
