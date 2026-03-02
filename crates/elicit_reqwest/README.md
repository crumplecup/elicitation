# elicit_reqwest

MCP-enabled HTTP workflows built on [`reqwest`], [`elicitation`], and [`rmcp`].

This crate wraps reqwest's core types as MCP tools and composes them into
verified HTTP workflows using the elicitation framework's action traits and
contract primitives.

## Quick start

Add the crate and wire the plugins into your rmcp server:

```toml
[dependencies]
elicit_reqwest = { version = "0.8" }
rmcp = { version = "0.1", features = ["server"] }
tokio = { version = "1", features = ["full"] }
```

```rust,no_run
use elicitation::PluginRegistry;
use elicit_reqwest::plugins::{
    Plugin, StatusCodePlugin, UrlPlugin, MethodPlugin,
    HeaderMapPlugin, RequestBuilderPlugin, WorkflowPlugin,
};
use rmcp::{ServerHandler, transport};

struct MyServer {
    http: PluginRegistry,
}

impl MyServer {
    fn new() -> Self {
        Self {
            http: PluginRegistry::new()
                .register("http",            Plugin::new())
                .register("status_code",     StatusCodePlugin)
                .register("url",             UrlPlugin)
                .register("method",          MethodPlugin)
                .register("header_map",      HeaderMapPlugin)
                .register("request_builder", RequestBuilderPlugin::new())
                .register("workflow",        WorkflowPlugin::default_client()),
        }
    }
}

// impl ServerHandler for MyServer { … }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = MyServer::new();
    rmcp::serve_server(server, transport::stdio()).await?;
    Ok(())
}
```

Tools are namespaced by plugin — an agent calls `workflow__fetch_json`,
`url__parse`, `header_map__insert`, and so on.

## What it does

`elicit_reqwest` exposes HTTP operations to AI agents as MCP tools, organized
into seven plugins. At the bottom layer, atomic tools mirror single reqwest
operations — parse a URL, inspect a status code, set a header. At the top
layer, workflow tools compose these atoms into phrase-level operations with
documented, machine-checkable properties.

```
Workflow tools  →  fetch_auth, post_json, paginated_get, api_call …
                         ↓  composed from
Plugin tools    →  url_build, header_map.insert, request_builder.send …
                         ↓  delegating to
Newtypes        →  Client, RequestBuilder, Response, Url, HeaderMap …
                         ↓  wrapping
reqwest         →  reqwest::Client, reqwest::RequestBuilder, …
```

## Newtype wrappers

Each reqwest type is wrapped using the `elicit_newtype!` macro, which places
the inner value behind an `Arc` so that builder types (which are normally
consuming) become `Clone`-able and can be passed across async boundaries and
MCP tool boundaries without sacrificing the original API shape.

| Wrapper | Wraps |
|---|---|
| `Client` | `reqwest::Client` |
| `RequestBuilder` | `reqwest::RequestBuilder` |
| `Response` | `reqwest::Response` |
| `Error` | `reqwest::Error` |
| `Url` | `url::Url` |
| `Method` | `reqwest::Method` |
| `StatusCode` | `reqwest::StatusCode` |
| `HeaderMap` | `http::HeaderMap` |
| `Version` | `http::Version` |

## MCP plugins

Seven plugins register a total of ~79 tools against an `rmcp` tool registry.
Each plugin groups tools by the type they operate on, so agent tool selection
is straightforward.

| Plugin | Namespace | Tools |
|---|---|---|
| `Plugin` | `http` | `get`, `post`, `put`, `delete`, `patch`, `head` |
| `StatusCodePlugin` | `status_code` | `from_u16`, `as_str`, `canonical_reason`, `is_success`, `is_client_error`, `is_server_error`, … |
| `UrlPlugin` | `url` | `parse`, `scheme`, `host`, `port`, `path`, `query`, `join`, `set_*` (6 variants), … |
| `MethodPlugin` | `method` | `from_str`, `as_str`, `is_safe`, `is_idempotent` |
| `HeaderMapPlugin` | `header_map` | `new`, `get`, `insert`, `append`, `remove`, `keys`, `values`, `clear`, … |
| `RequestBuilderPlugin` | `request_builder` | `new_*` (6), `with_*` (6), `inspect`, `send` |
| `WorkflowPlugin` | `workflow` | `url_build`, `fetch`, `fetch_json`, `fetch_auth`, `post_json`, `api_call`, `health_check`, `build_request`, `status_summary`, `paginated_get` |

Register plugins with an rmcp server:

```rust
use elicit_reqwest::{StatusCodePlugin, UrlPlugin, WorkflowPlugin};
use rmcp::ServerBuilder;

let server = ServerBuilder::new()
    .register(StatusCodePlugin)
    .register(UrlPlugin)
    .register(WorkflowPlugin::new())
    .build();
```

## Workflows and contracts

The `WorkflowPlugin` composes atomic tools into higher-level operations and
attaches contract proofs to every result. Contracts are zero-cost
`PhantomData` propositions assembled using the `And` combinator from the
elicitation framework.

```
FetchSucceeded    = And<UrlValid, And<RequestCompleted, StatusSuccess>>
AuthFetchSucceeded = And<Authorized, FetchSucceeded>
```

Each workflow tool constructs an `Established<Prop>` proof at runtime and
embeds it in the result under a `"contract"` field. Downstream tool calls can
require a specific proof as a precondition, turning a sequence of tool
invocations into a verified state machine — a *workflow*.

```
url_build ──→ Established<UrlValid>
     ↓
fetch ─────→ Established<FetchSucceeded>
     ↓
json parse → Established<And<FetchSucceeded, Decoded>>
```

Because each proposition is a distinct type, the type checker enforces that
steps happen in order and that required preconditions are satisfied before
proceeding. Agents cannot call `post_json` with an unvalidated URL and receive
a `FetchSucceeded` proof — the state machine simply does not allow that
transition.

### Constrained parameters

Workflow tools restrict open-ended string parameters with `Select` enums:

```rust
pub enum AuthType   { None, Bearer, Basic, ApiKey }
pub enum ContentType { Json, FormUrlEncoded, PlainText, OctetStream }
```

These enums appear in JSON schemas generated by `schemars`, so agents see an
explicit choice list rather than a free-form string field.

## Stateless request specs

`RequestSpec` decouples request construction from execution:

```rust
pub struct RequestSpec {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub timeout_secs: Option<f64>,
}
```

An agent can build up a `RequestSpec` incrementally using
`request_builder` plugin tools, inspect it, and only commit to execution with
`send`. This mirrors how a human would review a cURL command before running it.

## Relationship to elicitation

This crate depends on `elicitation` with the `reqwest` feature and consumes:

- **`elicit_newtype!`** — Arc-wrapping macro for non-Clone types
- **`reflect_methods!`** — auto-generates MCP tool signatures from impl blocks
- **`elicit_safe!`** — marks types as safe to surface in MCP schemas
- **Contract primitives** — `And`, `Established`, `Proposition` from `elicitation::contracts`
- **`ElicitSpec` / `TypeSpecPlugin`** — type specs for all wrapped types are
  registered and queryable via the `describe_type` / `explore_type` MCP tools
  from the parent crate

## License

Licensed under either of [Apache License 2.0](../../LICENSE-APACHE) or
[MIT License](../../LICENSE-MIT) at your option.
