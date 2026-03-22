//! Cross-crate workflow plugins for elicitation.
//!
//! `elicit_server` houses workflows that require visibility across multiple
//! elicitation crates simultaneously — things that can't live in `elicit_reqwest`
//! or `elicit_serde_json` without creating circular dependencies.
//!
//! # Plugins
//!
//! - [`EmitBinaryPlugin`] — recover agent tool compositions as compiled Rust
//!   binaries (requires `feature = "emit"`)
//! - [`SecureFetchPlugin`] — HTTPS-enforced URL validation + HTTP fetch
//!   (`elicit_url` + `elicit_reqwest`)
//! - [`FetchAndParsePlugin`] — HTTP fetch + JSON extraction
//!   (`elicit_reqwest` + `elicit_serde_json`)
//!
//! # Shadow-crate workflow plugins (registered via emit inventory)
//!
//! The following plugins live in their own crates but are included in the emit
//! dispatch pipeline when the `emit` feature is enabled:
//!
//! | Plugin | Crate | Tools |
//! |---|---|---|
//! | `ChronoWorkflowPlugin` | `elicit_chrono` | parse_datetime, assert_future, assert_in_range, compute_duration, add_seconds |
//! | `JiffWorkflowPlugin` | `elicit_jiff` | parse_timestamp, assert_future, assert_in_range, add_seconds |
//! | `TimeWorkflowPlugin` | `elicit_time` | parse_offset, assert_utc, format_offset |
//! | `UrlWorkflowPlugin` | `elicit_url` | parse_url, assert_https, assert_host |
//! | `ReqwestWorkflowPlugin` | `elicit_reqwest` | fetch_json, fetch_text, url_build, build_request, status_summary, assert_success, assert_json, head_request, fetch_bytes |
//! | `JsonWorkflowPlugin` | `elicit_serde_json` | parse_and_focus, validate_object, safe_merge, pointer_update, field_chain |
//! | `RegexWorkflowPlugin` | `elicit_regex` | compile, is_match, find_all, replace_all, capture_groups |
//! | `SqlxWorkflowPlugin` | `elicit_sqlx` | connect, query, execute, query_typed, transaction |
//! | `TokioTimePlugin` | `elicit_tokio` | sleep, sleep_until, timeout_create, timeout_check, timeout_await, interval_create, interval_tick |
//! | `TokioSyncPlugin` | `elicit_tokio` | semaphore_new, semaphore_acquire, semaphore_try_acquire, semaphore_release, semaphore_available, semaphore_close, notify_new, notify_one, notify_waiters, notified, barrier_new, barrier_wait |
//! | `TokioRuntimePlugin` | `elicit_tokio` | inspect_flavor |
//!
//! # Feature flags
//!
//! | Feature | Enables |
//! |---|---|
//! | `emit` | `EmitBinaryPlugin` + full code recovery pipeline |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod fetch_and_parse;
mod secure_fetch;
mod util;

#[cfg(feature = "emit")]
mod emit_plugin;

pub use fetch_and_parse::FetchAndParsePlugin;
pub use secure_fetch::SecureFetchPlugin;

#[cfg(feature = "emit")]
pub use emit_plugin::{EmitBinaryParams, EmitBinaryPlugin, WorkflowStep};

/// Look up a tool by name and deserialize its params, drawing from all
/// `elicit_server` handlers and its dep crates registered via `#[elicit_tool]`.
///
/// This is a thin wrapper around [`elicitation::emit_code::dispatch_emit`]
/// that anchors the linker to include every handler module in this crate and
/// its workflow dependencies, ensuring their `register_emit!` constructors are
/// present in the final binary.  Integration tests must call this function
/// (not the bare `dispatch_emit`) so the linker does not discard unreferenced
/// CGUs.
#[cfg(feature = "emit")]
pub fn emit_dispatch(
    tool: &str,
    params: serde_json::Value,
) -> Result<Box<dyn elicitation::emit_code::EmitCode>, String> {
    emit_dispatch_crate(tool, "", params)
}

/// Dispatch to a specific crate's emit registration by crate name.
///
/// Use this when multiple crates register the same tool name (e.g.
/// `"assert_future"` in `elicit_chrono`, `elicit_jiff`, `elicit_time`).
#[cfg(feature = "emit")]
pub fn emit_dispatch_crate(
    tool: &str,
    crate_name: &str,
    params: serde_json::Value,
) -> Result<Box<dyn elicitation::emit_code::EmitCode>, String> {
    // Each size_of call references a params type from a handler module,
    // pulling that CGU (and its register_emit! CTORs) into the link.
    let _ = [
        // elicit_server
        std::mem::size_of::<secure_fetch::SecureFetchParams>(),
        std::mem::size_of::<secure_fetch::ValidatedApiCallParams>(),
        std::mem::size_of::<fetch_and_parse::FetchAndExtractParams>(),
        std::mem::size_of::<fetch_and_parse::FetchAndValidateParams>(),
        // elicit_url
        std::mem::size_of::<elicit_url::ParseUrlParams>(),
        // elicit_reqwest
        std::mem::size_of::<elicit_reqwest::BuildRequestParams>(),
        // elicit_chrono
        std::mem::size_of::<elicit_chrono::ParseDateTimeParams>(),
        // elicit_jiff
        std::mem::size_of::<elicit_jiff::ParseTimestampParams>(),
        // elicit_time
        std::mem::size_of::<elicit_time::ParseOffsetParams>(),
        // elicit_serde_json
        std::mem::size_of::<elicit_serde_json::ParseAndFocusEmit>(),
        // elicit_sqlx (workflow emit newtypes)
        std::mem::size_of::<elicit_sqlx::workflow::WfConnectParams>(),
        // elicit_regex
        std::mem::size_of::<elicit_regex::CompileParams>(),
        // elicit_tokio
        std::mem::size_of::<elicit_tokio::SleepParams>(),
        std::mem::size_of::<elicit_tokio::SemaphoreNewParams>(),
        std::mem::size_of::<elicit_tokio::InspectFlavorParams>(),
    ];
    if crate_name.is_empty() {
        elicitation::emit_code::dispatch_emit(tool, params)
    } else {
        elicitation::emit_code::dispatch_emit_from(tool, crate_name, params)
    }
}
