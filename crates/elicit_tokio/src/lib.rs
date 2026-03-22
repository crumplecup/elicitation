//! Elicitation-enabled tokio shadow crate.
//!
//! Provides MCP tools for async time management, sync primitives, and runtime
//! inspection. All tokio runtime objects are stored server-side in UUID-keyed
//! registries — only serializable handles (UUIDs, durations, booleans) cross
//! the MCP boundary.
//!
//! # Shadow-crate workflow plugins
//!
//! | Plugin | Namespace | Tools |
//! |---|---|---|
//! | [`TokioTimePlugin`] | `tokio_time__*` | sleep, sleep_until, timeout_create, timeout_check, timeout_await, interval_create, interval_tick |
//! | [`TokioSyncPlugin`] | `tokio_sync__*` | semaphore_new, semaphore_acquire, semaphore_try_acquire, semaphore_release, semaphore_available, semaphore_close, notify_new, notify_one, notify_waiters, notified, barrier_new, barrier_wait |
//! | [`TokioRuntimePlugin`] | `tokio_runtime__*` | inspect_flavor |
//! | [`TokioFsPlugin`] | `tokio_fs__*` | read_to_string, read_bytes, write_text, write_bytes, create_dir, create_dir_all, remove_dir, remove_dir_all, remove_file, rename, copy, metadata, read_dir, canonicalize |
//!
//! # Feature flags
//!
//! - `emit` — enables `EmitCode` code-recovery support.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod fs;
mod runtime;
mod sync;
mod time;

pub use fs::{
    FromToParams, PathParams, ReadBytesParams, ReadToStringParams, TokioFsPlugin, WriteBytesParams,
    WriteTextParams,
};
pub use runtime::{InspectFlavorParams, RuntimeFlavorKind, TokioRuntimePlugin};
pub use sync::{
    BarrierNewParams, BarrierReached, BarrierWaitParams, NotificationReceived, NotifiedParams,
    NotifyNewParams, NotifyOneParams, NotifyWaitersParams, PermitAcquired, SemaphoreAcquireParams,
    SemaphoreAvailableParams, SemaphoreCloseParams, SemaphoreNewParams, SemaphoreReleaseParams,
    SemaphoreTryAcquireParams, TokioSyncPlugin,
};
pub use time::{
    IntervalCreateParams, IntervalTickParams, SleepCompleted, SleepParams, SleepUntilParams,
    TimeoutAwaitParams, TimeoutCheckParams, TimeoutCreateParams, TimeoutResolved, TokioTimePlugin,
};
