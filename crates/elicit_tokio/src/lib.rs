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
//!
//! # Feature flags
//!
//! - `emit` — enables `EmitCode` code-recovery support.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod time;

pub use time::{
    IntervalCreateParams, IntervalTickParams, SleepCompleted, SleepParams, SleepUntilParams,
    TimeoutAwaitParams, TimeoutCheckParams, TimeoutCreateParams, TimeoutResolved, TokioTimePlugin,
};
