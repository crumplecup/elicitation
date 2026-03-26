# elicit_tokio — Implementation Plan

> **Premise:** Expose tokio's observable, serializable API surface as MCP tools.
> Not a completionist port — a pragmatic one that works within the MCP constraint:
> **everything crosses the wire as JSON, so closures and futures cannot be tool arguments.**

---

## The Core Constraint

MCP is a JSON protocol. This means:

| Category | Examples | Viable as MCP tool? |
|---|---|---|
| Methods taking `F: Future` | `spawn(future)`, `block_on(future)` | ❌ Can't serialize futures |
| Methods taking `F: FnOnce` | `spawn_blocking(f)` | ❌ Can't serialize closures |
| Methods with `Context<'_>` | `poll_*`, `poll_closed` | ❌ Internal waker plumbing |
| Types with `'a` lifetimes | `LocalEnterGuard<'a>`, `Permit<'a, T>` | ❌ Can't `Arc`-wrap |
| Observable state | `is_finished()`, `available_permits()`, `metrics()` | ✅ Serializable |
| Time operations | `sleep()`, `interval()`, `timeout()` | ✅ Duration → JSON |
| Stateful time handles | Deadline UUIDs, interval ticks | ✅ UUID-keyed registry |
| Construction/config | `Builder::worker_threads()` | ✅ Serializable params |
| Generic outputs `T` | `JoinHandle<T>` where T unknown | ⚠️ UUID-keyed registry |

---

## What Actually Makes Sense to Elicit

### Tier 1: Stateless workflow tools (`#[elicit_tool]`)

Pure functions with serializable params and results:

| Module | Tools |
|---|---|
| `tokio::time` | `sleep`, `sleep_until` |
| `tokio::runtime` | `inspect_flavor`, `inspect_metrics` |

### Tier 2: Stateful UUID-keyed plugins

Types that persist across tool calls, referenced by UUID:

| Registry | Operations |
|---|---|
| `TimeoutRegistry` | `timeout_create(duration_ms)` → uuid, `timeout_check(uuid)`, `timeout_await(uuid)` |
| `IntervalRegistry` | `interval_create(period_ms)` → uuid, `interval_tick(uuid)` (advances + returns elapsed) |
| `SemaphoreRegistry` | `semaphore_new(n)`, `acquire(uuid)`, `try_acquire(uuid)`, `available_permits(uuid)` |
| `NotifyRegistry` | `notify_new()`, `notify_one(uuid)`, `notify_waiters(uuid)`, `notified(uuid)` |
| `BarrierRegistry` | `barrier_new(n)`, `barrier_wait(uuid)` |
| `JoinHandleRegistry` | (Phase 2+) abort, is_finished, await_result |

---

## Phase 1: Crate Scaffold + Time Module

Time is the right first part:
- No generics
- No futures-as-args
- Immediately useful to agents pacing workflows
- Proves the pattern compiles with tokio

### Crate structure

```
crates/elicit_tokio/
├── Cargo.toml
└── src/
    ├── lib.rs
    └── time.rs
```

### Cargo.toml (key deps)

```toml
[dependencies]
elicitation = { workspace = true }
elicitation_derive.workspace = true
tokio = { workspace = true, features = ["time"] }
# ... schemars, serde, serde_json, rmcp, tracing

[features]
emit = ["dep:quote", "elicitation/emit"]
```

Note: workspace `tokio` dep currently only has `macros`, `rt-multi-thread`,
`io-std`. We need to add `time` to the workspace dep **or** add it as a crate-local
feature override. Preferred: add `time` to workspace default features.

### `TokioTimePlugin` (stateful, UUID-keyed)

```rust
pub struct TokioTimePlugin {
    deadlines: Arc<Mutex<HashMap<Uuid, Instant>>>,
    intervals: Arc<Mutex<HashMap<Uuid, Interval>>>,
}
```

Tools under `"tokio_time"` namespace:

| Tool | Params | Returns | Establishes |
|---|---|---|---|
| `sleep` | `duration_ms: u64` | `{ elapsed_ms }` | `SleepCompleted` |
| `sleep_until` | `deadline: String` (RFC 3339) | `{ elapsed_ms }` | `SleepCompleted` |
| `timeout_create` | `duration_ms: u64` | `{ timeout_id: Uuid }` | — |
| `timeout_check` | `timeout_id: Uuid` | `{ elapsed_ms, remaining_ms, expired: bool }` | — |
| `timeout_await` | `timeout_id: Uuid` | `{ elapsed_ms, expired: bool }` | `TimeoutResolved` |
| `interval_create` | `period_ms: u64` | `{ interval_id: Uuid }` | — |
| `interval_tick` | `interval_id: Uuid` | `{ tick_number: u64, elapsed_since_start_ms }` | — |

### Propositions

```rust
pub struct SleepCompleted;      // sleep/sleep_until returned
pub struct TimeoutResolved;     // timeout was awaited (either fired or we checked it)
```

---

## Phase 2: Sync Primitives

Same UUID-keyed registry pattern:

- `Semaphore` → `semaphore_new(n)` → uuid; `acquire`, `try_acquire`,
  `available_permits`, `close`
- `Notify` → `notify_new()` → uuid; `notify_one`, `notify_waiters`, `notified`
  (blocks awaiting next notification)
- `Barrier` → `barrier_new(n)` → uuid; `barrier_wait` (returns leader/follower status)

---

## Phase 3: Runtime Inspection

`Handle::current()` works inside the tokio runtime the MCP server already runs on:

```rust
#[elicit_tool(plugin = "tokio_runtime", name = "inspect_flavor")]
async fn inspect_flavor(_: InspectFlavorParams) -> Result<CallToolResult, ErrorData> {
    let flavor = tokio::runtime::Handle::current().runtime_flavor();
    // → "current_thread" | "multi_thread"
}
```

Metrics require `tokio_unstable` cfg — feature-gated.

---

## Known Blockers

| Item | Status |
|---|---|
| workspace `tokio` dep needs `time` feature | ⚠️ Add before Phase 1b |
| `Interval` doesn't impl `Send + 'static` naively | ✅ Fine inside Arc<Mutex<...>> |
| `elicit_server` emit chain update | After Phase 1 lands |
| Generic JoinHandle/JoinSet wrapping | Deferred to Phase 2+ |
| `spawn(future)` from MCP | Permanently infeasible over JSON protocol |
| `#[tokio::main]` / `#[tokio::test]` proc-macros | Not runtime functions; skip entirely |
| `poll_*` methods | Not exposable via JSON; skip |
| Lifetime-parameterized types (`LocalEnterGuard<'a>`) | Not wrappable; skip |

---

## What the Original Plan Got Wrong

1. **Plugin registration pattern** — invented non-existent `Plugin` / `PluginRegistry` /
   `ToolDefinition` APIs. Real pattern: `#[derive(ElicitPlugin)]` + `#[elicit_tool]`.

2. **`elicit_newtype!` syntax** — the plan used `elicit_newtype!(pub struct Foo(inner))`.
   Actual syntax is `elicit_newtype!(path::to::Type, as WrapperName)`.

3. **`#[reflect_methods]` on generic types** — the macro doesn't handle `impl<T>` blocks.
   Generic newtypes need the UUID registry pattern instead.

4. **Completionist scope** — most of the tokio API requires futures/closures as
   arguments, which cannot cross the MCP JSON boundary.

5. **`tokio::main` / `tokio::test` as tools** — these are proc-macros for code
   generation, not runtime functions. Nothing to elicit.

6. **`timeout(duration, future)` as a stateless tool** — takes a future arg.
   The correct approach is a stateful `TokioTimePlugin` with a deadline registry:
   `timeout_create` → UUID, then `timeout_check` / `timeout_await` on that UUID.

---

## Implementation Order

1. **Phase 1a** — Add `time` feature to workspace `tokio` dep
2. **Phase 1b** — Crate scaffold: `Cargo.toml`, `lib.rs` (empty but compiling)
3. **Phase 1c** — `time.rs`: `TokioTimePlugin` with all 7 tools from the table above
4. **Phase 1d** — `just check-all elicit_tokio`; fix any issues
5. **Phase 1e** — Wire into `elicit_server` emit chain
6. **Phase 2** — Sync primitives (Semaphore, Notify, Barrier)
7. **Phase 3** — Runtime inspection (Handle::current)

---

## Phase 0: `tokio-types` Primitives in `elicitation` crate

> **Important:** For the UUID-registry phases (Phase 1 time, Phase 2 sync), tokio
> objects never cross the MCP boundary — they stay inside `Arc<Mutex<HashMap<Uuid, T>>>`.
> The MCP-boundary values are all existing primitives: `Uuid`, `u64`, `bool`, `String`.
> **Phase 1 requires zero new Elicitation primitives.**

However, to shadow types that appear as **tool outputs** (not just internal state),
we need owned mirror types with `Elicitation` impls — the same pattern as
`ColumnValue` mirroring `AnyValueKind`, or `DriverKind` mirroring sqlx driver enums.

### What's needed per phase

| Phase | Tokio type | Crosses MCP boundary? | Needs primitive? |
|---|---|---|---|
| 1 (time) | `Instant`, `Interval` | ❌ Stays in registry | No |
| 2 (sync) | `Semaphore`, `Notify`, `Barrier` | ❌ Stays in registry | No |
| 3 (runtime) | `RuntimeFlavor` | ✅ Output of inspect_flavor | Yes |
| 3 (runtime) | `RuntimeMetrics` | ✅ Output fields | Yes (feature-gated) |

### `tokio-types` feature in `elicitation` crate

Add to `crates/elicitation/Cargo.toml`:

```toml
[features]
tokio-types = ["tokio/rt"]   # rt feature exposes RuntimeFlavor
full = [..., "tokio-types"]
```

`tokio` is already a dep in `elicitation/Cargo.toml` (line 32 — no feature gate).
Just add `tokio-types` feature + gated module.

### New file: `crates/elicitation/src/primitives/tokio_types/mod.rs`

```
tokio_types/
├── mod.rs
└── runtime_flavor.rs    # Phase 3: RuntimeFlavorKind enum
```

`runtime_flavor.rs` — owned Select + Prompt + Elicitation for the 2 variants in
tokio 1.x: `CurrentThread`, `MultiThread`.

### When to implement Phase 0

Phase 0 primitives can be added just before Phase 3. Phase 1 and Phase 2
can proceed without them. Include Phase 0 in the same PR as Phase 3.
