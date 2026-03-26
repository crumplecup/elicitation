# elicit_tokio

MCP-enabled async runtime tooling built on [`tokio`], [`elicitation`], and [`rmcp`].

Wraps tokio's async primitives — time, sync, I/O, networking, channels, processes,
signals, and tasks — as MCP tools and code emitters, giving an agent a complete
vocabulary for constructing verified async workflows without writing any Rust by hand.

---

## What this crate provides

Three complementary layers:

1. **Stateful runtime tools** — tokio objects (semaphores, channels, sockets,
   child processes, …) live server-side in UUID-keyed registries; agents
   interact via serializable handles (`Uuid` values) that never carry live state
   across the MCP boundary
2. **Stateless tools** — operations that complete in a single call (`sleep`,
   file I/O, `process_run`, …) return their results directly with no server-side
   handle
3. **Emit-only tools** — runtime construction primitives (`build_current_thread`,
   `block_on`, `spawn`, …) that cannot execute inside an existing tokio runtime
   but participate in `emit_binary` composition to generate correct `main`
   scaffolding

---

## Quick start

```toml
[dependencies]
elicit_tokio = { version = "0.9" }
rmcp = { version = "0.1", features = ["server"] }
tokio = { version = "1", features = ["full"] }
```

```rust,no_run
use elicitation::PluginRegistry;
use elicit_tokio::{
    TokioTimePlugin, TokioSyncPlugin, TokioFsPlugin, TokioNetPlugin,
    TokioChannelPlugin, TokioProcessPlugin, TokioSignalPlugin,
    TokioRuntimePlugin, TokioTaskPlugin,
};

let registry = PluginRegistry::new()
    .register("tokio_time",    TokioTimePlugin::new())
    .register("tokio_sync",    TokioSyncPlugin::new())
    .register("tokio_fs",      TokioFsPlugin)
    .register("tokio_net",     TokioNetPlugin::new())
    .register("tokio_channel", TokioChannelPlugin::new())
    .register("tokio_process", TokioProcessPlugin::new())
    .register("tokio_signal",  TokioSignalPlugin)
    .register("tokio_runtime", TokioRuntimePlugin)
    .register("tokio_task",    TokioTaskPlugin);

// Time tools:    tokio_time__sleep, tokio_time__timeout_create, …
// Sync tools:    tokio_sync__semaphore_new, tokio_sync__barrier_wait, …
// Channel tools: tokio_channel__mpsc_create, tokio_channel__watch_send, …
// Net tools:     tokio_net__tcp_listener_bind, tokio_net__tcp_stream_connect, …
// Fs tools:      tokio_fs__read_to_string, tokio_fs__write_text, …
```

---

## Plugin inventory

| Plugin | Namespace | Tool count | Notes |
|---|---|---|---|
| [`TokioTimePlugin`] | `tokio_time__*` | 7 | Sleep, timeouts, intervals |
| [`TokioSyncPlugin`] | `tokio_sync__*` | 12 | Semaphores, notifications, barriers |
| [`TokioRuntimePlugin`] | `tokio_runtime__*` | 4 | 1 runtime, 3 emit-only |
| [`TokioFsPlugin`] | `tokio_fs__*` | 14 | Stateless file system ops |
| [`TokioNetPlugin`] | `tokio_net__*` | 15 | TCP listeners, TCP/UDP sockets |
| [`TokioProcessPlugin`] | `tokio_process__*` | 9 | One-shot run + interactive child |
| [`TokioTaskPlugin`] | `tokio_task__*` | 4 | 1 runtime, 3 emit-only |
| [`TokioSpawnPlugin`] | `tokio_spawn__*` | 3 + N | Factory: 3 fixed + 1 per workload |
| [`TokioChannelPlugin`] | `tokio_channel__*` | 37 | mpsc, oneshot, watch, broadcast, mutex, rwlock |
| [`TokioSignalPlugin`] | `tokio_signal__*` | 4 | Ctrl+C + 3 unix-only |
| [`TokioIoPlugin`] | `tokio_io__*` | 4 | In-memory duplex streams |
| [`TokioIoCopyPlugin`] | `tokio_io_copy__*` | N | Factory: 1 per `(R, W)` pair |
| [`TokioUnixPlugin`] | `tokio_unix__*` | 15 | **Unix only** — Unix domain sockets |

---

## How `elicit_tokio` satisfies the shadow crate motivation

The shadow crate motivation is stated in `SHADOW_CRATE_MOTIVATION.md`. The core
idea is:

> *Define a vocabulary of atomic, verified operations — let an agent compose
> them into tool chains — the tool chain **is** the method.*

Async Rust is among the hardest domains to verify after the fact: futures are
opaque types, runtimes are implicit, and the composition of async operations
spans many abstraction layers. `elicit_tokio` attacks this from the vocabulary
end: by making every tokio primitive an explicitly typed, MCP-callable tool with
a machine-checkable contract, agent-composed async workflows inherit those
contracts structurally.

### The server-side registry pattern

Tokio objects — `JoinHandle`, `Semaphore`, `TcpStream`, `Sender`, child
processes — are `!Send` or non-serializable. They cannot cross the MCP JSON
boundary. The solution is the **server-side registry**:

```text
Agent call: { "semaphore_id": "3f7a…", "permits": 3 }
                    │
                    ▼
  TokioSyncPlugin { semaphores: Arc<Mutex<HashMap<Uuid, Arc<Semaphore>>>> }
                    │
                    ▼  resolves UUID → Arc<Semaphore> → .acquire().await
                    │
Agent result: { "permit_id": "9c2b…", "available_permits": 2 }
```

Every stateful plugin holds one or more `Arc<Mutex<HashMap<Uuid, T>>>` registries.
Handles returned to agents are UUIDs. This pattern recurs across every stateful
plugin in this crate.

### Why UUIDs instead of indices

Integer indices would alias across plugin lifetimes and expose internal ordering.
UUIDs are collision-resistant, opaque, and survive serialization through any
intermediary without ambiguity.

---

## Time tools (`TokioTimePlugin`)

| Tool | Params | Returns | Establishes |
|---|---|---|---|
| `sleep` | `duration_ms: u64` | `{ elapsed_ms }` | `SleepCompleted` |
| `sleep_until` | `deadline_unix_ms: u64` | `{ elapsed_ms }` | `SleepCompleted` |
| `timeout_create` | `duration_ms: u64` | `{ timeout_id }` | — |
| `timeout_check` | `timeout_id` | `{ elapsed_ms, remaining_ms, expired }` | — |
| `timeout_await` | `timeout_id` | `{ elapsed_ms, expired }` | `TimeoutResolved` |
| `interval_create` | `period_ms: u64` | `{ interval_id }` | — |
| `interval_tick` | `interval_id` | `{ tick_number, elapsed_ms }` | — |

Timeout and interval objects live in server-side registries keyed by UUID.
`sleep` and `sleep_until` are one-shot: they block until the duration elapses,
then return. For recurring work, create an interval once and tick it repeatedly.

---

## Sync tools (`TokioSyncPlugin`)

| Category | Tool | Params | Returns | Establishes |
|---|---|---|---|---|
| Semaphore | `semaphore_new` | `permits: u32` | `{ semaphore_id }` | — |
| Semaphore | `semaphore_acquire` | `semaphore_id` | `{ permit_id, available_permits }` | `PermitAcquired` |
| Semaphore | `semaphore_try_acquire` | `semaphore_id` | `{ permit_id?, acquired, available_permits }` | `PermitAcquired` |
| Semaphore | `semaphore_release` | `permit_id` | `{ ok }` | — |
| Semaphore | `semaphore_available` | `semaphore_id` | `{ available_permits }` | — |
| Semaphore | `semaphore_close` | `semaphore_id` | `{ ok }` | — |
| Notify | `notify_new` | — | `{ notify_id }` | — |
| Notify | `notify_one` | `notify_id` | `{ ok }` | — |
| Notify | `notify_waiters` | `notify_id` | `{ ok }` | — |
| Notify | `notified` | `notify_id` | `{ ok }` | `NotificationReceived` |
| Barrier | `barrier_new` | `count: usize` | `{ barrier_id }` | — |
| Barrier | `barrier_wait` | `barrier_id` | `{ is_leader }` | `BarrierReached` |

`OwnedSemaphorePermit` is not serializable, so acquired permits are held
server-side under their own UUID. `semaphore_release` looks up the permit and
drops it, returning the permit to the semaphore.

---

## Runtime tools (`TokioRuntimePlugin`)

| Tool | Params | Returns | Notes |
|---|---|---|---|
| `inspect_flavor` | — | `{ flavor }` | Introspects the current runtime |
| `build_current_thread` | `enable_all, max_blocking_threads?` | error at runtime | Emit-only |
| `build_multi_thread` | `worker_threads?, enable_all, max_blocking_threads?` | error at runtime | Emit-only |
| `block_on` | `runtime_var, body` | error at runtime | Emit-only |

**Emit-only tools** report an error if called at runtime — you cannot build a
new tokio runtime from within an existing one. Their purpose is code generation:
when an agent calls `emit_binary`, the scaffold is enriched with correct runtime
construction boilerplate derived from these tool calls.

---

## File system tools (`TokioFsPlugin`)

| Tool | Params | Returns |
|---|---|---|
| `read_to_string` | `path` | `{ content }` |
| `read_bytes` | `path` | `{ bytes: [u8], len }` |
| `write_text` | `path, content` | `{ bytes_written }` |
| `write_bytes` | `path, bytes: [u8]` | `{ bytes_written }` |
| `create_dir` | `path` | `{ ok }` |
| `create_dir_all` | `path` | `{ ok }` |
| `remove_dir` | `path` | `{ ok }` |
| `remove_dir_all` | `path` | `{ ok }` |
| `remove_file` | `path` | `{ ok }` |
| `rename` | `from, to` | `{ ok }` |
| `copy` | `from, to` | `{ bytes_copied }` |
| `metadata` | `path` | `{ size, is_file, is_dir, is_symlink, readonly, modified_unix_ms? }` |
| `read_dir` | `path` | `{ entries: [{name, is_file, is_dir, is_symlink}] }` |
| `canonicalize` | `path` | `{ canonical_path }` |

All fs tools are stateless — no handles are stored server-side.

---

## Network tools (`TokioNetPlugin`)

### TCP

| Tool | Params | Returns | Establishes |
|---|---|---|---|
| `tcp_listener_bind` | `addr` | `{ listener_id, local_addr }` | `ListenerBound` |
| `tcp_listener_accept` | `listener_id` | `{ stream_id, peer_addr }` | `ConnectionAccepted` |
| `tcp_listener_local_addr` | `listener_id` | `{ addr }` | — |
| `tcp_listener_close` | `listener_id` | `{ ok }` | — |
| `tcp_stream_connect` | `addr` | `{ stream_id, local_addr, peer_addr }` | `StreamConnected` |
| `tcp_stream_read` | `stream_id, max_bytes?` | `{ data, bytes_read, eof }` | — |
| `tcp_stream_write` | `stream_id, data` | `{ bytes_written }` | — |
| `tcp_stream_local_addr` | `stream_id` | `{ addr }` | — |
| `tcp_stream_peer_addr` | `stream_id` | `{ addr }` | — |
| `tcp_stream_close` | `stream_id` | `{ ok }` | — |

### UDP

| Tool | Params | Returns | Establishes |
|---|---|---|---|
| `udp_socket_bind` | `addr` | `{ socket_id, local_addr }` | `ListenerBound` |
| `udp_socket_send_to` | `socket_id, data, addr` | `{ bytes_sent }` | — |
| `udp_socket_recv_from` | `socket_id, max_bytes?` | `{ data, bytes_received, from_addr }` | `DataReceived` |
| `udp_socket_local_addr` | `socket_id` | `{ addr }` | — |
| `udp_socket_close` | `socket_id` | `{ ok }` | — |

Listener and socket objects live in server-side registries keyed by UUID.
Read/write operations return owned byte vectors so data can be inspected or
passed to subsequent tool calls.

---

## Process tools (`TokioProcessPlugin`)

| Tool | Params | Returns | Notes |
|---|---|---|---|
| `process_run` | `program, args?, stdin_bytes?, env?, cwd?` | `{ stdout_bytes, stderr_bytes, exit_code, success }` | Run-to-completion; no registry entry |
| `process_spawn` | `program, args?, env?, cwd?, pipe_stdin, pipe_stdout, pipe_stderr` | `{ child_id, pid }` | Background child |
| `process_stdin_write` | `child_id, data` | `{ bytes_written }` | Requires `pipe_stdin = true` |
| `process_stdout_read` | `child_id, max_bytes?` | `{ data, bytes_read, eof }` | Requires `pipe_stdout = true` |
| `process_stderr_read` | `child_id, max_bytes?` | `{ data, bytes_read, eof }` | Requires `pipe_stderr = true` |
| `process_wait` | `child_id` | `{ exit_code, success }` | Blocks until exit |
| `process_try_wait` | `child_id` | `{ exited, exit_code?, success? }` | Non-blocking poll |
| `process_kill` | `child_id` | `{ ok }` | SIGKILL (unix) / TerminateProcess (windows) |
| `process_id` | `child_id` | `{ pid }` | OS process ID |

`process_run` is the ergonomic one-shot path for programs that don't need
interactive I/O. `process_spawn` starts a background child whose stdin/stdout/
stderr are accessible via separate tool calls — suitable for long-running or
interactive processes.

---

## Task tools (`TokioTaskPlugin`)

| Tool | Params | Returns | Notes |
|---|---|---|---|
| `yield_now` | — | `{ ok }` | Yields to the scheduler |
| `spawn` | `body` | error at runtime | Emit-only |
| `spawn_blocking` | `body` | error at runtime | Emit-only |
| `block_in_place` | `body` | error at runtime | Emit-only |

`spawn`, `spawn_blocking`, and `block_in_place` are emit-only for the same
reason as the runtime construction tools — they generate correct task-spawning
scaffolding in `emit_binary` output without executing at MCP server time.

---

## Spawn plugin (`TokioSpawnPlugin`)

Closures and futures cannot cross the MCP JSON boundary. The spawn plugin uses
a **factory pattern**: user types implement `BlockingWorkload` or `AsyncWorkload`,
then register with the plugin builder. The plugin dynamically generates one MCP
tool per registered type.

```rust,no_run
use elicit_tokio::{BlockingWorkload, TokioSpawnPlugin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
struct CompressParams { data: Vec<u8>, level: u32 }

#[derive(Serialize)]
struct CompressOutput { compressed: Vec<u8> }

impl BlockingWorkload for CompressParams {
    type Output = CompressOutput;
    fn execute(self) -> CompressOutput {
        // … compression logic …
        CompressOutput { compressed: vec![] }
    }
}

let plugin = TokioSpawnPlugin::builder()
    .register_blocking::<CompressParams>("compress", "Compress data in a blocking thread")
    .build();
```

### Tool surface

**Companion tools (always present):**

| Tool | Params | Returns |
|---|---|---|
| `tokio_spawn__join` | `handle_id` | `{ output }` — awaits completion |
| `tokio_spawn__try_join` | `handle_id` | `{ output?, ready }` — non-blocking poll |
| `tokio_spawn__abort` | `handle_id` | `{ ok }` — cancel the task |

**Dynamic workload tools (one per registered type):**

| Tool | Params | Returns |
|---|---|---|
| `tokio_spawn__<name>` | `T`'s fields | `{ handle_id }` |

---

## Channel tools (`TokioChannelPlugin`)

All channel endpoints and shared-state objects live server-side. Values are
`serde_json::Value` so agents can pass arbitrary JSON through channels.

### mpsc (multi-producer, single-consumer)

| Tool | Params | Returns |
|---|---|---|
| `mpsc_create` | `capacity` | `{ sender_id, receiver_id }` |
| `mpsc_send` | `sender_id, value` | `{ ok }` |
| `mpsc_try_send` | `sender_id, value` | `{ ok, full, closed }` |
| `mpsc_recv` | `receiver_id` | `{ value?, closed }` |
| `mpsc_try_recv` | `receiver_id` | `{ value?, empty, closed }` |
| `mpsc_sender_close` | `sender_id` | `{ ok }` |
| `mpsc_receiver_close` | `receiver_id` | `{ ok }` |

### oneshot (single send, single receive)

| Tool | Params | Returns |
|---|---|---|
| `oneshot_create` | — | `{ sender_id, receiver_id }` |
| `oneshot_send` | `sender_id, value` | `{ ok }` |
| `oneshot_recv` | `receiver_id` | `{ value?, closed }` |
| `oneshot_try_recv` | `receiver_id` | `{ value?, empty, closed }` |

### watch (single-value broadcast)

| Tool | Params | Returns |
|---|---|---|
| `watch_create` | `initial_value` | `{ sender_id, receiver_id }` |
| `watch_send` | `sender_id, value` | `{ ok }` |
| `watch_borrow` | `receiver_id` | `{ value }` |
| `watch_changed` | `receiver_id` | `{ ok }` |
| `watch_subscribe` | `sender_id` | `{ receiver_id }` |
| `watch_sender_close` | `sender_id` | `{ ok }` |
| `watch_receiver_close` | `receiver_id` | `{ ok }` |

### broadcast (multi-producer, multi-consumer)

| Tool | Params | Returns |
|---|---|---|
| `broadcast_create` | `capacity` | `{ sender_id, receiver_id }` |
| `broadcast_send` | `sender_id, value` | `{ receivers_count }` |
| `broadcast_recv` | `receiver_id` | `{ value?, closed }` |
| `broadcast_try_recv` | `receiver_id` | `{ value?, empty, lagged, closed }` |
| `broadcast_subscribe` | `sender_id` | `{ receiver_id }` |
| `broadcast_sender_close` | `sender_id` | `{ ok }` |
| `broadcast_receiver_close` | `receiver_id` | `{ ok }` |

### Mutex\<Value\> and RwLock\<Value\>

A JSON-value mutex and rwlock for shared mutable state across tool calls.

| Tool | Params | Returns |
|---|---|---|
| `mutex_create` | `value?` | `{ mutex_id }` |
| `mutex_lock` | `mutex_id` | `{ value }` |
| `mutex_update` | `mutex_id, value` | `{ old_value }` |
| `mutex_try_lock` | `mutex_id` | `{ value?, acquired }` |
| `mutex_close` | `mutex_id` | `{ ok }` |
| `rwlock_create` | `value?` | `{ rwlock_id }` |
| `rwlock_read` | `rwlock_id` | `{ value }` |
| `rwlock_write` | `rwlock_id, value` | `{ old_value }` |
| `rwlock_try_read` | `rwlock_id` | `{ value?, acquired }` |
| `rwlock_try_write` | `rwlock_id, value` | `{ old_value?, acquired }` |
| `rwlock_close` | `rwlock_id` | `{ ok }` |

---

## Signal tools (`TokioSignalPlugin`)

| Tool | Params | Returns | Platform |
|---|---|---|---|
| `ctrl_c` | — | `{ ok }` | All |
| `unix_signal_create` | `kind: UnixSignalKind` | `{ signal_id }` | Unix only |
| `unix_signal_recv` | `signal_id` | `{ ok }` | Unix only |
| `unix_signal_close` | `signal_id` | `{ ok }` | Unix only |

`ctrl_c` waits for Ctrl+C and is the cross-platform signal primitive. Unix
signal handling uses a server-side registry of `Signal` objects keyed by UUID.
`UnixSignalKind` is a serializable enum covering `hangup`, `interrupt`, `quit`,
`terminate`, `user_defined1`, `user_defined2`, `pipe`, `alarm`, `child`, `io`,
and `window_change`.

Unix-specific types and tools are cfg-gated: `UnixSignalKind`, `unix_signal_*`
tools, and `TokioUnixPlugin` are absent on Windows.

---

## I/O tools (`TokioIoPlugin`)

| Tool | Params | Returns |
|---|---|---|
| `duplex_create` | `max_buf_size?` | `{ read_id, write_id }` |
| `duplex_read` | `read_id, max_bytes?` | `{ data, bytes_read, eof }` |
| `duplex_write` | `write_id, data` | `{ bytes_written }` |
| `duplex_close` | `read_id or write_id` | `{ ok }` |

`tokio::io::duplex` creates an in-memory bidirectional byte pipe — useful for
testing or for connecting components that expect async reader/writer handles.
Both ends of the duplex are stored server-side under separate UUIDs.

---

## I/O copy plugin (`TokioIoCopyPlugin`)

`tokio::io::copy` requires mutable access to both a reader and a writer
simultaneously — impossible without knowing the concrete types at plugin
construction time. The factory pattern solves this by capturing both plugin
registries at construction time.

```rust,no_run
use elicit_tokio::{TokioIoCopyPlugin, TokioNetPlugin, TokioIoPlugin};
use tokio::net::TcpStream;
use tokio::io::DuplexStream;

let net = TokioNetPlugin::new();
let io  = TokioIoPlugin::new();

let copy_plugin = TokioIoCopyPlugin::builder()
    .register::<TcpStream, DuplexStream>(
        "tcp_to_duplex",
        "Copy bytes from a TCP stream into a duplex pipe",
        net.tcp_stream_registry(),
        io.duplex_stream_registry(),
    )
    .build();
// Produces: tokio_io_copy__tcp_to_duplex { reader_id, writer_id } → { bytes_copied }
```

---

## Unix domain socket tools (`TokioUnixPlugin`)

**Unix only.** Absent on Windows.

### UnixListener

| Tool | Params | Returns |
|---|---|---|
| `unix_listener_bind` | `path` | `{ listener_id, local_path }` |
| `unix_listener_accept` | `listener_id` | `{ stream_id, peer_path? }` |
| `unix_listener_local_addr` | `listener_id` | `{ path? }` |
| `unix_listener_close` | `listener_id` | `{ ok }` |

### UnixStream

| Tool | Params | Returns |
|---|---|---|
| `unix_stream_connect` | `path` | `{ stream_id, local_path?, peer_path? }` |
| `unix_stream_read` | `stream_id, max_bytes?` | `{ data, bytes_read, eof }` |
| `unix_stream_write` | `stream_id, data` | `{ bytes_written }` |
| `unix_stream_local_addr` | `stream_id` | `{ path? }` |
| `unix_stream_peer_addr` | `stream_id` | `{ path? }` |
| `unix_stream_close` | `stream_id` | `{ ok }` |

### UnixDatagram

| Tool | Params | Returns |
|---|---|---|
| `unix_datagram_bind` | `path` | `{ socket_id, local_path? }` |
| `unix_datagram_send_to` | `socket_id, data, path` | `{ bytes_sent }` |
| `unix_datagram_recv_from` | `socket_id, max_bytes?` | `{ data, bytes_received, from_path? }` |
| `unix_datagram_local_addr` | `socket_id` | `{ path? }` |
| `unix_datagram_close` | `socket_id` | `{ ok }` |

---

## Propositions and contracts

Each tool that completes a meaningful async operation establishes a **proposition** —
a zero-sized `PhantomData` proof marker that records what the tool guarantees:

| Proposition | Plugin | Established by |
|---|---|---|
| `SleepCompleted` | Time | `sleep`, `sleep_until` |
| `TimeoutResolved` | Time | `timeout_await` |
| `PermitAcquired` | Sync | `semaphore_acquire`, `semaphore_try_acquire` |
| `NotificationReceived` | Sync | `notified` |
| `BarrierReached` | Sync | `barrier_wait` |
| `RuntimeFlavored` | Runtime | `inspect_flavor` |
| `FileRead` | Fs | `read_to_string`, `read_bytes` |
| `FileWritten` | Fs | `write_text`, `write_bytes` |
| `ListenerBound` | Net | `tcp_listener_bind`, `udp_socket_bind` |
| `ConnectionAccepted` | Net | `tcp_listener_accept` |
| `StreamConnected` | Net | `tcp_stream_connect` |
| `DataReceived` | Net | `udp_socket_recv_from` |
| `ProcessSpawned` | Process | `process_spawn` |
| `ProcessExited` | Process | `process_wait`, `process_try_wait` |
| `TaskSpawned` | Spawn | workload tools |
| `TaskJoined` | Spawn | `join` |
| `TaskAborted` | Spawn | `abort` |
| `MessageSent` | Channel | `mpsc_send`, `broadcast_send`, `oneshot_send` |
| `MessageReceived` | Channel | `mpsc_recv`, `broadcast_recv`, `oneshot_recv` |
| `ChannelClosed` | Channel | `*_close` tools |
| `TaskYielded` | Task | `yield_now` |
| `CtrlCReceived` | Signal | `ctrl_c` |
| `SignalHandlerRegistered` | Signal (unix) | `unix_signal_create` |
| `SignalReceived` | Signal (unix) | `unix_signal_recv` |
| `DuplexCreated` | Io | `duplex_create` |
| `BytesCopied` | IoCopy | `tokio_io_copy__*` |
| `UnixListenerBound` | Unix (unix) | `unix_listener_bind` |
| `UnixConnectionAccepted` | Unix (unix) | `unix_listener_accept` |
| `UnixStreamConnected` | Unix (unix) | `unix_stream_connect` |
| `UnixDataReceived` | Unix (unix) | `unix_datagram_recv_from` |

Propositions compose with `And<P, Q>` and `both(p, q)`, allowing agent tool
chains to carry cumulative proof state forward across steps.

---

## What we chose not to shadow, and why

### `JoinHandle<T>` as an open generic

`JoinHandle<T>` is generic over the output type, which belongs to the user's
binary. `TokioSpawnPlugin` handles this through the workload factory pattern:
the `T` is resolved at plugin construction time, not at the MCP boundary. The
`join` companion tool returns the output as `serde_json::Value`, erasing the
type at the boundary while preserving the value.

### `Runtime` and `Builder`

`tokio::runtime::Runtime` cannot be constructed inside an existing async context
— `Builder::build()` panics if called from within a tokio runtime. We expose
`inspect_flavor` for introspection and provide emit-only tools for the builder
methods. This matches the actual usage pattern: runtime construction belongs in
`main`, not in a handler running inside a runtime.

### `AsyncRead` / `AsyncWrite` as traits

These are object-unsafe generics — there is no `Box<dyn AsyncRead>` that works
universally with tokio's combinators. We work with concrete types (`TcpStream`,
`DuplexStream`, Unix stream variants) and provide `TokioIoCopyPlugin` as the
factory for cross-type copy operations when both ends are known at plugin
construction time.

### `Mutex<T>` as an open generic

`tokio::sync::Mutex<T>` is generic over the protected value. We expose a
`TokioChannelPlugin` mutex that holds `serde_json::Value` — a late-binding
approach that gives agents a fully functional shared-state primitive without
requiring the type `T` to be known at plugin construction time.

---

## Feature flags

- `emit` — enables `EmitCode` code-recovery support for emit-only tools

[`tokio`]: https://docs.rs/tokio
[`elicitation`]: https://docs.rs/elicitation
[`rmcp`]: https://docs.rs/rmcp
