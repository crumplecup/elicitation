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

Three plugins cover the full tower + tower-http surface:

1. **`TowerLimitPlugin`** — core flow-control layers (concurrency, rate,
   timeout, buffer, load-shed, spawn-ready)
2. **`TowerRetryPlugin`** — backoff strategies, TPS budgets, retry policies,
   and service filters
3. **`TowerHttpPlugin`** — all tower-http middleware layers (compression,
   tracing, CORS, headers, timeouts, …)

---

## Quick start

```toml
[dependencies]
elicit_tower = { version = "0.10" }
rmcp = { version = "0.1", features = ["server"] }
```

```rust,no_run
use elicitation::PluginRegistry;
use elicit_tower::{TowerLimitPlugin, TowerRetryPlugin, TowerHttpPlugin};

let registry = PluginRegistry::new()
    .register("tower_limit", TowerLimitPlugin::new())
    .register("tower_retry", TowerRetryPlugin::new())
    .register("tower_http",  TowerHttpPlugin::new());

// Limit tools:  tower_limit__concurrency_limit_layer_new, tower_limit__timeout_layer_new, …
// Retry tools:  tower_retry__backoff_new, tower_retry__retry_layer_new, …
// HTTP tools:   tower_http__compression_layer_new, tower_http__cors_layer_new, …
```

---

## Plugin inventory

| Plugin | Namespace | Tool count | Notes |
|---|---|---|---|
| [`TowerLimitPlugin`] | `tower_limit__*` | 8 | Rate, concurrency, timeout, buffer, load-shed, spawn-ready |
| [`TowerRetryPlugin`] | `tower_retry__*` | 6 | Exponential backoff, TPS budget, retry + filter layers |
| [`TowerHttpPlugin`]  | `tower_http__*`  | 15 | All tower-http layers + describe |

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

---

## Propositions

Each plugin exposes a `VerifiedWorkflow` proposition that asserts the workflow
is non-trivially constrained:

| Proposition | Plugin |
|---|---|
| [`TowerLayerCreated`] | `TowerLimitPlugin` |
| [`TowerRateCreated`] | `TowerLimitPlugin` |
| [`TowerBackoffCreated`] | `TowerRetryPlugin` |
| [`TowerBudgetCreated`] | `TowerRetryPlugin` |
| [`TowerRetryLayerCreated`] | `TowerRetryPlugin` |
| [`TowerHttpLayerCreated`] | `TowerHttpPlugin` |

---

## Feature flags

This crate has no optional features. The `elicitation` dependency is pulled in
with the `tower-types` feature which provides the underlying trenchcoat types
(`TowerConcurrencyLimitLayer`, `TowerRate`, `TowerExponentialBackoffMaker`, etc.)
used by these plugins.
