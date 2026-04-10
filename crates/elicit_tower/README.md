# elicit_tower

MCP-enabled middleware tooling built on [`tower`], [`tower-http`], [`elicitation`], and [`rmcp`].

Wraps tower's middleware stack — rate limiting, concurrency control, timeouts,
buffering, retry policies, and HTTP layers — as MCP tools and code emitters,
giving an agent a complete vocabulary for constructing verified service middleware
without writing any Rust by hand.

---

## What this crate provides

Layer configurations are serializable survey structs stored in UUID-keyed
registries server-side. Agents interact via opaque `layer_id` handles and call
`*__layer_describe` to inspect what was created.

Seven plugins cover the full tower + tower-http surface:

1. **`TowerLimitPlugin`** — core flow-control layers (concurrency, rate,
   timeout, buffer, load-shed, spawn-ready)
2. **`TowerRetryPlugin`** — backoff strategies, TPS budgets, retry policies,
   and service filters
3. **`TowerHttpPlugin`** — all tower-http middleware layers (compression,
   tracing, CORS, headers, timeouts, …)
4. **`TowerUtilPlugin`** — utility layers (map-err, map-request, map-response,
   map-future, then, and-then, box-clone-service, box-service)
5. **`TowerBuilderPlugin`** — `ServiceBuilder` composition (layer stacking,
   `make_service`, `into_inner`)
6. **`TowerBalancePlugin`** — load-balanced service sets (p2c with peak-ewma
   or pending-requests, `Balance` service, `Discover`-based construction)
7. **`TowerSteerPlugin`** — request steering (`Steer` + index-based selector)

---

## Quick start

```toml
[dependencies]
elicit_tower = { version = "0.10" }
rmcp = { version = "0.1", features = ["server"] }
```

```rust,no_run
use elicitation::PluginRegistry;
use elicit_tower::{
    TowerBalancePlugin, TowerBuilderPlugin, TowerHttpPlugin,
    TowerLimitPlugin, TowerRetryPlugin, TowerSteerPlugin, TowerUtilPlugin,
};

let registry = PluginRegistry::new()
    .register("tower_limit",   TowerLimitPlugin::new())
    .register("tower_retry",   TowerRetryPlugin::new())
    .register("tower_http",    TowerHttpPlugin::new())
    .register("tower_util",    TowerUtilPlugin::new())
    .register("tower_builder", TowerBuilderPlugin::new())
    .register("tower_balance", TowerBalancePlugin::new())
    .register("tower_steer",   TowerSteerPlugin::new());

// Limit tools:   tower_limit__concurrency_limit_layer_new, …
// Retry tools:   tower_retry__backoff_new, tower_retry__retry_layer_new, …
// HTTP tools:    tower_http__compression_layer_new, tower_http__cors_layer_new, …
// Util tools:    tower_util__map_err_layer_new, tower_util__box_service_new, …
// Builder tools: tower_builder__new, tower_builder__layer, …
// Balance tools: tower_balance__peak_ewma_new, tower_balance__balance_new, …
// Steer tools:   tower_steer__steer_new, tower_steer__describe, …
```

---

## Plugin inventory

| Plugin | Namespace | Tool count | Notes |
|---|---|---|---|
| [`TowerLimitPlugin`]   | `tower_limit__*`   | 8  | Rate, concurrency, timeout, buffer, load-shed, spawn-ready |
| [`TowerRetryPlugin`]   | `tower_retry__*`   | 6  | Exponential backoff, TPS budget, retry + filter layers |
| [`TowerHttpPlugin`]    | `tower_http__*`    | 15 | All tower-http layers + describe |
| [`TowerUtilPlugin`]    | `tower_util__*`    | 10 | map-err/req/res/future, then, and-then, box-clone, box-service |
| [`TowerBuilderPlugin`] | `tower_builder__*` | 4  | ServiceBuilder composition: layer, make_service, into_inner |
| [`TowerBalancePlugin`] | `tower_balance__*` | 4  | P2c load balancing: peak-ewma, pending-requests, balance |
| [`TowerSteerPlugin`]   | `tower_steer__*`   | 2  | Request steering with selector expression |

---

## Tool reference

### `TowerLimitPlugin` — `tower_limit__*`

| Tool | Description |
|---|---|
| `tower_limit__concurrency_limit_layer_new` | Create a `ConcurrencyLimitLayer` with a max in-flight count |
| `tower_limit__rate_limit_layer_new` | Create a `RateLimitLayer` from a `Rate` |
| `tower_limit__rate_new` | Create a `Rate` (requests per duration) |
| `tower_limit__timeout_layer_new` | Create a `TimeoutLayer` with a millisecond timeout |
| `tower_limit__buffer_layer_new` | Create a `BufferLayer` with a request queue bound |
| `tower_limit__load_shed_layer_new` | Create a `LoadShedLayer` (returns 503 when overloaded) |
| `tower_limit__spawn_ready_layer_new` | Create a `SpawnReadyLayer` (drives inner service to readiness on a task) |
| `tower_limit__layer_describe` | Describe a previously created layer by its `layer_id` |

### `TowerRetryPlugin` — `tower_retry__*`

| Tool | Description |
|---|---|
| `tower_retry__backoff_new` | Create an `ExponentialBackoffMaker` with min/max/jitter config |
| `tower_retry__budget_new` | Create a `TpsBudget` with RPS, TTL, and min-per-sec config |
| `tower_retry__retry_layer_new` | Create a `RetryLayer` referencing a backoff and budget by ID |
| `tower_retry__filter_layer_new` | Create a `FilterLayer` with a predicate description |
| `tower_retry__backoff_describe` | Describe a previously created backoff by its `backoff_id` |
| `tower_retry__budget_describe` | Describe a previously created budget by its `budget_id` |

### `TowerHttpPlugin` — `tower_http__*`

| Tool | Description |
|---|---|
| `tower_http__normalize_path_layer_new` | Normalize trailing slashes on request paths |
| `tower_http__propagate_header_layer_new` | Copy a named header from request to response |
| `tower_http__set_status_layer_new` | Override the response status code |
| `tower_http__compression_layer_new` | Compress responses (gzip / deflate / br / zstd) |
| `tower_http__decompression_layer_new` | Decompress request bodies |
| `tower_http__http_timeout_layer_new` | Apply a per-request HTTP timeout |
| `tower_http__trace_layer_new` | Attach tracing spans to every request |
| `tower_http__catch_panic_layer_new` | Convert handler panics to 500 responses |
| `tower_http__cors_layer_new` | Configure CORS policy |
| `tower_http__validate_request_header_layer_new` | Reject requests missing a required header value |
| `tower_http__set_request_header_layer_new` | Inject a header into every request |
| `tower_http__set_response_header_layer_new` | Inject a header into every response |
| `tower_http__sensitive_request_headers_layer_new` | Mark request headers as sensitive (redacted in traces) |
| `tower_http__sensitive_response_headers_layer_new` | Mark response headers as sensitive |
| `tower_http__layer_describe` | Describe a previously created HTTP layer by its `layer_id` |

### `TowerUtilPlugin` — `tower_util__*`

| Tool | Description |
|---|---|
| `tower_util__map_err_layer_new` | Create a `MapErrLayer` with a closure/fn expression |
| `tower_util__map_request_layer_new` | Create a `MapRequestLayer` with a closure/fn expression |
| `tower_util__map_response_layer_new` | Create a `MapResponseLayer` with a closure/fn expression |
| `tower_util__map_future_layer_new` | Create a `MapFutureLayer` with a closure/fn expression |
| `tower_util__then_layer_new` | Create a `ThenLayer` with a callback expression |
| `tower_util__and_then_layer_new` | Create an `AndThenLayer` with a callback expression |
| `tower_util__box_clone_service_layer_new` | Create a `BoxCloneServiceLayer` (type-erased clone-able service) |
| `tower_util__box_service_new` | Create a `BoxService` descriptor with request/response type names |
| `tower_util__layer_describe` | Describe a previously created util layer by its `layer_id` |
| `tower_util__box_service_describe` | Describe a previously created box service by its `service_id` |

> All `*_fn` / closure fields accept any Rust expression: a closure literal
> (`|e| e.to_string()`), a free function name (`my_fn`), or a macro invocation.

### `TowerBuilderPlugin` — `tower_builder__*`

| Tool | Description |
|---|---|
| `tower_builder__new` | Create a new `ServiceBuilder` descriptor |
| `tower_builder__layer` | Push a `TowerLayerKind`-encoded layer onto the builder |
| `tower_builder__describe` | Describe a builder by its `builder_id` |
| `tower_builder__emit` | Emit Rust code constructing the `ServiceBuilder` |

### `TowerBalancePlugin` — `tower_balance__*`

| Tool | Description |
|---|---|
| `tower_balance__peak_ewma_new` | Create a `PeakEwma` load-metric descriptor |
| `tower_balance__pending_requests_new` | Create a `PendingRequests` load-metric descriptor |
| `tower_balance__balance_new` | Create a `Balance` service descriptor referencing a load metric |
| `tower_balance__describe` | Describe a load metric or balance service by ID |

### `TowerSteerPlugin` — `tower_steer__*`

| Tool | Description |
|---|---|
| `tower_steer__steer_new` | Create a `Steer` descriptor with selector expression and named sub-services |
| `tower_steer__describe` | Describe a previously created steer descriptor by its `steer_id` |

---

## Propositions

Each plugin exposes a `VerifiedWorkflow` proposition that asserts the workflow
is non-trivially constrained:

| Proposition | Plugin |
|---|---|
| [`TowerLayerCreated`]          | `TowerLimitPlugin` |
| [`TowerRateCreated`]           | `TowerLimitPlugin` |
| [`TowerBackoffCreated`]        | `TowerRetryPlugin` |
| [`TowerBudgetCreated`]         | `TowerRetryPlugin` |
| [`TowerRetryLayerCreated`]     | `TowerRetryPlugin` |
| [`TowerHttpLayerCreated`]      | `TowerHttpPlugin` |
| [`TowerUtilLayerCreated`]      | `TowerUtilPlugin` |
| [`TowerBoxServiceCreated`]     | `TowerUtilPlugin` |
| [`TowerServiceBuilderCreated`] | `TowerBuilderPlugin` |
| [`TowerServiceBuilderLayerAdded`] | `TowerBuilderPlugin` |
| [`TowerServiceBuilderDone`]    | `TowerBuilderPlugin` |
| [`TowerLoadCreated`]           | `TowerBalancePlugin` |
| [`TowerBalanceCreated`]        | `TowerBalancePlugin` |
| [`TowerSteerCreated`]          | `TowerSteerPlugin` |

---

## Feature flags

This crate has no optional features. The `elicitation` dependency is pulled in
with the `tower-types` feature which provides the underlying trenchcoat types
(`TowerConcurrencyLimitLayer`, `TowerRate`, `TowerExponentialBackoffMaker`, etc.)
used by these plugins.
